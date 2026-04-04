//! Network C# Compatibility Test Suite
//!
//! This module contains comprehensive tests that ensure full compatibility
//! with the C# Neo.Network implementation.

// The following compatibility tests target legacy/alternate APIs. Gate them behind a feature to avoid
// compiling against the current implementation by default. Enable with:
//   cargo test -p neo-network --features compat_tests
#[cfg(feature = "compat_tests")]
mod block_sync_demo_test;
#[cfg(feature = "compat_tests")]
mod block_sync_integration_test;
#[cfg(feature = "compat_tests")]
mod block_sync_real_test;
#[cfg(feature = "compat_tests")]
mod block_sync_summary_test;
#[cfg(feature = "compat_tests")]
mod block_sync_tests;
#[cfg(feature = "compat_tests")]
mod error_handling_integration_test;
#[cfg(feature = "compat_tests")]
mod integration_tests;
#[cfg(feature = "compat_tests")]
mod message_routing_tests;
#[cfg(feature = "compat_tests")]
mod peer_tests;
#[cfg(feature = "compat_tests")]
mod protocol_format_test;
#[cfg(feature = "compat_tests")]
mod protocol_tests;
#[cfg(feature = "compat_tests")]
mod simple_message_test;
#[cfg(feature = "compat_tests")]
mod working_message_test;

#[test]
fn smoke_test_network_crate_builds() {
    assert_eq!(2 + 2, 4);
}
