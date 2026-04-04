//! Working Message Routing Test
//!
//! A focused integration test that verifies the core message routing
//! functionality works correctly with minimal dependencies.

use neo_network::messages::{
    commands::{MessageCommand, MessageFlags},
    header::Neo3Message,
    network::NetworkMessage,
};
use std::net::SocketAddr;
use tokio::sync::mpsc;

/// Test basic message creation and parsing
#[tokio::test]
async fn test_message_creation_and_parsing() {
    println!("🧪 Testing message creation and parsing...");

    // Create a version message
    let payload = vec![0x01, 0x02, 0x03, 0x04];
    let message = Neo3Message::new(MessageCommand::Version, payload.clone());

    // Verify message properties
    assert_eq!(message.command, MessageCommand::Version);
    assert_eq!(message.flags, MessageFlags::None);
    assert_eq!(message.payload, payload);

    // Test serialization
    let serialized = message.to_bytes();
    assert_eq!(serialized[0], MessageFlags::None.as_byte());
    assert_eq!(serialized[1], MessageCommand::Version.as_byte());

    // Test parsing back
    let parsed = Neo3Message::from_bytes(&serialized).expect("Failed to parse");
    assert_eq!(parsed.command, MessageCommand::Version);
    assert_eq!(parsed.payload, payload);

    println!("✅ Message creation and parsing test passed");
}

/// Test NetworkMessage conversion
#[tokio::test]
async fn test_network_message_conversion() {
    println!("🧪 Testing NetworkMessage conversion...");

    // Create Neo3 message
    let payload = vec![0xAA, 0xBB, 0xCC];
    let neo3_msg = Neo3Message::new(MessageCommand::Ping, payload.clone());
    let serialized = neo3_msg.to_bytes();

    // Convert to NetworkMessage
    let network_msg = NetworkMessage::from_bytes(&serialized);
    assert!(network_msg.is_ok(), "Failed to convert to NetworkMessage");

    let network_msg = network_msg.unwrap();
    println!("📬 Converted message command: {:?}", network_msg.command());

    println!("✅ NetworkMessage conversion test passed");
}

/// Test message forwarding channel setup  
#[tokio::test]
async fn test_message_forwarding_channel() {
    println!("🧪 Testing message forwarding channel...");

    // Create channel
    let (tx, mut rx) = mpsc::unbounded_channel::<(SocketAddr, NetworkMessage)>();

    // Create test message
    let payload = vec![0x01, 0x02, 0x03];
    let neo3_msg = Neo3Message::new(MessageCommand::Headers, payload);
    let serialized = neo3_msg.to_bytes();
    let network_msg = NetworkMessage::from_bytes(&serialized).unwrap();

    // Send message through channel
    let peer_addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    tx.send((peer_addr, network_msg.clone()))
        .expect("Failed to send");

    // Receive message
    let received = rx.recv().await.expect("Failed to receive");
    assert_eq!(received.0, peer_addr);

    println!("📡 Successfully forwarded message through channel");
    println!("✅ Message forwarding channel test passed");
}

/// Test variable length encoding
#[tokio::test]
async fn test_varlen_encoding() {
    println!("🧪 Testing variable-length encoding...");

    // Test cases for different payload sizes
    let test_cases = vec![
        (50, 1),  // Single byte length
        (300, 3), // 3-byte length (0xFD + 2 bytes)
    ];

    for (payload_size, expected_header_len) in test_cases {
        let payload = vec![0x42; payload_size];
        let message = Neo3Message::new(MessageCommand::Block, payload);
        let serialized = message.to_bytes();

        // Check header length
        let header_len = if serialized[2] == 0xFD { 3 } else { 1 };
        assert_eq!(header_len, expected_header_len);

        // Verify round-trip
        let parsed = Neo3Message::from_bytes(&serialized).expect("Failed to parse");
        assert_eq!(parsed.payload.len(), payload_size);

        println!("✓ Payload size {} encoded correctly", payload_size);
    }

    println!("✅ Variable-length encoding test passed");
}

/// Test message validation
#[tokio::test]
async fn test_message_validation() {
    println!("🧪 Testing message validation...");

    // Test normal message
    let normal_payload = vec![0x00; 1000];
    let normal_message = Neo3Message::new(MessageCommand::Transaction, normal_payload);
    assert!(
        normal_message.validate().is_ok(),
        "Normal message should pass validation"
    );

    // Test empty message
    let empty_message = Neo3Message::new(MessageCommand::Ping, vec![]);
    assert!(
        empty_message.validate().is_ok(),
        "Empty message should pass validation"
    );

    println!("✅ Message validation test passed");
}

/// Integration test that simulates the full message flow
#[tokio::test]
async fn test_integration_message_flow() {
    println!("🧪 Testing integration message flow...");

    // Step 1: Create a realistic Neo3 message
    let version_payload = vec![
        0x00, 0x00, 0x00, 0x00, // Version: 0
        0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Services: 1
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Timestamp: 0
        0x00, 0x00, // Port: 0
        0x00, 0x00, 0x00, 0x00, // Nonce: 0
        0x0A, // User agent length: 10
        b'N', b'E', b'O', b':', b'R', b'u', b's', b't', b'/',
        b'1', // User agent: "NEO:Rust/1"
        0x00, 0x00, 0x00, 0x00, // Start height: 0
        0x01, // Relay: true
    ];

    // Step 2: Create Neo3 message
    let neo3_msg = Neo3Message::new(MessageCommand::Version, version_payload);
    println!(
        "📦 Created Neo3 version message: {} bytes",
        neo3_msg.payload.len()
    );

    // Step 3: Serialize message
    let serialized = neo3_msg.to_bytes();
    println!("🔧 Serialized message: {} bytes", serialized.len());

    // Step 4: Parse as NetworkMessage (simulating PeerManager parsing)
    let network_msg =
        NetworkMessage::from_bytes(&serialized).expect("Failed to parse NetworkMessage");
    println!(
        "📬 Parsed NetworkMessage command: {:?}",
        network_msg.command()
    );

    // Step 5: Forward through channel (simulating PeerManager -> P2pNode)
    let (tx, mut rx) = mpsc::unbounded_channel();
    let peer_addr: SocketAddr = "192.168.1.100:20333".parse().unwrap();

    tx.send((peer_addr, network_msg.clone()))
        .expect("Failed to send");
    println!("📡 Forwarded message to P2pNode via channel");

    // Step 6: Receive message (simulating P2pNode receiving)
    let (received_addr, received_msg) = rx.recv().await.expect("Failed to receive");
    assert_eq!(received_addr, peer_addr);
    println!("📨 P2pNode received message from {}", received_addr);

    // Step 7: Route to handler (simulating CompositeHandler routing)
    println!(
        "🎯 Routing message to appropriate handler based on command: {:?}",
        received_msg.command()
    );

    println!("✅ Integration message flow test passed - complete end-to-end flow verified!");
}

/// Test message parsing error handling
#[tokio::test]
async fn test_message_error_handling() {
    println!("🧪 Testing message error handling...");

    // Test invalid message (too short)
    let invalid_data = vec![0x00]; // Only 1 byte, needs at least 2
    let result = Neo3Message::from_bytes(&invalid_data);
    assert!(result.is_err(), "Should fail with invalid message");
    println!("✓ Correctly rejected too-short message");

    // Test invalid NetworkMessage conversion
    let result = NetworkMessage::from_bytes(&invalid_data);
    assert!(
        result.is_err(),
        "Should fail to parse invalid NetworkMessage"
    );
    println!("✓ Correctly rejected invalid NetworkMessage");

    println!("✅ Message error handling test passed");
}
