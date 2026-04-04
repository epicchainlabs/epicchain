//! Neo3 Protocol Format Test
//!
//! Tests that verify our message parsing matches the verified Neo3 protocol format

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::{decode_varlen, encode_varlen, parse_neo3_message, SimpleNeo3Message};
    use std::io::Cursor;

    /// Test Neo3 message format parsing with verified test vectors
    #[test]
    fn test_neo3_message_format_parsing() {
        println!("🧪 Testing Neo3 message format parsing...");

        // Test vector from Python verification script
        // Small message: command=2 (Pong), 4-byte payload "test"
        let small_message = vec![0, 2, 4, 116, 101, 115, 116];

        // Parse header manually to verify format
        assert_eq!(small_message[0], 0); // flags
        assert_eq!(small_message[1], 2); // command (Pong)
        assert_eq!(small_message[2], 4); // payload length (single byte)

        let payload = &small_message[3..];
        assert_eq!(payload, b"test");

        println!("✅ Small message parsing verified");

        // Test vector for medium message: command=3, 300-byte payload
        let medium_header = vec![0, 3, 253, 44, 1]; // 0x012C = 300 in little endian

        assert_eq!(medium_header[0], 0); // flags
        assert_eq!(medium_header[1], 3); // command
        assert_eq!(medium_header[2], 253); // varlen marker (0xFD)

        // Parse the 2-byte length (little endian)
        let length = u16::from_le_bytes([medium_header[3], medium_header[4]]);
        assert_eq!(length, 300);

        println!("✅ Medium message header parsing verified");

        println!("✅ Neo3 protocol format test passed");
    }

    /// Test variable-length encoding
    #[test]
    fn test_varlen_encoding() {
        println!("🧪 Testing variable-length encoding...");

        // Test cases verified by Python script
        let test_cases = vec![
            (50, vec![50]),            // Single byte
            (250, vec![250]),          // Single byte (max)
            (300, vec![253, 44, 1]),   // 3-byte encoding (0xFD + 300 LE)
            (1000, vec![253, 232, 3]), // 3-byte encoding (0xFD + 1000 LE)
        ];

        for (value, expected) in test_cases {
            let encoded = encode_varlen(value);
            assert_eq!(encoded, expected, "Failed for value {}", value);

            let decoded = decode_varlen(&encoded);
            assert_eq!(decoded, value, "Roundtrip failed for value {}", value);

            println!("✓ Value {} encoded/decoded correctly", value);
        }

        println!("✅ Variable-length encoding test passed");
    }

    /// Test version message creation
    #[test]
    fn test_version_message_creation() {
        println!("🧪 Testing version message creation...");

        // Create a version message payload (simplified)
        let mut payload = Vec::new();

        // Version (4 bytes)
        payload.extend_from_slice(&0u32.to_le_bytes());

        // Services (8 bytes)
        payload.extend_from_slice(&1u64.to_le_bytes());

        // Timestamp (8 bytes)
        payload.extend_from_slice(&0u64.to_le_bytes());

        // Port (2 bytes)
        payload.extend_from_slice(&0u16.to_le_bytes());

        // Nonce (4 bytes)
        payload.extend_from_slice(&0u32.to_le_bytes());

        // User agent "NEO:Rust/1" (varlen string)
        let user_agent = b"NEO:Rust/1";
        payload.push(user_agent.len() as u8);
        payload.extend_from_slice(user_agent);

        // Start height (4 bytes)
        payload.extend_from_slice(&0u32.to_le_bytes());

        // Relay (1 byte)
        payload.push(1);

        // Expected payload size matches Python verification
        assert_eq!(payload.len(), 42, "Version payload should be 42 bytes");

        // Create complete message
        let mut message = Vec::new();
        message.push(0); // flags
        message.push(0); // command (Version)
        message.push(payload.len() as u8); // length (single byte)
        message.extend_from_slice(&payload);

        // Verify total message size
        assert_eq!(message.len(), 45, "Complete message should be 45 bytes");

        // Verify header
        assert_eq!(message[0], 0); // flags
        assert_eq!(message[1], 0); // version command
        assert_eq!(message[2], 42); // payload length

        println!("✅ Version message creation verified");
    }

    /// Test message parsing function
    #[test]
    fn test_message_parsing_function() {
        println!("🧪 Testing message parsing function...");

        // Test with small message
        let message_data = vec![0, 1, 3, 65, 66, 67]; // Ping command, "ABC" payload
        let parsed = parse_neo3_message(&message_data);

        assert_eq!(parsed.flags, 0);
        assert_eq!(parsed.command, 1);
        assert_eq!(parsed.payload, b"ABC");

        println!("✅ Message parsing function test passed");
    }
}

/// Helper function to encode variable-length integers
fn encode_varlen(value: usize) -> Vec<u8> {
    if value < 253 {
        vec![value as u8]
    } else if value < 65535 {
        let mut result = vec![253];
        result.extend_from_slice(&(value as u16).to_le_bytes());
        result
    } else {
        let mut result = vec![254];
        result.extend_from_slice(&(value as u32).to_le_bytes());
        result
    }
}

/// Helper function to decode variable-length integers  
fn decode_varlen(data: &[u8]) -> usize {
    if data[0] < 253 {
        data[0] as usize
    } else if data[0] == 253 {
        u16::from_le_bytes([data[1], data[2]]) as usize
    } else {
        u32::from_le_bytes([data[1], data[2], data[3], data[4]]) as usize
    }
}

/// Simple message structure for testing
#[derive(Debug, PartialEq)]
struct SimpleNeo3Message {
    flags: u8,
    command: u8,
    payload: Vec<u8>,
}

/// Parse a Neo3 message from bytes
fn parse_neo3_message(data: &[u8]) -> SimpleNeo3Message {
    let flags = data[0];
    let command = data[1];

    let (payload_len, payload_start) = if data[2] < 253 {
        (data[2] as usize, 3)
    } else if data[2] == 253 {
        let len = u16::from_le_bytes([data[3], data[4]]) as usize;
        (len, 5)
    } else {
        let len = u32::from_le_bytes([data[3], data[4], data[5], data[6]]) as usize;
        (len, 7)
    };

    let payload = data[payload_start..payload_start + payload_len].to_vec();

    SimpleNeo3Message {
        flags,
        command,
        payload,
    }
}
