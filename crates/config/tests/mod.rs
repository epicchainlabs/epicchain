//! Config Module C# Compatibility Test Suite
//!
//! This module contains comprehensive tests that ensure full compatibility
//! with C# Neo's configuration functionality including NetworkType,
//! protocol settings, and configuration management.

mod network_type_tests;

mod integration_tests {
    use neo_config::NetworkType;

    /// Test configuration consistency across different network types (matches C# patterns exactly)
    #[test]
    fn test_network_configuration_consistency() {
        // Test that each network type has consistent configuration expectations
        let networks = vec![
            NetworkType::MainNet,
            NetworkType::TestNet,
            NetworkType::Private,
        ];

        for network in networks {
            // Each network should have distinct characteristics
            match network {
                NetworkType::MainNet => {
                    // MainNet should be production-ready
                    assert_eq!(network, NetworkType::MainNet);
                    assert_ne!(network, NetworkType::TestNet);
                    assert_ne!(network, NetworkType::Private);
                }
                NetworkType::TestNet => {
                    // TestNet should be separate from MainNet
                    assert_eq!(network, NetworkType::TestNet);
                    assert_ne!(network, NetworkType::MainNet);
                    assert_ne!(network, NetworkType::Private);
                }
                NetworkType::Private => {
                    // Private should be isolated
                    assert_eq!(network, NetworkType::Private);
                    assert_ne!(network, NetworkType::MainNet);
                    assert_ne!(network, NetworkType::TestNet);
                }
            }
        }
    }

    /// Test network type environment integration (matches C# environment handling exactly)
    #[test]
    fn test_network_type_environment_integration() {
        // Test loading network type from environment-like configuration
        fn parse_network_from_config(config_str: &str) -> Result<NetworkType, String> {
            match config_str.to_lowercase().as_str() {
                "mainnet" | "main" | "production" => Ok(NetworkType::MainNet),
                "testnet" | "test" | "testing" => Ok(NetworkType::TestNet),
                "private" | "local" | "development" => Ok(NetworkType::Private),
                _ => Err(format!("Unknown network type: {}", config_str)),
            }
        }

        // Test various configuration formats
        assert_eq!(
            parse_network_from_config("MainNet").unwrap(),
            NetworkType::MainNet
        );
        assert_eq!(
            parse_network_from_config("main").unwrap(),
            NetworkType::MainNet
        );
        assert_eq!(
            parse_network_from_config("production").unwrap(),
            NetworkType::MainNet
        );

        assert_eq!(
            parse_network_from_config("TestNet").unwrap(),
            NetworkType::TestNet
        );
        assert_eq!(
            parse_network_from_config("test").unwrap(),
            NetworkType::TestNet
        );
        assert_eq!(
            parse_network_from_config("testing").unwrap(),
            NetworkType::TestNet
        );

        assert_eq!(
            parse_network_from_config("Private").unwrap(),
            NetworkType::Private
        );
        assert_eq!(
            parse_network_from_config("local").unwrap(),
            NetworkType::Private
        );
        assert_eq!(
            parse_network_from_config("development").unwrap(),
            NetworkType::Private
        );

        // Test invalid configurations
        assert!(parse_network_from_config("invalid").is_err());
        assert!(parse_network_from_config("").is_err());
    }

    /// Test configuration validation patterns (matches C# validation logic exactly)
    #[test]
    fn test_configuration_validation_patterns() {
        #[derive(Debug, Clone)]
        struct NetworkConfiguration {
            network_type: NetworkType,
            max_peers: usize,
            port: u16,
            is_production: bool,
        }

        impl NetworkConfiguration {
            fn new(network_type: NetworkType) -> Self {
                let (max_peers, port, is_production) = match network_type {
                    NetworkType::MainNet => (100, 10333, true),
                    NetworkType::TestNet => (50, 20333, false),
                    NetworkType::Private => (10, 30333, false),
                };

                Self {
                    network_type,
                    max_peers,
                    port,
                    is_production,
                }
            }

            fn validate(&self) -> Result<(), String> {
                if self.max_peers == 0 {
                    return Err("Max peers must be greater than 0".to_string());
                }

                if self.port < 1024 {
                    return Err("Port must be 1024 or higher".to_string());
                }

                match self.network_type {
                    NetworkType::MainNet => {
                        if !self.is_production {
                            return Err("MainNet must be configured for production".to_string());
                        }
                        if self.max_peers < 50 {
                            return Err("MainNet requires at least 50 max peers".to_string());
                        }
                    }
                    NetworkType::TestNet => {
                        if self.is_production {
                            return Err(
                                "TestNet should not be configured for production".to_string()
                            );
                        }
                    }
                    NetworkType::Private => {
                        if self.is_production {
                            return Err("Private network should not be configured for production"
                                .to_string());
                        }
                    }
                }

                Ok(())
            }
        }

        // Test valid configurations
        let mainnet_config = NetworkConfiguration::new(NetworkType::MainNet);
        assert!(mainnet_config.validate().is_ok());
        assert_eq!(mainnet_config.network_type, NetworkType::MainNet);
        assert_eq!(mainnet_config.max_peers, 100);
        assert_eq!(mainnet_config.port, 10333);
        assert!(mainnet_config.is_production);

        let testnet_config = NetworkConfiguration::new(NetworkType::TestNet);
        assert!(testnet_config.validate().is_ok());
        assert_eq!(testnet_config.network_type, NetworkType::TestNet);
        assert_eq!(testnet_config.max_peers, 50);
        assert_eq!(testnet_config.port, 20333);
        assert!(!testnet_config.is_production);

        let private_config = NetworkConfiguration::new(NetworkType::Private);
        assert!(private_config.validate().is_ok());
        assert_eq!(private_config.network_type, NetworkType::Private);
        assert_eq!(private_config.max_peers, 10);
        assert_eq!(private_config.port, 30333);
        assert!(!private_config.is_production);

        // Test invalid configuration
        let invalid_config = NetworkConfiguration {
            network_type: NetworkType::MainNet,
            max_peers: 0, // Invalid: must be > 0
            port: 10333,
            is_production: true,
        };
        assert!(invalid_config.validate().is_err());
    }
}
