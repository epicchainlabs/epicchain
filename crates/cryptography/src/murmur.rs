//! Murmur hash implementation for Neo.
//!
//! This module provides Murmur32 and Murmur128 hash functions.

// No config imports needed - using literal values

/// Computes the Murmur32 hash of the given data.
///
/// # Arguments
///
/// * `data` - The data to hash
/// * `seed` - The seed for the hash function
///
/// # Returns
///
/// The Murmur32 hash of the data
pub fn murmur32(data: &[u8], seed: u32) -> u32 {
    const C1: u32 = 0xcc9e2d51;
    const C2: u32 = 0x1b873593;
    const R1: u32 = 15;
    const R2: u32 = 13;
    const M: u32 = 5;
    const N: u32 = 0xe6546b64;

    let mut hash = seed;
    let len = data.len();
    let nblocks = len / 4;

    // Process 4-byte blocks
    for i in 0..nblocks {
        let mut k = u32::from_le_bytes([
            data[i * 4],
            data[i * 4 + 1],
            data[i * 4 + 2],
            data[i * 4 + 3],
        ]);

        k = k.wrapping_mul(C1);
        k = k.rotate_left(R1);
        k = k.wrapping_mul(C2);

        hash ^= k;
        hash = hash.rotate_left(R2);
        hash = hash.wrapping_mul(M).wrapping_add(N);
    }

    // Process remaining bytes
    let mut k = 0u32;
    let offset = nblocks * 4;
    match len & 3 {
        3 => {
            k ^= (data[offset + 2] as u32) << 16;
            k ^= (data[offset + 1] as u32) << 8;
            k ^= data[offset] as u32;
            k = k.wrapping_mul(C1);
            k = k.rotate_left(R1);
            k = k.wrapping_mul(C2);
            hash ^= k;
        }
        2 => {
            k ^= (data[offset + 1] as u32) << 8;
            k ^= data[offset] as u32;
            k = k.wrapping_mul(C1);
            k = k.rotate_left(R1);
            k = k.wrapping_mul(C2);
            hash ^= k;
        }
        1 => {
            k ^= data[offset] as u32;
            k = k.wrapping_mul(C1);
            k = k.rotate_left(R1);
            k = k.wrapping_mul(C2);
            hash ^= k;
        }
        _ => {}
    }

    // Finalization
    hash ^= len as u32;
    hash ^= hash >> 16;
    hash = hash.wrapping_mul(0x85ebca6b);
    hash ^= hash >> 13;
    hash = hash.wrapping_mul(0xc2b2ae35);
    hash ^= hash >> 16;

    hash
}

/// Computes the Murmur128 hash of the given data.
///
/// # Arguments
///
/// * `data` - The data to hash
/// * `seed` - The seed for the hash function
///
/// # Returns
///
/// The Murmur128 hash of the data as two 64-bit values
pub fn murmur128(data: &[u8], seed: u32) -> (u64, u64) {
    const C1: u64 = 0x87c37b91114253d5;
    const C2: u64 = 0x4cf5ad432745937f;
    const R1: u32 = 31;
    const R2: u32 = 33;
    const M: u64 = 5;
    const N: u64 = 0x52dce729;
    const K: u64 = 0x38495ab5;

    let mut h1 = seed as u64;
    let mut h2 = seed as u64;
    let len = data.len();
    let nblocks = len / 16;

    // Process 16-byte blocks
    for i in 0..nblocks {
        let mut k1 = u64::from_le_bytes([
            data[i * 16],
            data[i * 16 + 1],
            data[i * 16 + 2],
            data[i * 16 + 3],
            data[i * 16 + 4],
            data[i * 16 + 5],
            data[i * 16 + 6],
            data[i * 16 + 7],
        ]);
        let mut k2 = u64::from_le_bytes([
            data[i * 16 + 8],
            data[i * 16 + 9],
            data[i * 16 + 10],
            data[i * 16 + 11],
            data[i * 16 + 12],
            data[i * 16 + 13],
            data[i * 16 + 14],
            data[i * 16 + 15],
        ]);

        k1 = k1.wrapping_mul(C1);
        k1 = k1.rotate_left(R1);
        k1 = k1.wrapping_mul(C2);
        h1 ^= k1;

        h1 = h1.rotate_left(R2);
        h1 = h1.wrapping_mul(M).wrapping_add(N);

        k2 = k2.wrapping_mul(C2);
        k2 = k2.rotate_left(R2);
        k2 = k2.wrapping_mul(C1);
        h2 ^= k2;

        h2 = h2.rotate_left(R1);
        h2 = h2.wrapping_mul(M).wrapping_add(K);
    }

    // Process remaining bytes
    let mut k1 = 0u64;
    let mut k2 = 0u64;
    let offset = nblocks * 16;
    let remaining = len & 15;

    if remaining >= 15 {
        k2 ^= (data[offset + 14] as u64) << 48;
    }
    if remaining >= 14 {
        k2 ^= (data[offset + 13] as u64) << 40;
    }
    if remaining >= 13 {
        k2 ^= (data[offset + 12] as u64) << 32;
    }
    if remaining >= 12 {
        k2 ^= (data[offset + 11] as u64) << 24;
    }
    if remaining >= 11 {
        k2 ^= (data[offset + 10] as u64) << 16;
    }
    if remaining >= 10 {
        k2 ^= (data[offset + 9] as u64) << 8;
    }
    if remaining >= 9 {
        k2 ^= data[offset + 8] as u64;
        k2 = k2.wrapping_mul(C2);
        k2 = k2.rotate_left(R2);
        k2 = k2.wrapping_mul(C1);
        h2 ^= k2;
    }

    if remaining >= 8 {
        k1 ^= (data[offset + 7] as u64) << 56;
    }
    if remaining >= 7 {
        k1 ^= (data[offset + 6] as u64) << 48;
    }
    if remaining >= 6 {
        k1 ^= (data[offset + 5] as u64) << 40;
    }
    if remaining >= 5 {
        k1 ^= (data[offset + 4] as u64) << 32;
    }
    if remaining >= 4 {
        k1 ^= (data[offset + 3] as u64) << 24;
    }
    if remaining >= 3 {
        k1 ^= (data[offset + 2] as u64) << 16;
    }
    if remaining >= 2 {
        k1 ^= (data[offset + 1] as u64) << 8;
    }
    if remaining >= 1 {
        k1 ^= data[offset] as u64;
        k1 = k1.wrapping_mul(C1);
        k1 = k1.rotate_left(R1);
        k1 = k1.wrapping_mul(C2);
        h1 ^= k1;
    }

    // Finalization
    h1 ^= len as u64;
    h2 ^= len as u64;

    h1 = h1.wrapping_add(h2);
    h2 = h2.wrapping_add(h1);

    h1 ^= h1 >> 33;
    h1 = h1.wrapping_mul(0xff51afd7ed558ccd);
    h1 ^= h1 >> 33;
    h1 = h1.wrapping_mul(0xc4ceb9fe1a85ec53);
    h1 ^= h1 >> 33;

    h2 ^= h2 >> 33;
    h2 = h2.wrapping_mul(0xff51afd7ed558ccd);
    h2 ^= h2 >> 33;
    h2 = h2.wrapping_mul(0xc4ceb9fe1a85ec53);
    h2 ^= h2 >> 33;

    h1 = h1.wrapping_add(h2);
    h2 = h2.wrapping_add(h1);

    (h1, h2)
}
