//! Priority message queue implementation for Neo.
//!
//! This module provides a priority queue for messages in the Neo actor model.

use super::Message;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fmt;

/// A priority queue for messages.
pub struct PriorityMessageQueue<M: Message> {
    /// The underlying binary heap
    queue: BinaryHeap<PriorityMessage<M>>,

    /// The maximum size of the queue
    max_size: usize,
}

/// A message with a priority.
struct PriorityMessage<M: Message> {
    /// The message
    message: M,

    /// The priority of the message
    priority: u8,

    /// The sequence number of the message
    sequence: u64,
}

impl<M: Message> PriorityMessage<M> {
    /// Creates a new priority message.
    ///
    /// # Arguments
    ///
    /// * `message` - The message
    /// * `sequence` - The sequence number of the message
    ///
    /// # Returns
    ///
    /// A new priority message
    fn new(message: M, sequence: u64) -> Self {
        let priority = message.priority();
        Self {
            message,
            priority,
            sequence,
        }
    }
}

impl<M: Message> PartialEq for PriorityMessage<M> {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority && self.sequence == other.sequence
    }
}

impl<M: Message> Eq for PriorityMessage<M> {}

impl<M: Message> PartialOrd for PriorityMessage<M> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<M: Message> Ord for PriorityMessage<M> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Higher priority comes first
        self.priority
            .cmp(&other.priority)
            // Then lower sequence number comes first
            .then_with(|| Reverse(self.sequence).cmp(&Reverse(other.sequence)))
    }
}

impl<M: Message> PriorityMessageQueue<M> {
    /// Creates a new priority message queue with the given maximum size.
    ///
    /// # Arguments
    ///
    /// * `max_size` - The maximum size of the queue
    ///
    /// # Returns
    ///
    /// A new priority message queue
    pub fn new(max_size: usize) -> Self {
        Self {
            queue: BinaryHeap::new(),
            max_size,
        }
    }

    /// Returns the maximum size of the queue.
    pub fn max_size(&self) -> usize {
        self.max_size
    }

    /// Returns the number of messages in the queue.
    pub fn len(&self) -> usize {
        self.queue.len()
    }

    /// Returns whether the queue is empty.
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    /// Returns whether the queue is full.
    pub fn is_full(&self) -> bool {
        self.queue.len() >= self.max_size
    }

    /// Pushes a message onto the queue.
    ///
    /// # Arguments
    ///
    /// * `message` - The message to push
    /// * `sequence` - The sequence number of the message
    ///
    /// # Returns
    ///
    /// `true` if the message was pushed, `false` if the queue is full
    pub fn push(&mut self, message: M, sequence: u64) -> bool {
        if self.is_full() {
            return false;
        }

        let priority_message = PriorityMessage::new(message, sequence);
        self.queue.push(priority_message);

        true
    }

    /// Pops the highest-priority message from the queue.
    ///
    /// # Returns
    ///
    /// The highest-priority message, or None if the queue is empty
    pub fn pop(&mut self) -> Option<M> {
        self.queue
            .pop()
            .map(|priority_message| priority_message.message)
    }

    /// Clears the queue.
    pub fn clear(&mut self) {
        self.queue.clear();
    }
}

impl<M: Message> fmt::Debug for PriorityMessageQueue<M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PriorityMessageQueue")
            .field("len", &self.len())
            .field("max_size", &self.max_size)
            .finish()
    }
}
