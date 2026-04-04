//! Composite message handler that delegates to multiple handlers

use crate::messages::MessageCommand;
use crate::p2p::protocol::MessageHandler;
use crate::{NetworkMessage, NetworkResult};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::{debug, info};

/// Composite message handler that routes messages to appropriate handlers
pub struct CompositeMessageHandler {
    /// Map of message types to handlers
    handlers: HashMap<MessageCommand, Arc<dyn MessageHandler>>,
    /// Default handler for unregistered message types
    default_handler: Arc<dyn MessageHandler>,
}

impl CompositeMessageHandler {
    /// Creates a new composite message handler
    pub fn new(default_handler: Arc<dyn MessageHandler>) -> Self {
        Self {
            handlers: HashMap::new(),
            default_handler,
        }
    }

    /// Registers a handler for a specific message type
    pub fn register_handler(
        &mut self,
        message_type: MessageCommand,
        handler: Arc<dyn MessageHandler>,
    ) {
        info!("Registering handler for message type: {:?}", message_type);
        self.handlers.insert(message_type, handler);
    }

    /// Registers handlers for multiple message types
    pub fn register_handlers(&mut self, handlers: Vec<(MessageCommand, Arc<dyn MessageHandler>)>) {
        for (msg_type, handler) in handlers {
            self.register_handler(msg_type, handler);
        }
    }
}

#[async_trait::async_trait]
impl MessageHandler for CompositeMessageHandler {
    async fn handle_message(
        &self,
        peer_address: SocketAddr,
        message: &NetworkMessage,
    ) -> NetworkResult<()> {
        let command = message.header.command;

        // Try to find a specific handler for this message type
        if let Some(handler) = self.handlers.get(&command) {
            debug!(
                "Routing {:?} message from {} to specific handler",
                command, peer_address
            );
            handler.handle_message(peer_address, message).await
        } else {
            debug!(
                "Routing {:?} message from {} to default handler",
                command, peer_address
            );
            self.default_handler
                .handle_message(peer_address, message)
                .await
        }
    }
}
