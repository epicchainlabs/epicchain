// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// modifications are permitted.

//! Transaction serialization implementation matching C# Neo N3 exactly.

use crate::signer::Signer;
use crate::witness::Witness;
use neo_io::serializable::helper::get_var_size;
use neo_io::Serializable;
use std::sync::Mutex;

use super::attributes::TransactionAttribute;
use super::core::{Transaction, HEADER_SIZE, MAX_TRANSACTION_ATTRIBUTES};

impl Serializable for Transaction {
    fn size(&self) -> usize {
        let mut size = HEADER_SIZE;

        // Signers size
        size += get_var_size(self.signers.len() as u64);
        for signer in &self.signers {
            size += signer.size();
        }

        // Attributes size
        size += get_var_size(self.attributes.len() as u64);
        for attribute in &self.attributes {
            size += attribute.size();
        }

        // Script size
        size += get_var_size(self.script.len() as u64) + self.script.len();

        // Witnesses size
        size += get_var_size(self.witnesses.len() as u64);
        for witness in &self.witnesses {
            size += witness.size();
        }

        size
    }

    fn serialize(&self, writer: &mut neo_io::BinaryWriter) -> neo_io::IoResult<()> {
        // Write header
        writer.write_bytes(&[self.version])?;
        writer.write_bytes(&self.nonce.to_le_bytes())?;
        writer.write_bytes(&self.system_fee.to_le_bytes())?;
        writer.write_bytes(&self.network_fee.to_le_bytes())?;
        writer.write_bytes(&self.valid_until_block.to_le_bytes())?;

        // Write signers
        writer.write_var_int(self.signers.len() as u64)?;
        for signer in &self.signers {
            Serializable::serialize(signer, writer)?;
        }

        // Write attributes
        writer.write_var_int(self.attributes.len() as u64)?;
        for attribute in &self.attributes {
            attribute
                .serialize(writer)
                .map_err(|e| neo_io::IoError::InvalidData {
                    context: "attribute".to_string(),
                    value: e.to_string(),
                })?;
        }

        // Write script
        writer.write_var_bytes(&self.script)?;

        // Write witnesses
        writer.write_var_int(self.witnesses.len() as u64)?;
        for witness in &self.witnesses {
            Serializable::serialize(witness, writer)?;
        }

        Ok(())
    }

    fn deserialize(reader: &mut neo_io::MemoryReader) -> neo_io::IoResult<Self> {
        // Read header
        let version = reader.read_byte()?;
        if version > 0 {
            return Err(neo_io::IoError::InvalidData {
                context: "version".to_string(),
                value: version.to_string(),
            });
        }

        let nonce = reader.read_u32()?;
        let system_fee = reader.read_u64()? as i64;
        let network_fee = reader.read_u64()? as i64;
        let valid_until_block = reader.read_u32()?;

        if system_fee < 0 {
            return Err(neo_io::IoError::InvalidData {
                context: "system_fee".to_string(),
                value: system_fee.to_string(),
            });
        }

        if network_fee < 0 {
            return Err(neo_io::IoError::InvalidData {
                context: "network_fee".to_string(),
                value: network_fee.to_string(),
            });
        }

        // Read signers
        let signer_count = reader.read_var_int(MAX_TRANSACTION_ATTRIBUTES as u64)? as usize;
        if signer_count == 0 {
            return Err(neo_io::IoError::InvalidData {
                context: "signers".to_string(),
                value: "empty".to_string(),
            });
        }
        if signer_count > MAX_TRANSACTION_ATTRIBUTES {
            return Err(neo_io::IoError::InvalidData {
                context: "signers".to_string(),
                value: format!("count {signer_count}"),
            });
        }

        let mut signers = Vec::with_capacity(signer_count);
        for _ in 0..signer_count {
            let signer = <Signer as Serializable>::deserialize(reader)?;
            signers.push(signer);
        }

        // Read attributes
        let attr_count = reader.read_var_int(MAX_TRANSACTION_ATTRIBUTES as u64)? as usize;
        if attr_count > (MAX_TRANSACTION_ATTRIBUTES - signers.len()) {
            return Err(neo_io::IoError::InvalidData {
                context: "attributes".to_string(),
                value: format!("count {attr_count}"),
            });
        }

        let mut attributes = Vec::with_capacity(attr_count);
        for _ in 0..attr_count {
            let attribute = TransactionAttribute::deserialize(reader).map_err(|e| {
                neo_io::IoError::InvalidData {
                    context: "attribute".to_string(),
                    value: e.to_string(),
                }
            })?;
            attributes.push(attribute);
        }

        // Read script
        let script = reader.read_var_bytes(u16::MAX as usize)?;
        if script.is_empty() {
            return Err(neo_io::IoError::InvalidData {
                context: "script".to_string(),
                value: "empty".to_string(),
            });
        }

        // Read witnesses
        let witness_count = reader.read_var_int(MAX_TRANSACTION_ATTRIBUTES as u64)? as usize;
        if witness_count != signers.len() {
            return Err(neo_io::IoError::InvalidData {
                context: "witness_count".to_string(),
                value: format!("{} != {}", witness_count, signers.len()),
            });
        }

        let mut witnesses = Vec::with_capacity(witness_count);
        for _ in 0..witness_count {
            let witness = <Witness as Serializable>::deserialize(reader)?;
            witnesses.push(witness);
        }

        Ok(Self {
            version,
            nonce,
            system_fee,
            network_fee,
            valid_until_block,
            signers,
            attributes,
            script,
            witnesses,
            _hash: Mutex::new(None),
            _size: Mutex::new(0),
        })
    }
}

impl Transaction {
    /// Gets the size of the transaction in bytes (matches C# Size property exactly).
    pub fn size(&self) -> usize {
        <Self as Serializable>::size(self)
    }

    /// Serializes the transaction to bytes (production-ready implementation).
    pub fn to_bytes(&self) -> neo_io::IoResult<Vec<u8>> {
        let mut writer = neo_io::BinaryWriter::new();
        self.serialize(&mut writer)?;
        Ok(writer.to_bytes())
    }

    /// Deserializes a transaction from bytes (production-ready implementation).
    pub fn from_bytes(data: &[u8]) -> neo_io::IoResult<Self> {
        let mut reader = neo_io::MemoryReader::new(data);
        Self::deserialize(&mut reader)
    }

    /// Serializes the transaction to hex string (production-ready implementation).
    pub fn to_hex(&self) -> neo_io::IoResult<String> {
        let bytes = self.to_bytes()?;
        Ok(hex::encode(bytes))
    }

    /// Deserializes a transaction from hex string (production-ready implementation).
    pub fn from_hex(hex_str: &str) -> neo_io::IoResult<Self> {
        let bytes = hex::decode(hex_str).map_err(|e| neo_io::IoError::InvalidData {
            context: "hex".to_string(),
            value: e.to_string(),
        })?;
        Self::from_bytes(&bytes)
    }

    /// Validates the transaction structure during deserialization (production-ready implementation).
    fn validate_deserialized_structure(&self) -> neo_io::IoResult<()> {
        // Check basic constraints
        if self.signers.is_empty() {
            return Err(neo_io::IoError::InvalidData {
                context: "signers".to_string(),
                value: "must have at least one signer".to_string(),
            });
        }

        if self.signers.len() > 16 {
            return Err(neo_io::IoError::InvalidData {
                context: "signers".to_string(),
                value: format!("too many signers: {}", self.signers.len()),
            });
        }

        if self.attributes.len() > MAX_TRANSACTION_ATTRIBUTES {
            return Err(neo_io::IoError::InvalidData {
                context: "attributes".to_string(),
                value: format!("too many attributes: {}", self.attributes.len()),
            });
        }

        if self.witnesses.len() != self.signers.len() {
            return Err(neo_io::IoError::InvalidData {
                context: "witnesses".to_string(),
                value: format!(
                    "witness count {} must match signer count {}",
                    self.witnesses.len(),
                    self.signers.len()
                ),
            });
        }

        if self.script.is_empty() {
            return Err(neo_io::IoError::InvalidData {
                context: "script".to_string(),
                value: "cannot be empty".to_string(),
            });
        }

        if self.script.len() > u16::MAX as usize {
            return Err(neo_io::IoError::InvalidData {
                context: "script".to_string(),
                value: format!("too large: {} bytes", self.script.len()),
            });
        }

        // Check size constraints
        let size = self.size();
        if size > super::core::MAX_TRANSACTION_SIZE {
            return Err(neo_io::IoError::InvalidData {
                context: "transaction".to_string(),
                value: format!(
                    "size {} exceeds maximum {}",
                    size,
                    super::core::MAX_TRANSACTION_SIZE
                ),
            });
        }

        Ok(())
    }

    /// Creates a transaction from serialized data with validation (production-ready implementation).
    pub fn from_bytes_validated(data: &[u8]) -> neo_io::IoResult<Self> {
        let transaction = Self::from_bytes(data)?;
        transaction.validate_deserialized_structure()?;
        Ok(transaction)
    }

    /// Creates a transaction from hex string with validation (production-ready implementation).
    pub fn from_hex_validated(hex_str: &str) -> neo_io::IoResult<Self> {
        let transaction = Self::from_hex(hex_str)?;
        transaction.validate_deserialized_structure()?;
        Ok(transaction)
    }
}
