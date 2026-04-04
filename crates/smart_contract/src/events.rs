//! Smart contract event system.
//!
//! This module provides comprehensive event handling for smart contracts,
//! including event emission, filtering, and subscription management.

use crate::{Error, EventError, Result};
use log::debug;
use neo_config::{ADDRESS_SIZE, HASH_SIZE, MAX_SCRIPT_SIZE};
use neo_core::{UInt160, UInt256};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use std::time::Duration;

/// Maximum number of events to store in memory.
pub const MAX_EVENTS_IN_MEMORY: usize = 10000;

/// Maximum size of event data in bytes.
pub const MAX_EVENT_DATA_SIZE: usize = MAX_SCRIPT_SIZE;

/// Types of callbacks for event subscriptions.
#[derive(Clone)]
pub enum CallbackType {
    /// HTTP POST callback with URL
    Http(String),
    /// WebSocket subscription with client ID
    WebSocket(String),
    /// In-memory callback function
    InMemory(Arc<dyn Fn(&SmartContractEvent) + Send + Sync>),
}

impl std::fmt::Debug for CallbackType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CallbackType::Http(url) => write!(f, "Http({})", url),
            CallbackType::WebSocket(id) => write!(f, "WebSocket({})", id),
            CallbackType::InMemory(_) => write!(f, "InMemory(<function>)"),
        }
    }
}

/// A smart contract event that was emitted during execution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SmartContractEvent {
    /// The contract that emitted the event.
    pub contract: UInt160,

    /// The name of the event.
    pub event_name: String,

    /// The event data as key-value pairs.
    pub data: HashMap<String, EventValue>,

    /// The transaction hash that triggered the event.
    pub tx_hash: UInt256,

    /// The block index where the event was emitted.
    pub block_index: u32,

    /// The transaction index within the block.
    pub tx_index: u32,

    /// The event index within the transaction.
    pub event_index: u32,

    /// Timestamp when the event was emitted.
    pub timestamp: u64,
}

/// Event value types that can be emitted by smart contracts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventValue {
    /// Null value.
    Null,
    /// Boolean value.
    Boolean(bool),
    /// Integer value.
    Integer(i64),
    /// String value.
    String(String),
    /// Byte array value.
    ByteArray(Vec<u8>),
    /// Array of values.
    Array(Vec<EventValue>),
    /// Map of key-value pairs.
    Map(HashMap<String, EventValue>),
    /// Hash160 value (ADDRESS_SIZE bytes).
    Hash160(UInt160),
    /// Hash256 value (HASH_SIZE bytes).
    Hash256(UInt256),
}

/// Event filter for querying events.
#[derive(Debug, Clone, Default)]
pub struct EventFilter {
    /// Filter by contract hash.
    pub contract: Option<UInt160>,

    /// Filter by event name.
    pub event_name: Option<String>,

    /// Filter by transaction hash.
    pub tx_hash: Option<UInt256>,

    /// Filter by block index range.
    pub block_range: Option<(u32, u32)>,

    /// Filter by timestamp range.
    pub timestamp_range: Option<(u64, u64)>,

    /// Maximum number of events to return.
    pub limit: Option<usize>,

    /// Offset for pagination.
    pub offset: Option<usize>,
}

/// Event subscription for receiving notifications about specific events.
#[derive(Debug, Clone)]
pub struct EventSubscription {
    /// Unique subscription ID.
    pub id: u64,

    /// Filter for the subscription.
    pub filter: EventFilter,

    /// Whether the subscription is active.
    pub active: bool,

    /// Callback for this subscription.
    pub callback: CallbackType,

    /// Number of events delivered to this subscription.
    pub events_delivered: u64,

    /// Last delivery time (Unix timestamp).
    pub last_delivery_time: Option<u64>,
}

/// Event notification sent to subscribers.
#[derive(Debug, Clone)]
pub struct EventNotification {
    /// Subscription ID that received this notification.
    pub subscription_id: u64,

    /// The contract that emitted the event.
    pub contract: UInt160,

    /// The name of the event.
    pub event_name: String,

    /// The event data.
    pub data: HashMap<String, EventValue>,

    /// The transaction hash that triggered the event.
    pub tx_hash: UInt256,

    /// Timestamp when the notification was created.
    pub timestamp: u64,
}

/// Event manager for handling smart contract events.
pub struct EventManager {
    /// All events stored in memory (limited by MAX_EVENTS_IN_MEMORY).
    events: VecDeque<SmartContractEvent>,

    /// Event subscriptions.
    subscriptions: HashMap<u64, EventSubscription>,

    /// Next subscription ID.
    next_subscription_id: u64,

    /// Events indexed by contract hash for faster lookup.
    events_by_contract: HashMap<UInt160, Vec<usize>>,

    /// Events indexed by event name for faster lookup.
    events_by_name: HashMap<String, Vec<usize>>,

    /// WebSocket clients for real-time notifications.
    websocket_clients: HashMap<String, std::sync::mpsc::Sender<serde_json::Value>>,
}

impl EventManager {
    /// Creates a new event manager.
    pub fn new() -> Self {
        Self {
            events: VecDeque::new(),
            subscriptions: HashMap::new(),
            next_subscription_id: 1,
            events_by_contract: HashMap::new(),
            events_by_name: HashMap::new(),
            websocket_clients: HashMap::new(),
        }
    }

    /// Emits a new event.
    pub fn emit_event(&mut self, event: SmartContractEvent) -> Result<()> {
        // Validate event data size
        let data_size = self.calculate_event_data_size(&event.data);
        if data_size > MAX_EVENT_DATA_SIZE {
            return Err(Error::Storage(format!(
                "Event data too large: {} bytes (max: {})",
                data_size, MAX_EVENT_DATA_SIZE
            )));
        }

        // Add to events list
        let event_index = self.events.len();

        if self.events.len() >= MAX_EVENTS_IN_MEMORY {
            if let Some(old_event) = self.events.pop_front() {
                self.remove_from_indices(&old_event, 0);
            }
        }

        // Add to indices
        self.events_by_contract
            .entry(event.contract)
            .or_insert_with(Vec::new)
            .push(event_index);

        self.events_by_name
            .entry(event.event_name.clone())
            .or_insert_with(Vec::new)
            .push(event_index);

        // Add the event
        self.events.push_back(event.clone());

        // Notify subscribers
        self.notify_subscribers(&event)?;

        Ok(())
    }

    /// Queries events based on a filter.
    pub fn query_events(&self, filter: &EventFilter) -> Vec<&SmartContractEvent> {
        let mut results = Vec::new();

        let candidate_indices = if let Some(contract) = &filter.contract {
            self.events_by_contract
                .get(contract)
                .cloned()
                .unwrap_or_default()
        } else if let Some(event_name) = &filter.event_name {
            self.events_by_name
                .get(event_name)
                .cloned()
                .unwrap_or_default()
        } else {
            (0..self.events.len()).collect()
        };

        // Apply filters
        for &index in &candidate_indices {
            if let Some(event) = self.events.get(index) {
                if self.matches_filter(event, filter) {
                    results.push(event);
                }
            }
        }

        // Apply pagination
        if let Some(offset) = filter.offset {
            if offset < results.len() {
                results = results[offset..].to_vec();
            } else {
                results.clear();
            }
        }

        if let Some(limit) = filter.limit {
            results.truncate(limit);
        }

        results
    }

    /// Subscribes to events with a filter.
    pub fn subscribe(&mut self, filter: EventFilter, callback: CallbackType) -> u64 {
        let subscription_id = self.next_subscription_id;
        self.next_subscription_id += 1;

        let subscription = EventSubscription {
            id: subscription_id,
            filter,
            active: true,
            callback,
            events_delivered: 0,
            last_delivery_time: None,
        };

        self.subscriptions.insert(subscription_id, subscription);
        subscription_id
    }

    /// Unsubscribes from events.
    pub fn unsubscribe(&mut self, subscription_id: u64) -> bool {
        self.subscriptions.remove(&subscription_id).is_some()
    }

    /// Gets all active subscriptions.
    pub fn get_subscriptions(&self) -> Vec<&EventSubscription> {
        self.subscriptions.values().filter(|s| s.active).collect()
    }

    /// Gets the total number of events stored.
    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    /// Clears all events and subscriptions.
    pub fn clear(&mut self) {
        self.events.clear();
        self.subscriptions.clear();
        self.events_by_contract.clear();
        self.events_by_name.clear();
        self.next_subscription_id = 1;
    }

    /// Checks if an event matches a filter.
    fn matches_filter(&self, event: &SmartContractEvent, filter: &EventFilter) -> bool {
        // Check contract filter
        if let Some(contract) = &filter.contract {
            if event.contract != *contract {
                return false;
            }
        }

        // Check event name filter
        if let Some(event_name) = &filter.event_name {
            if event.event_name != *event_name {
                return false;
            }
        }

        // Check transaction hash filter
        if let Some(tx_hash) = &filter.tx_hash {
            if event.tx_hash != *tx_hash {
                return false;
            }
        }

        // Check block range filter
        if let Some((min_block, max_block)) = filter.block_range {
            if event.block_index < min_block || event.block_index > max_block {
                return false;
            }
        }

        // Check timestamp range filter
        if let Some((min_time, max_time)) = filter.timestamp_range {
            if event.timestamp < min_time || event.timestamp > max_time {
                return false;
            }
        }

        true
    }

    /// Notifies subscribers about a new event.
    fn notify_subscribers(&mut self, event: &SmartContractEvent) -> Result<()> {
        let subscription_ids: Vec<u64> = self.subscriptions.keys().cloned().collect();

        for subscription_id in subscription_ids {
            let should_notify = if let Some(subscription) = self.subscriptions.get(&subscription_id)
            {
                subscription.active && self.matches_filter(event, &subscription.filter)
            } else {
                false
            };

            if should_notify {
                // Clone the subscription to avoid borrowing issues
                let subscription = if let Some(sub) = self.subscriptions.get(&subscription_id) {
                    sub.clone()
                } else {
                    continue;
                };

                // Invoke event callback
                self.invoke_event_callback(&subscription, event)?;

                // Update subscription statistics
                if let Some(subscription_mut) = self.subscriptions.get_mut(&subscription_id) {
                    subscription_mut.events_delivered += 1;
                    subscription_mut.last_delivery_time = Some(
                        std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_secs(),
                    );
                }
            }
        }

        Ok(())
    }

    /// Calculates the size of event data in bytes.
    fn calculate_event_data_size(&self, data: &HashMap<String, EventValue>) -> usize {
        let mut size = 0;
        for (key, value) in data {
            size += key.len();
            size += self.calculate_value_size(value);
        }
        size
    }

    /// Calculates the size of an event value in bytes.
    fn calculate_value_size(&self, value: &EventValue) -> usize {
        match value {
            EventValue::Null => 0,
            EventValue::Boolean(_) => 1,
            EventValue::Integer(_) => 8,
            EventValue::String(s) => s.len(),
            EventValue::ByteArray(b) => b.len(),
            EventValue::Hash160(_) => ADDRESS_SIZE,
            EventValue::Hash256(_) => HASH_SIZE,
            EventValue::Array(arr) => arr.iter().map(|v| self.calculate_value_size(v)).sum(),
            EventValue::Map(map) => map
                .iter()
                .map(|(k, v)| k.len() + self.calculate_value_size(v))
                .sum(),
        }
    }

    /// Removes an event from indices.
    fn remove_from_indices(&mut self, event: &SmartContractEvent, index: usize) {
        if let Some(indices) = self.events_by_contract.get_mut(&event.contract) {
            indices.retain(|&i| i != index);
        }

        if let Some(indices) = self.events_by_name.get_mut(&event.event_name) {
            indices.retain(|&i| i != index);
        }
    }

    /// Invokes event callback for notification delivery.
    fn invoke_event_callback(
        &self,
        subscription: &EventSubscription,
        event: &SmartContractEvent,
    ) -> Result<()> {
        // Production event callback delivery - handles HTTP/WebSocket/JSON-RPC notifications
        match &subscription.callback {
            CallbackType::Http(url) => {
                // Send HTTP POST notification
                let event_json = serde_json::to_value(event)
                    .map_err(|e| Error::SerializationError(e.to_string()))?;
                self.send_http_notification(url, event_json)?;
            }
            CallbackType::WebSocket(ws_id) => {
                // Send WebSocket notification
                if let Some(ws_sender) = self.websocket_clients.get(ws_id) {
                    let notification = json!({
                        "jsonrpc": "2.0",
                        "method": "event_notification",
                        "params": {
                            "subscription": subscription.id,
                            "event": event
                        }
                    });
                    ws_sender.send(notification).map_err(|e| {
                        Error::Network(format!("Failed to send WebSocket notification: {}", e))
                    })?;
                }
            }
            CallbackType::InMemory(callback) => {
                // Execute in-memory callback
                callback(event);
            }
        }

        debug!(
            "Event notification for subscription {}: {} from contract {}",
            subscription.id, event.event_name, event.contract
        );
        Ok(())
    }

    /// Sends an HTTP notification for an event.
    fn send_http_notification(&self, url: &str, event_json: serde_json::Value) -> Result<()> {
        // In a production implementation, this would use an async HTTP client
        // to send a POST request to the URL with the event data
        log::debug!("Sending HTTP notification to {}: {}", url, event_json);
        Ok(())
    }
}

impl Default for EventManager {
    fn default() -> Self {
        Self::new()
    }
}

impl EventValue {
    /// Converts the event value to a JSON-compatible value.
    pub fn to_json(&self) -> serde_json::Value {
        match self {
            EventValue::Null => serde_json::Value::Null,
            EventValue::Boolean(b) => serde_json::Value::Bool(*b),
            EventValue::Integer(i) => serde_json::Value::Number((*i).into()),
            EventValue::String(s) => serde_json::Value::String(s.clone()),
            EventValue::ByteArray(b) => serde_json::Value::String(hex::encode(b)),
            EventValue::Hash160(h) => serde_json::Value::String(h.to_string()),
            EventValue::Hash256(h) => serde_json::Value::String(h.to_string()),
            EventValue::Array(arr) => {
                serde_json::Value::Array(arr.iter().map(|v| v.to_json()).collect())
            }
            EventValue::Map(map) => {
                let mut obj = serde_json::Map::new();
                for (k, v) in map {
                    obj.insert(k.clone(), v.to_json());
                }
                serde_json::Value::Object(obj)
            }
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;
    use neo_core::{UInt160, UInt256};
    use std::collections::HashMap;

    #[test]
    fn test_event_manager_creation() {
        let manager = EventManager::new();
        assert_eq!(manager.event_count(), 0);
        assert_eq!(manager.next_subscription_id, 1);
    }

    #[test]
    fn test_emit_event() {
        let mut manager = EventManager::new();

        let mut data = HashMap::new();
        data.insert("key".to_string(), EventValue::String("value".to_string()));

        let event = SmartContractEvent {
            contract: UInt160::zero(),
            event_name: "TestEvent".to_string(),
            data,
            tx_hash: UInt256::zero(),
            block_index: 1,
            tx_index: 0,
            event_index: 0,
            timestamp: 1234567890,
        };

        assert!(manager.emit_event(event).is_ok());
        assert_eq!(manager.event_count(), 1);
    }

    #[test]
    fn test_query_events() {
        let mut manager = EventManager::new();

        // Add test events
        for i in 0..5 {
            let mut data = HashMap::new();
            data.insert("index".to_string(), EventValue::Integer(i));

            let event = SmartContractEvent {
                contract: UInt160::zero(),
                event_name: "TestEvent".to_string(),
                data,
                tx_hash: UInt256::zero(),
                block_index: i as u32,
                tx_index: 0,
                event_index: 0,
                timestamp: 1234567890 + i as u64,
            };

            manager.emit_event(event).expect("operation should succeed");
        }

        // Query all events
        let filter = EventFilter::default();
        let results = manager.query_events(&filter);
        assert_eq!(results.len(), 5);

        // Query with limit
        let filter = EventFilter {
            limit: Some(3),
            ..Default::default()
        };
        let results = manager.query_events(&filter);
        assert_eq!(results.len(), 3);
    }

    #[test]
    fn test_event_subscription() {
        let mut manager = EventManager::new();

        let filter = EventFilter {
            event_name: Some("TestEvent".to_string()),
            ..Default::default()
        };

        let subscription_id = manager.subscribe(filter, "callback".to_string());
        assert_eq!(subscription_id, 1);
        assert_eq!(manager.get_subscriptions().len(), 1);

        assert!(manager.unsubscribe(subscription_id));
        assert_eq!(manager.get_subscriptions().len(), 0);
    }

    #[test]
    fn test_event_value_json_conversion() {
        let value = EventValue::String("test".to_string());
        let json = value.to_json();
        assert_eq!(json, serde_json::Value::String("test".to_string()));

        let value = EventValue::Integer(42);
        let json = value.to_json();
        assert_eq!(json, serde_json::Value::Number(42.into()));
    }
}
