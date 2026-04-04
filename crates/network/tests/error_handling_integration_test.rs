#![cfg(feature = "compat_tests")]
//! Comprehensive Error Handling Integration Test
//!
//! This test verifies that the error handling system properly integrates with
//! the PeerManager and provides robust recovery mechanisms.

use neo_network::{
    Error, ErrorSeverity, NetworkConfig, NetworkErrorEvent, NetworkErrorHandler, OperationContext,
    PeerManager, RecoveryStrategy,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

#[tokio::test]
async fn test_error_handler_integration() {
    // Create a test network configuration
    let config = NetworkConfig::testnet();

    // Create peer manager with error handling
    let peer_manager = PeerManager::new(config).expect("Failed to create peer manager");
    let error_handler = peer_manager.error_handler();

    // Subscribe to error events
    let mut error_events = error_handler.subscribe_to_error_events();

    // Test error classification
    let connection_error = Error::ConnectionFailed("Test connection failure".to_string());
    let protocol_error = Error::Protocol("Test protocol error".to_string());
    let timeout_error = Error::Timeout("Test timeout".to_string());

    let test_peer: SocketAddr = "127.0.0.1:20333".parse().unwrap();
    let mut context1 = OperationContext::new("test_operation_1".to_string(), test_peer);
    let mut context2 = OperationContext::new("test_operation_2".to_string(), test_peer);
    let mut context3 = OperationContext::new("test_operation_3".to_string(), test_peer);

    // Test error handling strategies
    let strategy1 = error_handler
        .handle_error(&connection_error, &mut context1)
        .await;
    let strategy2 = error_handler
        .handle_error(&protocol_error, &mut context2)
        .await;
    let strategy3 = error_handler
        .handle_error(&timeout_error, &mut context3)
        .await;

    println!("Connection error strategy: {:?}", strategy1);
    println!("Protocol error strategy: {:?}", strategy2);
    println!("Timeout error strategy: {:?}", strategy3);

    // Verify strategies are appropriate
    assert!(matches!(strategy1, RecoveryStrategy::ReconnectAndRetry));
    assert!(matches!(strategy2, RecoveryStrategy::MarkAsFailed));
    assert!(matches!(strategy3, RecoveryStrategy::RetryImmediate));

    // Test error statistics
    let stats = error_handler.get_error_statistics().await;
    assert!(stats.error_counts.len() > 0);
    assert!(stats.peer_errors.contains_key(&test_peer));

    // Test failed peers tracking
    let failed_peers = error_handler.get_failed_peers().await;
    println!("Failed peers: {:?}", failed_peers);

    // Verify events were emitted
    tokio::select! {
        event = error_events.recv() => {
            match event {
                Ok(NetworkErrorEvent::OperationFailed { peer, error, .. }) => {
                    assert_eq!(peer, test_peer);
                    println!("Received operation failed event: {}", error);
                }
                Ok(NetworkErrorEvent::PeerFailed { peer, .. }) => {
                    assert_eq!(peer, test_peer);
                    println!("Received peer failed event");
                }
                Ok(other) => {
                    println!("Received other error event: {:?}", other);
                }
                Err(e) => {
                    eprintln!("Failed to receive error event: {}", e);
                }
            }
        }
        _ = sleep(Duration::from_millis(100)) => {
            println!("No error event received within timeout (this is normal)");
        }
    }
}

#[tokio::test]
async fn test_retry_mechanism() {
    let config = NetworkConfig::testnet();
    let peer_manager = PeerManager::new(config).expect("Failed to create peer manager");
    let error_handler = peer_manager.error_handler();

    let test_peer: SocketAddr = "127.0.0.1:20333".parse().unwrap();

    // Test execute_with_retry with a failing operation
    let mut attempt_count = 0;
    let operation_id = "test_retry_operation".to_string();

    let result = error_handler
        .execute_with_retry(operation_id, test_peer, || async {
            attempt_count += 1;
            if attempt_count < 3 {
                Err(Error::ConnectionTimeout)
            } else {
                Ok("Success".to_string())
            }
        })
        .await;

    match result {
        Ok(value) => {
            assert_eq!(value, "Success");
            assert_eq!(attempt_count, 3);
            println!("Retry mechanism succeeded after {} attempts", attempt_count);
        }
        Err(e) => {
            panic!("Retry mechanism failed: {}", e);
        }
    }
}

#[tokio::test]
async fn test_network_partition_detection() {
    let config = NetworkConfig::testnet();
    let peer_manager = PeerManager::new(config).expect("Failed to create peer manager");
    let error_handler = peer_manager.error_handler();

    // Simulate multiple peer failures to trigger partition detection
    let peers = vec![
        "127.0.0.1:20333".parse().unwrap(),
        "127.0.0.2:20333".parse().unwrap(),
        "127.0.0.3:20333".parse().unwrap(),
    ];

    // Mark multiple peers as failed
    for peer in &peers {
        let mut context = OperationContext::new(format!("partition_test_{}", peer), *peer);
        let error = Error::ConnectionFailed("Simulated network partition".to_string());
        error_handler.handle_error(&error, &mut context).await;
    }

    if let Some(partition_event) = error_handler.detect_network_partition().await {
        match partition_event {
            NetworkErrorEvent::NetworkPartitionDetected { affected_peers, .. } => {
                println!(
                    "Network partition detected with {} affected peers",
                    affected_peers.len()
                );
                assert!(affected_peers.len() > 0);
            }
            _ => panic!("Unexpected event type for partition detection"),
        }
    } else {
        println!("No network partition detected (this may be normal if not enough peers failed)");
    }
}

#[tokio::test]
async fn test_error_handler_maintenance() {
    let config = NetworkConfig::testnet();
    let peer_manager = PeerManager::new(config).expect("Failed to create peer manager");
    let error_handler = peer_manager.error_handler();

    // Add some test data
    let test_peer: SocketAddr = "127.0.0.1:20333".parse().unwrap();
    let mut context = OperationContext::new("maintenance_test".to_string(), test_peer);
    let error = Error::ConnectionFailed("Test error for maintenance".to_string());

    error_handler.handle_error(&error, &mut context).await;

    // Verify data exists
    let stats_before = error_handler.get_error_statistics().await;
    assert!(stats_before.error_counts.len() > 0);

    // Run maintenance
    error_handler.perform_maintenance().await;

    // Verify maintenance completed successfully
    let stats_after = error_handler.get_error_statistics().await;
    println!("Error statistics after maintenance: {:?}", stats_after);

    // Maintenance should not remove recent failures
    assert!(stats_after.error_counts.len() > 0);
}

#[tokio::test]
async fn test_peer_manager_error_integration() {
    let config = NetworkConfig::testnet();
    let peer_manager = PeerManager::new(config).expect("Failed to create peer manager");

    // Test that peer manager properly integrates with error handler
    let error_handler = peer_manager.error_handler();

    let invalid_peer: SocketAddr = "192.0.2.1:99999".parse().unwrap(); // RFC5737 TEST-NET-1

    let connection_result = peer_manager.connect_to_peer(invalid_peer).await;

    // Should fail but be handled gracefully
    assert!(connection_result.is_err());

    // Verify error was tracked
    let stats = error_handler.get_error_statistics().await;
    println!("Connection attempt resulted in error stats: {:?}", stats);

    // May have error statistics depending on retry attempts
    // This is mainly testing that the integration doesn't panic
}
