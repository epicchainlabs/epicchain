//! Memory pool for optimizing allocations in hot paths
//!
//! This module provides object pooling to reduce allocation overhead
//! for frequently created and destroyed objects in the VM.

// ExecutionContext import removed - not used in current implementation
use crate::stack_item::StackItem;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

/// Maximum number of pooled objects per type
const MAX_POOL_SIZE: usize = 1024;

/// Thread-safe object pool for reusing allocations
pub struct ObjectPool<T> {
    pool: Arc<Mutex<VecDeque<T>>>,
    factory: Arc<dyn Fn() -> T + Send + Sync>,
    reset: Arc<dyn Fn(&mut T) + Send + Sync>,
    max_size: usize,
    allocations: Arc<AtomicUsize>,
    hits: Arc<AtomicUsize>,
}

impl<T> ObjectPool<T> {
    /// Creates a new object pool
    pub fn new<F, R>(factory: F, reset: R) -> Self
    where
        F: Fn() -> T + Send + Sync + 'static,
        R: Fn(&mut T) + Send + Sync + 'static,
    {
        Self {
            pool: Arc::new(Mutex::new(VecDeque::with_capacity(MAX_POOL_SIZE))),
            factory: Arc::new(factory),
            reset: Arc::new(reset),
            max_size: MAX_POOL_SIZE,
            allocations: Arc::new(AtomicUsize::new(0)),
            hits: Arc::new(AtomicUsize::new(0)),
        }
    }

    /// Gets an object from the pool or creates a new one
    pub fn get(&self) -> PooledObject<T> {
        let mut pool = self.pool.lock().unwrap();
        let obj = if let Some(obj) = pool.pop_front() {
            self.hits.fetch_add(1, Ordering::Relaxed);
            obj
        } else {
            self.allocations.fetch_add(1, Ordering::Relaxed);
            (self.factory)()
        };

        PooledObject {
            inner: Some(obj),
            pool: Arc::clone(&self.pool),
            reset: Arc::clone(&self.reset),
            max_size: self.max_size,
        }
    }

    /// Returns the current size of the pool
    pub fn size(&self) -> usize {
        self.pool.lock().unwrap().len()
    }

    /// Clears the pool
    pub fn clear(&self) {
        self.pool.lock().unwrap().clear();
    }

    /// Gets the total number of allocations made
    pub fn total_allocations(&self) -> usize {
        self.allocations.load(Ordering::Relaxed)
    }

    /// Gets the number of pool hits (reused objects)
    pub fn pool_hits(&self) -> usize {
        self.hits.load(Ordering::Relaxed)
    }

    /// Gets the hit ratio as a percentage (0-100)
    pub fn hit_ratio(&self) -> f32 {
        let hits = self.hits.load(Ordering::Relaxed);
        let allocations = self.allocations.load(Ordering::Relaxed);
        if allocations == 0 {
            0.0
        } else {
            (hits as f32 / (hits + allocations) as f32) * 100.0
        }
    }
}

/// RAII wrapper that returns objects to the pool when dropped
pub struct PooledObject<T> {
    inner: Option<T>,
    pool: Arc<Mutex<VecDeque<T>>>,
    reset: Arc<dyn Fn(&mut T) + Send + Sync>,
    max_size: usize,
}

impl<T> PooledObject<T> {
    /// Takes ownership of the inner value
    pub fn take(mut self) -> T {
        self.inner.take().expect("Value already taken")
    }
}

impl<T> std::ops::Deref for PooledObject<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner.as_ref().expect("Value already taken")
    }
}

impl<T> std::ops::DerefMut for PooledObject<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.as_mut().expect("Value already taken")
    }
}

impl<T> Drop for PooledObject<T> {
    fn drop(&mut self) {
        if let Some(mut obj) = self.inner.take() {
            // Reset the object before returning to pool
            (self.reset)(&mut obj);

            // Return to pool if not full
            let mut pool = self.pool.lock().unwrap();
            if pool.len() < self.max_size {
                pool.push_back(obj);
            }
        }
    }
}

/// Global memory pools for VM objects
pub struct VmMemoryPools {
    /// Pool for Vec<u8> buffers
    pub byte_buffers: ObjectPool<Vec<u8>>,
    /// Pool for instruction buffers
    pub instruction_buffers: ObjectPool<Vec<crate::instruction::Instruction>>,
    /// Pool for stack item vectors
    pub stack_item_vecs: ObjectPool<Vec<StackItem>>,
}

impl VmMemoryPools {
    /// Creates new VM memory pools
    pub fn new() -> Self {
        Self {
            byte_buffers: ObjectPool::new(
                || Vec::with_capacity(512), // Increased for typical script sizes
                |v| {
                    v.clear();
                    v.shrink_to(512);
                }, // Prevent excessive growth
            ),
            instruction_buffers: ObjectPool::new(
                || Vec::with_capacity(128), // Increased for complex scripts
                |v| {
                    v.clear();
                    v.shrink_to(128);
                },
            ),
            stack_item_vecs: ObjectPool::new(
                || Vec::with_capacity(32), // Increased for typical stack operations
                |v| {
                    v.clear();
                    v.shrink_to(32);
                },
            ),
        }
    }

    /// Gets a byte buffer from the pool
    pub fn get_byte_buffer(&self) -> PooledObject<Vec<u8>> {
        self.byte_buffers.get()
    }

    /// Gets an instruction buffer from the pool
    pub fn get_instruction_buffer(&self) -> PooledObject<Vec<crate::instruction::Instruction>> {
        self.instruction_buffers.get()
    }

    /// Gets a stack item vector from the pool
    pub fn get_stack_item_vec(&self) -> PooledObject<Vec<StackItem>> {
        self.stack_item_vecs.get()
    }

    /// Clears all pools
    pub fn clear_all(&self) {
        self.byte_buffers.clear();
        self.instruction_buffers.clear();
        self.stack_item_vecs.clear();
    }

    /// Gets statistics about pool usage
    pub fn stats(&self) -> MemoryPoolStats {
        let byte_buffer_allocs = self.byte_buffers.total_allocations();
        let instruction_allocs = self.instruction_buffers.total_allocations();
        let stack_item_allocs = self.stack_item_vecs.total_allocations();

        // Estimate memory usage (rough calculation)
        let estimated_memory = byte_buffer_allocs * 512 + // avg byte buffer size
            instruction_allocs * 128 * std::mem::size_of::<crate::instruction::Instruction>() +
            stack_item_allocs * 32 * std::mem::size_of::<crate::stack_item::StackItem>();

        MemoryPoolStats {
            byte_buffers_pooled: self.byte_buffers.size(),
            instruction_buffers_pooled: self.instruction_buffers.size(),
            stack_item_vecs_pooled: self.stack_item_vecs.size(),
            byte_buffers_allocated: byte_buffer_allocs,
            instruction_buffers_allocated: instruction_allocs,
            stack_item_vecs_allocated: stack_item_allocs,
            total_memory_used: estimated_memory,
        }
    }

    /// Gets detailed pool performance metrics
    pub fn performance_metrics(&self) -> PoolPerformanceMetrics {
        PoolPerformanceMetrics {
            byte_buffer_hit_ratio: self.byte_buffers.hit_ratio(),
            instruction_buffer_hit_ratio: self.instruction_buffers.hit_ratio(),
            stack_item_vec_hit_ratio: self.stack_item_vecs.hit_ratio(),
            overall_efficiency: (self.byte_buffers.hit_ratio()
                + self.instruction_buffers.hit_ratio()
                + self.stack_item_vecs.hit_ratio())
                / 3.0,
        }
    }
}

impl Default for VmMemoryPools {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics about memory pool usage
#[derive(Debug, Clone)]
pub struct MemoryPoolStats {
    pub byte_buffers_pooled: usize,
    pub instruction_buffers_pooled: usize,
    pub stack_item_vecs_pooled: usize,
    pub byte_buffers_allocated: usize,
    pub instruction_buffers_allocated: usize,
    pub stack_item_vecs_allocated: usize,
    pub total_memory_used: usize,
}

/// Performance metrics for memory pools
#[derive(Debug, Clone)]
pub struct PoolPerformanceMetrics {
    pub byte_buffer_hit_ratio: f32,
    pub instruction_buffer_hit_ratio: f32,
    pub stack_item_vec_hit_ratio: f32,
    pub overall_efficiency: f32,
}

// Thread-local storage for memory pools - using thread_local! macro
thread_local! {
    static POOLS: VmMemoryPools = VmMemoryPools::new();
}

/// Gets the thread-local memory pools
///
/// Returns a reference to the thread-local memory pools with safe access.
/// Uses a closure-based API to ensure memory safety.
pub fn with_pools<F, R>(f: F) -> R
where
    F: FnOnce(&VmMemoryPools) -> R,
{
    POOLS.with(f)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_object_pool() {
        let pool: ObjectPool<Vec<u8>> = ObjectPool::new(|| Vec::with_capacity(100), |v| v.clear());

        // Get object from pool
        let mut obj1 = pool.get();
        obj1.push(1);
        obj1.push(2);
        assert_eq!(obj1.len(), 2);

        // Drop returns to pool
        drop(obj1);
        assert_eq!(pool.size(), 1);

        // Get reused object
        let obj2 = pool.get();
        assert_eq!(obj2.len(), 0); // Should be cleared
        assert!(obj2.capacity() >= 100); // Should maintain capacity
    }

    #[test]
    fn test_vm_memory_pools() {
        let pools = VmMemoryPools::new();

        // Test byte buffer pool
        {
            let mut buffer = pools.get_byte_buffer();
            buffer.extend_from_slice(b"test");
            assert_eq!(buffer.len(), 4);
        }

        // Check that buffer was returned to pool
        assert_eq!(pools.byte_buffers.size(), 1);

        // Get stats
        let stats = pools.stats();
        assert_eq!(stats.byte_buffers_pooled, 1);
    }
}
