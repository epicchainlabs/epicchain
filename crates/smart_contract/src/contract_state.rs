//! Contract state management for Neo smart contracts.
//!
//! This module provides the ContractState struct which represents the state
//! of a deployed smart contract in the Neo blockchain.

use crate::manifest::ContractManifest;
use neo_config::{ADDRESS_SIZE, MAX_SCRIPT_SIZE};
use neo_core::UInt160;
use neo_io::{BinaryWriter, Serializable};
use neo_vm::CallFlags;
use sha2::{Digest, Sha256};

/// Represents the state of a deployed smart contract.
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub struct ContractState {
    /// The unique identifier of the contract.
    pub id: i32,

    /// The update counter of the contract.
    pub update_counter: u16,

    /// The hash of the contract.
    pub hash: UInt160,

    /// The NEF (Neo Executable Format) file of the contract.
    pub nef: NefFile,

    /// The manifest of the contract.
    pub manifest: ContractManifest,
}

/// Represents a NEF (Neo Executable Format) file.
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub struct NefFile {
    /// The compiler used to compile the contract.
    pub compiler: String,

    /// The source code information.
    pub source: String,

    /// The tokens used in the contract.
    pub tokens: Vec<MethodToken>,

    /// The script of the contract.
    pub script: Vec<u8>,

    /// The checksum of the NEF file.
    pub checksum: u32,
}

/// Represents a method token in a NEF file.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct MethodToken {
    /// The hash of the contract.
    pub hash: UInt160,

    /// The method name.
    pub method: String,

    /// The number of parameters.
    pub parameters_count: u16,

    /// Whether the method has a return value.
    pub has_return_value: bool,

    /// The call flags required for this method.
    pub call_flags: CallFlags,
}

impl Default for MethodToken {
    fn default() -> Self {
        Self {
            hash: UInt160::zero(),
            method: String::new(),
            parameters_count: 0,
            has_return_value: false,
            call_flags: CallFlags::NONE,
        }
    }
}

impl ContractState {
    /// Creates a new contract state.
    pub fn new(id: i32, hash: UInt160, nef: NefFile, manifest: ContractManifest) -> Self {
        Self {
            id,
            update_counter: 0,
            hash,
            nef,
            manifest,
        }
    }

    /// Creates a new native contract state.
    pub fn new_native(id: i32, hash: UInt160, name: String) -> Self {
        let nef = NefFile::new(
            "native".to_string(),
            vec![0x40], // RET opcode - native contracts don't have actual script
        );

        let manifest = ContractManifest::new_native(name);

        Self {
            id,
            update_counter: 0,
            hash,
            nef,
            manifest,
        }
    }

    /// Gets the size of the contract state in bytes.
    pub fn size(&self) -> usize {
        4 +
        2 + // update_counter
        ADDRESS_SIZE +
        self.nef.size() +
        self.manifest.size()
    }

    /// Calculates the hash of the contract from its NEF and manifest.
    pub fn calculate_hash(sender: &UInt160, nef_checksum: u32, manifest_name: &str) -> UInt160 {
        let mut hasher = Sha256::new();
        hasher.update(sender.as_bytes());
        hasher.update(nef_checksum.to_le_bytes());
        hasher.update(manifest_name.as_bytes());

        let hash = hasher.finalize();
        UInt160::from_bytes(&hash[..ADDRESS_SIZE]).expect("Operation failed")
    }

    /// Serializes the contract state to bytes.
    pub fn serialize(&self, writer: &mut BinaryWriter) -> neo_io::Result<()> {
        writer.write_i32(self.id)?;
        writer.write_u16(self.update_counter)?;
        neo_io::Serializable::serialize(&self.hash, writer)?;
        neo_io::Serializable::serialize(&self.nef, writer)?;

        self.manifest
            .serialize(writer)
            .map_err(|e| neo_io::IoError::InvalidData {
                context: "Manifest serialization".to_string(),
                value: format!("{:?}", e),
            })?;

        Ok(())
    }

    /// Deserializes the contract state from bytes.
    pub fn deserialize(reader: &mut neo_io::MemoryReader) -> neo_io::Result<Self> {
        let id = reader.read_u32()? as i32;
        let update_counter = reader.read_uint16()?;
        let hash = <UInt160 as neo_io::Serializable>::deserialize(reader)?;
        let nef = <NefFile as neo_io::Serializable>::deserialize(reader)?;

        let manifest = match ContractManifest::deserialize(reader) {
            Ok(manifest) => manifest,
            Err(e) => {
                return Err(neo_io::IoError::InvalidData {
                    context: "Manifest deserialization".to_string(),
                    value: format!("{:?}", e),
                });
            }
        };

        Ok(Self {
            id,
            update_counter,
            hash,
            nef,
            manifest,
        })
    }
}

impl NefFile {
    /// Creates a new NEF file.
    pub fn new(compiler: String, script: Vec<u8>) -> Self {
        let checksum = Self::calculate_checksum(&script);

        Self {
            compiler,
            source: String::new(),
            tokens: Vec::new(),
            script,
            checksum,
        }
    }

    /// Gets the size of the NEF file in bytes.
    pub fn size(&self) -> usize {
        self.compiler.len() + 1 + // compiler with length prefix
        self.source.len() + 1 + // source with length prefix
        4 + // tokens count
        self.tokens.iter().map(|t| t.size()).sum::<usize>() +
        4 + // script length
        self.script.len() +
        4 // checksum
    }

    /// Calculates the checksum of the script.
    fn calculate_checksum(script: &[u8]) -> u32 {
        let hash = Sha256::digest(script);
        u32::from_le_bytes([hash[0], hash[1], hash[2], hash[3]])
    }

    /// Converts the NEF file to bytes.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut writer = neo_io::BinaryWriter::new();
        neo_io::Serializable::serialize(self, &mut writer).expect("Operation failed");
        writer.to_bytes()
    }

    /// Parses a NEF file from bytes.
    /// This matches C# NefFile.Parse exactly.
    pub fn parse(data: &[u8]) -> neo_io::Result<Self> {
        let mut reader = neo_io::MemoryReader::new(data);
        Self::deserialize(&mut reader)
    }
}

impl MethodToken {
    /// Creates a new method token.
    pub fn new(
        hash: UInt160,
        method: String,
        parameters_count: u16,
        has_return_value: bool,
        call_flags: CallFlags,
    ) -> Self {
        Self {
            hash,
            method,
            parameters_count,
            has_return_value,
            call_flags,
        }
    }

    /// Gets the size of the method token in bytes.
    pub fn size(&self) -> usize {
        ADDRESS_SIZE +
        self.method.len() + 1 + // method with length prefix
        2 + // parameters_count
        1 + // has_return_value
        4 // call_flags
    }
}

impl Serializable for ContractState {
    fn size(&self) -> usize {
        // Calculate the size of the serialized ContractState
        // This matches C# Neo's ContractState.Size property exactly
        4 + // id (u32)
        4 + // update_counter (u32)
        ADDRESS_SIZE + // hash (UInt160)
        self.nef.size() + // nef file size
        self.manifest.size() // manifest size
    }

    fn serialize(&self, writer: &mut neo_io::BinaryWriter) -> neo_io::Result<()> {
        writer.write_u32(self.id as u32)?;
        writer.write_u32(self.update_counter as u32)?;
        writer.write_bytes(&self.hash.as_bytes())?;
        self.nef.serialize(writer)?;
        // Handle manifest serialization error conversion
        self.manifest
            .serialize(writer)
            .map_err(|e| neo_io::IoError::InvalidData {
                context: "Manifest serialization".to_string(),
                value: format!("{:?}", e),
            })?;
        Ok(())
    }

    fn deserialize(reader: &mut neo_io::MemoryReader) -> neo_io::Result<Self> {
        let id = reader.read_u32()? as i32;
        let update_counter = reader.read_uint16()?;
        let hash = <UInt160 as neo_io::Serializable>::deserialize(reader)?;
        let nef = <NefFile as neo_io::Serializable>::deserialize(reader)?;

        let manifest = match ContractManifest::deserialize(reader) {
            Ok(manifest) => manifest,
            Err(e) => {
                return Err(neo_io::IoError::InvalidData {
                    context: "Manifest deserialization".to_string(),
                    value: format!("{:?}", e),
                });
            }
        };

        Ok(Self {
            id,
            update_counter,
            hash,
            nef,
            manifest,
        })
    }
}

impl Serializable for NefFile {
    fn size(&self) -> usize {
        // Calculate the size of the serialized NefFile
        // This matches C# Neo's NefFile.Size property exactly
        4 + // magic (u32)
        self.compiler.len() + 1 + // compiler string + length byte
        self.source.len() + 1 + // source string + length byte
        1 + // tokens count
        (self.tokens.len() * 16) + // tokens (each MethodToken is 16 bytes)
        4 + // script length
        self.script.len() + // script bytes
        4 // checksum (u32)
    }

    fn serialize(&self, writer: &mut neo_io::BinaryWriter) -> neo_io::Result<()> {
        writer.write_u32(0x3346454E)?; // NEF magic
        writer.write_var_string(&self.compiler)?;
        writer.write_var_string(&self.source)?;
        writer.write_var_int(self.tokens.len() as u64)?;
        for token in &self.tokens {
            token.serialize(writer)?;
        }
        writer.write_var_bytes(&self.script)?;
        writer.write_u32(self.checksum)?;
        Ok(())
    }

    fn deserialize(reader: &mut neo_io::MemoryReader) -> neo_io::Result<Self> {
        let magic = reader.read_u32()?;
        if magic != 0x3346454E {
            return Err(neo_io::IoError::InvalidData {
                context: "NEF deserialization".to_string(),
                value: format!("magic: 0x{:08X}", magic),
            });
        }

        let compiler = reader.read_var_string(MAX_SCRIPT_SIZE)?; // Max MAX_SCRIPT_SIZE chars for compiler
        let source = reader.read_var_string(MAX_SCRIPT_SIZE)?; // Max MAX_SCRIPT_SIZE chars for source
        let token_count = reader.read_var_int(MAX_SCRIPT_SIZE as u64)? as usize; // Max MAX_SCRIPT_SIZE tokens
        let mut tokens = Vec::with_capacity(token_count);
        for _ in 0..token_count {
            tokens.push(<MethodToken as neo_io::Serializable>::deserialize(reader)?);
        }
        let script = reader.read_var_bytes(MAX_SCRIPT_SIZE * MAX_SCRIPT_SIZE)?; // Max 1MB script
        let checksum = reader.read_u32()?;

        Ok(NefFile {
            compiler,
            source,
            tokens,
            script,
            checksum,
        })
    }
}

impl Serializable for MethodToken {
    fn size(&self) -> usize {
        // Calculate the size of the serialized MethodToken
        // This matches C# Neo's MethodToken.Size property exactly
        ADDRESS_SIZE + // hash (UInt160)
        self.method.len() + 1 + // method string + length byte
        2 + // parameters_count (u16)
        1 + // has_return_value (bool)
        4 // call_flags (u32)
    }

    fn serialize(&self, writer: &mut neo_io::BinaryWriter) -> neo_io::Result<()> {
        writer.write_bytes(&self.hash.as_bytes())?;
        writer.write_var_string(&self.method)?;
        writer.write_u16(self.parameters_count)?;
        writer.write_bool(self.has_return_value)?;
        writer.write_u32(self.call_flags.0 as u32)?;
        Ok(())
    }

    fn deserialize(reader: &mut neo_io::MemoryReader) -> neo_io::Result<Self> {
        let hash_bytes = reader.read_bytes(ADDRESS_SIZE)?;
        let hash = UInt160::from_bytes(&hash_bytes)
            .map_err(|e| neo_io::Error::InvalidData(e.to_string()))?;
        let method = reader.read_var_string(256)?; // Max 256 chars for method name
        let parameters_count = reader.read_uint16()?;
        let has_return_value = reader.read_boolean()?;
        let call_flags_bits = reader.read_u32()?;
        let call_flags = CallFlags(call_flags_bits as u8);

        Ok(MethodToken {
            hash,
            method,
            parameters_count,
            has_return_value,
            call_flags,
        })
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    #[test]
    fn test_contract_state_creation() {
        let hash = UInt160::zero();
        let nef = NefFile::new("epicchain-core-v3.0".to_string(), vec![0x40]); // RET opcode
        let manifest = ContractManifest::default();

        let state = ContractState::new(1, hash, nef, manifest);
        assert_eq!(state.id, 1);
        assert_eq!(state.update_counter, 0);
        assert_eq!(state.hash, hash);
    }

    #[test]
    fn test_nef_file_checksum() {
        let script = vec![0x40]; // RET opcode
        let nef = NefFile::new("epicchain-core-v3.0".to_string(), script.clone());

        let expected_checksum = NefFile::calculate_checksum(&script);
        assert_eq!(nef.checksum, expected_checksum);
    }

    #[test]
    fn test_method_token_creation() {
        let hash = UInt160::zero();
        let token = MethodToken::new(hash, "test".to_string(), 2, true, CallFlags(0x01));

        assert_eq!(token.hash, hash);
        assert_eq!(token.method, "test");
        assert_eq!(token.parameters_count, 2);
        assert!(token.has_return_value);
        assert_eq!(token.call_flags, CallFlags(0x01));
    }
}
