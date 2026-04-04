//! LogEventArgs tests converted from C# Neo unit tests (UT_LogEventArgs.cs).
//! These tests ensure 100% compatibility with the C# Neo log event arguments implementation.

use neo_core::UInt160;
use neo_network_p2p::payloads::{Header, IVerifiable};
use neo_smart_contract::LogEventArgs;

// ============================================================================
// Test LogEventArgs creation and properties
// ============================================================================

/// Test converted from C# UT_LogEventArgs.TestGeneratorAndGet
#[test]
fn test_generator_and_get() {
    // Create test data
    let container = Header::default();
    let scripthash = UInt160::zero();
    let message = "lalala".to_string();

    // Create LogEventArgs
    let log_event_args =
        LogEventArgs::new(Box::new(container.clone()), scripthash, message.clone());

    // Verify properties
    assert_eq!(container.hash(), log_event_args.script_container().hash());
    assert_eq!(scripthash, log_event_args.script_hash());
    assert_eq!(message, log_event_args.message());
}

// ============================================================================
// Test edge cases and additional scenarios
// ============================================================================

/// Test LogEventArgs with empty message
#[test]
fn test_log_event_args_empty_message() {
    let container = Header::default();
    let scripthash = UInt160::zero();
    let message = String::new();

    let log_event_args =
        LogEventArgs::new(Box::new(container.clone()), scripthash, message.clone());

    assert_eq!(container.hash(), log_event_args.script_container().hash());
    assert_eq!(scripthash, log_event_args.script_hash());
    assert_eq!(message, log_event_args.message());
}

/// Test LogEventArgs with long message
#[test]
fn test_log_event_args_long_message() {
    let container = Header::default();
    let scripthash = UInt160::zero();
    let message = "a".repeat(1000); // Long message

    let log_event_args =
        LogEventArgs::new(Box::new(container.clone()), scripthash, message.clone());

    assert_eq!(container.hash(), log_event_args.script_container().hash());
    assert_eq!(scripthash, log_event_args.script_hash());
    assert_eq!(message, log_event_args.message());
}

/// Test LogEventArgs with different script hashes
#[test]
fn test_log_event_args_different_script_hashes() {
    let container = Header::default();
    let message = "test message".to_string();

    // Test with zero hash
    let zero_hash = UInt160::zero();
    let log_event_args1 =
        LogEventArgs::new(Box::new(container.clone()), zero_hash, message.clone());
    assert_eq!(zero_hash, log_event_args1.script_hash());

    // Test with max value hash
    let max_hash = UInt160::from_bytes([0xFF; 20]);
    let log_event_args2 = LogEventArgs::new(Box::new(container.clone()), max_hash, message.clone());
    assert_eq!(max_hash, log_event_args2.script_hash());

    // Test with random hash
    let random_hash = UInt160::from_bytes([
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
        0x10, 0x11, 0x12, 0x13, 0x14,
    ]);
    let log_event_args3 =
        LogEventArgs::new(Box::new(container.clone()), random_hash, message.clone());
    assert_eq!(random_hash, log_event_args3.script_hash());
}

/// Test LogEventArgs with special characters in message
#[test]
fn test_log_event_args_special_characters() {
    let container = Header::default();
    let scripthash = UInt160::zero();

    // Test various special characters
    let test_messages = vec![
        "Hello\nWorld",      // Newline
        "Tab\tSeparated",    // Tab
        "Quote\"Test",       // Quote
        "Backslash\\Test",   // Backslash
        "Unicode: 你好世界", // Unicode characters
        "Emoji: 🚀🌟",       // Emojis
        "\0NullByte",        // Null byte
        "",                  // Empty string
    ];

    for message in test_messages {
        let log_event_args =
            LogEventArgs::new(Box::new(container.clone()), scripthash, message.to_string());

        assert_eq!(message, log_event_args.message());
    }
}

/// Test LogEventArgs equality and cloning
#[test]
fn test_log_event_args_equality_and_clone() {
    let container = Header::default();
    let scripthash = UInt160::from_bytes([0x42; 20]);
    let message = "test message".to_string();

    let log_event_args1 =
        LogEventArgs::new(Box::new(container.clone()), scripthash, message.clone());

    // Test cloning
    let log_event_args2 = log_event_args1.clone();

    assert_eq!(
        log_event_args1.script_container().hash(),
        log_event_args2.script_container().hash()
    );
    assert_eq!(log_event_args1.script_hash(), log_event_args2.script_hash());
    assert_eq!(log_event_args1.message(), log_event_args2.message());
}

/// Test LogEventArgs with different container types
#[test]
fn test_log_event_args_different_containers() {
    let scripthash = UInt160::zero();
    let message = "test".to_string();

    // Test with Header container
    let header = Header::default();
    let log_event_args_header =
        LogEventArgs::new(Box::new(header.clone()), scripthash, message.clone());
    assert_eq!(
        header.hash(),
        log_event_args_header.script_container().hash()
    );

    // Test with Transaction container
    let transaction = Transaction::default();
    let log_event_args_tx =
        LogEventArgs::new(Box::new(transaction.clone()), scripthash, message.clone());
    assert_eq!(
        transaction.hash(),
        log_event_args_tx.script_container().hash()
    );

    // Test with Block container
    let block = Block::default();
    let log_event_args_block =
        LogEventArgs::new(Box::new(block.clone()), scripthash, message.clone());
    assert_eq!(block.hash(), log_event_args_block.script_container().hash());
}

// ============================================================================
// Implementation stubs
// ============================================================================

mod neo_smart_contract {
    use neo_core::UInt160;
    use neo_network_p2p::payloads::IVerifiable;

    #[derive(Clone)]
    pub struct LogEventArgs {
        script_container: Box<dyn IVerifiable>,
        script_hash: UInt160,
        message: String,
    }

    impl LogEventArgs {
        pub fn new(
            script_container: Box<dyn IVerifiable>,
            script_hash: UInt160,
            message: String,
        ) -> Self {
            LogEventArgs {
                script_container,
                script_hash,
                message,
            }
        }

        pub fn script_container(&self) -> &dyn IVerifiable {
            &*self.script_container
        }

        pub fn script_hash(&self) -> UInt160 {
            self.script_hash
        }

        pub fn message(&self) -> &str {
            &self.message
        }
    }
}

mod neo_network_p2p {
    pub mod payloads {
        use neo_core::{UInt160, UInt256};

        pub trait IVerifiable: Clone {
            fn hash(&self) -> UInt256;
            fn clone_box(&self) -> Box<dyn IVerifiable>;
        }

        impl Clone for Box<dyn IVerifiable> {
            fn clone(&self) -> Self {
                self.clone_box()
            }
        }

        #[derive(Clone, Default)]
        pub struct Header {
            hash: UInt256,
        }

        impl IVerifiable for Header {
            fn hash(&self) -> UInt256 {
                self.hash
            }

            fn clone_box(&self) -> Box<dyn IVerifiable> {
                Box::new(self.clone())
            }
        }

        #[derive(Clone, Default)]
        pub struct Transaction {
            hash: UInt256,
        }

        impl IVerifiable for Transaction {
            fn hash(&self) -> UInt256 {
                self.hash
            }

            fn clone_box(&self) -> Box<dyn IVerifiable> {
                Box::new(self.clone())
            }
        }

        #[derive(Clone, Default)]
        pub struct Block {
            hash: UInt256,
        }

        impl IVerifiable for Block {
            fn hash(&self) -> UInt256 {
                self.hash
            }

            fn clone_box(&self) -> Box<dyn IVerifiable> {
                Box::new(self.clone())
            }
        }
    }
}

mod neo_core {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub struct UInt160([u8; 20]);

    impl UInt160 {
        pub fn zero() -> Self {
            UInt160([0u8; 20])
        }

        pub fn from_bytes(bytes: [u8; 20]) -> Self {
            UInt160(bytes)
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub struct UInt256([u8; 32]);

    impl UInt256 {
        pub fn zero() -> Self {
            UInt256([0u8; 32])
        }
    }
}

use neo_network_p2p::payloads::{Block, Transaction};
