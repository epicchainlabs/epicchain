//! Header cache implementation.
//!
//! This module provides header caching functionality that exactly matches C# Neo HeaderCache.

use crate::Header;
use parking_lot::RwLock;
use std::collections::VecDeque;

/// Maximum number of headers that can be cached (matches C# Neo HeaderCache.MaxHeaders)
pub const MAX_HEADERS: usize = 10_000;

/// Used to cache the headers of the blocks that have not been received (matches C# Neo HeaderCache exactly)
pub struct HeaderCache {
    /// Internal queue for storing headers (matches C# IndexedQueue<Header>)
    headers: RwLock<VecDeque<Header>>,
}

impl HeaderCache {
    /// Creates a new header cache (matches C# Neo HeaderCache constructor)
    pub fn new() -> Self {
        Self {
            headers: RwLock::new(VecDeque::new()),
        }
    }

    /// Gets the number of headers in the cache (matches C# Neo HeaderCache.Count property)
    pub fn count(&self) -> usize {
        let headers = self.headers.read();
        headers.len()
    }

    /// Checks if the cache is full (matches C# Neo HeaderCache.Full property)
    pub fn full(&self) -> bool {
        self.count() >= MAX_HEADERS
    }

    /// Gets the last header in the cache (matches C# Neo HeaderCache.Last property)
    pub fn last(&self) -> Option<Header> {
        let headers = self.headers.read();
        headers.back().cloned()
    }

    /// Adds a header to the cache (matches C# Neo HeaderCache.Add method)
    /// Returns true if the header was added, false if the cache is full
    pub fn add(&self, header: Header) -> bool {
        let mut headers = self.headers.write();

        // Enforce the cache limit when Full
        if headers.len() >= MAX_HEADERS {
            return false;
        }

        headers.push_back(header);
        true
    }

    /// Tries to remove the first header from the cache (matches C# Neo HeaderCache.TryRemoveFirst method)
    /// Returns Some(header) if successful, None if the cache is empty
    pub fn try_remove_first(&self) -> Option<Header> {
        let mut headers = self.headers.write();
        headers.pop_front()
    }

    /// Gets an iterator over all headers in the cache (matches C# Neo HeaderCache.GetEnumerator)
    pub fn iter(&self) -> Vec<Header> {
        let headers = self.headers.read();
        headers.iter().cloned().collect()
    }

    /// Clears all headers from the cache
    pub fn clear(&self) {
        let mut headers = self.headers.write();
        headers.clear();
    }

    /// Checks if the cache is empty
    pub fn is_empty(&self) -> bool {
        let headers = self.headers.read();
        headers.is_empty()
    }
}

impl Default for HeaderCache {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for HeaderCache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HeaderCache")
            .field("count", &self.count())
            .field("full", &self.full())
            .finish()
    }
}

impl IntoIterator for HeaderCache {
    type Item = Header;
    type IntoIter = std::vec::IntoIter<Header>;

    fn into_iter(self) -> Self::IntoIter {
        let headers = self.headers.into_inner();
        headers.into_iter().collect::<Vec<_>>().into_iter()
    }
}

impl<'a> IntoIterator for &'a HeaderCache {
    type Item = Header;
    type IntoIter = std::vec::IntoIter<Header>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter().into_iter()
    }
}
