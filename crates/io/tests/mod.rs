//! I/O Module C# Compatibility Test Suite
//!
//! This module contains comprehensive tests that ensure full compatibility
//! with C# Neo's I/O functionality including BinaryReader, BinaryWriter,
//! and ISerializable implementations.

mod binary_reader_tests;
mod binary_writer_tests;
mod serialization_tests;

mod integration_tests {
    use neo_io::{BinaryWriter, MemoryReader, Serializable};

    /// Test complete workflow matching C# Neo transaction serialization patterns
    #[test]
    fn test_transaction_like_serialization_workflow() {
        // Simulate a transaction-like structure as used in C# Neo
        #[derive(Debug, Clone, PartialEq)]
        struct MockTransaction {
            pub version: u8,
            pub nonce: u32,
            pub system_fee: u64,
            pub network_fee: u64,
            pub valid_until_block: u32,
            pub script: Vec<u8>,
            pub witnesses: Vec<MockWitness>,
        }

        #[derive(Debug, Clone, PartialEq)]
        struct MockWitness {
            pub invocation_script: Vec<u8>,
            pub verification_script: Vec<u8>,
        }

        impl Serializable for MockWitness {
            fn size(&self) -> usize {
                neo_io::serializable::helper::get_var_size(self.invocation_script.len() as u64)
                    + self.invocation_script.len()
                    + neo_io::serializable::helper::get_var_size(
                        self.verification_script.len() as u64
                    )
                    + self.verification_script.len()
            }

            fn serialize(&self, writer: &mut BinaryWriter) -> neo_io::IoResult<()> {
                writer.write_var_bytes(&self.invocation_script)?;
                writer.write_var_bytes(&self.verification_script)?;
                Ok(())
            }

            fn deserialize(reader: &mut MemoryReader) -> neo_io::IoResult<Self> {
                let invocation_script = reader.read_var_bytes(1024)?;
                let verification_script = reader.read_var_bytes(1024)?;
                Ok(MockWitness {
                    invocation_script,
                    verification_script,
                })
            }
        }

        impl Serializable for MockTransaction {
            fn size(&self) -> usize {
                1 + 4
                    + 8
                    + 8
                    + 4
                    + neo_io::serializable::helper::get_var_size(self.script.len() as u64)
                    + self.script.len()
                    + neo_io::serializable::helper::get_var_size(self.witnesses.len() as u64)
                    + self.witnesses.iter().map(|w| w.size()).sum::<usize>()
            }

            fn serialize(&self, writer: &mut BinaryWriter) -> neo_io::IoResult<()> {
                writer.write_byte(self.version)?;
                writer.write_u32(self.nonce)?;
                writer.write_u64(self.system_fee)?;
                writer.write_u64(self.network_fee)?;
                writer.write_u32(self.valid_until_block)?;
                writer.write_var_bytes(&self.script)?;
                writer.write_var_int(self.witnesses.len() as u64)?;
                for witness in &self.witnesses {
                    witness.serialize(writer)?;
                }
                Ok(())
            }

            fn deserialize(reader: &mut MemoryReader) -> neo_io::IoResult<Self> {
                let version = reader.read_byte()?;
                let nonce = reader.read_uint32()?;
                let system_fee = reader.read_uint64()?;
                let network_fee = reader.read_uint64()?;
                let valid_until_block = reader.read_uint32()?;
                let script = reader.read_var_bytes(65536)?;
                let witness_count = reader.read_var_int(16)? as usize;
                let mut witnesses = Vec::with_capacity(witness_count);
                for _ in 0..witness_count {
                    witnesses.push(MockWitness::deserialize(reader)?);
                }

                Ok(MockTransaction {
                    version,
                    nonce,
                    system_fee,
                    network_fee,
                    valid_until_block,
                    script,
                    witnesses,
                })
            }
        }

        // Create a complex transaction
        let original_tx = MockTransaction {
            version: 0,
            nonce: 123456789,
            system_fee: 1000000,
            network_fee: 500000,
            valid_until_block: 1000000,
            script: vec![0x0C, 0x05, 0x48, 0x65, 0x6C, 0x6C, 0x6F], // PUSHDATA1 "Hello"
            witnesses: vec![
                MockWitness {
                    invocation_script: vec![0x0C, 0x40], // PUSHDATA1 64 bytes
                    verification_script: vec![0x41, 0x9E, 0xD0, 0xDC], // Some script
                },
                MockWitness {
                    invocation_script: vec![],
                    verification_script: vec![0x56], // PUSH6
                },
            ],
        };

        // Test serialization round-trip
        let mut writer = BinaryWriter::new();
        original_tx.serialize(&mut writer).unwrap();
        let serialized = writer.to_bytes();

        let mut reader = MemoryReader::new(&serialized);
        let deserialized_tx = MockTransaction::deserialize(&mut reader).unwrap();

        assert_eq!(original_tx, deserialized_tx);
    }

    /// Test block-like serialization workflow (matches C# Neo block patterns)
    #[test]
    fn test_block_like_serialization_workflow() {
        #[derive(Debug, Clone, PartialEq)]
        struct MockBlockHeader {
            pub version: u32,
            pub prev_hash: [u8; 32],
            pub merkle_root: [u8; 32],
            pub timestamp: u64,
            pub nonce: u64,
            pub index: u32,
            pub primary_index: u8,
            pub next_consensus: [u8; 20],
        }

        impl Serializable for MockBlockHeader {
            fn size(&self) -> usize {
                4 + 32 + 32 + 8 + 8 + 4 + 1 + 20
            }

            fn serialize(&self, writer: &mut BinaryWriter) -> neo_io::IoResult<()> {
                writer.write_u32(self.version)?;
                writer.write_bytes(&self.prev_hash)?;
                writer.write_bytes(&self.merkle_root)?;
                writer.write_u64(self.timestamp)?;
                writer.write_u64(self.nonce)?;
                writer.write_u32(self.index)?;
                writer.write_byte(self.primary_index)?;
                writer.write_bytes(&self.next_consensus)?;
                Ok(())
            }

            fn deserialize(reader: &mut MemoryReader) -> neo_io::IoResult<Self> {
                let version = reader.read_uint32()?;
                let prev_hash = {
                    let bytes = reader.read_bytes(32)?;
                    let mut hash = [0u8; 32];
                    hash.copy_from_slice(&bytes);
                    hash
                };
                let merkle_root = {
                    let bytes = reader.read_bytes(32)?;
                    let mut root = [0u8; 32];
                    root.copy_from_slice(&bytes);
                    root
                };
                let timestamp = reader.read_uint64()?;
                let nonce = reader.read_uint64()?;
                let index = reader.read_uint32()?;
                let primary_index = reader.read_byte()?;
                let next_consensus = {
                    let bytes = reader.read_bytes(20)?;
                    let mut consensus = [0u8; 20];
                    consensus.copy_from_slice(&bytes);
                    consensus
                };

                Ok(MockBlockHeader {
                    version,
                    prev_hash,
                    merkle_root,
                    timestamp,
                    nonce,
                    index,
                    primary_index,
                    next_consensus,
                })
            }
        }

        let original_header = MockBlockHeader {
            version: 0,
            prev_hash: [0x01; 32],
            merkle_root: [0x02; 32],
            timestamp: 1234567890,
            nonce: 9876543210,
            index: 100,
            primary_index: 0,
            next_consensus: [0x03; 20],
        };

        // Test round-trip
        let mut writer = BinaryWriter::new();
        original_header.serialize(&mut writer).unwrap();
        let serialized = writer.to_bytes();

        let mut reader = MemoryReader::new(&serialized);
        let deserialized_header = MockBlockHeader::deserialize(&mut reader).unwrap();

        assert_eq!(original_header, deserialized_header);

        // Verify exact size matches C# expectations
        assert_eq!(serialized.len(), 4 + 32 + 32 + 8 + 8 + 4 + 1 + 20); // 109 bytes
    }
}
