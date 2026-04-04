//! LZ4 compression support for Neo network messages
//!
//! Matches C# Neo compression behavior exactly

use crate::{NetworkError, NetworkResult as Result};

/// Minimum size for compression (matches C# CompressionMinSize)
pub const COMPRESSION_MIN_SIZE: usize = 128;

/// Minimum compression benefit threshold (matches C# CompressionThreshold)  
pub const COMPRESSION_THRESHOLD: usize = 64;

/// Compress payload using LZ4 (matches C# CompressLz4)
pub fn compress_lz4(data: &[u8]) -> Result<Vec<u8>> {
    if data.len() < COMPRESSION_MIN_SIZE {
        return Ok(data.to_vec()); // No compression for small payloads
    }

    // Use lz4_flex crate for LZ4 compression (matches C# implementation)
    #[cfg(feature = "compression")]
    {
        use lz4_flex::compress_prepend_size;
        let compressed = compress_prepend_size(data);

        // Only use compression if it saves at least COMPRESSION_THRESHOLD bytes
        if data.len() > compressed.len() + COMPRESSION_THRESHOLD {
            Ok(compressed)
        } else {
            Ok(data.to_vec()) // Compression not beneficial
        }
    }

    #[cfg(not(feature = "compression"))]
    {
        Ok(data.to_vec()) // No compression available
    }
}

/// Decompress LZ4 payload (matches C# DecompressLz4)
pub fn decompress_lz4(data: &[u8], max_size: usize) -> Result<Vec<u8>> {
    #[cfg(feature = "compression")]
    {
        use lz4_flex::decompress_size_prepended;

        match decompress_size_prepended(data) {
            Ok(decompressed) => {
                if decompressed.len() > max_size {
                    return Err(NetworkError::InvalidMessage {
                        peer: std::net::SocketAddr::from(([0, 0, 0, 0], 0)),
                        message_type: "compressed_data".to_string(),
                        reason: format!(
                            "Decompressed size {} exceeds limit {}",
                            decompressed.len(),
                            max_size
                        ),
                    });
                }
                Ok(decompressed)
            }
            Err(e) => Err(NetworkError::InvalidMessage {
                peer: std::net::SocketAddr::from(([0, 0, 0, 0], 0)),
                message_type: "compressed_data".to_string(),
                reason: format!("LZ4 decompression failed: {}", e),
            }),
        }
    }

    #[cfg(not(feature = "compression"))]
    {
        if data.len() > max_size {
            return Err(NetworkError::InvalidMessage {
                peer: std::net::SocketAddr::from(([0, 0, 0, 0], 0)),
                message: format!("Payload size {} exceeds limit {}", data.len(), max_size),
            });
        }
        Ok(data.to_vec()) // Return as-is if no compression support
    }
}

/// Check if payload should be compressed (matches C# compression logic)
pub fn should_compress(payload_size: usize) -> bool {
    payload_size >= COMPRESSION_MIN_SIZE
}

/// Calculate compression benefit (matches C# compression decision logic)
pub fn compression_beneficial(original_size: usize, compressed_size: usize) -> bool {
    original_size > compressed_size + COMPRESSION_THRESHOLD
}
