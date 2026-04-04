//! Simple Message Routing Test
//!
//! A focused test to verify basic message parsing and routing functionality

use neo_network::messages::{
    commands::{MessageCommand, MessageFlags},
    header::Neo3Message,
};

#[tokio::test]
async fn test_basic_neo3_message_creation() {
    println!("🧪 Testing basic Neo3 message creation...");

    let payload = vec![0x01, 0x02, 0x03, 0x04];
    let message = Neo3Message::new(MessageCommand::Version, payload.clone());

    assert_eq!(message.command, MessageCommand::Version);
    assert_eq!(message.flags, MessageFlags::None);
    assert_eq!(message.payload, payload);

    println!("✅ Basic Neo3 message creation test passed");
}

#[tokio::test]
async fn test_neo3_message_serialization() {
    println!("🧪 Testing Neo3 message serialization...");

    let payload = vec![0x01, 0x02, 0x03];
    let message = Neo3Message::new(MessageCommand::Ping, payload.clone());

    let serialized = message.to_bytes();

    // Check header structure
    assert_eq!(serialized[0], MessageFlags::None.as_byte());
    assert_eq!(serialized[1], MessageCommand::Ping.as_byte());
    assert_eq!(serialized[2], 0x03); // Payload length (small, single byte)
    assert_eq!(&serialized[3..], &payload);

    println!("✅ Neo3 message serialization test passed");
}

#[tokio::test]
async fn test_neo3_message_roundtrip() {
    println!("🧪 Testing Neo3 message roundtrip...");

    let original_payload = vec![0xAA, 0xBB, 0xCC, 0xDD];
    let original_message = Neo3Message::new(MessageCommand::Block, original_payload.clone());

    let serialized = original_message.to_bytes();
    let parsed_message = Neo3Message::from_bytes(&serialized).expect("Failed to parse");

    assert_eq!(parsed_message.command, MessageCommand::Block);
    assert_eq!(parsed_message.flags, MessageFlags::None);
    assert_eq!(parsed_message.payload, original_payload);

    println!("✅ Neo3 message roundtrip test passed");
}

#[tokio::test]
async fn test_varlen_encoding() {
    println!("🧪 Testing variable-length encoding...");

    // Test small payload (single byte length)
    let small_payload = vec![0x42; 100];
    let small_message = Neo3Message::new(MessageCommand::Version, small_payload);
    let small_serialized = small_message.to_bytes();
    assert_eq!(small_serialized[2], 100); // Length as single byte

    // Test medium payload (3-byte length: 0xFD + 2 bytes)
    let medium_payload = vec![0x42; 300];
    let medium_message = Neo3Message::new(MessageCommand::Headers, medium_payload);
    let medium_serialized = medium_message.to_bytes();
    assert_eq!(medium_serialized[2], 0xFD); // Variable length marker
    assert_eq!(medium_serialized[3], (300 & 0xFF) as u8); // Low byte
    assert_eq!(medium_serialized[4], ((300 >> 8) & 0xFF) as u8); // High byte

    println!("✅ Variable-length encoding test passed");
}

#[tokio::test]
async fn test_message_validation() {
    println!("🧪 Testing message validation...");

    // Test normal message
    let normal_payload = vec![0x00; 1000];
    let normal_message = Neo3Message::new(MessageCommand::Transaction, normal_payload);
    assert!(normal_message.validate().is_ok());

    // Test oversized message (would exceed 16MB limit)
    let huge_payload = vec![0x00; 17 * 1024 * 1024]; // 17MB
    let huge_message = Neo3Message::new_uncompressed(MessageCommand::Block, huge_payload);
    assert!(huge_message.validate().is_err());

    println!("✅ Message validation test passed");
}
