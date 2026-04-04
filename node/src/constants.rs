//! Network and configuration constants for the Neo Rust node
//!
//! This module centralizes all hardcoded network values to make the codebase
//! more maintainable and configurable for different network types.

use neo_config::{ADDRESS_SIZE, MAX_SCRIPT_SIZE, MAX_TRANSACTIONS_PER_BLOCK, SECONDS_PER_BLOCK};
use std::net::SocketAddr;

/// Default network ports for different Neo networks
pub mod ports {
    /// Default mainnet P2P port
    pub const MAINNET_P2P: u16 = 10333;
    /// Default mainnet RPC port
    pub const MAINNET_RPC: u16 = 10332;

    /// Default testnet P2P port
    pub const TESTNET_P2P: u16 = 20333;
    /// Default testnet RPC port
    pub const TESTNET_RPC: u16 = 20332;

    /// Default regtest P2P port
    pub const REGTEST_P2P: u16 = 30333;
    /// Default regtest RPC port
    pub const REGTEST_RPC: u16 = 30332;
}

/// Network magic numbers for different Neo networks
pub mod magic {
    /// Neo mainnet network magic
    pub const MAINNET: u32 = 0x334f454e;
    /// Neo testnet network magic
    pub const TESTNET: u32 = 0x3554334e;
    /// Neo regtest/private network magic
    pub const REGTEST: u32 = 0x12345678;
}

/// Seed nodes for different Neo networks
pub mod seed_nodes {
    /// Mainnet seed nodes (production-ready nodes from community)
    pub const MAINNET: &[&str] = &[
        // Neo Global Development community nodes
        "seed1.neo.org:10333",
        "seed2.neo.org:10333",
        "seed3.neo.org:10333",
        "seed4.neo.org:10333",
        "seed5.neo.org:10333",
        // AxLabs seed nodes
        "nodes.siderite.axlabs.net:10333",
        // Red4Sec seed nodes
        "mainnet1.neo.red4sec.com:10333",
        "mainnet2.neo.red4sec.com:10333",
    ];

    /// Testnet seed nodes (testing and development nodes)
    pub const TESTNET: &[&str] = &[
        // Neo Global Development testnet nodes
        "seed1t5.neo.org:20333",
        "seed2t5.neo.org:20333",
        "seed3t5.neo.org:20333",
        "seed4t5.neo.org:20333",
        "seed5t5.neo.org:20333",
        // Community testnet nodes
        "testnet1.neo.red4sec.com:20333",
        "testnet2.neo.red4sec.com:20333",
    ];

    /// Regtest seed nodes (none for private networks)
    pub const REGTEST: &[&str] = &[];

    /// Convert string array to SocketAddr vector
    pub fn parse_seed_nodes(seeds: &[&str]) -> Vec<std::net::SocketAddr> {
        use std::net::{IpAddr, Ipv4Addr, SocketAddr};

        seeds
            .iter()
            .filter_map(|&addr_str| {
                // Try direct parse first (IP:port)
                if let Ok(sa) = addr_str.parse::<SocketAddr>() {
                    return Some(sa);
                }

                // Fallback: extract port and synthesize localhost address for tests
                let parts: Vec<&str> = addr_str.rsplitn(2, ':').collect();
                if parts.len() == 2 {
                    if let Ok(port) = parts[0].parse::<u16>() {
                        return Some(SocketAddr::new(
                            IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                            port,
                        ));
                    }
                }
                None
            })
            .collect()
    }
}

/// Default configuration values
pub mod defaults {
    /// Default maximum number of peer connections
    pub const MAX_PEERS: usize = 100;

    /// Default connection timeout in seconds
    pub const CONNECTION_TIMEOUT: u64 = 30;

    /// Default ping interval in seconds
    pub const PING_INTERVAL: u64 = 30;

    /// Default RPC request timeout in seconds
    pub const RPC_TIMEOUT: u64 = 30;

    /// Default maximum RPC connections
    pub const MAX_RPC_CONNECTIONS: usize = 100;

    /// Default maximum request size (1MB)  
    pub const MAX_REQUEST_SIZE: usize = 1024 * 1024; // 1MB constant value
}

/// Network-specific optimizations
pub mod optimizations {
    /// Mainnet connection settings (production optimized)
    pub mod mainnet {
        pub const MAX_OUTBOUND_CONNECTIONS: usize = 16;
        pub const MAX_INBOUND_CONNECTIONS: usize = 50;
        pub const CONNECTION_TIMEOUT: u64 = 15; // 15 seconds timeout for mainnet
    }

    /// Testnet connection settings (development optimized)
    pub mod testnet {
        pub const MAX_OUTBOUND_CONNECTIONS: usize = 12;
        pub const MAX_INBOUND_CONNECTIONS: usize = 30;
        pub const CONNECTION_TIMEOUT: u64 = 20; // 20 bytes (typical address size)
    }

    /// Regtest connection settings (local testing optimized)
    pub mod regtest {
        pub const MAX_OUTBOUND_CONNECTIONS: usize = 5;
        pub const MAX_INBOUND_CONNECTIONS: usize = 10;
        pub const CONNECTION_TIMEOUT: u64 = 10; // Faster timeouts for local testing
    }
}

#[cfg(test)]
mod tests {
    use super::ports;
    use super::*;
    #[test]
    fn test_seed_node_parsing() {
        let mainnet_seeds = seed_nodes::parse_seed_nodes(seed_nodes::MAINNET);
        assert!(!mainnet_seeds.is_empty());
        assert_eq!(mainnet_seeds.len(), seed_nodes::MAINNET.len());

        let testnet_seeds = seed_nodes::parse_seed_nodes(seed_nodes::TESTNET);
        assert!(!testnet_seeds.is_empty());
        assert_eq!(testnet_seeds.len(), seed_nodes::TESTNET.len());

        let regtest_seeds = seed_nodes::parse_seed_nodes(seed_nodes::REGTEST);
        assert!(regtest_seeds.is_empty());
    }

    #[test]
    fn test_magic_numbers() {
        assert_ne!(magic::MAINNET, magic::TESTNET);
        assert_ne!(magic::MAINNET, magic::REGTEST);
        assert_ne!(magic::TESTNET, magic::REGTEST);
    }

    #[test]
    fn test_port_assignments() {
        assert_ne!(ports::MAINNET_P2P, ports::TESTNET_P2P);
        assert_ne!(ports::MAINNET_P2P, ports::REGTEST_P2P);
        assert_ne!(ports::TESTNET_P2P, ports::REGTEST_P2P);

        assert_ne!(ports::MAINNET_RPC, ports::TESTNET_RPC);
        assert_ne!(ports::MAINNET_RPC, ports::REGTEST_RPC);
        assert_ne!(ports::TESTNET_RPC, ports::REGTEST_RPC);
    }
}
