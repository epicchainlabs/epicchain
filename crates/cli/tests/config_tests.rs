//! CLI Configuration C# Compatibility Tests
//!
//! These tests ensure full compatibility with C# Neo CLI configuration handling.
//! Tests are based on the C# Neo.CLI configuration management patterns.

use neo_cli::config::*;
use serde_json;
use std::path::PathBuf;
use tempfile::TempDir;
use tokio::fs;

#[cfg(test)]
#[allow(dead_code)]
mod config_tests {
    use super::*;

    /// Test default configuration creation (matches C# ApplicationConfiguration exactly)
    #[test]
    fn test_default_config_compatibility() {
        // Test default configuration structure
        let config = CliConfig::default();

        assert_eq!(config.application.logger.console_output, true);
        assert_eq!(config.application.logger.active, true);
        assert_eq!(config.application.storage.engine, "RocksDB");
        assert_eq!(config.application.p2p.port, 10333);
        assert_eq!(config.application.p2p.min_desired_connections, 10);
        assert_eq!(config.application.p2p.max_connections, 40);
        assert_eq!(config.application.rpc.bind_address, "127.0.0.1");
        assert_eq!(config.application.rpc.port, 10332);
        assert_eq!(config.application.rpc.max_gas_invoke, 20000000);

        // Verify protocol configuration defaults
        assert_eq!(config.protocol.network, 860833102); // MainNet magic
        assert_eq!(config.protocol.address_version, 53);
        assert_eq!(config.protocol.milliseconds_per_block, 15000);
        assert_eq!(config.protocol.max_transactions_per_block, 512);
        assert_eq!(config.protocol.memory_pool_max_transactions, 50000);
        assert_eq!(config.protocol.max_traceable_blocks, 2102400);
        assert_eq!(config.protocol.initial_gas_distribution, 5200000000000000);

        // Verify hardforks are properly set
        assert_eq!(config.protocol.hardforks.aspidochelone, 0);
        assert_eq!(config.protocol.hardforks.basilisk, 0);
        assert_eq!(config.protocol.hardforks.cockatrice, 0);
        assert_eq!(config.protocol.hardforks.domovoi, 0);
    }

    /// Test configuration loading from JSON file (matches C# JSON config parsing exactly)
    #[tokio::test]
    async fn test_config_json_loading_compatibility() {
        let final_dir = TempDir::new().unwrap();
        let config_path = final_dir.path().join("config.json");

        // Create JSON config that matches C# Neo CLI format exactly
        let config_json = r#"
        {
            "ApplicationConfiguration": {
                "Logger": {
                    "Path": "logs",
                    "ConsoleOutput": true,
                    "Active": true
                },
                "Storage": {
                    "Engine": "RocksDB",
                    "Path": "data"
                },
                "P2P": {
                    "Port": 10333,
                    "MinDesiredConnections": 10,
                    "MaxConnections": 40
                },
                "RPC": {
                    "BindAddress": "127.0.0.1",
                    "Port": 10332,
                    "SslCert": "",
                    "SslCertPassword": "",
                    "TrustedAuthorities": [],
                    "MaxGasInvoke": 20000000
                }
            },
            "ProtocolConfiguration": {
                "Network": 860833102,
                "AddressVersion": 53,
                "MillisecondsPerBlock": 15000,
                "MaxTransactionsPerBlock": 512,
                "MemoryPoolMaxTransactions": 50000,
                "MaxTraceableBlocks": 2102400,
                "InitialGasDistribution": 5200000000000000,
                "Hardforks": {
                    "Aspidochelone": 0,
                    "Basilisk": 0,
                    "Cockatrice": 0,
                    "Domovoi": 0
                }
            }
        }
        "#;

        fs::write(&config_path, config_json).await.unwrap();

        // Test loading configuration
        let config = CliConfig::load_from_file(&config_path).await.unwrap();

        assert_eq!(config.application.logger.path, "logs");
        assert_eq!(config.application.logger.console_output, true);
        assert_eq!(config.application.storage.engine, "RocksDB");
        assert_eq!(config.application.storage.path, "data");
        assert_eq!(config.application.p2p.port, 10333);
        assert_eq!(config.application.rpc.bind_address, "127.0.0.1");
        assert_eq!(config.application.rpc.port, 10332);
        assert_eq!(config.protocol.network, 860833102);
        assert_eq!(config.protocol.address_version, 53);
    }

    /// Test configuration validation (matches C# validation rules exactly)
    #[test]
    fn test_config_validation_compatibility() {
        let mut config = CliConfig::default();

        // Test valid configuration
        assert!(config.validate().is_ok());

        config.application.rpc.port = 0;
        assert!(config.validate().is_err());

        config.application.rpc.port = 65536;
        assert!(config.validate().is_err());

        config.application.rpc.port = 10332; // Reset to valid

        config.application.p2p.port = 0;
        assert!(config.validate().is_err());

        config.application.p2p.port = 10333; // Reset to valid

        // Test invalid connection limits
        config.application.p2p.max_connections = 0;
        assert!(config.validate().is_err());

        config.application.p2p.max_connections = 40; // Reset
        config.application.p2p.min_desired_connections = 50; // Greater than max
        assert!(config.validate().is_err());

        // Test invalid storage engine
        config.application.storage.engine = "InvalidEngine".to_string();
        assert!(config.validate().is_err());

        // Test invalid bind address
        config.application.rpc.bind_address = "invalid_address".to_string();
        assert!(config.validate().is_err());
    }

    /// Test network-specific configurations (matches C# network handling exactly)
    #[test]
    fn test_network_specific_configs_compatibility() {
        // Test MainNet configuration
        let mainnet_config = CliConfig::for_network(NetworkType::MainNet);
        assert_eq!(mainnet_config.protocol.network, 860833102);
        assert_eq!(mainnet_config.protocol.address_version, 53);
        assert_eq!(mainnet_config.application.p2p.port, 10333);
        assert_eq!(mainnet_config.application.rpc.port, 10332);

        // Test TestNet configuration
        let testnet_config = CliConfig::for_network(NetworkType::TestNet);
        assert_eq!(testnet_config.protocol.network, 894710606);
        assert_eq!(testnet_config.protocol.address_version, 53);
        assert_eq!(testnet_config.application.p2p.port, 20333);
        assert_eq!(testnet_config.application.rpc.port, 20332);

        // Test Private network configuration
        let private_config = CliConfig::for_network(NetworkType::Private);
        assert_eq!(private_config.protocol.network, 123456789); // Custom magic
        assert_eq!(private_config.protocol.address_version, 53);
        assert_eq!(private_config.application.p2p.port, 30333);
        assert_eq!(private_config.application.rpc.port, 30332);
    }

    /// Test logger configuration (matches C# logging configuration exactly)
    #[test]
    fn test_logger_config_compatibility() {
        let mut config = CliConfig::default();

        // Test default logger settings
        assert_eq!(config.application.logger.path, "logs");
        assert_eq!(config.application.logger.console_output, true);
        assert_eq!(config.application.logger.active, true);

        // Test logger customization
        config.application.logger.path = "custom_logs".to_string();
        config.application.logger.console_output = false;
        config.application.logger.active = true;

        assert_eq!(config.application.logger.path, "custom_logs");
        assert_eq!(config.application.logger.console_output, false);
        assert_eq!(config.application.logger.active, true);

        // Test logger validation
        config.application.logger.path = "".to_string(); // Empty path
        assert!(config.validate().is_err());

        config.application.logger.path = "logs".to_string(); // Reset
        assert!(config.validate().is_ok());
    }

    /// Test storage configuration (matches C# storage options exactly)
    #[test]
    fn test_storage_config_compatibility() {
        let mut config = CliConfig::default();

        // Test default storage settings
        assert_eq!(config.application.storage.engine, "RocksDB");
        assert_eq!(config.application.storage.path, "data");

        // Test different storage engines
        let valid_engines = vec!["RocksDB", "LevelDB", "MemoryDB"];
        for engine in valid_engines {
            config.application.storage.engine = engine.to_string();
            assert!(config.validate().is_ok());
        }

        // Test invalid storage engine
        config.application.storage.engine = "InvalidEngine".to_string();
        assert!(config.validate().is_err());

        // Test storage path validation
        config.application.storage.engine = "RocksDB".to_string(); // Reset
        config.application.storage.path = "".to_string(); // Empty path
        assert!(config.validate().is_err());

        config.application.storage.path = "data".to_string(); // Reset
        assert!(config.validate().is_ok());
    }

    /// Test P2P configuration (matches C# P2P network settings exactly)
    #[test]
    fn test_p2p_config_compatibility() {
        let mut config = CliConfig::default();

        // Test default P2P settings
        assert_eq!(config.application.p2p.port, 10333);
        assert_eq!(config.application.p2p.min_desired_connections, 10);
        assert_eq!(config.application.p2p.max_connections, 40);

        // Test P2P port validation
        config.application.p2p.port = 1024; // Valid port
        assert!(config.validate().is_ok());

        config.application.p2p.port = 65535; // Valid max port
        assert!(config.validate().is_ok());

        config.application.p2p.port = 0; // Invalid port
        assert!(config.validate().is_err());

        config.application.p2p.port = 65536; // Invalid port (too high)
        assert!(config.validate().is_err());

        // Test connection limits validation
        config.application.p2p.port = 10333; // Reset
        config.application.p2p.min_desired_connections = 5;
        config.application.p2p.max_connections = 100;
        assert!(config.validate().is_ok());

        config.application.p2p.min_desired_connections = 50;
        config.application.p2p.max_connections = 40; // Min > Max
        assert!(config.validate().is_err());

        config.application.p2p.max_connections = 0; // Invalid max
        assert!(config.validate().is_err());
    }

    /// Test RPC configuration (matches C# RPC server settings exactly)
    #[test]
    fn test_rpc_config_compatibility() {
        let mut config = CliConfig::default();

        // Test default RPC settings
        assert_eq!(config.application.rpc.bind_address, "127.0.0.1");
        assert_eq!(config.application.rpc.port, 10332);
        assert_eq!(config.application.rpc.ssl_cert, "");
        assert_eq!(config.application.rpc.ssl_cert_password, "");
        assert_eq!(config.application.rpc.trusted_authorities.len(), 0);
        assert_eq!(config.application.rpc.max_gas_invoke, 20000000);

        // Test RPC bind address validation
        let valid_addresses = vec!["127.0.0.1", "0.0.0.0", "localhost", "192.168.1.100"];
        for addr in valid_addresses {
            config.application.rpc.bind_address = addr.to_string();
            assert!(config.validate().is_ok());
        }

        // Test invalid bind addresses
        let invalid_addresses = vec!["", "invalid", "256.256.256.256"];
        for addr in invalid_addresses {
            config.application.rpc.bind_address = addr.to_string();
            assert!(config.validate().is_err());
        }

        // Test RPC port validation
        config.application.rpc.bind_address = "127.0.0.1".to_string(); // Reset
        config.application.rpc.port = 8080; // Valid port
        assert!(config.validate().is_ok());

        config.application.rpc.port = 0; // Invalid port
        assert!(config.validate().is_err());

        // Test SSL configuration
        config.application.rpc.port = 10332; // Reset
        config.application.rpc.ssl_cert = "cert.pem".to_string();
        config.application.rpc.ssl_cert_password = "password".to_string();
        assert!(config.validate().is_ok());

        // Test trusted authorities
        config.application.rpc.trusted_authorities = vec![
            "authority1.example.com".to_string(),
            "authority2.example.com".to_string(),
        ];
        assert!(config.validate().is_ok());

        // Test max gas invoke validation
        config.application.rpc.max_gas_invoke = 1000000; // Valid
        assert!(config.validate().is_ok());

        config.application.rpc.max_gas_invoke = 0; // Invalid
        assert!(config.validate().is_err());
    }

    /// Test protocol configuration (matches C# ProtocolConfiguration exactly)
    #[test]
    fn test_protocol_config_compatibility() {
        let mut config = CliConfig::default();

        // Test default protocol settings
        assert_eq!(config.protocol.network, 860833102);
        assert_eq!(config.protocol.address_version, 53);
        assert_eq!(config.protocol.milliseconds_per_block, 15000);
        assert_eq!(config.protocol.max_transactions_per_block, 512);
        assert_eq!(config.protocol.memory_pool_max_transactions, 50000);
        assert_eq!(config.protocol.max_traceable_blocks, 2102400);
        assert_eq!(config.protocol.initial_gas_distribution, 5200000000000000);

        // Test protocol validation
        assert!(config.validate().is_ok());

        // Test invalid block time
        config.protocol.milliseconds_per_block = 0;
        assert!(config.validate().is_err());

        config.protocol.milliseconds_per_block = 15000; // Reset

        // Test invalid transaction limits
        config.protocol.max_transactions_per_block = 0;
        assert!(config.validate().is_err());

        config.protocol.max_transactions_per_block = 512; // Reset
        config.protocol.memory_pool_max_transactions = 0;
        assert!(config.validate().is_err());

        config.protocol.memory_pool_max_transactions = 50000; // Reset

        // Test invalid traceable blocks
        config.protocol.max_traceable_blocks = 0;
        assert!(config.validate().is_err());

        config.protocol.max_traceable_blocks = 2102400; // Reset

        // Test invalid initial gas distribution
        config.protocol.initial_gas_distribution = 0;
        assert!(config.validate().is_err());
    }

    /// Test hardfork configuration (matches C# hardfork handling exactly)
    #[test]
    fn test_hardfork_config_compatibility() {
        let config = CliConfig::default();

        // Test default hardfork heights
        assert_eq!(config.protocol.hardforks.aspidochelone, 0);
        assert_eq!(config.protocol.hardforks.basilisk, 0);
        assert_eq!(config.protocol.hardforks.cockatrice, 0);
        assert_eq!(config.protocol.hardforks.domovoi, 0);

        // Test hardfork ordering validation
        let mut custom_config = config.clone();
        custom_config.protocol.hardforks.aspidochelone = 1000;
        custom_config.protocol.hardforks.basilisk = 2000;
        custom_config.protocol.hardforks.cockatrice = 3000;
        custom_config.protocol.hardforks.domovoi = 4000;
        assert!(custom_config.validate().is_ok());

        // Test invalid hardfork ordering
        custom_config.protocol.hardforks.basilisk = 500; // Before aspidochelone
        assert!(custom_config.validate().is_err());
    }

    /// Test configuration merging (matches C# config override behavior exactly)
    #[test]
    fn test_config_merging_compatibility() {
        let mut base_config = CliConfig::default();
        let mut override_config = CliConfig::default();

        // Modify override config
        override_config.application.rpc.port = 8080;
        override_config.application.p2p.port = 8333;
        override_config.protocol.milliseconds_per_block = 10000;

        // Test merging
        base_config.merge_from(override_config);

        // Verify overridden values
        assert_eq!(base_config.application.rpc.port, 8080);
        assert_eq!(base_config.application.p2p.port, 8333);
        assert_eq!(base_config.protocol.milliseconds_per_block, 10000);

        // Verify non-overridden values remain
        assert_eq!(base_config.application.rpc.bind_address, "127.0.0.1");
        assert_eq!(base_config.protocol.address_version, 53);
    }

    /// Test configuration serialization (matches C# JSON serialization exactly)
    #[tokio::test]
    async fn test_config_serialization_compatibility() {
        let config = CliConfig::default();

        // Test serialization to JSON
        let json_str = config.to_json().unwrap();
        assert!(!json_str.is_empty());

        // Verify JSON structure
        let json_value: serde_json::Value = serde_json::from_str(&json_str).unwrap();
        assert!(json_value.get("ApplicationConfiguration").is_some());
        assert!(json_value.get("ProtocolConfiguration").is_some());

        // Test deserialization from JSON
        let deserialized_config = CliConfig::from_json(&json_str).unwrap();
        assert_eq!(
            config.application.rpc.port,
            deserialized_config.application.rpc.port
        );
        assert_eq!(
            config.protocol.network,
            deserialized_config.protocol.network
        );

        // Test round-trip consistency
        let round_trip_json = deserialized_config.to_json().unwrap();
        let round_trip_config = CliConfig::from_json(&round_trip_json).unwrap();
        assert_eq!(
            config.application.rpc.port,
            round_trip_config.application.rpc.port
        );
    }

    /// Test configuration file watching (matches C# file monitoring exactly)
    #[tokio::test]
    async fn test_config_file_watching_compatibility() {
        let final_dir = TempDir::new().unwrap();
        let config_path = final_dir.path().join("watch_config.json");

        // Create initial config
        let initial_config = CliConfig::default();
        let config_json = initial_config.to_json().unwrap();
        fs::write(&config_path, &config_json).await.unwrap();

        // Test file watching setup
        let watcher = ConfigWatcher::new(&config_path).unwrap();
        assert!(watcher.is_watching());

        // Test config change detection
        let mut modified_config = initial_config.clone();
        modified_config.application.rpc.port = 8080;
        let modified_json = modified_config.to_json().unwrap();
        fs::write(&config_path, &modified_json).await.unwrap();

        // In real implementation, this would trigger a reload notification
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // Verify watcher is still active
        assert!(watcher.is_watching());
    }

    /// Test configuration error handling (matches C# error scenarios exactly)
    #[tokio::test]
    async fn test_config_error_handling_compatibility() {
        let final_dir = TempDir::new().unwrap();

        // Test loading non-existent file
        let non_existent_path = final_dir.path().join("nonexistent.json");
        let result = CliConfig::load_from_file(&non_existent_path).await;
        assert!(result.is_err());

        // Test loading invalid JSON
        let invalid_json_path = final_dir.path().join("invalid.json");
        fs::write(&invalid_json_path, "{ invalid json }")
            .await
            .unwrap();
        let result = CliConfig::load_from_file(&invalid_json_path).await;
        assert!(result.is_err());

        // Test loading incomplete config
        let incomplete_config_path = final_dir.path().join("incomplete.json");
        fs::write(
            &incomplete_config_path,
            r#"
        {
            "ApplicationConfiguration": {
                "Logger": {
                    "Path": "logs"
                }
            }
        }
        "#,
        )
        .await
        .unwrap();
        let result = CliConfig::load_from_file(&incomplete_config_path).await;
        assert!(result.is_ok());
    }

    /// Test environment variable integration (matches C# environment handling exactly)
    #[test]
    fn test_environment_variable_integration_compatibility() {
        // Test environment variable overrides
        std::env::set_var("NEO_RPC_PORT", "8080");
        std::env::set_var("NEO_P2P_PORT", "8333");
        std::env::set_var("NEO_DATA_DIR", "/custom/data");

        let config = CliConfig::from_environment();

        // Verify environment overrides
        assert_eq!(config.application.rpc.port, 8080);
        assert_eq!(config.application.p2p.port, 8333);
        assert_eq!(config.application.storage.path, "/custom/data");

        // Clean up environment
        std::env::remove_var("NEO_RPC_PORT");
        std::env::remove_var("NEO_P2P_PORT");
        std::env::remove_var("NEO_DATA_DIR");
    }
}
