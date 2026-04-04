//! Safe Operations Module
//!
//! Provides safe alternatives to common operations that might panic,
//! replacing unwrap(), expect(), and panic! patterns with recoverable errors.

use crate::error_handling::{NeoError, Result};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};

/// Safe array access without panicking
/// Defines a trait interface.
pub trait SafeIndex<T> {
    /// Get element at index, returning None if out of bounds
    fn safe_get(&self, index: usize) -> Option<&T>;

    /// Get mutable element at index, returning None if out of bounds
    fn safe_get_mut(&mut self, index: usize) -> Option<&mut T>;

    /// Get element or default if out of bounds
    fn get_or_default(&self, index: usize) -> T
    where
        T: Default + Clone;
}

impl<T> SafeIndex<T> for Vec<T> {
    fn safe_get(&self, index: usize) -> Option<&T> {
        if index < self.len() {
            Some(&self[index])
        } else {
            tracing::warn!(
                "Index {} out of bounds for vector of length {}",
                index,
                self.len()
            );
            None
        }
    }

    fn safe_get_mut(&mut self, index: usize) -> Option<&mut T> {
        let len = self.len();
        if index < len {
            Some(&mut self[index])
        } else {
            tracing::warn!("Index {} out of bounds for vector of length {}", index, len);
            None
        }
    }

    fn get_or_default(&self, index: usize) -> T
    where
        T: Default + Clone,
    {
        self.safe_get(index).cloned().unwrap_or_default()
    }
}

/// Safe hashmap operations
/// Defines a trait interface.
pub trait SafeMap<K, V> {
    /// Get value with proper error handling
    fn safe_get(&self, key: &K) -> Option<&V>;

    /// Insert with overflow protection
    fn safe_insert(&mut self, key: K, value: V) -> Result<Option<V>>;

    /// Remove with existence check
    fn safe_remove(&mut self, key: &K) -> Option<V>;
}

impl<K: Eq + std::hash::Hash + std::fmt::Debug, V> SafeMap<K, V> for HashMap<K, V> {
    fn safe_get(&self, key: &K) -> Option<&V> {
        let result = self.get(key);
        if result.is_none() {
            tracing::debug!("Key {:?} not found in map", key);
        }
        result
    }

    fn safe_insert(&mut self, key: K, value: V) -> Result<Option<V>> {
        // Check for capacity issues
        if self.len() >= self.capacity() && self.capacity() > usize::MAX / 2 {
            return Err(NeoError::Internal(
                "HashMap capacity overflow risk".to_string(),
            ));
        }
        Ok(self.insert(key, value))
    }

    fn safe_remove(&mut self, key: &K) -> Option<V> {
        let result = self.remove(key);
        if result.is_none() {
            tracing::debug!("Attempted to remove non-existent key {:?}", key);
        }
        result
    }
}

/// Safe arithmetic operations that prevent overflow/underflow panics
/// Defines a trait interface.
pub trait SafeArithmetic: Sized {
    /// Safe addition with overflow check
    fn safe_add(self, rhs: Self) -> Result<Self>;

    /// Safe subtraction with underflow check
    fn safe_sub(self, rhs: Self) -> Result<Self>;

    /// Safe multiplication with overflow check
    fn safe_mul(self, rhs: Self) -> Result<Self>;

    /// Safe division with zero check
    fn safe_div(self, rhs: Self) -> Result<Self>;
}

macro_rules! impl_safe_arithmetic {
    ($($t:ty)*) => {
        $(
            impl SafeArithmetic for $t {
                fn safe_add(self, rhs: Self) -> Result<Self> {
                    self.checked_add(rhs)
                        .ok_or_else(|| NeoError::Internal(format!("Arithmetic overflow: {} + {}", self, rhs)))
                }

                fn safe_sub(self, rhs: Self) -> Result<Self> {
                    self.checked_sub(rhs)
                        .ok_or_else(|| NeoError::Internal(format!("Arithmetic underflow: {} - {}", self, rhs)))
                }

                fn safe_mul(self, rhs: Self) -> Result<Self> {
                    self.checked_mul(rhs)
                        .ok_or_else(|| NeoError::Internal(format!("Arithmetic overflow: {} * {}", self, rhs)))
                }

                fn safe_div(self, rhs: Self) -> Result<Self> {
                    if rhs == 0 {
                        Err(NeoError::Internal("Division by zero".to_string()))
                    } else {
                        self.checked_div(rhs)
                            .ok_or_else(|| NeoError::Internal(format!("Division overflow: {} / {}", self, rhs)))
                    }
                }
            }
        )*
    }
}

impl_safe_arithmetic!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);

/// Safe mutex operations without poisoning panics
/// Represents a data structure.
pub struct SafeMutex<T> {
    inner: Arc<Mutex<T>>,
}

impl<T> SafeMutex<T> {
    /// Creates a new instance.
    /// Creates a new instance.
    pub fn new(value: T) -> Self {
        Self {
            inner: Arc::new(Mutex::new(value)),
        }
    }

    /// Lock the mutex, recovering from poisoned state if necessary
    pub fn safe_lock(&self) -> Result<std::sync::MutexGuard<'_, T>> {
        match self.inner.lock() {
            Ok(guard) => Ok(guard),
            Err(poisoned) => {
                tracing::warn!("Mutex was poisoned, recovering");
                Ok(poisoned.into_inner())
            }
        }
    }

    /// Try to lock without blocking
    pub fn safe_try_lock(&self) -> Result<Option<std::sync::MutexGuard<'_, T>>> {
        match self.inner.try_lock() {
            Ok(guard) => Ok(Some(guard)),
            Err(std::sync::TryLockError::WouldBlock) => Ok(None),
            Err(std::sync::TryLockError::Poisoned(poisoned)) => {
                tracing::warn!("Mutex was poisoned, recovering");
                Ok(Some(poisoned.into_inner()))
            }
        }
    }
}

impl<T: Clone> Clone for SafeMutex<T> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

/// Safe RwLock operations
/// Represents a data structure.
pub struct SafeRwLock<T> {
    inner: Arc<RwLock<T>>,
}

impl<T> SafeRwLock<T> {
    /// Creates a new instance.
    /// Creates a new instance.
    pub fn new(value: T) -> Self {
        Self {
            inner: Arc::new(RwLock::new(value)),
        }
    }

    /// Read lock with poison recovery
    pub fn safe_read(&self) -> Result<std::sync::RwLockReadGuard<'_, T>> {
        match self.inner.read() {
            Ok(guard) => Ok(guard),
            Err(poisoned) => {
                tracing::warn!("RwLock was poisoned during read, recovering");
                Ok(poisoned.into_inner())
            }
        }
    }

    /// Write lock with poison recovery
    pub fn safe_write(&self) -> Result<std::sync::RwLockWriteGuard<'_, T>> {
        match self.inner.write() {
            Ok(guard) => Ok(guard),
            Err(poisoned) => {
                tracing::warn!("RwLock was poisoned during write, recovering");
                Ok(poisoned.into_inner())
            }
        }
    }
}

impl<T: Clone> Clone for SafeRwLock<T> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

/// Safe string parsing without panics
/// Defines a trait interface.
pub trait SafeParse {
    /// Output type for the operation.
    type Output;

    /// Parse with proper error handling
    fn safe_parse(&self) -> Result<Self::Output>;
}

impl SafeParse for str {
    type Output = i64;

    fn safe_parse(&self) -> Result<Self::Output> {
        self.parse()
            .map_err(|e| NeoError::InvalidInput(format!("Failed to parse '{}': {}", self, e)))
    }
}

/// Safe conversion between types
/// Defines a trait interface.
pub trait SafeConvert<T> {
    /// Convert with overflow/underflow protection
    fn safe_into(self) -> Result<T>;
}

impl SafeConvert<u32> for usize {
    fn safe_into(self) -> Result<u32> {
        u32::try_from(self)
            .map_err(|_| NeoError::InvalidInput(format!("Value {} too large for u32", self)))
    }
}

impl SafeConvert<usize> for u32 {
    fn safe_into(self) -> Result<usize> {
        Ok(self as usize)
    }
}

impl SafeConvert<i32> for usize {
    fn safe_into(self) -> Result<i32> {
        i32::try_from(self)
            .map_err(|_| NeoError::InvalidInput(format!("Value {} too large for i32", self)))
    }
}

/// Safe file operations
pub mod file {
    use super::*;
    use std::fs;
    use std::io::Read;
    use std::path::Path;

    /// Safe file reading with size limits
    pub fn safe_read_file<P: AsRef<Path>>(path: P, max_size: usize) -> Result<Vec<u8>> {
        let path = path.as_ref();

        // Check file exists
        if !path.exists() {
            return Err(NeoError::NotFound(format!("File not found: {:?}", path)));
        }

        // Check file size
        let metadata = fs::metadata(path)
            .map_err(|e| NeoError::Internal(format!("Failed to read metadata: {}", e)))?;

        if metadata.len() > max_size as u64 {
            return Err(NeoError::InvalidInput(format!(
                "File too large: {} bytes (max: {} bytes)",
                metadata.len(),
                max_size
            )));
        }

        // Read file
        let mut file = fs::File::open(path)
            .map_err(|e| NeoError::Internal(format!("Failed to open file: {}", e)))?;

        let mut buffer = Vec::with_capacity(metadata.len() as usize);
        file.read_to_end(&mut buffer)
            .map_err(|e| NeoError::Internal(format!("Failed to read file: {}", e)))?;

        Ok(buffer)
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_index() {
        let vec = vec![1, 2, 3];
        assert_eq!(vec.safe_get(1), Some(&2));
        assert_eq!(vec.safe_get(10), None);
        assert_eq!(vec.get_or_default(10), 0);
    }

    #[test]
    fn test_safe_arithmetic() {
        assert!(255u8.safe_add(1).is_err());
        assert_eq!(10u32.safe_add(20).unwrap(), 30);
        assert!(0u32.safe_sub(1).is_err());
        assert!(10u32.safe_div(0).is_err());
    }

    #[test]
    fn test_safe_mutex() {
        let mutex = SafeMutex::new(42);
        let guard = mutex.safe_lock().unwrap();
        assert_eq!(*guard, 42);
    }

    #[test]
    fn test_safe_parse() {
        assert_eq!("42".safe_parse().unwrap(), 42i64);
        assert!("not_a_number".safe_parse().is_err());
    }

    #[test]
    fn test_safe_convert() {
        use SafeConvert;

        let large: usize = 1_000_000_000_000;
        let result: Result<u32> = large.safe_into();
        assert!(result.is_err());

        let small: usize = 42;
        let result: Result<u32> = small.safe_into();
        assert_eq!(result.unwrap(), 42u32);
    }
}
