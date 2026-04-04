#![cfg(feature = "compat_tests")]
//! Demonstration test for block sync functionality
//!
//! This test shows how the block sync components work together

use neo_network::sync::{SyncEvent, SyncState};
use std::time::Duration;
use tokio::time::timeout;

#[tokio::test]
async fn test_sync_event_flow() {
    // Create event channel similar to sync manager
    let (event_tx, mut event_rx) = tokio::sync::broadcast::channel(100);

    // Simulate sync flow
    tokio::spawn(async move {
        // 1. Sync starts
        let _ = event_tx.send(SyncEvent::SyncStarted { target_height: 100 });
        tokio::time::sleep(Duration::from_millis(10)).await;

        // 2. Headers progress
        for i in (0..=100).step_by(20) {
            let _ = event_tx.send(SyncEvent::HeadersProgress {
                current: i,
                target: 100,
            });
            tokio::time::sleep(Duration::from_millis(5)).await;
        }

        // 3. Blocks progress
        for i in (0..=100).step_by(10) {
            let _ = event_tx.send(SyncEvent::BlocksProgress {
                current: i,
                target: 100,
            });
            tokio::time::sleep(Duration::from_millis(5)).await;
        }

        // 4. Sync completes
        let _ = event_tx.send(SyncEvent::SyncCompleted { final_height: 100 });
    });

    // Monitor events
    let mut events_received = Vec::new();
    let result = timeout(Duration::from_secs(2), async {
        while let Ok(event) = event_rx.recv().await {
            match &event {
                SyncEvent::SyncStarted { target_height } => {
                    assert_eq!(*target_height, 100);
                    events_received.push("start");
                }
                SyncEvent::HeadersProgress { current, target } => {
                    assert!(*current <= *target);
                    if *current == *target {
                        events_received.push("headers_done");
                    }
                }
                SyncEvent::BlocksProgress { current, target } => {
                    assert!(*current <= *target);
                    if *current == *target {
                        events_received.push("blocks_done");
                    }
                }
                SyncEvent::SyncCompleted { final_height } => {
                    assert_eq!(*final_height, 100);
                    events_received.push("complete");
                    break;
                }
                _ => {}
            }
        }
    })
    .await;

    assert!(result.is_ok(), "Should complete within timeout");
    assert!(events_received.contains(&"start"));
    assert!(events_received.contains(&"complete"));
}

#[test]
fn test_sync_state_machine() {
    // Test state transitions
    let states = vec![
        (SyncState::Idle, "Idle"),
        (SyncState::SyncingHeaders, "Syncing Headers"),
        (SyncState::SyncingBlocks, "Syncing Blocks"),
        (SyncState::LoadingSnapshot, "Loading Snapshot"),
        (SyncState::Synchronized, "Synchronized"),
        (SyncState::Failed, "Failed"),
    ];

    for (state, expected_display) in states {
        assert_eq!(format!("{}", state), expected_display);
    }

    // Test state equality
    assert_eq!(SyncState::Idle, SyncState::Idle);
    assert_ne!(SyncState::Idle, SyncState::Synchronized);
}

#[test]
fn test_block_sync_constants() {
    use neo_network::sync::{MAX_BLOCKS_PER_REQUEST, MAX_HEADERS_PER_REQUEST, SYNC_TIMEOUT};

    // Verify sync constants are reasonable
    assert!(MAX_BLOCKS_PER_REQUEST > 0 && MAX_BLOCKS_PER_REQUEST <= 500);
    assert!(MAX_HEADERS_PER_REQUEST > 0 && MAX_HEADERS_PER_REQUEST <= 2000);
    assert!(SYNC_TIMEOUT.as_secs() >= 30); // At least 30 seconds
}

#[tokio::test]
async fn test_sync_progress_calculation() {
    // Simulate progress tracking
    let mut current_height = 0u32;
    let target_height = 1000u32;

    let mut progress_updates = Vec::new();

    // Simulate sync progress
    while current_height < target_height {
        current_height += 100; // Sync 100 blocks at a time
        let progress = (current_height as f64 / target_height as f64) * 100.0;
        progress_updates.push(progress);

        // Verify progress is increasing
        if progress_updates.len() > 1 {
            let prev = progress_updates[progress_updates.len() - 2];
            assert!(progress > prev, "Progress should increase");
        }
    }

    assert_eq!(current_height, target_height);
    assert_eq!(*progress_updates.last().unwrap(), 100.0);
}

#[tokio::test]
async fn test_new_best_height_event() {
    let (tx, mut rx) = tokio::sync::broadcast::channel(10);

    // Simulate height updates
    let heights = vec![10, 50, 100, 500, 1000];

    for height in &heights {
        let _ = tx.send(SyncEvent::NewBestHeight {
            height: *height,
            peer: "127.0.0.1:20333".parse().unwrap(),
        });
    }

    // Verify all heights received
    let mut received_heights = Vec::new();
    while let Ok(event) = rx.try_recv() {
        if let SyncEvent::NewBestHeight { height, .. } = event {
            received_heights.push(height);
        }
    }

    assert_eq!(received_heights, heights);
}
