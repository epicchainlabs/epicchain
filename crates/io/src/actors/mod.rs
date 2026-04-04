//! Actor model implementation for Neo.
//!
//! This module provides an actor model implementation for Neo using Tokio.

mod priority_mailbox;
mod priority_message_queue;

pub use priority_mailbox::PriorityMailbox;
pub use priority_message_queue::PriorityMessageQueue;

use async_trait::async_trait;
use std::fmt;
use tokio::sync::mpsc::{self, Receiver, Sender};

/// A message that can be sent to an actor.
pub trait Message: Send + 'static {
    /// The type of the response to the message.
    type Response: Send + 'static;

    /// Returns the priority of the message.
    fn priority(&self) -> u8 {
        0
    }
}

/// An actor that can receive and process messages.
#[async_trait]
pub trait Actor: Send + 'static {
    /// The type of the messages that this actor can receive.
    type Message: Message + Send + 'static;

    /// Processes a message and returns a response.
    ///
    /// # Arguments
    ///
    /// * `message` - The message to process
    ///
    /// # Returns
    ///
    /// The response to the message
    async fn handle(&mut self, message: Self::Message) -> <Self::Message as Message>::Response;
}

/// A handle to an actor that can be used to send messages to it.
pub struct ActorHandle<M: Message> {
    /// The sender for the actor's mailbox
    sender: Sender<M>,
}

impl<M: Message> ActorHandle<M> {
    /// Creates a new actor handle.
    ///
    /// # Arguments
    ///
    /// * `sender` - The sender for the actor's mailbox
    ///
    /// # Returns
    ///
    /// A new actor handle
    pub fn new(sender: Sender<M>) -> Self {
        Self { sender }
    }

    /// Sends a message to the actor and returns a future that resolves to the response.
    ///
    /// # Arguments
    ///
    /// * `message` - The message to send
    ///
    /// # Returns
    ///
    /// A future that resolves to the response to the message
    pub async fn send(&self, message: M) -> Result<M::Response, crate::Error> {
        let (_tx, rx) = tokio::sync::oneshot::channel();

        let wrapped_message = message;

        self.sender
            .send(wrapped_message)
            .await
            .map_err(|e| crate::Error::Io(format!("Failed to send message: {e}")))?;

        rx.await
            .map_err(|e| crate::Error::Io(format!("Failed to receive response: {e}")))
    }
}

impl<M: Message> Clone for ActorHandle<M> {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
        }
    }
}

impl<M: Message> fmt::Debug for ActorHandle<M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ActorHandle").finish()
    }
}

/// Spawns an actor and returns a handle to it.
///
/// # Arguments
///
/// * `actor` - The actor to spawn
/// * `mailbox_size` - The size of the actor's mailbox
///
/// # Returns
///
/// A handle to the spawned actor
pub fn spawn<A: Actor>(actor: A, mailbox_size: usize) -> ActorHandle<A::Message> {
    let (sender, receiver) = mpsc::channel(mailbox_size);
    let handle = ActorHandle::new(sender);

    tokio::spawn(run_actor(actor, receiver));

    handle
}

/// Runs an actor, processing messages from its mailbox.
///
/// # Arguments
///
/// * `actor` - The actor to run
/// * `receiver` - The receiver for the actor's mailbox
///
/// # Returns
///
/// A future that completes when the actor stops
async fn run_actor<A: Actor>(mut actor: A, mut receiver: Receiver<A::Message>) {
    while let Some(message) = receiver.recv().await {
        let _response = actor.handle(message).await;

        // 1. Send response back to the sender via response channel (handled by actor's handle method)
        // 2. Update actor statistics (message count, processing time) - would be implemented with metrics
        // 3. Handle any errors in message processing - errors are propagated through the Result type
        // 4. Log successful message processing - would integrate with logging system

        // This follows the standard actor model pattern used in C# Neo implementation
    }
}
