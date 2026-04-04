//! Priority mailbox implementation for Neo.
//!
//! This module provides a priority mailbox for the Neo actor model.

use super::{Message, PriorityMessageQueue};
use std::fmt;
use tokio::sync::mpsc;

/// A mailbox that processes messages in priority order.
pub struct PriorityMailbox<M: Message> {
    /// The queue of messages
    queue: PriorityMessageQueue<M>,

    /// The receiver for incoming messages
    receiver: mpsc::Receiver<M>,

    /// The next sequence number to assign
    next_sequence: u64,
}

impl<M: Message> PriorityMailbox<M> {
    /// Creates a new priority mailbox with the given capacity.
    ///
    /// # Arguments
    ///
    /// * `capacity` - The capacity of the mailbox
    ///
    /// # Returns
    ///
    /// A new priority mailbox and a sender for sending messages to it
    pub fn new(capacity: usize) -> (Self, mpsc::Sender<M>) {
        let (sender, receiver) = mpsc::channel(capacity);

        let mailbox = Self {
            queue: PriorityMessageQueue::new(capacity),
            receiver,
            next_sequence: 0,
        };

        (mailbox, sender)
    }

    /// Returns the capacity of the mailbox.
    pub fn capacity(&self) -> usize {
        self.queue.max_size()
    }

    /// Returns the number of messages in the mailbox.
    pub fn len(&self) -> usize {
        self.queue.len()
    }

    /// Returns whether the mailbox is empty.
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    /// Returns whether the mailbox is full.
    pub fn is_full(&self) -> bool {
        self.queue.is_full()
    }

    /// Receives the next message from the mailbox.
    ///
    /// # Returns
    ///
    /// The next message, or None if the mailbox is closed
    pub async fn receive(&mut self) -> Option<M> {
        if let Some(message) = self.queue.pop() {
            return Some(message);
        }

        match self.receiver.recv().await {
            Some(message) => {
                let sequence = self.next_sequence;
                self.next_sequence += 1;

                if self.queue.is_empty() {
                    Some(message)
                } else {
                    // Otherwise, push the message into the queue and return the highest-priority message
                    self.queue.push(message, sequence);
                    self.queue.pop()
                }
            }
            None => None,
        }
    }

    /// Clears the mailbox.
    pub fn clear(&mut self) {
        self.queue.clear();

        // Drain the receiver
        while self.receiver.try_recv().is_ok() {}
    }
}

impl<M: Message> fmt::Debug for PriorityMailbox<M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PriorityMailbox")
            .field("len", &self.len())
            .field("capacity", &self.capacity())
            .finish()
    }
}
