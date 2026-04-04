//! Performance optimization utilities for the VM
//!
//! This module provides optimized alternatives to expensive operations
//! like cloning and memory allocation in hot paths.

use crate::{StackItem, VmError, VmResult};
use std::borrow::Cow;
use std::rc::Rc;
use std::sync::Arc;

/// Smart cloning strategies for stack items
pub struct SmartClone;

impl SmartClone {
    /// Clone only if necessary using reference counting
    ///
    /// This method avoids cloning when the item is already reference-counted
    /// or when a reference would suffice.
    pub fn clone_if_needed(item: &StackItem) -> StackItem {
        // For already reference-counted items, just clone the Rc/Arc
        // which is much cheaper than deep cloning
        match item {
            StackItem::Array(arr) => StackItem::Array(arr.clone()), // Rc clone
            StackItem::Map(map) => StackItem::Map(map.clone()),     // Rc clone
            StackItem::Struct(s) => StackItem::Struct(s.clone()),   // Rc clone
            StackItem::Buffer(buf) => StackItem::Buffer(buf.clone()), // Rc clone
            // For primitive types, cloning is cheap anyway
            _ => item.clone(),
        }
    }

    /// Get a reference or clone only if mutation is needed
    pub fn cow_item<'a>(item: &'a StackItem) -> Cow<'a, StackItem> {
        Cow::Borrowed(item)
    }

    /// Share data using Arc for thread-safe access without cloning
    pub fn share_data<T: Clone>(data: T) -> Arc<T> {
        Arc::new(data)
    }
}

/// Optimized stack operations
pub struct OptimizedStack;

impl OptimizedStack {
    /// Push without cloning when possible
    pub fn push_ref(stack: &mut Vec<StackItem>, item: StackItem) {
        // Direct move instead of clone when we own the item
        stack.push(item);
    }

    /// Peek without cloning
    pub fn peek_ref(stack: &[StackItem], index: usize) -> VmResult<&StackItem> {
        stack
            .get(stack.len() - 1 - index)
            .ok_or_else(|| VmError::StackUnderflow {
                requested: index + 1,
                available: stack.len(),
            })
    }

    /// Pop and return ownership without extra cloning
    pub fn pop_owned(stack: &mut Vec<StackItem>) -> VmResult<StackItem> {
        stack.pop().ok_or_else(|| VmError::StackUnderflow {
            requested: 1,
            available: 0,
        })
    }
}

/// Memory pool for frequent allocations
pub struct MemoryPool<T> {
    pool: Vec<T>,
    factory: Box<dyn Fn() -> T>,
}

impl<T> MemoryPool<T> {
    /// Create a new memory pool
    pub fn new(initial_capacity: usize, factory: impl Fn() -> T + 'static) -> Self {
        let mut pool = Vec::with_capacity(initial_capacity);
        for _ in 0..initial_capacity {
            pool.push(factory());
        }
        Self {
            pool,
            factory: Box::new(factory),
        }
    }

    /// Get an item from the pool or create a new one
    pub fn get(&mut self) -> T {
        self.pool.pop().unwrap_or_else(|| (self.factory)())
    }

    /// Return an item to the pool for reuse
    pub fn put(&mut self, item: T) {
        if self.pool.len() < self.pool.capacity() {
            self.pool.push(item);
        }
    }
}

/// String interning for frequently used strings
pub struct StringInterner {
    interned: std::collections::HashMap<String, Rc<str>>,
}

impl StringInterner {
    /// Create a new string interner
    pub fn new() -> Self {
        Self {
            interned: std::collections::HashMap::new(),
        }
    }

    /// Intern a string to avoid duplicates
    pub fn intern(&mut self, s: &str) -> Rc<str> {
        if let Some(interned) = self.interned.get(s) {
            interned.clone()
        } else {
            let interned: Rc<str> = s.into();
            self.interned.insert(s.to_string(), interned.clone());
            interned
        }
    }
}

/// Lazy evaluation for expensive operations
pub struct LazyValue<T> {
    value: Option<T>,
    factory: Option<Box<dyn FnOnce() -> T>>,
}

impl<T> LazyValue<T> {
    /// Create a new lazy value
    pub fn new(factory: impl FnOnce() -> T + 'static) -> Self {
        Self {
            value: None,
            factory: Some(Box::new(factory)),
        }
    }

    /// Get the value, computing it if necessary
    pub fn get(&mut self) -> &T {
        if self.value.is_none() {
            if let Some(factory) = self.factory.take() {
                self.value = Some(factory());
            }
        }
        self.value
            .as_ref()
            .expect("LazyValue should have value after factory call")
    }
}

/// Batch operations to reduce overhead
pub struct BatchProcessor<T> {
    batch: Vec<T>,
    batch_size: usize,
    processor: Box<dyn Fn(&mut Vec<T>)>,
}

impl<T> BatchProcessor<T> {
    /// Create a new batch processor
    pub fn new(batch_size: usize, processor: impl Fn(&mut Vec<T>) + 'static) -> Self {
        Self {
            batch: Vec::with_capacity(batch_size),
            batch_size,
            processor: Box::new(processor),
        }
    }

    /// Add an item to the batch
    pub fn add(&mut self, item: T) {
        self.batch.push(item);
        if self.batch.len() >= self.batch_size {
            self.flush();
        }
    }

    /// Process all pending items
    pub fn flush(&mut self) {
        if !self.batch.is_empty() {
            (self.processor)(&mut self.batch);
            self.batch.clear();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_pool() {
        let mut pool = MemoryPool::new(5, || vec![0u8; 1024]);

        let mut items = Vec::new();
        for _ in 0..10 {
            items.push(pool.get());
        }

        // Return some items
        for item in items.drain(0..5) {
            pool.put(item);
        }

        // Get should reuse returned items
        let reused = pool.get();
        assert_eq!(reused.len(), 1024);
    }

    #[test]
    fn test_string_interner() {
        let mut interner = StringInterner::new();

        let s1 = interner.intern("hello");
        let s2 = interner.intern("hello");

        // Should return the same Rc
        assert!(Rc::ptr_eq(&s1, &s2));
    }

    #[test]
    fn test_lazy_value() {
        let mut lazy = LazyValue::new(|| {
            // Expensive computation
            vec![1, 2, 3, 4, 5]
        });

        // Value is computed on first access
        let value_len = {
            let value = lazy.get();
            value.len()
        };
        assert_eq!(value_len, 5);

        // Subsequent accesses return the same value
        let value2_len = {
            let value2 = lazy.get();
            value2.len()
        };
        assert_eq!(value_len, value2_len);
    }
}
