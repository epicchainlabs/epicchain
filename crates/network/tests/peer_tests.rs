#![cfg(feature = "compat_tests")]
//! Network Peer Management C# Compatibility Tests
//!
//! These tests ensure full compatibility with C# Neo.Network peer management.
//! Tests are based on the C# Neo.Network.P2P.RemoteNode test suite.

use neo_network::peers::*;
use neo_network::*;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[cfg(test)]
#[allow(dead_code)]
mod peer_tests {
    use super::*;

    /// Test peer creation and initialization (matches C# RemoteNode exactly)
    #[test]
    fn test_peer_creation_compatibility() {
        let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100)), 10333);
        let config = PeerConfig {
            max_connections: 100,
            connection_timeout: Duration::from_secs(10),
            handshake_timeout: Duration::from_secs(30),
            ping_interval: Duration::from_secs(30),
            max_ping_time: Duration::from_secs(5),
            user_agent: "/NEO:3.6.0/".to_string(),
            services: NodeServices::NodeNetwork as u64,
            relay: true,
        };

        let peer = Peer::new(address, config.clone());

        // Verify initial state
        assert_eq!(peer.address(), address);
        assert_eq!(peer.state(), PeerState::Disconnected);
        assert_eq!(peer.version(), None);
        assert_eq!(peer.height(), 0);
        assert_eq!(peer.user_agent(), None);
        assert_eq!(peer.services(), 0);
        assert_eq!(peer.last_ping_time(), None);
        assert!(!peer.is_connected());
        assert!(!peer.is_relay());
    }

    /// Test peer connection process (matches C# connection flow exactly)
    #[test]
    fn test_peer_connection_compatibility() {
        let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 10333);
        let config = PeerConfig::default();
        let mut peer = Peer::new(address, config);

        // Test connection initiation
        assert_eq!(peer.state(), PeerState::Disconnected);

        // Start connection
        peer.connect().unwrap();
        assert_eq!(peer.state(), PeerState::Connecting);

        // Simulate successful TCP connection
        peer.set_connected(true);
        assert_eq!(peer.state(), PeerState::Connected);

        // Test handshake process
        let version_msg = VersionMessage {
            version: 0x00,
            services: NodeServices::NodeNetwork as u64,
            timestamp: current_timestamp(),
            port: 10333,
            nonce: 0x1234567890ABCDEF,
            user_agent: "/NEO:3.6.0/".to_string(),
            start_height: 50000,
        };

        // Send our version
        peer.send_version(version_msg.clone()).unwrap();
        assert_eq!(peer.state(), PeerState::VersionSent);

        // Receive their version
        peer.receive_version(version_msg.clone()).unwrap();
        assert_eq!(peer.state(), PeerState::VersionReceived);

        // Send verack
        peer.send_verack().unwrap();
        assert_eq!(peer.state(), PeerState::VerackSent);

        // Receive verack - handshake complete
        peer.receive_verack().unwrap();
        assert_eq!(peer.state(), PeerState::Ready);

        // Verify peer properties after handshake
        assert!(peer.is_connected());
        assert_eq!(peer.version(), Some(0x00));
        assert_eq!(peer.height(), 50000);
        assert_eq!(peer.user_agent(), Some("/NEO:3.6.0/".to_string()));
        assert_eq!(peer.services(), NodeServices::NodeNetwork as u64);
    }

    /// Test peer disconnection (matches C# disconnection handling exactly)
    #[test]
    fn test_peer_disconnection_compatibility() {
        let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 10333);
        let config = PeerConfig::default();
        let mut peer = create_connected_peer(address, config);

        // Verify connected state
        assert!(peer.is_connected());
        assert_eq!(peer.state(), PeerState::Ready);

        // Test graceful disconnection
        peer.disconnect(DisconnectReason::UserRequested).unwrap();
        assert_eq!(peer.state(), PeerState::Disconnected);
        assert!(!peer.is_connected());
        assert_eq!(
            peer.disconnect_reason(),
            Some(DisconnectReason::UserRequested)
        );

        // Test disconnect cleanup
        assert_eq!(peer.version(), None);
        assert_eq!(peer.height(), 0);
        assert_eq!(peer.user_agent(), None);
        assert_eq!(peer.services(), 0);
    }

    /// Test ping/pong mechanism (matches C# ping handling exactly)
    #[test]
    fn test_ping_pong_compatibility() {
        let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 10333);
        let config = PeerConfig::default();
        let mut peer = create_connected_peer(address, config);

        let start_time = current_timestamp();

        // Send ping
        let ping_nonce = 0xDEADBEEF;
        peer.send_ping(ping_nonce).unwrap();

        assert_eq!(peer.last_ping_nonce(), Some(ping_nonce));
        assert!(peer.last_ping_time().is_some());

        // Receive pong with matching nonce
        let pong_delay = Duration::from_millis(50);
        std::thread::sleep(pong_delay);

        peer.receive_pong(ping_nonce, current_timestamp()).unwrap();

        // Verify ping time was recorded
        let ping_time = peer.last_ping_duration().unwrap();
        assert!(ping_time >= pong_delay);
        assert!(ping_time < Duration::from_millis(200)); // Should be reasonable

        // Test mismatched nonce
        let wrong_nonce = 0xCAFEBABE;
        let result = peer.receive_pong(wrong_nonce, current_timestamp());
        assert!(result.is_err());
    }

    /// Test message handling (matches C# message processing exactly)
    #[test]
    fn test_message_handling_compatibility() {
        let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 10333);
        let config = PeerConfig::default();
        let mut peer = create_connected_peer(address, config);

        // Track received messages
        let received_messages = Arc::new(Mutex::new(Vec::new()));
        let received_clone = received_messages.clone();

        peer.on_message_received(move |msg_type, _payload| {
            received_clone.lock().unwrap().push(msg_type);
        });

        // Send various message types
        peer.send_inv(InventoryType::Block, vec![create_test_hash(1)])
            .unwrap();
        peer.send_getdata(InventoryType::TX, vec![create_test_hash(2)])
            .unwrap();
        peer.send_addr(vec![create_test_address()]).unwrap();

        // Simulate receiving messages
        peer.handle_message(MessageType::Inv, &[]).unwrap();
        peer.handle_message(MessageType::GetData, &[]).unwrap();
        peer.handle_message(MessageType::Addr, &[]).unwrap();

        // Verify messages were tracked
        let messages = received_messages.lock().unwrap();
        assert_eq!(messages.len(), 3);
        assert!(messages.contains(&MessageType::Inv));
        assert!(messages.contains(&MessageType::GetData));
        assert!(messages.contains(&MessageType::Addr));

        // Test message statistics
        assert_eq!(peer.messages_sent(), 3);
        assert_eq!(peer.messages_received(), 3);
        assert!(peer.bytes_sent() > 0);
        assert!(peer.bytes_received() > 0);
    }

    /// Test peer timeout handling (matches C# timeout behavior exactly)
    #[test]
    fn test_timeout_handling_compatibility() {
        let config = PeerConfig {
            ping_interval: Duration::from_millis(100),
            max_ping_time: Duration::from_millis(50),
            connection_timeout: Duration::from_millis(200),
            ..Default::default()
        };

        let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 10333);
        let mut peer = Peer::new(address, config);

        // Test connection timeout
        peer.connect().unwrap();
        assert_eq!(peer.state(), PeerState::Connecting);

        // Simulate timeout
        std::thread::sleep(Duration::from_millis(250));
        let timed_out = peer.check_timeout().unwrap();
        assert!(timed_out);
        assert_eq!(peer.state(), PeerState::Disconnected);
        assert_eq!(peer.disconnect_reason(), Some(DisconnectReason::Timeout));

        // Test ping timeout
        let mut peer = create_connected_peer(address, config);

        let ping_nonce = 0x12345678;
        peer.send_ping(ping_nonce).unwrap();

        // Wait longer than max ping time
        std::thread::sleep(Duration::from_millis(100));
        let ping_timed_out = peer.check_ping_timeout().unwrap();
        assert!(ping_timed_out);
        assert_eq!(peer.state(), PeerState::Disconnected);
        assert_eq!(
            peer.disconnect_reason(),
            Some(DisconnectReason::PingTimeout)
        );
    }

    /// Test peer address management (matches C# address handling exactly)
    #[test]
    fn test_address_management_compatibility() {
        let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100)), 10333);
        let config = PeerConfig::default();
        let mut peer = create_connected_peer(address, config);

        // Test address announcement
        let addresses = vec![
            create_network_address("192.168.1.101", 10333),
            create_network_address("192.168.1.102", 10333),
            create_network_address("192.168.1.103", 10333),
        ];

        peer.announce_addresses(&addresses).unwrap();

        // Test getaddr request
        peer.request_addresses().unwrap();

        // Verify address tracking
        assert_eq!(peer.known_addresses().len(), 3);

        let own_address = create_network_address("192.168.1.100", 10333);
        let filtered_addresses = peer.filter_addresses(&[own_address]);
        assert_eq!(filtered_addresses.len(), 0); // Should be filtered out
    }

    /// Test peer synchronization (matches C# sync behavior exactly)
    #[test]
    fn test_synchronization_compatibility() {
        let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 10333);
        let config = PeerConfig::default();
        let mut peer = create_connected_peer(address, config);

        // Set peer height higher than ours
        peer.set_height(100000);

        // Request block headers
        let locator_hashes = vec![create_test_hash(50000)];
        let stop_hash = create_test_hash(60000);

        peer.request_headers(&locator_hashes, &stop_hash).unwrap();

        // Request blocks
        let block_hashes = vec![
            create_test_hash(50001),
            create_test_hash(50002),
            create_test_hash(50003),
        ];

        peer.request_blocks(&block_hashes).unwrap();

        // Test sync status
        assert!(peer.is_syncing());
        assert_eq!(peer.requested_blocks(), 3);

        // Complete sync
        peer.mark_block_received(&create_test_hash(50001));
        peer.mark_block_received(&create_test_hash(50002));
        peer.mark_block_received(&create_test_hash(50003));

        assert!(!peer.is_syncing());
        assert_eq!(peer.requested_blocks(), 0);
    }

    /// Test peer reputation system (matches C# reputation exactly)
    #[test]
    fn test_reputation_system_compatibility() {
        let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 10333);
        let config = PeerConfig::default();
        let mut peer = create_connected_peer(address, config);

        // Initial reputation should be neutral
        assert_eq!(peer.reputation(), 0);

        // Test positive actions
        peer.add_reputation(ReputationChange::ValidBlock, 10);
        peer.add_reputation(ReputationChange::ValidTransaction, 5);
        assert_eq!(peer.reputation(), 15);

        // Test negative actions
        peer.add_reputation(ReputationChange::InvalidBlock, -20);
        peer.add_reputation(ReputationChange::InvalidTransaction, -5);
        assert_eq!(peer.reputation(), -10);

        // Test reputation limits
        peer.add_reputation(ReputationChange::Misbehavior, -1000);
        assert!(peer.reputation() >= -100); // Should be clamped

        peer.add_reputation(ReputationChange::GoodBehavior, 1000);
        assert!(peer.reputation() <= 100); // Should be clamped

        // Test auto-disconnect on bad reputation
        peer.set_reputation(-100);
        let should_disconnect = peer.should_disconnect_for_reputation();
        assert!(should_disconnect);
    }

    /// Test bloom filter support (matches C# bloom filter exactly)
    #[test]
    fn test_bloom_filter_compatibility() {
        let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 10333);
        let config = PeerConfig::default();
        let mut peer = create_connected_peer(address, config);

        // Test filter load
        let bloom_filter = BloomFilter {
            data: vec![0xFF; 1024],
            hash_functions: 5,
            tweak: 12345,
            flags: BloomFilterFlags::UpdateAll,
        };

        peer.load_bloom_filter(bloom_filter.clone()).unwrap();
        assert!(peer.has_bloom_filter());
        assert_eq!(peer.bloom_filter().unwrap().hash_functions, 5);

        // Test filter add
        let data_to_add = vec![0x01, 0x02, 0x03];
        peer.add_to_bloom_filter(&data_to_add).unwrap();

        // Test filter clear
        peer.clear_bloom_filter().unwrap();
        assert!(!peer.has_bloom_filter());

        // Test filtered transaction sending
        peer.load_bloom_filter(bloom_filter).unwrap();
        let tx = create_test_transaction();
        let should_relay = peer.should_relay_transaction(&tx);
        assert!(should_relay); // Should relay if matches filter
    }

    /// Test peer ban system (matches C# ban mechanism exactly)
    #[test]
    fn test_ban_system_compatibility() {
        let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100)), 10333);
        let config = PeerConfig::default();
        let mut peer = create_connected_peer(address, config);

        // Test temporary ban
        let ban_duration = Duration::from_secs(3600); // 1 hour
        peer.ban_temporarily(BanReason::Misbehavior, ban_duration)
            .unwrap();

        assert!(peer.is_banned());
        assert_eq!(peer.ban_reason(), Some(BanReason::Misbehavior));
        assert!(peer.ban_expiry().is_some());

        // Test ban expiry
        assert!(!peer.is_ban_expired());

        // Test permanent ban
        peer.ban_permanently(BanReason::Protocol).unwrap();
        assert!(peer.is_banned());
        assert!(peer.ban_expiry().is_none()); // Permanent

        // Test unban
        peer.unban().unwrap();
        assert!(!peer.is_banned());
        assert_eq!(peer.ban_reason(), None);
    }

    /// Test peer statistics (matches C# statistics exactly)
    #[test]
    fn test_peer_statistics_compatibility() {
        let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 10333);
        let config = PeerConfig::default();
        let mut peer = create_connected_peer(address, config);

        // Initial stats
        let stats = peer.statistics();
        assert_eq!(stats.messages_sent, 0);
        assert_eq!(stats.messages_received, 0);
        assert_eq!(stats.bytes_sent, 0);
        assert_eq!(stats.bytes_received, 0);
        assert_eq!(stats.connection_time, Duration::from_secs(0));

        // Simulate activity
        peer.increment_messages_sent(5);
        peer.increment_messages_received(3);
        peer.increment_bytes_sent(1024);
        peer.increment_bytes_received(512);

        std::thread::sleep(Duration::from_millis(100));
        peer.update_connection_time();

        let updated_stats = peer.statistics();
        assert_eq!(updated_stats.messages_sent, 5);
        assert_eq!(updated_stats.messages_received, 3);
        assert_eq!(updated_stats.bytes_sent, 1024);
        assert_eq!(updated_stats.bytes_received, 512);
        assert!(updated_stats.connection_time > Duration::from_millis(50));

        // Test averages
        assert!(updated_stats.average_ping_time().is_none()); // No pings yet

        // Add ping measurements
        peer.record_ping_time(Duration::from_millis(50));
        peer.record_ping_time(Duration::from_millis(100));
        peer.record_ping_time(Duration::from_millis(75));

        let avg_ping = peer.statistics().average_ping_time().unwrap();
        assert_eq!(avg_ping, Duration::from_millis(75));
    }

    /// Test concurrent peer operations (matches C# thread safety exactly)
    #[test]
    fn test_concurrent_operations_compatibility() {
        let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 10333);
        let config = PeerConfig::default();
        let peer = Arc::new(Mutex::new(create_connected_peer(address, config)));

        let handles = (0..10)
            .map(|i| {
                let peer_clone = peer.clone();
                std::thread::spawn(move || {
                    let mut p = peer_clone.lock().unwrap();
                    p.increment_messages_sent(1);
                    p.increment_bytes_sent(100);

                    // Simulate ping
                    let nonce = i as u64;
                    let _ = p.send_ping(nonce);
                    let _ = p.receive_pong(nonce, current_timestamp());
                })
            })
            .collect::<Vec<_>>();

        for handle in handles {
            handle.join().unwrap();
        }

        // Verify concurrent operations worked
        let final_peer = peer.lock().unwrap();
        assert_eq!(final_peer.messages_sent(), 10);
        assert_eq!(final_peer.bytes_sent(), 1000);
    }

    // Helper functions

    fn create_connected_peer(address: SocketAddr, config: PeerConfig) -> Peer {
        let mut peer = Peer::new(address, config);

        // Simulate full connection and handshake
        peer.connect().unwrap();
        peer.set_connected(true);

        let version_msg = VersionMessage {
            version: 0x00,
            services: NodeServices::NodeNetwork as u64,
            timestamp: current_timestamp(),
            port: 10333,
            nonce: 0x1234567890ABCDEF,
            user_agent: "/NEO:3.6.0/".to_string(),
            start_height: 50000,
        };

        peer.send_version(version_msg.clone()).unwrap();
        peer.receive_version(version_msg).unwrap();
        peer.send_verack().unwrap();
        peer.receive_verack().unwrap();

        peer
    }

    fn create_test_hash(value: u64) -> UInt256 {
        let mut bytes = [0u8; 32];
        bytes[0..8].copy_from_slice(&value.to_le_bytes());
        UInt256::from_bytes(&bytes).unwrap()
    }

    fn create_test_address() -> NetworkAddress {
        NetworkAddress {
            timestamp: current_timestamp(),
            services: NodeServices::NodeNetwork as u64,
            address: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 200)),
            port: 10333,
        }
    }

    fn create_network_address(ip: &str, port: u16) -> NetworkAddress {
        NetworkAddress {
            timestamp: current_timestamp(),
            services: NodeServices::NodeNetwork as u64,
            address: ip.parse().unwrap(),
            port,
        }
    }

    fn create_test_transaction() -> Transaction {
        Transaction {
            version: 0,
            nonce: 123456,
            system_fee: 0,
            network_fee: 0,
            valid_until_block: 999999,
            attributes: vec![],
            signers: vec![],
            script: vec![0x51], // PUSH1
            witnesses: vec![],
        }
    }

    fn current_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}
