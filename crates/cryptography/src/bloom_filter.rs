//! Bloom filter implementation for Neo.
//!
//! This module provides a Bloom filter implementation for efficient set membership testing.

use crate::murmur;
use std::fmt;

/// A Bloom filter for efficient set membership testing.
#[derive(Clone)]
pub struct BloomFilter {
    /// The bit array for the filter
    bits: Vec<u8>,

    /// The number of hash functions
    k: u8,

    /// The number of elements added to the filter
    count: usize,

    /// The tweak value for the hash functions
    tweak: u32,
}

impl BloomFilter {
    /// Creates a new Bloom filter with the specified parameters.
    ///
    /// # Arguments
    ///
    /// * `m` - The size of the filter in bits
    /// * `k` - The number of hash functions
    /// * `tweak` - The tweak value for the hash functions
    ///
    /// # Returns
    ///
    /// A new Bloom filter
    pub fn new(m: usize, k: u8, tweak: u32) -> Self {
        let bits_size = m.div_ceil(8); // Round up to the nearest byte
        Self {
            bits: vec![0; bits_size],
            k,
            count: 0,
            tweak,
        }
    }

    /// Creates a new Bloom filter with the specified parameters and elements.
    ///
    /// # Arguments
    ///
    /// * `m` - The size of the filter in bits
    /// * `k` - The number of hash functions
    /// * `tweak` - The tweak value for the hash functions
    /// * `elements` - The elements to add to the filter
    ///
    /// # Returns
    ///
    /// A new Bloom filter with the specified elements
    pub fn new_with_elements(m: usize, k: u8, tweak: u32, elements: &[&[u8]]) -> Self {
        let mut filter = Self::new(m, k, tweak);
        for element in elements {
            filter.add(element);
        }
        filter
    }

    /// Creates a new Bloom filter with optimal parameters for the expected number of elements
    /// and desired false positive rate.
    ///
    /// # Arguments
    ///
    /// * `n` - The expected number of elements
    /// * `p` - The desired false positive rate (between 0 and 1)
    /// * `tweak` - The tweak value for the hash functions
    ///
    /// # Returns
    ///
    /// A new Bloom filter with optimal parameters
    pub fn new_optimal(n: usize, p: f64, tweak: u32) -> Self {
        let m = Self::optimal_size(n, p);
        let k = Self::optimal_k(m, n);
        Self::new(m, k, tweak)
    }

    /// Calculates the optimal size of the filter in bits for the expected number of elements
    /// and desired false positive rate.
    ///
    /// # Arguments
    ///
    /// * `n` - The expected number of elements
    /// * `p` - The desired false positive rate (between 0 and 1)
    ///
    /// # Returns
    ///
    /// The optimal size of the filter in bits
    pub fn optimal_size(n: usize, p: f64) -> usize {
        let m = -((n as f64) * p.ln() / (2.0f64.ln().powi(2))).ceil() as usize;
        m.max(1)
    }

    /// Calculates the optimal number of hash functions for the given filter size and
    /// expected number of elements.
    ///
    /// # Arguments
    ///
    /// * `m` - The size of the filter in bits
    /// * `n` - The expected number of elements
    ///
    /// # Returns
    ///
    /// The optimal number of hash functions
    pub fn optimal_k(m: usize, n: usize) -> u8 {
        let k = ((m as f64) / (n as f64) * 2.0f64.ln()).ceil() as u8;
        k.max(1)
    }

    /// Returns the size of the filter in bits.
    pub fn size(&self) -> usize {
        self.bits.len() * 8
    }

    /// Returns the number of hash functions used by the filter.
    pub fn k(&self) -> u8 {
        self.k
    }

    /// Returns the number of elements added to the filter.
    pub fn count(&self) -> usize {
        self.count
    }

    /// Returns the tweak value used by the filter.
    pub fn tweak(&self) -> u32 {
        self.tweak
    }

    /// Adds an element to the filter.
    ///
    /// # Arguments
    ///
    /// * `element` - The element to add
    pub fn add(&mut self, element: &[u8]) {
        for i in 0..self.k {
            let index = self.hash(element, i) % (self.size() as u32);
            self.set_bit(index as usize);
        }
        self.count += 1;
    }

    /// Checks if an element might be in the filter.
    ///
    /// # Arguments
    ///
    /// * `element` - The element to check
    ///
    /// # Returns
    ///
    /// `true` if the element might be in the filter, `false` if it is definitely not
    pub fn contains(&self, element: &[u8]) -> bool {
        for i in 0..self.k {
            let index = self.hash(element, i) % (self.size() as u32);
            if !self.get_bit(index as usize) {
                return false;
            }
        }
        true
    }

    /// Clears the filter.
    pub fn clear(&mut self) {
        self.bits.fill(0);
        self.count = 0;
    }

    /// Returns the estimated false positive rate of the filter.
    pub fn false_positive_rate(&self) -> f64 {
        let m = self.size() as f64;
        let k = self.k as f64;
        let n = self.count as f64;

        (1.0 - (1.0 - 1.0 / m).powf(k * n)).powf(k)
    }

    /// Computes a hash for the given element and index.
    ///
    /// # Arguments
    ///
    /// * `element` - The element to hash
    /// * `index` - The index of the hash function
    ///
    /// # Returns
    ///
    /// The hash value
    fn hash(&self, element: &[u8], index: u8) -> u32 {
        let seed = self
            .tweak
            .wrapping_add(index as u32)
            .wrapping_mul(0xFBA4C795);
        murmur::murmur32(element, seed)
    }

    /// Sets the bit at the specified index.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the bit to set
    fn set_bit(&mut self, index: usize) {
        let byte_index = index / 8;
        let bit_index = index % 8;
        self.bits[byte_index] |= 1 << bit_index;
    }

    /// Gets the bit at the specified index.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the bit to get
    ///
    /// # Returns
    ///
    /// `true` if the bit is set, `false` otherwise
    fn get_bit(&self, index: usize) -> bool {
        let byte_index = index / 8;
        let bit_index = index % 8;
        (self.bits[byte_index] & (1 << bit_index)) != 0
    }

    /// Returns the raw bit array of the filter.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.bits.clone()
    }

    /// Creates a filter from a raw bit array.
    ///
    /// # Arguments
    ///
    /// * `bits` - The raw bit array
    /// * `k` - The number of hash functions
    /// * `tweak` - The tweak value for the hash functions
    /// * `count` - The number of elements added to the filter
    ///
    /// # Returns
    ///
    /// A new Bloom filter with the specified parameters
    pub fn from_bytes(bits: Vec<u8>, k: u8, tweak: u32, count: usize) -> Self {
        Self {
            bits,
            k,
            tweak,
            count,
        }
    }
}

impl fmt::Debug for BloomFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BloomFilter")
            .field("size", &self.size())
            .field("k", &self.k)
            .field("count", &self.count)
            .field("tweak", &self.tweak)
            .field("false_positive_rate", &self.false_positive_rate())
            .finish()
    }
}
