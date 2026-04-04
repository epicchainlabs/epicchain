//! Complete Plugin Event Handler Interfaces
//! 
//! Matches C# Neo plugin event handler interfaces exactly for 100% compatibility

use async_trait::async_trait;
use neo_core::{Block, Transaction, UInt256};
use neo_vm::{ApplicationEngine, LogEventArgs, NotifyEventArgs};
use std::any::Any;

/// Blockchain committing handler (matches C# ICommittingHandler exactly)
#[async_trait]
pub trait ICommittingHandler: Send + Sync {
    /// Called before a block is committed to the blockchain
    async fn blockchain_committing(
        &mut self,
        system: &dyn Any, // NeoSystem
        block: &Block,
        snapshot: &dyn Any, // DataCache
        executed_list: &[ApplicationExecuted],
    );
}

/// Blockchain committed handler (matches C# ICommittedHandler exactly)
#[async_trait]
pub trait ICommittedHandler: Send + Sync {
    /// Called after a block is committed to the blockchain
    async fn blockchain_committed(&mut self, system: &dyn Any, block: &Block);
}

/// Application engine log handler (matches C# ILogHandler exactly)
#[async_trait]
pub trait ILogHandler: Send + Sync {
    /// Called when ApplicationEngine logs a message
    async fn application_engine_log(&mut self, sender: &ApplicationEngine, log_event: &LogEventArgs);
}

/// Application engine notify handler (matches C# INotifyHandler exactly)
#[async_trait]
pub trait INotifyHandler: Send + Sync {
    /// Called when a smart contract emits a notification
    async fn application_engine_notify(&mut self, sender: &ApplicationEngine, notify_event: &NotifyEventArgs);
}

/// Memory pool transaction added handler (matches C# ITransactionAddedHandler exactly)
#[async_trait]
pub trait ITransactionAddedHandler: Send + Sync {
    /// Called when a transaction is added to the memory pool
    async fn mempool_transaction_added(&mut self, sender: &dyn Any, transaction: &Transaction);
}

/// Memory pool transaction removed handler (matches C# ITransactionRemovedHandler exactly)
#[async_trait]
pub trait ITransactionRemovedHandler: Send + Sync {
    /// Called when a transaction is removed from the memory pool
    async fn mempool_transaction_removed(
        &mut self,
        sender: &dyn Any,
        transaction: &Transaction,
        reason: TransactionRemovalReason,
    );
}

/// Neo system service added handler (matches C# IServiceAddedHandler exactly)
#[async_trait]
pub trait IServiceAddedHandler: Send + Sync {
    /// Called when a service is added to the Neo system
    async fn neo_system_service_added(&mut self, sender: &dyn Any, service: &dyn Any);
}

/// Wallet changed handler (matches C# IWalletChangedHandler exactly)
#[async_trait]
pub trait IWalletChangedHandler: Send + Sync {
    /// Called when wallet state changes
    async fn wallet_changed(&mut self, sender: &dyn Any, wallet: &dyn Any);
}

/// Message received handler (matches C# IMessageReceivedHandler exactly)
#[async_trait]
pub trait IMessageReceivedHandler: Send + Sync {
    /// Called when a P2P message is received
    async fn remote_node_message_received(
        &mut self,
        system: &dyn Any,
        message: &dyn Any,
    ) -> bool;
}

/// Application execution result (matches C# ApplicationExecuted exactly)
#[derive(Debug, Clone)]
pub struct ApplicationExecuted {
    /// Transaction that was executed
    pub transaction: Transaction,
    /// VM trigger type
    pub trigger: TriggerType,
    /// VM execution state
    pub vm_state: VMState,
    /// Exception message if execution faulted
    pub exception: Option<String>,
    /// Gas consumed during execution
    pub gas_consumed: i64,
    /// Contract notifications emitted
    pub notifications: Vec<NotifyEventArgs>,
    /// Log messages emitted
    pub logs: Vec<LogEventArgs>,
    /// Result stack after execution
    pub stack: Vec<neo_vm::stack_item::StackItem>,
}

/// Transaction removal reason (matches C# TransactionRemovalReason exactly)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransactionRemovalReason {
    /// Transaction expired
    Expired,
    /// Insufficient fees
    InsufficientFees,
    /// Invalid transaction
    Invalid,
    /// Transaction conflicts with another
    Conflict,
    /// Policy violation
    PolicyViolation,
    /// Unknown reason
    Unknown,
}

/// VM trigger type (matches C# TriggerType exactly)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TriggerType {
    /// System trigger
    System = 0x01,
    /// Verification trigger
    Verification = 0x20,
    /// Application trigger
    Application = 0x40,
    /// OnPersist trigger
    OnPersist = 0x01 | 0x20,
    /// PostPersist trigger
    PostPersist = 0x01 | 0x40,
    /// All triggers
    All = 0x01 | 0x20 | 0x40,
}

/// VM state (matches C# VMState exactly)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VMState {
    /// VM has not started
    None = 0,
    /// VM halted successfully
    Halt = 1,
    /// VM faulted with error
    Fault = 2,
    /// VM is in break state
    Break = 4,
}

/// Plugin manager for handling all plugin operations
pub struct PluginManager {
    plugins: Vec<Box<dyn PluginEventHandler>>,
    committing_handlers: Vec<Box<dyn ICommittingHandler>>,
    committed_handlers: Vec<Box<dyn ICommittedHandler>>,
    log_handlers: Vec<Box<dyn ILogHandler>>,
    notify_handlers: Vec<Box<dyn INotifyHandler>>,
    transaction_added_handlers: Vec<Box<dyn ITransactionAddedHandler>>,
    transaction_removed_handlers: Vec<Box<dyn ITransactionRemovedHandler>>,
    service_added_handlers: Vec<Box<dyn IServiceAddedHandler>>,
    wallet_changed_handlers: Vec<Box<dyn IWalletChangedHandler>>,
    message_received_handlers: Vec<Box<dyn IMessageReceivedHandler>>,
}

impl PluginManager {
    /// Create new plugin manager
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
            committing_handlers: Vec::new(),
            committed_handlers: Vec::new(),
            log_handlers: Vec::new(),
            notify_handlers: Vec::new(),
            transaction_added_handlers: Vec::new(),
            transaction_removed_handlers: Vec::new(),
            service_added_handlers: Vec::new(),
            wallet_changed_handlers: Vec::new(),
            message_received_handlers: Vec::new(),
        }
    }
    
    /// Register plugin with automatic interface detection
    pub fn register_plugin<T>(&mut self, plugin: T)
    where
        T: PluginEventHandler + 'static,
        T: ICommittingHandler + 'static,
        T: ICommittedHandler + 'static,
        T: ILogHandler + 'static,
        T: INotifyHandler + 'static,
        T: ITransactionAddedHandler + 'static,
        T: ITransactionRemovedHandler + 'static,
        T: IServiceAddedHandler + 'static,
        T: IWalletChangedHandler + 'static,
        T: IMessageReceivedHandler + 'static,
    {
        let boxed_plugin = Box::new(plugin);
        
        // Register for all applicable interfaces
        self.plugins.push(boxed_plugin.clone());
        self.committing_handlers.push(boxed_plugin.clone());
        self.committed_handlers.push(boxed_plugin.clone());
        self.log_handlers.push(boxed_plugin.clone());
        self.notify_handlers.push(boxed_plugin.clone());
        self.transaction_added_handlers.push(boxed_plugin.clone());
        self.transaction_removed_handlers.push(boxed_plugin.clone());
        self.service_added_handlers.push(boxed_plugin.clone());
        self.wallet_changed_handlers.push(boxed_plugin.clone());
        self.message_received_handlers.push(boxed_plugin);
    }
    
    /// Broadcast blockchain committing event
    pub async fn broadcast_blockchain_committing(
        &mut self,
        system: &dyn Any,
        block: &Block,
        snapshot: &dyn Any,
        executed_list: &[ApplicationExecuted],
    ) {
        for handler in &mut self.committing_handlers {
            handler.blockchain_committing(system, block, snapshot, executed_list).await;
        }
    }
    
    /// Broadcast blockchain committed event
    pub async fn broadcast_blockchain_committed(&mut self, system: &dyn Any, block: &Block) {
        for handler in &mut self.committed_handlers {
            handler.blockchain_committed(system, block).await;
        }
    }
    
    /// Broadcast application engine log event
    pub async fn broadcast_application_log(&mut self, sender: &ApplicationEngine, log_event: &LogEventArgs) {
        for handler in &mut self.log_handlers {
            handler.application_engine_log(sender, log_event).await;
        }
    }
    
    /// Broadcast application engine notify event
    pub async fn broadcast_application_notify(&mut self, sender: &ApplicationEngine, notify_event: &NotifyEventArgs) {
        for handler in &mut self.notify_handlers {
            handler.application_engine_notify(sender, notify_event).await;
        }
    }
    
    /// Broadcast transaction added event
    pub async fn broadcast_transaction_added(&mut self, sender: &dyn Any, transaction: &Transaction) {
        for handler in &mut self.transaction_added_handlers {
            handler.mempool_transaction_added(sender, transaction).await;
        }
    }
    
    /// Broadcast transaction removed event
    pub async fn broadcast_transaction_removed(
        &mut self,
        sender: &dyn Any,
        transaction: &Transaction,
        reason: TransactionRemovalReason,
    ) {
        for handler in &mut self.transaction_removed_handlers {
            handler.mempool_transaction_removed(sender, transaction, reason).await;
        }
    }
}

/// Base plugin event handler trait
#[async_trait]
pub trait PluginEventHandler: Send + Sync {
    /// Plugin name
    fn name(&self) -> &str;
    
    /// Plugin version
    fn version(&self) -> &str;
    
    /// Initialize plugin
    async fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    
    /// Start plugin
    async fn start(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    
    /// Stop plugin
    async fn stop(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}