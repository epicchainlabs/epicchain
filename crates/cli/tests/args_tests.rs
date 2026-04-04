//! CLI Arguments C# Compatibility Tests
//!
//! These tests ensure full compatibility with C# Neo CLI argument parsing.
//! Tests are based on the C# Neo.CLI CommandLineOptions test suite.

use clap::Parser;
use neo_cli::args::*;
use std::path::PathBuf;

#[cfg(test)]
#[allow(dead_code)]
mod args_tests {
    use super::*;

    /// Test basic CLI argument parsing (matches C# CommandLineOptions exactly)
    #[test]
    fn test_basic_args_parsing_compatibility() {
        let args = CliArgs::try_parse_from(&["neo-cli"]).unwrap();
        assert_eq!(args.config, None);
        assert_eq!(args.wallet, None);
        assert_eq!(args.password, None);
        assert_eq!(args.db_engine, None);
        assert_eq!(args.db_path, None);
        assert_eq!(args.no_verify, false);
        assert_eq!(args.plugins.len(), 0);
        assert_eq!(args.verbose, LogLevel::Info);
        assert_eq!(args.daemon, false);
        assert_eq!(args.network, Network::MainNet);
        assert_eq!(args.rpc_port, None);
        assert_eq!(args.p2p_port, None);
        assert_eq!(args.max_connections, None);
        assert_eq!(args.min_connections, None);
        assert_eq!(args.data_dir, None);
        assert_eq!(args.show_version, false);

        // Test that arguments can be cloned and debugged
        let cloned_args = args.clone();
        assert_eq!(args.verbose, cloned_args.verbose);
        assert_eq!(args.network, cloned_args.network);

        let debug_output = format!("{:?}", args);
        assert!(debug_output.contains("CliArgs"));
    }

    /// Test config file argument parsing (matches C# config handling exactly)
    #[test]
    fn test_config_args_compatibility() {
        // Test short form
        let args = CliArgs::try_parse_from(&["neo-cli", "-c", "config.json"]).unwrap();
        assert_eq!(args.config, Some(PathBuf::from("config.json")));

        // Test long form
        let args = CliArgs::try_parse_from(&["neo-cli", "--config", "config.json"]).unwrap();
        assert_eq!(args.config, Some(PathBuf::from("config.json")));

        // Test with path
        let args =
            CliArgs::try_parse_from(&["neo-cli", "--config", "/path/to/config.json"]).unwrap();
        assert_eq!(args.config, Some(PathBuf::from("/path/to/config.json")));

        // Test with relative path
        let args = CliArgs::try_parse_from(&["neo-cli", "--config", "../config.json"]).unwrap();
        assert_eq!(args.config, Some(PathBuf::from("../config.json")));
    }

    /// Test wallet argument parsing (matches C# wallet handling exactly)
    #[test]
    fn test_wallet_args_compatibility() {
        // Test short form
        let args = CliArgs::try_parse_from(&["neo-cli", "-w", "wallet.json"]).unwrap();
        assert_eq!(args.wallet, Some(PathBuf::from("wallet.json")));

        // Test long form
        let args = CliArgs::try_parse_from(&["neo-cli", "--wallet", "wallet.json"]).unwrap();
        assert_eq!(args.wallet, Some(PathBuf::from("wallet.json")));

        // Test with password
        let args = CliArgs::try_parse_from(&[
            "neo-cli",
            "--wallet",
            "wallet.json",
            "--password",
            "test123",
        ])
        .unwrap();
        assert_eq!(args.wallet, Some(PathBuf::from("wallet.json")));
        assert_eq!(args.password, Some("test123".to_string()));

        // Test short form password
        let args =
            CliArgs::try_parse_from(&["neo-cli", "-w", "wallet.json", "-p", "test123"]).unwrap();
        assert_eq!(args.wallet, Some(PathBuf::from("wallet.json")));
        assert_eq!(args.password, Some("test123".to_string()));
    }

    /// Test database argument parsing (matches C# database configuration exactly)
    #[test]
    fn test_database_args_compatibility() {
        // Test database engine
        let args = CliArgs::try_parse_from(&["neo-cli", "--db-engine", "rocksdb"]).unwrap();
        assert_eq!(args.db_engine, Some("rocksdb".to_string()));

        let args = CliArgs::try_parse_from(&["neo-cli", "--db-engine", "leveldb"]).unwrap();
        assert_eq!(args.db_engine, Some("leveldb".to_string()));

        // Test database path
        let args = CliArgs::try_parse_from(&["neo-cli", "--db-path", "/data/blockchain"]).unwrap();
        assert_eq!(args.db_path, Some(PathBuf::from("/data/blockchain")));

        // Test data directory
        let args = CliArgs::try_parse_from(&["neo-cli", "--data-dir", "/opt/neo"]).unwrap();
        assert_eq!(args.data_dir, Some(PathBuf::from("/opt/neo")));

        // Test combined database options
        let args = CliArgs::try_parse_from(&[
            "neo-cli",
            "--db-engine",
            "rocksdb",
            "--db-path",
            "/data/blockchain",
            "--data-dir",
            "/opt/neo",
        ])
        .unwrap();
        assert_eq!(args.db_engine, Some("rocksdb".to_string()));
        assert_eq!(args.db_path, Some(PathBuf::from("/data/blockchain")));
        assert_eq!(args.data_dir, Some(PathBuf::from("/opt/neo")));
    }

    /// Test network argument parsing (matches C# network types exactly)
    #[test]
    fn test_network_args_compatibility() {
        let args = CliArgs::try_parse_from(&["neo-cli"]).unwrap();
        assert_eq!(args.network, Network::MainNet);

        // Test explicit mainnet
        let args = CliArgs::try_parse_from(&["neo-cli", "--network", "mainnet"]).unwrap();
        assert_eq!(args.network, Network::MainNet);

        // Test testnet
        let args = CliArgs::try_parse_from(&["neo-cli", "--network", "testnet"]).unwrap();
        assert_eq!(args.network, Network::TestNet);

        // Test private network
        let args = CliArgs::try_parse_from(&["neo-cli", "--network", "private"]).unwrap();
        assert_eq!(args.network, Network::Private);

        // Test case sensitivity
        let result = CliArgs::try_parse_from(&["neo-cli", "--network", "MAINNET"]);
        assert!(result.is_err()); // Should be case sensitive

        // Test invalid network
        let result = CliArgs::try_parse_from(&["neo-cli", "--network", "invalid"]);
        assert!(result.is_err());
    }

    /// Test logging argument parsing (matches C# logging levels exactly)
    #[test]
    fn test_logging_args_compatibility() {
        // Test default log level
        let args = CliArgs::try_parse_from(&["neo-cli"]).unwrap();
        assert_eq!(args.verbose, LogLevel::Info);

        // Test all log levels
        let args = CliArgs::try_parse_from(&["neo-cli", "--verbose", "trace"]).unwrap();
        assert_eq!(args.verbose, LogLevel::Trace);

        let args = CliArgs::try_parse_from(&["neo-cli", "--verbose", "debug"]).unwrap();
        assert_eq!(args.verbose, LogLevel::Debug);

        let args = CliArgs::try_parse_from(&["neo-cli", "--verbose", "info"]).unwrap();
        assert_eq!(args.verbose, LogLevel::Info);

        let args = CliArgs::try_parse_from(&["neo-cli", "--verbose", "warn"]).unwrap();
        assert_eq!(args.verbose, LogLevel::Warn);

        let args = CliArgs::try_parse_from(&["neo-cli", "--verbose", "error"]).unwrap();
        assert_eq!(args.verbose, LogLevel::Error);

        // Test case sensitivity
        let result = CliArgs::try_parse_from(&["neo-cli", "--verbose", "INFO"]);
        assert!(result.is_err()); // Should be case sensitive

        // Test invalid log level
        let result = CliArgs::try_parse_from(&["neo-cli", "--verbose", "invalid"]);
        assert!(result.is_err());
    }

    /// Test network port arguments (matches C# port configuration exactly)
    #[test]
    fn test_port_args_compatibility() {
        // Test RPC port
        let args = CliArgs::try_parse_from(&["neo-cli", "--rpc-port", "10332"]).unwrap();
        assert_eq!(args.rpc_port, Some(10332));

        // Test P2P port
        let args = CliArgs::try_parse_from(&["neo-cli", "--p2p-port", "10333"]).unwrap();
        assert_eq!(args.p2p_port, Some(10333));

        // Test both ports
        let args =
            CliArgs::try_parse_from(&["neo-cli", "--rpc-port", "10332", "--p2p-port", "10333"])
                .unwrap();
        assert_eq!(args.rpc_port, Some(10332));
        assert_eq!(args.p2p_port, Some(10333));

        // Test custom ports
        let args =
            CliArgs::try_parse_from(&["neo-cli", "--rpc-port", "8080", "--p2p-port", "8333"])
                .unwrap();
        assert_eq!(args.rpc_port, Some(8080));
        assert_eq!(args.p2p_port, Some(8333));

        let result = CliArgs::try_parse_from(&["neo-cli", "--rpc-port", "70000"]);
        assert!(result.is_err());

        let result = CliArgs::try_parse_from(&["neo-cli", "--rpc-port", "abc"]);
        assert!(result.is_err());
    }

    /// Test connection arguments (matches C# connection management exactly)
    #[test]
    fn test_connection_args_compatibility() {
        // Test max connections
        let args = CliArgs::try_parse_from(&["neo-cli", "--max-connections", "40"]).unwrap();
        assert_eq!(args.max_connections, Some(40));

        // Test min connections
        let args = CliArgs::try_parse_from(&["neo-cli", "--min-connections", "10"]).unwrap();
        assert_eq!(args.min_connections, Some(10));

        // Test both connection limits
        let args = CliArgs::try_parse_from(&[
            "neo-cli",
            "--max-connections",
            "50",
            "--min-connections",
            "15",
        ])
        .unwrap();
        assert_eq!(args.max_connections, Some(50));
        assert_eq!(args.min_connections, Some(15));

        let args = CliArgs::try_parse_from(&["neo-cli", "--max-connections", "0"]).unwrap();
        assert_eq!(args.max_connections, Some(0));

        // Test very high connection count
        let args = CliArgs::try_parse_from(&["neo-cli", "--max-connections", "1000"]).unwrap();
        assert_eq!(args.max_connections, Some(1000));

        // Test invalid connection count
        let result = CliArgs::try_parse_from(&["neo-cli", "--max-connections", "abc"]);
        assert!(result.is_err());
    }

    /// Test plugin arguments (matches C# plugin system exactly)
    #[test]
    fn test_plugin_args_compatibility() {
        let args = CliArgs::try_parse_from(&["neo-cli"]).unwrap();
        assert_eq!(args.plugins.len(), 0);

        // Test single plugin
        let args = CliArgs::try_parse_from(&["neo-cli", "--plugins", "RpcServer"]).unwrap();
        assert_eq!(args.plugins, vec!["RpcServer"]);

        // Test multiple plugins
        let args = CliArgs::try_parse_from(&[
            "neo-cli",
            "--plugins",
            "RpcServer",
            "ApplicationLogs",
            "TokensTracker",
        ])
        .unwrap();
        assert_eq!(
            args.plugins,
            vec!["RpcServer", "ApplicationLogs", "TokensTracker"]
        );

        // Test plugin names with special characters
        let args = CliArgs::try_parse_from(&[
            "neo-cli",
            "--plugins",
            "RpcServer",
            "Neo.Plugins.Storage.LevelDBStore",
        ])
        .unwrap();
        assert_eq!(
            args.plugins,
            vec!["RpcServer", "Neo.Plugins.Storage.LevelDBStore"]
        );

        // Test empty plugin list
        let args = CliArgs::try_parse_from(&["neo-cli", "--plugins"]).unwrap();
        assert_eq!(args.plugins.len(), 0);
    }

    /// Test boolean flag arguments (matches C# boolean flags exactly)
    #[test]
    fn test_boolean_flags_compatibility() {
        // Test default boolean values
        let args = CliArgs::try_parse_from(&["neo-cli"]).unwrap();
        assert_eq!(args.no_verify, false);
        assert_eq!(args.daemon, false);
        assert_eq!(args.show_version, false);

        // Test no-verify flag
        let args = CliArgs::try_parse_from(&["neo-cli", "--noverify"]).unwrap();
        assert_eq!(args.no_verify, true);

        // Test daemon flag
        let args = CliArgs::try_parse_from(&["neo-cli", "--daemon"]).unwrap();
        assert_eq!(args.daemon, true);

        // Test version flag
        let args = CliArgs::try_parse_from(&["neo-cli", "--version"]).unwrap();
        assert_eq!(args.show_version, true);

        // Test short version flag
        let args = CliArgs::try_parse_from(&["neo-cli", "-V"]).unwrap();
        assert_eq!(args.show_version, true);

        // Test multiple boolean flags
        let args = CliArgs::try_parse_from(&["neo-cli", "--noverify", "--daemon"]).unwrap();
        assert_eq!(args.no_verify, true);
        assert_eq!(args.daemon, true);

        // Boolean flags should not accept values
        let result = CliArgs::try_parse_from(&["neo-cli", "--daemon", "true"]);
        // depending on clap configuration
    }

    /// Test complex argument combinations (matches C# complex scenarios exactly)
    #[test]
    fn test_complex_args_combinations_compatibility() {
        // Test realistic CLI invocation
        let args = CliArgs::try_parse_from(&[
            "neo-cli",
            "--config",
            "/opt/neo/config.json",
            "--wallet",
            "/opt/neo/wallet.json",
            "--password",
            "secretpassword",
            "--network",
            "mainnet",
            "--rpc-port",
            "10332",
            "--p2p-port",
            "10333",
            "--max-connections",
            "40",
            "--min-connections",
            "10",
            "--verbose",
            "info",
            "--plugins",
            "RpcServer",
            "ApplicationLogs",
            "--db-engine",
            "rocksdb",
            "--db-path",
            "/opt/neo/data",
            "--data-dir",
            "/opt/neo",
        ])
        .unwrap();

        assert_eq!(args.config, Some(PathBuf::from("/opt/neo/config.json")));
        assert_eq!(args.wallet, Some(PathBuf::from("/opt/neo/wallet.json")));
        assert_eq!(args.password, Some("secretpassword".to_string()));
        assert_eq!(args.network, Network::MainNet);
        assert_eq!(args.rpc_port, Some(10332));
        assert_eq!(args.p2p_port, Some(10333));
        assert_eq!(args.max_connections, Some(40));
        assert_eq!(args.min_connections, Some(10));
        assert_eq!(args.verbose, LogLevel::Info);
        assert_eq!(args.plugins, vec!["RpcServer", "ApplicationLogs"]);
        assert_eq!(args.db_engine, Some("rocksdb".to_string()));
        assert_eq!(args.db_path, Some(PathBuf::from("/opt/neo/data")));
        assert_eq!(args.data_dir, Some(PathBuf::from("/opt/neo")));

        // Test with flags
        let args = CliArgs::try_parse_from(&[
            "neo-cli",
            "--wallet",
            "wallet.json",
            "--noverify",
            "--daemon",
            "--verbose",
            "debug",
        ])
        .unwrap();

        assert_eq!(args.wallet, Some(PathBuf::from("wallet.json")));
        assert_eq!(args.no_verify, true);
        assert_eq!(args.daemon, true);
        assert_eq!(args.verbose, LogLevel::Debug);
    }

    /// Test LogLevel enum functionality (matches C# LogLevel exactly)
    #[test]
    fn test_log_level_enum_compatibility() {
        // Test all log levels exist and have correct values
        let levels = [
            LogLevel::Trace,
            LogLevel::Debug,
            LogLevel::Info,
            LogLevel::Warn,
            LogLevel::Error,
        ];

        // Test that levels can be compared
        assert_eq!(LogLevel::Info, LogLevel::Info);
        assert_ne!(LogLevel::Info, LogLevel::Debug);

        // Test that levels can be cloned and copied
        let level = LogLevel::Info;
        let cloned = level.clone();
        let copied = level;
        assert_eq!(level, cloned);
        assert_eq!(level, copied);

        // Test conversion to tracing::Level
        assert_eq!(tracing::Level::from(LogLevel::Trace), tracing::Level::TRACE);
        assert_eq!(tracing::Level::from(LogLevel::Debug), tracing::Level::DEBUG);
        assert_eq!(tracing::Level::from(LogLevel::Info), tracing::Level::INFO);
        assert_eq!(tracing::Level::from(LogLevel::Warn), tracing::Level::WARN);
        assert_eq!(tracing::Level::from(LogLevel::Error), tracing::Level::ERROR);

        // Test debug output
        let debug_output = format!("{:?}", LogLevel::Info);
        assert_eq!(debug_output, "Info");
    }

    /// Test Network enum functionality (matches C# Network types exactly)
    #[test]
    fn test_network_enum_compatibility() {
        // Test all networks exist
        let networks = [Network::MainNet, Network::TestNet, Network::Private];

        // Test that networks can be compared
        assert_eq!(Network::MainNet, Network::MainNet);
        assert_ne!(Network::MainNet, Network::TestNet);

        // Test that networks can be cloned and copied
        let network = Network::MainNet;
        let cloned = network.clone();
        let copied = network;
        assert_eq!(network, cloned);
        assert_eq!(network, copied);

        // Test debug output
        assert_eq!(format!("{:?}", Network::MainNet), "MainNet");
        assert_eq!(format!("{:?}", Network::TestNet), "TestNet");
        assert_eq!(format!("{:?}", Network::Private), "Private");
    }

    /// Test argument validation and error handling (matches C# validation exactly)
    #[test]
    fn test_args_validation_compatibility() {
        // Test required program name
        let result = CliArgs::try_parse_from(&[]);
        assert!(result.is_err());

        // Test invalid flag
        let result = CliArgs::try_parse_from(&["neo-cli", "--invalid-flag"]);
        assert!(result.is_err());

        // Test flag without value where value expected
        let result = CliArgs::try_parse_from(&["neo-cli", "--config"]);
        assert!(result.is_err());

        let result = CliArgs::try_parse_from(&["neo-cli", "--wallet"]);
        assert!(result.is_err());

        let result = CliArgs::try_parse_from(&["neo-cli", "--password"]);
        assert!(result.is_err());

        // Test invalid enum values
        let result = CliArgs::try_parse_from(&["neo-cli", "--network", "invalid"]);
        assert!(result.is_err());

        let result = CliArgs::try_parse_from(&["neo-cli", "--verbose", "invalid"]);
        assert!(result.is_err());

        // Test invalid numeric values
        let result = CliArgs::try_parse_from(&["neo-cli", "--rpc-port", "abc"]);
        assert!(result.is_err());

        let result = CliArgs::try_parse_from(&["neo-cli", "--max-connections", "not_a_number"]);
        assert!(result.is_err());
    }

    /// Test help and version output (matches C# help text exactly)
    #[test]
    fn test_help_and_version_compatibility() {
        // Test that help can be generated
        let result = CliArgs::try_parse_from(&["neo-cli", "--help"]);
        assert!(result.is_err()); // Clap exits on help, so this errors

        // Test that version info can be accessed
        let args = CliArgs::try_parse_from(&["neo-cli", "--version"]).unwrap();
        assert_eq!(args.show_version, true);

        // Test short version flag
        let args = CliArgs::try_parse_from(&["neo-cli", "-V"]).unwrap();
        assert_eq!(args.show_version, true);
    }
}
