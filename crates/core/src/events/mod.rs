// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// modifications are permitted.

//! Event handling for Neo blockchain.

use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Trait for event handlers.
pub trait EventHandler: Send + Sync {
    /// Handles an event.
    ///
    /// # Arguments
    ///
    /// * `sender` - The sender of the event.
    /// * `args` - The event arguments.
    fn handle(&self, sender: &dyn Any, args: &dyn Any);
}

/// Event manager for Neo blockchain.
#[derive(Default)]
pub struct EventManager {
    handlers: RwLock<HashMap<String, Vec<Arc<dyn EventHandler>>>>,
}

impl EventManager {
    /// Creates a new EventManager.
    ///
    /// # Returns
    ///
    /// A new EventManager instance.
    pub fn new() -> Self {
        Self {
            handlers: RwLock::new(HashMap::new()),
        }
    }

    /// Registers an event handler.
    ///
    /// # Arguments
    ///
    /// * `event_name` - The name of the event.
    /// * `handler` - The event handler.
    ///
    /// # Returns
    ///
    /// A boolean indicating whether the handler was successfully registered.
    pub fn register<H: EventHandler + 'static>(&self, event_name: &str, handler: H) -> bool {
        if let Ok(mut handlers) = self.handlers.write() {
            let entry = handlers
                .entry(event_name.to_string())
                .or_insert_with(Vec::new);
            entry.push(Arc::new(handler));
            true
        } else {
            false
        }
    }

    /// Unregisters an event handler.
    ///
    /// # Arguments
    ///
    /// * `event_name` - The name of the event.
    /// * `handler` - The event handler.
    ///
    /// # Returns
    ///
    /// A boolean indicating whether the handler was successfully unregistered.
    pub fn unregister(&self, event_name: &str, handler: Arc<dyn EventHandler>) -> bool {
        if let Ok(mut handlers) = self.handlers.write() {
            if let Some(entry) = handlers.get_mut(event_name) {
                // Remove the handler by pointer equality
                let len = entry.len();
                entry.retain(|h| !Arc::ptr_eq(h, &handler));
                return len != entry.len();
            }
        }
        false
    }

    /// Triggers an event.
    ///
    /// # Arguments
    ///
    /// * `event_name` - The name of the event.
    /// * `sender` - The sender of the event.
    /// * `args` - The event arguments.
    pub fn trigger(&self, event_name: &str, sender: &dyn Any, args: &dyn Any) {
        if let Ok(handlers) = self.handlers.read() {
            if let Some(entry) = handlers.get(event_name) {
                for handler in entry {
                    handler.handle(sender, args);
                }
            }
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;
    // No additional imports needed in this unit test
    use std::any::Any;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;

    struct TestHandler {
        called: Arc<AtomicBool>,
    }
    impl EventHandler for TestHandler {
        fn handle(&self, _sender: &dyn Any, _args: &dyn Any) {
            self.called.store(true, Ordering::SeqCst);
        }
    }
    #[test]
    fn test_event_manager() {
        let manager = EventManager::new();
        let called = Arc::new(AtomicBool::new(false));
        let handler = TestHandler {
            called: called.clone(),
        };
        // Register handler
        assert!(manager.register("test_event", handler));
        // Trigger event
        manager.trigger("test_event", &"sender", &"args");
        assert!(called.load(Ordering::SeqCst));
    }
}
