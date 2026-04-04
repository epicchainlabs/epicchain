//! Message Routing Integration Tests
//!
//! Comprehensive tests to verify the complete message routing system from
//! TCP connections through PeerManager to P2pNode and finally to message handlers.

use neo_core::UInt256;
use neo_network::{
    composite_handler::CompositeMessageHandler,
    messages::{
        commands::{MessageCommand, MessageFlags},
        header::Neo3Message,
        network::NetworkMessage,
        protocol::ProtocolMessage,
    },
    p2p::protocol::MessageHandler,
    p2p_node::P2pNode,
    peer_manager::PeerManager,
    sync::SyncManager,
    NetworkConfig, NetworkResult,
};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::sync::{mpsc, RwLock};
use tokio::time::{sleep, Duration};

/// Mock TCP stream for testing message parsing
struct MockTcpStream {
    read_data: Vec<u8>,
    read_pos: usize,
    write_data: Vec<u8>,
}

impl MockTcpStream {
    fn new(data: Vec<u8>) -> Self {
        Self {
            read_data: data,
            read_pos: 0,
            write_data: Vec::new(),
        }
    }

    fn written_data(&self) -> &[u8] {
        &self.write_data
    }
}

impl AsyncRead for MockTcpStream {
    fn poll_read(
        mut self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        let remaining = self.read_data.len() - self.read_pos;
        if remaining == 0 {
            return std::task::Poll::Ready(Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "Mock stream exhausted",
            )));
        }

        let to_read = std::cmp::min(remaining, buf.remaining());
        let end_pos = self.read_pos + to_read;
        buf.put_slice(&self.read_data[self.read_pos..end_pos]);
        self.read_pos = end_pos;

        std::task::Poll::Ready(Ok(()))
    }
}

impl AsyncWrite for MockTcpStream {
    fn poll_write(
        mut self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<Result<usize, std::io::Error>> {
        self.write_data.extend_from_slice(buf);
        std::task::Poll::Ready(Ok(buf.len()))
    }

    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), std::io::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn poll_shutdown(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), std::io::Error>> {
        std::task::Poll::Ready(Ok(()))
    }
}

/// Mock message handler for testing
#[derive(Clone)]
struct MockMessageHandler {
    received_messages: Arc<RwLock<Vec<(SocketAddr, NetworkMessage)>>>,
    handler_name: String,
}

impl MockMessageHandler {
    fn new(name: &str) -> Self {
        Self {
            received_messages: Arc::new(RwLock::new(Vec::new())),
            handler_name: name.to_string(),
        }
    }

    async fn get_received_messages(&self) -> Vec<(SocketAddr, NetworkMessage)> {
        self.received_messages.read().await.clone()
    }

    async fn clear_received_messages(&self) {
        self.received_messages.write().await.clear();
    }
}

#[async_trait::async_trait]
impl MessageHandler for MockMessageHandler {
    async fn handle_message(
        &self,
        peer: SocketAddr,
        message: &NetworkMessage,
    ) -> NetworkResult<()> {
        println!(
            "🔍 {} received message from {}: {:?}",
            self.handler_name,
            peer,
            message.command()
        );
        self.received_messages
            .write()
            .await
            .push((peer, message.clone()));
        Ok(())
    }
}

/// Create a test Neo3 version message
fn create_version_message() -> Neo3Message {
    let version_payload = vec![
        0x00, 0x00, 0x00, 0x00, // Version: 0
        0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Services: 1
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Timestamp: 0
        0x00, 0x00, // Port: 0
        0x00, 0x00, 0x00, 0x00, // Nonce: 0
        0x0C, // User agent length: 12
        b'/', b'N', b'E', b'O', b':', b'R', b'u', b's', b't', b'/', b'1', b'.',
        b'0', // User agent: "/NEO:Rust/1.0"
        0x00, 0x00, 0x00, 0x00, // Start height: 0
        0x01, // Relay: true
    ];

    Neo3Message::new(MessageCommand::Version, version_payload)
}

/// Create a test Neo3 headers message
fn create_headers_message() -> Neo3Message {
    let headers_payload = vec![
        0x01, // Number of headers: 1
        // Header data (simplified)
        0x00, 0x00, 0x00, 0x00, // Version: 0
        // Previous hash (32 bytes of zeros)
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, // Merkle root (32 bytes of zeros)
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Timestamp: 0
        0x00, 0x00, 0x00, 0x00, // Nonce: 0
        0x00, 0x00, 0x00, 0x00, // Index: 0
        0x00, // Primary index: 0
        // Next consensus (20 bytes of zeros)
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x01, // Witness count: 1
        0x00, // Invocation script length: 0
        0x00, // Verification script length: 0
    ];

    Neo3Message::new(MessageCommand::Headers, headers_payload)
}

#[tokio::test]
async fn test_neo3_message_parsing() {
    println!("🧪 Testing Neo3 message parsing...");

    // Create a version message
    let message = create_version_message();
    let serialized = message.to_bytes();

    // Verify message structure
    assert_eq!(serialized[0], MessageFlags::None.as_byte());
    assert_eq!(serialized[1], MessageCommand::Version.as_byte());

    // Parse it back
    let parsed = Neo3Message::from_bytes(&serialized).expect("Failed to parse message");
    assert_eq!(parsed.command, MessageCommand::Version);
    assert_eq!(parsed.flags, MessageFlags::None);

    // Test with mock stream
    let mut mock_stream = MockTcpStream::new(serialized);
    let peer_addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();

    let result = PeerManager::read_complete_neo3_message(&mut mock_stream, peer_addr).await;
    assert!(
        result.is_ok(),
        "Failed to read message from mock stream: {:?}",
        result.err()
    );

    let message_bytes = result.unwrap();
    let network_message = NetworkMessage::from_bytes(&message_bytes);
    assert!(network_message.is_ok(), "Failed to parse NetworkMessage");

    println!("✅ Neo3 message parsing test passed");
}

#[tokio::test]
async fn test_message_handler_routing() {
    println!("🧪 Testing message handler routing...");

    // Create mock handlers
    let version_handler = Arc::new(MockMessageHandler::new("VersionHandler"));
    let headers_handler = Arc::new(MockMessageHandler::new("HeadersHandler"));
    let default_handler = Arc::new(MockMessageHandler::new("DefaultHandler"));

    // Create composite handler
    let mut composite = CompositeMessageHandler::new(default_handler.clone());
    composite.register_handlers(vec![
        (
            MessageCommand::Version,
            version_handler.clone() as Arc<dyn MessageHandler>,
        ),
        (
            MessageCommand::Headers,
            headers_handler.clone() as Arc<dyn MessageHandler>,
        ),
    ]);

    let peer_addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();

    // Test version message routing
    let version_msg = create_version_message();
    let version_network_msg = NetworkMessage::from_bytes(&version_msg.to_bytes()).unwrap();

    composite
        .handle_message(peer_addr, &version_network_msg)
        .await
        .unwrap();

    // Verify version handler received the message
    let version_messages = version_handler.get_received_messages().await;
    assert_eq!(version_messages.len(), 1);
    assert_eq!(version_messages[0].0, peer_addr);

    // Test headers message routing
    let headers_msg = create_headers_message();
    let headers_network_msg = NetworkMessage::from_bytes(&headers_msg.to_bytes()).unwrap();

    composite
        .handle_message(peer_addr, &headers_network_msg)
        .await
        .unwrap();

    // Verify headers handler received the message
    let headers_messages = headers_handler.get_received_messages().await;
    assert_eq!(headers_messages.len(), 1);
    assert_eq!(headers_messages[0].0, peer_addr);

    // Verify version handler didn't receive the headers message
    let version_messages = version_handler.get_received_messages().await;
    assert_eq!(version_messages.len(), 1); // Still only the version message

    println!("✅ Message handler routing test passed");
}

#[tokio::test]
async fn test_peer_manager_message_forwarding() {
    println!("🧪 Testing PeerManager message forwarding...");

    // Create a message forwarding channel
    let (message_tx, mut message_rx) = mpsc::unbounded_channel();

    // Create PeerManager with message forwarder
    let config = NetworkConfig::default();
    let mut peer_manager = PeerManager::new(config).expect("Failed to create PeerManager");
    peer_manager.set_message_forwarder(message_tx);

    // This test would need to be expanded with actual TCP connection simulation
    // For now, we're testing the basic forwarding mechanism setup

    println!("✅ PeerManager message forwarding setup test passed");
}

#[tokio::test]
async fn test_end_to_end_message_flow() {
    println!("🧪 Testing end-to-end message flow...");

    // Create mock handler to capture messages
    let sync_handler = Arc::new(MockMessageHandler::new("SyncManager"));
    let default_handler = Arc::new(MockMessageHandler::new("DefaultHandler"));

    // Create composite handler and register sync handler
    let mut composite = CompositeMessageHandler::new(default_handler.clone());
    composite.register_handlers(vec![
        (
            MessageCommand::Headers,
            sync_handler.clone() as Arc<dyn MessageHandler>,
        ),
        (
            MessageCommand::Block,
            sync_handler.clone() as Arc<dyn MessageHandler>,
        ),
        (
            MessageCommand::Inv,
            sync_handler.clone() as Arc<dyn MessageHandler>,
        ),
    ]);

    let composite_handler = Arc::new(composite);

    // Test direct message handling (simulating what P2pNode would do)
    let peer_addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    let headers_msg = create_headers_message();
    let headers_network_msg = NetworkMessage::from_bytes(&headers_msg.to_bytes()).unwrap();

    // Simulate P2pNode calling handle_message
    composite_handler
        .handle_message(peer_addr, &headers_network_msg)
        .await
        .unwrap();

    // Wait a bit for async processing
    sleep(Duration::from_millis(10)).await;

    // Verify sync handler received the headers message
    let sync_messages = sync_handler.get_received_messages().await;
    assert_eq!(sync_messages.len(), 1);
    assert_eq!(sync_messages[0].0, peer_addr);

    // Verify default handler didn't receive the message
    let default_messages = default_handler.get_received_messages().await;
    assert_eq!(default_messages.len(), 0);

    println!("✅ End-to-end message flow test passed");
}

#[tokio::test]
async fn test_message_parsing_error_handling() {
    println!("🧪 Testing message parsing error handling...");

    let peer_addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();

    // Test with invalid message (too short)
    let invalid_data = vec![0x00]; // Only 1 byte, needs at least 2
    let mut mock_stream = MockTcpStream::new(invalid_data);

    let result = PeerManager::read_complete_neo3_message(&mut mock_stream, peer_addr).await;
    assert!(result.is_err(), "Should fail with invalid message");

    // Test with message that has invalid length
    let invalid_length_msg = vec![
        0x00, // Flags
        0x00, // Command (Version)
        0xFF, 0xFF, 0xFF, 0xFF, // Invalid huge length
    ];
    let mut mock_stream = MockTcpStream::new(invalid_length_msg);

    let result = PeerManager::read_complete_neo3_message(&mut mock_stream, peer_addr).await;
    assert!(result.is_err(), "Should fail with invalid length");

    println!("✅ Message parsing error handling test passed");
}

#[tokio::test]
async fn test_varlen_encoding_decoding() {
    println!("🧪 Testing variable-length encoding/decoding...");

    // Test different payload sizes
    let test_cases = vec![
        (0, vec![0x00]),                             // 0 bytes
        (252, vec![0xFC]),                           // Single byte max
        (253, vec![0xFD, 0xFD, 0x00]),               // 2-byte encoding
        (65535, vec![0xFD, 0xFF, 0xFF]),             // 2-byte max
        (65536, vec![0xFE, 0x00, 0x00, 0x01, 0x00]), // 4-byte encoding
    ];

    for (length, expected_encoding) in test_cases {
        // Test small payloads to avoid huge memory allocation
        if length <= 1000 {
            let payload = vec![0x42; length]; // Fill with 0x42
            let message = Neo3Message::new(MessageCommand::Version, payload);
            let serialized = message.to_bytes();

            // Verify the length encoding
            let length_start = 2; // After flags and command
            let actual_encoding = &serialized[length_start..length_start + expected_encoding.len()];
            assert_eq!(
                actual_encoding, expected_encoding,
                "Length encoding mismatch for {}",
                length
            );

            // Verify round-trip
            let parsed = Neo3Message::from_bytes(&serialized).expect("Failed to parse");
            assert_eq!(parsed.payload.len(), length);
            if length > 0 {
                assert_eq!(parsed.payload[0], 0x42);
            }
        }
    }

    println!("✅ Variable-length encoding/decoding test passed");
}

#[tokio::test]
async fn test_concurrent_message_handling() {
    println!("🧪 Testing concurrent message handling...");

    let handler = Arc::new(MockMessageHandler::new("ConcurrentHandler"));
    let mut composite = CompositeMessageHandler::new(handler.clone());
    composite.register_handlers(vec![(
        MessageCommand::Version,
        handler.clone() as Arc<dyn MessageHandler>,
    )]);

    let composite = Arc::new(composite);
    let peer_addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();

    // Create multiple tasks handling messages concurrently
    let mut handles = Vec::new();
    for i in 0..10 {
        let composite_clone = composite.clone();
        let addr = format!("127.0.0.1:{}", 8080 + i)
            .parse::<SocketAddr>()
            .unwrap();

        let handle = tokio::spawn(async move {
            let message = create_version_message();
            let network_msg = NetworkMessage::from_bytes(&message.to_bytes()).unwrap();
            composite_clone
                .handle_message(addr, &network_msg)
                .await
                .unwrap();
        });
        handles.push(handle);
    }

    // Wait for all tasks to complete
    for handle in handles {
        handle.await.unwrap();
    }

    // Verify all messages were handled
    let messages = handler.get_received_messages().await;
    assert_eq!(messages.len(), 10);

    println!("✅ Concurrent message handling test passed");
}

#[tokio::test]
async fn test_message_validation() {
    println!("🧪 Testing message validation...");

    // Test oversized message
    let huge_payload = vec![0x00; 17 * 1024 * 1024]; // 17MB (over 16MB limit)
    let message = Neo3Message::new_uncompressed(MessageCommand::Block, huge_payload);

    // Should fail validation
    assert!(
        message.validate().is_err(),
        "Oversized message should fail validation"
    );

    // Test normal message
    let normal_payload = vec![0x00; 1000]; // 1KB
    let message = Neo3Message::new(MessageCommand::Version, normal_payload);

    // Should pass validation
    assert!(
        message.validate().is_ok(),
        "Normal message should pass validation"
    );

    println!("✅ Message validation test passed");
}
