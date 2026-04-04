// Transaction Fuzzing Target for Neo-RS
// Tests transaction parsing and validation with random inputs

#![no_main]
use libfuzzer_sys::fuzz_target;
use neo_core::transaction::Transaction;
use neo_core::transaction::validation::TransactionValidator;

fuzz_target!(|data: &[u8]| {
    // Fuzz transaction deserialization
    if let Ok(tx) = Transaction::from_bytes(data) {
        // If parsing succeeds, validate the transaction structure
        let validator = TransactionValidator::new();
        let _ = validator.validate(&tx);
        
        // Test serialization roundtrip
        if let Ok(serialized) = tx.to_bytes() {
            if let Ok(tx2) = Transaction::from_bytes(&serialized) {
                // Compare core fields (excluding cache fields)
                assert_eq!(tx.version(), tx2.version(), "Version mismatch in roundtrip");
                assert_eq!(tx.nonce(), tx2.nonce(), "Nonce mismatch in roundtrip");
                assert_eq!(tx.system_fee(), tx2.system_fee(), "System fee mismatch in roundtrip");
                assert_eq!(tx.network_fee(), tx2.network_fee(), "Network fee mismatch in roundtrip");
                assert_eq!(tx.valid_until_block(), tx2.valid_until_block(), "Valid until block mismatch in roundtrip");
                assert_eq!(tx.signers(), tx2.signers(), "Signers mismatch in roundtrip");
                assert_eq!(tx.attributes(), tx2.attributes(), "Attributes mismatch in roundtrip");
                assert_eq!(tx.script(), tx2.script(), "Script mismatch in roundtrip");
                assert_eq!(tx.witnesses(), tx2.witnesses(), "Witnesses mismatch in roundtrip");
            }
        }
        
        // Test transaction properties
        assert!(tx.size() > 0, "Transaction size must be positive");
        
        // Test fee calculation (system_fee + network_fee)
        if let Ok(fee) = tx.fee() {
            assert!(fee >= 0 || (tx.system_fee() < 0 || tx.network_fee() < 0), 
                    "Transaction fee calculation error");
        }
        
        // Test hash generation doesn't panic
        let _ = tx.hash();
        
        // Test script hash extraction
        let _ = tx.get_script_hashes();
    }
    
    // Also test with validation
    if let Ok(tx) = Transaction::from_bytes_validated(data) {
        // This transaction passed all validation checks
        assert!(tx.size() <= neo_config::MAX_TRANSACTION_SIZE);
        assert!(!tx.signers().is_empty());
        assert!(!tx.script().is_empty());
        assert_eq!(tx.witnesses().len(), tx.signers().len());
    }
});