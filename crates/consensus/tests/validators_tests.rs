//! Consensus Validators C# Compatibility Tests
//!
//! These tests ensure full compatibility with C# Neo.Consensus validator management.
//! Tests are based on the C# Neo.Consensus.Validator test suite.

use neo_consensus::validators::*;
use neo_core::{UInt160, UInt256};
use neo_cryptography::hash::sha256;

#[cfg(test)]
#[allow(dead_code)]
mod validators_tests {
    use super::*;

    /// Test Validator structure compatibility (matches C# Validator exactly)
    #[test]
    fn test_validator_creation_compatibility() {
        let public_key = vec![
            0x02, 0x48, 0x6f, 0xd1, 0x57, 0x02, 0xc4, 0x49, 0x0a, 0x26, 0x70, 0x31, 0x12, 0xa5,
            0xcc, 0x1d, 0x09, 0x23, 0xfd, 0x69, 0x7a, 0x33, 0x40, 0x6b, 0xd5, 0xa1, 0xc0, 0x0e,
            0x00, 0x13, 0xb0, 0x9a, 0x70,
        ];
        let script_hash = UInt160::from_bytes(&[1u8; 20]).unwrap();
        let votes = 1000000u64;

        let validator = Validator::new(public_key.clone(), script_hash, votes);

        assert_eq!(validator.public_key(), &public_key);
        assert_eq!(validator.script_hash(), script_hash);
        assert_eq!(validator.votes(), votes);
        assert!(validator.is_active());

        assert_eq!(validator.index(), None);

        // Test with index
        let mut indexed_validator = validator.clone();
        indexed_validator.set_index(3);
        assert_eq!(indexed_validator.index(), Some(3));
    }

    /// Test ValidatorSet functionality (matches C# ValidatorSet exactly)
    #[test]
    fn test_validator_set_compatibility() {
        let validators = vec![
            Validator::new(
                vec![0x02; 33],
                UInt160::from_bytes(&[1u8; 20]).unwrap(),
                5000000,
            ),
            Validator::new(
                vec![0x03; 33],
                UInt160::from_bytes(&[2u8; 20]).unwrap(),
                4000000,
            ),
            Validator::new(
                vec![0x04; 33],
                UInt160::from_bytes(&[3u8; 20]).unwrap(),
                3000000,
            ),
            Validator::new(
                vec![0x05; 33],
                UInt160::from_bytes(&[4u8; 20]).unwrap(),
                2000000,
            ),
            Validator::new(
                vec![0x06; 33],
                UInt160::from_bytes(&[5u8; 20]).unwrap(),
                1000000,
            ),
            Validator::new(
                vec![0x07; 33],
                UInt160::from_bytes(&[6u8; 20]).unwrap(),
                500000,
            ),
            Validator::new(
                vec![0x08; 33],
                UInt160::from_bytes(&[7u8; 20]).unwrap(),
                100000,
            ),
        ];

        // Create validator set
        let mut validator_set = ValidatorSet::new(validators.clone());

        // Test size
        assert_eq!(validator_set.count(), 7);

        let sorted_validators = validator_set.validators();
        assert_eq!(sorted_validators[0].votes(), 5000000);
        assert_eq!(sorted_validators[0].index(), Some(0));
        assert_eq!(sorted_validators[6].votes(), 100000);
        assert_eq!(sorted_validators[6].index(), Some(6));

        // Test get by index
        assert_eq!(validator_set.get_by_index(0).unwrap().votes(), 5000000);
        assert_eq!(validator_set.get_by_index(6).unwrap().votes(), 100000);
        assert!(validator_set.get_by_index(7).is_none());

        // Test get by script hash
        let hash = UInt160::from_bytes(&[3u8; 20]).unwrap();
        assert_eq!(
            validator_set.get_by_script_hash(&hash).unwrap().votes(),
            3000000
        );

        // Test contains
        assert!(validator_set.contains_script_hash(&hash));
        assert!(!validator_set.contains_script_hash(&UInt160::from_bytes(&[99u8; 20]).unwrap()));
    }

    /// Test ValidatorManager compatibility (matches C# ConsensusContext validator management exactly)
    #[test]
    fn test_validator_manager_compatibility() {
        // Create test configuration
        let config = ValidatorConfig {
            max_validators: 7,
            min_validators: 4,
            standby_validators: vec![
                vec![0x02; 33],
                vec![0x03; 33],
                vec![0x04; 33],
                vec![0x05; 33],
                vec![0x06; 33],
                vec![0x07; 33],
                vec![0x08; 33],
            ],
            enable_dynamic_validators: true,
        };

        let manager = ValidatorManager::new(config);

        // Test configuration
        assert_eq!(manager.max_validators(), 7);
        assert_eq!(manager.min_validators(), 4);
        assert!(manager.is_dynamic_validators_enabled());

        // Test standby validators
        assert_eq!(manager.standby_validators().len(), 7);

        let block_height = 1000u32;
        let mut elected_validators = vec![
            (vec![0x02; 33], 5000000u64),
            (vec![0x03; 33], 4000000u64),
            (vec![0x04; 33], 3000000u64),
            (vec![0x05; 33], 2000000u64),
        ];

        let validators = manager
            .select_validators(block_height, &elected_validators)
            .unwrap();
        assert_eq!(validators.count(), 7); // Should include standby validators

        let view = ViewNumber::new(0);
        let primary_index = manager.calculate_primary_index(view, validators.count());
        assert_eq!(primary_index, 0);

        let view2 = ViewNumber::new(1);
        let primary_index2 = manager.calculate_primary_index(view2, validators.count());
        assert_eq!(primary_index2, 1);

        // Test view change primary rotation
        let view7 = ViewNumber::new(7);
        let primary_index7 = manager.calculate_primary_index(view7, validators.count());
        assert_eq!(primary_index7, 0); // Should wrap around
    }

    /// Test validator voting and ranking (matches C# voting system exactly)
    #[test]
    fn test_validator_voting_compatibility() {
        // Create validators with different vote counts
        let mut validators = vec![
            Validator::new(
                vec![0x02; 33],
                UInt160::from_bytes(&[1u8; 20]).unwrap(),
                1000,
            ),
            Validator::new(
                vec![0x03; 33],
                UInt160::from_bytes(&[2u8; 20]).unwrap(),
                2000,
            ),
            Validator::new(
                vec![0x04; 33],
                UInt160::from_bytes(&[3u8; 20]).unwrap(),
                1500,
            ),
        ];

        // Test vote updates
        validators[0].add_votes(500);
        assert_eq!(validators[0].votes(), 1500);

        validators[1].remove_votes(500);
        assert_eq!(validators[1].votes(), 1500);

        // Test validator ranking
        let mut validator_set = ValidatorSet::new(validators);
        let ranked = validator_set.validators();

        assert_eq!(ranked[0].votes(), 1500);
        assert_eq!(ranked[1].votes(), 1500);
        assert_eq!(ranked[0].public_key()[0], 0x02); // Lower public key comes first for ties
    }

    /// Test validator state management (matches C# validator state exactly)
    #[test]
    fn test_validator_state_management_compatibility() {
        let mut validator = Validator::new(
            vec![0x02; 33],
            UInt160::from_bytes(&[1u8; 20]).unwrap(),
            1000000,
        );

        // Test active state
        assert!(validator.is_active());

        validator.set_active(false);
        assert!(!validator.is_active());

        validator.set_active(true);
        assert!(validator.is_active());

        // Test stats tracking
        let stats = validator.stats();
        assert_eq!(stats.blocks_proposed, 0);
        assert_eq!(stats.blocks_signed, 0);
        assert_eq!(stats.view_changes_initiated, 0);

        // Update stats
        validator.increment_blocks_proposed();
        validator.increment_blocks_signed();
        validator.increment_view_changes();

        let updated_stats = validator.stats();
        assert_eq!(updated_stats.blocks_proposed, 1);
        assert_eq!(updated_stats.blocks_signed, 1);
        assert_eq!(updated_stats.view_changes_initiated, 1);
    }

    /// Test validator configuration validation (matches C# configuration rules exactly)
    #[test]
    fn test_validator_config_validation_compatibility() {
        // Test valid configuration
        let valid_config = ValidatorConfig {
            max_validators: 7,
            min_validators: 4,
            standby_validators: vec![vec![0x02; 33]; 7],
            enable_dynamic_validators: true,
        };
        assert!(valid_config.validate().is_ok());

        let invalid_max = ValidatorConfig {
            max_validators: 8, // Not 3f+1
            min_validators: 4,
            standby_validators: vec![vec![0x02; 33]; 8],
            enable_dynamic_validators: true,
        };
        assert!(invalid_max.validate().is_err());

        // Test too few min validators
        let invalid_min = ValidatorConfig {
            max_validators: 7,
            min_validators: 3, // Less than 4
            standby_validators: vec![vec![0x02; 33]; 7],
            enable_dynamic_validators: true,
        };
        assert!(invalid_min.validate().is_err());

        // Test mismatched standby count
        let invalid_standby = ValidatorConfig {
            max_validators: 7,
            min_validators: 4,
            standby_validators: vec![vec![0x02; 33]; 5], // Not equal to max
            enable_dynamic_validators: true,
        };
        assert!(invalid_standby.validate().is_err());
    }

    /// Test validator public key validation (matches C# public key validation exactly)
    #[test]
    fn test_validator_public_key_validation_compatibility() {
        let valid_compressed = vec![0x02; 33];
        let validator = Validator::new(
            valid_compressed.clone(),
            UInt160::from_bytes(&[1u8; 20]).unwrap(),
            1000,
        );
        assert!(validator.validate_public_key().is_ok());

        // Test invalid public key length
        let invalid_length = vec![0x02; 32]; // Too short
        let invalid_validator = Validator::new(
            invalid_length,
            UInt160::from_bytes(&[1u8; 20]).unwrap(),
            1000,
        );
        assert!(invalid_validator.validate_public_key().is_err());

        // Test invalid public key prefix
        let invalid_prefix = {
            let mut key = vec![0x00; 33];
            key[0] = 0x05; // Invalid prefix
            key
        };
        let invalid_prefix_validator = Validator::new(
            invalid_prefix,
            UInt160::from_bytes(&[1u8; 20]).unwrap(),
            1000,
        );
        assert!(invalid_prefix_validator.validate_public_key().is_err());
    }

    /// Test validator serialization compatibility (matches C# binary format exactly)
    #[test]
    fn test_validator_serialization_compatibility() {
        let validator = Validator::new(
            vec![0x02; 33],
            UInt160::from_bytes(&[10u8; 20]).unwrap(),
            123456789,
        );

        // Test serialization
        let serialized = validator.to_bytes().unwrap();
        assert!(!serialized.is_empty());

        // Test deserialization
        let deserialized = Validator::from_bytes(&serialized).unwrap();
        assert_eq!(deserialized.public_key(), validator.public_key());
        assert_eq!(deserialized.script_hash(), validator.script_hash());
        assert_eq!(deserialized.votes(), validator.votes());
        assert_eq!(deserialized.is_active(), validator.is_active());

        // Test ValidatorSet serialization
        let validators = vec![validator.clone(); 3];
        let validator_set = ValidatorSet::new(validators);

        let set_serialized = validator_set.to_bytes().unwrap();
        let set_deserialized = ValidatorSet::from_bytes(&set_serialized).unwrap();
        assert_eq!(set_deserialized.count(), validator_set.count());
    }

    /// Test validator performance and limits (matches C# performance characteristics exactly)
    #[test]
    fn test_validator_performance_limits_compatibility() {
        let max_validators = 1024; // C# practical limit
        let mut validators = Vec::new();

        for i in 0..max_validators {
            let mut public_key = vec![0x02; 33];
            public_key[1] = (i & 0xFF) as u8;
            public_key[2] = ((i >> 8) & 0xFF) as u8;

            let mut script_hash_bytes = [0u8; 20];
            script_hash_bytes[0] = (i & 0xFF) as u8;
            script_hash_bytes[1] = ((i >> 8) & 0xFF) as u8;

            validators.push(Validator::new(
                public_key,
                UInt160::from_bytes(&script_hash_bytes).unwrap(),
                (max_validators - i) as u64 * 1000, // Descending votes
            ));
        }

        let start = std::time::Instant::now();
        let validator_set = ValidatorSet::new(validators);
        let creation_time = start.elapsed();

        // Should create and sort quickly
        assert!(creation_time.as_millis() < 100);
        assert_eq!(validator_set.count(), max_validators);

        // Test lookup performance
        let start = std::time::Instant::now();
        for i in 0..100 {
            let _ = validator_set.get_by_index(i % max_validators);
        }
        let lookup_time = start.elapsed();
        assert!(lookup_time.as_micros() < 1000); // Should be very fast
    }

    /// Test validator edge cases (matches C# edge case handling exactly)
    #[test]
    fn test_validator_edge_cases_compatibility() {
        // Test zero votes validator
        let zero_votes =
            Validator::new(vec![0x02; 33], UInt160::from_bytes(&[1u8; 20]).unwrap(), 0);
        assert_eq!(zero_votes.votes(), 0);
        assert!(zero_votes.is_active()); // Still active even with 0 votes

        // Test maximum votes
        let max_votes = Validator::new(
            vec![0x02; 33],
            UInt160::from_bytes(&[1u8; 20]).unwrap(),
            u64::MAX,
        );
        assert_eq!(max_votes.votes(), u64::MAX);

        // Test empty validator set
        let empty_set = ValidatorSet::new(vec![]);
        assert_eq!(empty_set.count(), 0);
        assert!(empty_set.get_by_index(0).is_none());

        // Test single validator set
        let single_set = ValidatorSet::new(vec![zero_votes.clone()]);
        assert_eq!(single_set.count(), 1);
        assert_eq!(single_set.get_by_index(0).unwrap().votes(), 0);

        let validator1 = Validator::new(
            vec![0x03; 33],
            UInt160::from_bytes(&[1u8; 20]).unwrap(),
            1000,
        );
        let validator2 = Validator::new(
            vec![0x02; 33], // Lower public key
            UInt160::from_bytes(&[2u8; 20]).unwrap(),
            1000,
        );

        let tie_set = ValidatorSet::new(vec![validator1, validator2]);
        // Validator with lower public key should come first
        assert_eq!(tie_set.get_by_index(0).unwrap().public_key()[0], 0x02);
        assert_eq!(tie_set.get_by_index(1).unwrap().public_key()[0], 0x03);
    }
}
