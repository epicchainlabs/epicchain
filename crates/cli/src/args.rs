use clap::{Parser, ValueEnum};
use std::path::PathBuf;

/// Command-line arguments for Neo CLI
/// This matches the C# CommandLineOptions structure
#[derive(Parser, Debug, Clone)]
#[command(
    name = "neo-cli",
    version = env!("CARGO_PKG_VERSION"),
    about = "Neo CLI - Command-line interface for Neo blockchain node",
    long_about = "Neo CLI provides a command-line interface for running and interacting with a Neo blockchain node. It supports wallet management, blockchain synchronization, consensus participation, and RPC services.",
    disable_version_flag = true
)]
pub struct CliArgs {
    /// Specifies the config file
    #[arg(short = 'c', long = "config", value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// The path of the neo3 wallet (*.json)
    #[arg(short = 'w', long = "wallet", value_name = "FILE")]
    pub wallet: Option<PathBuf>,

    /// Password to decrypt the wallet
    #[arg(short = 'p', long = "password", value_name = "PASSWORD")]
    pub password: Option<String>,

    /// Specify the database engine
    #[arg(long = "db-engine", value_name = "ENGINE")]
    pub db_engine: Option<String>,

    /// Specify the database path
    #[arg(long = "db-path", value_name = "PATH")]
    pub db_path: Option<PathBuf>,

    /// Indicates whether the blocks need to be verified when importing
    #[arg(long = "noverify")]
    pub no_verify: bool,

    /// The list of plugins to install
    #[arg(long = "plugins", value_name = "PLUGIN", num_args = 0..)]
    pub plugins: Vec<String>,

    /// The verbose log level
    #[arg(long = "verbose", value_enum, default_value = "info")]
    pub verbose: LogLevel,

    /// Run in daemon mode (background)
    #[arg(long = "daemon")]
    pub daemon: bool,

    /// Network to connect to
    #[arg(long = "network", value_enum, default_value = "mainnet")]
    pub network: Network,

    /// RPC server port
    #[arg(long = "rpc-port", value_name = "PORT")]
    pub rpc_port: Option<u16>,

    /// P2P network port
    #[arg(long = "p2p-port", value_name = "PORT")]
    pub p2p_port: Option<u16>,

    /// Maximum number of connections
    #[arg(long = "max-connections", value_name = "COUNT")]
    pub max_connections: Option<u32>,

    /// Minimum desired connections
    #[arg(long = "min-connections", value_name = "COUNT")]
    pub min_connections: Option<u32>,

    /// Data directory for blockchain data
    #[arg(long = "data-dir", value_name = "DIR")]
    pub data_dir: Option<PathBuf>,

    /// Show version information and exit
    #[arg(short = 'V', long = "version")]
    pub show_version: bool,
}

/// Log level enumeration
#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    /// Trace level logging
    Trace,
    /// Debug level logging
    Debug,
    /// Info level logging
    Info,
    /// Warning level logging
    Warn,
    /// Error level logging
    Error,
}

impl From<LogLevel> for tracing::Level {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Trace => tracing::Level::TRACE,
            LogLevel::Debug => tracing::Level::DEBUG,
            LogLevel::Info => tracing::Level::INFO,
            LogLevel::Warn => tracing::Level::WARN,
            LogLevel::Error => tracing::Level::ERROR,
        }
    }
}

/// Network enumeration
#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Network {
    /// MainNet network
    Mainnet,
    /// TestNet network
    Testnet,
    /// Private network
    Private,
}

impl CliArgs {
    /// Check if command-line options were configured
    /// This matches the C# CommandLineOptions.IsValid property
    pub fn is_valid(&self) -> bool {
        self.config.is_some()
            || self.wallet.is_some()
            || self.password.is_some()
            || self.db_engine.is_some()
            || self.db_path.is_some()
            || !self.plugins.is_empty()
            || self.no_verify
            || self.daemon
            || self.rpc_port.is_some()
            || self.p2p_port.is_some()
            || self.max_connections.is_some()
            || self.min_connections.is_some()
            || self.data_dir.is_some()
    }

    /// Get the effective data directory
    pub fn get_data_dir(&self) -> PathBuf {
        self.data_dir.clone().unwrap_or_else(|| {
            dirs::data_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("neo")
        })
    }

    /// Get the effective database path
    pub fn get_db_path(&self) -> PathBuf {
        self.db_path
            .clone()
            .unwrap_or_else(|| self.get_data_dir().join("chain"))
    }

    /// Get the effective configuration file path
    pub fn get_config_path(&self) -> PathBuf {
        self.config
            .clone()
            .unwrap_or_else(|| PathBuf::from("config.json"))
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_args_default() {
        let args = CliArgs::parse_from(&["neo-cli"]);
        assert_eq!(args.verbose, LogLevel::Info);
        assert_eq!(args.network, Network::Mainnet);
        assert!(!args.no_verify);
        assert!(!args.daemon);
        assert!(!args.show_version);
        assert!(!args.is_valid()); // No options set
    }

    #[test]
    fn test_cli_args_with_options() {
        let args = CliArgs::parse_from(&[
            "neo-cli",
            "--config",
            "test.json",
            "--wallet",
            "wallet.json",
            "--verbose",
            "debug",
            "--network",
            "testnet",
            "--daemon",
        ]);

        assert_eq!(args.config, Some(PathBuf::from("test.json")));
        assert_eq!(args.wallet, Some(PathBuf::from("wallet.json")));
        assert_eq!(args.verbose, LogLevel::Debug);
        assert_eq!(args.network, Network::Testnet);
        assert!(args.daemon);
        assert!(args.is_valid()); // Options are set
    }

    #[test]
    fn test_log_level_conversion() {
        assert_eq!(tracing::Level::from(LogLevel::Trace), tracing::Level::TRACE);
        assert_eq!(tracing::Level::from(LogLevel::Debug), tracing::Level::DEBUG);
        assert_eq!(tracing::Level::from(LogLevel::Info), tracing::Level::INFO);
        assert_eq!(tracing::Level::from(LogLevel::Warn), tracing::Level::WARN);
        assert_eq!(tracing::Level::from(LogLevel::Error), tracing::Level::ERROR);
    }

    #[test]
    fn test_path_methods() {
        let args = CliArgs::parse_from(&["neo-cli"]);

        // Test default paths
        let data_dir = args.get_data_dir();
        assert!(data_dir.ends_with("neo"));

        let db_path = args.get_db_path();
        assert!(db_path.ends_with("chain"));

        let config_path = args.get_config_path();
        assert_eq!(config_path, PathBuf::from("config.json"));
    }

    #[test]
    fn test_custom_paths() {
        let args = CliArgs::parse_from(&[
            "neo-cli",
            "--config",
            "custom.json",
            "--data-dir",
            "/custom/data",
            "--db-path",
            "/custom/db",
        ]);

        assert_eq!(args.get_config_path(), PathBuf::from("custom.json"));
        assert_eq!(args.get_data_dir(), PathBuf::from("/custom/data"));
        assert_eq!(args.get_db_path(), PathBuf::from("/custom/db"));
    }
}
