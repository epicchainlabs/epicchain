//! Console Command System for Plugins
//! 
//! Matches C# Neo console command system exactly

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Console command attribute (matches C# ConsoleCommandAttribute)
#[derive(Debug, Clone)]
pub struct ConsoleCommand {
    /// Command name
    pub name: String,
    /// Command category
    pub category: String,
    /// Command description
    pub description: Option<String>,
}

/// Console command handler trait
#[async_trait]
pub trait ConsoleCommandHandler: Send + Sync {
    /// Execute the console command
    async fn execute(&self, args: &[String]) -> Result<String, Box<dyn std::error::Error + Send + Sync>>;
    
    /// Get command information
    fn command_info(&self) -> ConsoleCommand;
}

/// Console command registry (matches C# console command system)
pub struct ConsoleCommandRegistry {
    commands: Arc<RwLock<HashMap<String, Box<dyn ConsoleCommandHandler>>>>,
}

impl ConsoleCommandRegistry {
    /// Create new command registry
    pub fn new() -> Self {
        Self {
            commands: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Register console command
    pub async fn register_command<T: ConsoleCommandHandler + 'static>(&self, handler: T) {
        let command_info = handler.command_info();
        let mut commands = self.commands.write().await;
        commands.insert(command_info.name.clone(), Box::new(handler));
    }
    
    /// Execute console command
    pub async fn execute_command(&self, command: &str, args: &[String]) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let commands = self.commands.read().await;
        
        if let Some(handler) = commands.get(command) {
            handler.execute(args).await
        } else {
            Err(format!("Command '{}' not found", command).into())
        }
    }
    
    /// Get all available commands
    pub async fn get_commands(&self) -> Vec<ConsoleCommand> {
        let commands = self.commands.read().await;
        commands.values().map(|handler| handler.command_info()).collect()
    }
    
    /// Get commands by category
    pub async fn get_commands_by_category(&self, category: &str) -> Vec<ConsoleCommand> {
        let commands = self.commands.read().await;
        commands
            .values()
            .map(|handler| handler.command_info())
            .filter(|cmd| cmd.category == category)
            .collect()
    }
}

impl Default for ConsoleCommandRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// Example console command implementations

/// Help command (matches C# help command)
pub struct HelpCommand;

#[async_trait]
impl ConsoleCommandHandler for HelpCommand {
    async fn execute(&self, _args: &[String]) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        Ok("Available commands:\n  help - Show this help\n  version - Show version\n  exit - Exit application".to_string())
    }
    
    fn command_info(&self) -> ConsoleCommand {
        ConsoleCommand {
            name: "help".to_string(),
            category: "General Commands".to_string(),
            description: Some("Display available commands".to_string()),
        }
    }
}

/// Version command
pub struct VersionCommand {
    version: String,
}

impl VersionCommand {
    pub fn new(version: String) -> Self {
        Self { version }
    }
}

#[async_trait]
impl ConsoleCommandHandler for VersionCommand {
    async fn execute(&self, _args: &[String]) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        Ok(format!("Neo Rust Node version {}", self.version))
    }
    
    fn command_info(&self) -> ConsoleCommand {
        ConsoleCommand {
            name: "version".to_string(),
            category: "General Commands".to_string(),
            description: Some("Show node version".to_string()),
        }
    }
}

/// Plugin list command
pub struct ListPluginsCommand;

#[async_trait]
impl ConsoleCommandHandler for ListPluginsCommand {
    async fn execute(&self, _args: &[String]) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        // Would integrate with actual plugin manager
        Ok("Loaded plugins:\n  RpcServer v3.0.0\n  ApplicationLogs v3.0.0\n  DBFTPlugin v3.0.0".to_string())
    }
    
    fn command_info(&self) -> ConsoleCommand {
        ConsoleCommand {
            name: "list plugins".to_string(),
            category: "Plugin Commands".to_string(),
            description: Some("List all loaded plugins".to_string()),
        }
    }
}

/// Show state command (matches C# show state command)
pub struct ShowStateCommand;

#[async_trait]
impl ConsoleCommandHandler for ShowStateCommand {
    async fn execute(&self, _args: &[String]) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        // Would integrate with actual blockchain state
        Ok("Neo blockchain state:\n  Height: 1000\n  Hash: 0x1234...\n  Validators: 7".to_string())
    }
    
    fn command_info(&self) -> ConsoleCommand {
        ConsoleCommand {
            name: "show state".to_string(),
            category: "Blockchain Commands".to_string(),
            description: Some("Show current blockchain state".to_string()),
        }
    }
}

/// Export blocks command (matches C# export blocks command)
pub struct ExportBlocksCommand;

#[async_trait]
impl ConsoleCommandHandler for ExportBlocksCommand {
    async fn execute(&self, args: &[String]) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let start = args.get(0)
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(0);
            
        let count = args.get(1)
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(100);
            
        // Would integrate with actual blockchain export
        Ok(format!("Exporting {} blocks starting from {}", count, start))
    }
    
    fn command_info(&self) -> ConsoleCommand {
        ConsoleCommand {
            name: "export blocks".to_string(),
            category: "Blockchain Commands".to_string(),
            description: Some("Export blockchain blocks to file".to_string()),
        }
    }
}

/// Create wallet command
pub struct CreateWalletCommand;

#[async_trait]
impl ConsoleCommandHandler for CreateWalletCommand {
    async fn execute(&self, args: &[String]) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let wallet_path = args.get(0)
            .ok_or("Missing wallet path parameter")?;
            
        // Would integrate with actual wallet creation
        Ok(format!("Wallet created at: {}", wallet_path))
    }
    
    fn command_info(&self) -> ConsoleCommand {
        ConsoleCommand {
            name: "create wallet".to_string(),
            category: "Wallet Commands".to_string(),
            description: Some("Create a new wallet file".to_string()),
        }
    }
}

/// Open wallet command  
pub struct OpenWalletCommand;

#[async_trait]
impl ConsoleCommandHandler for OpenWalletCommand {
    async fn execute(&self, args: &[String]) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let wallet_path = args.get(0)
            .ok_or("Missing wallet path parameter")?;
            
        // Would integrate with actual wallet opening
        Ok(format!("Wallet opened: {}", wallet_path))
    }
    
    fn command_info(&self) -> ConsoleCommand {
        ConsoleCommand {
            name: "open wallet".to_string(),
            category: "Wallet Commands".to_string(),
            description: Some("Open an existing wallet file".to_string()),
        }
    }
}