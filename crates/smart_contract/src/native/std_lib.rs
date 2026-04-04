//! StdLib native contract implementation.
//!
//! The StdLib contract provides standard utility functions for smart contracts,
//! including string manipulation, JSON operations, and mathematical functions.

use crate::application_engine::ApplicationEngine;
use crate::native::{NativeContract, NativeMethod};
use crate::{Error, Result};
use base64::{engine::general_purpose, Engine as _};
use neo_config::SECONDS_PER_BLOCK;
use neo_core::UInt160;
use serde_json::Value as JsonValue;
use std::str::FromStr;

/// The StdLib native contract.
pub struct StdLib {
    hash: UInt160,
    methods: Vec<NativeMethod>,
}

impl StdLib {
    /// Creates a new StdLib contract.
    pub fn new() -> Self {
        // StdLib contract hash: 0xacce6fd80d44e1796aa0c2c625e9e4e0ce39efc0
        let hash = UInt160::from_bytes(&[
            0xac, 0xce, 0x6f, 0xd8, 0x0d, 0x44, 0xe1, 0x79, 0x6a, 0xa0, 0xc2, 0xc6, 0x25, 0xe9,
            0xe4, 0xe0, 0xce, 0x39, 0xef, 0xc0,
        ])
        .expect("Operation failed");

        let methods = vec![
            NativeMethod::safe("atoi".to_string(), 1 << 12),
            NativeMethod::safe("itoa".to_string(), 1 << 12),
            NativeMethod::safe("base64Encode".to_string(), 1 << 12),
            NativeMethod::safe("base64Decode".to_string(), 1 << 12),
            // JSON operations
            NativeMethod::safe("jsonSerialize".to_string(), 1 << 12),
            NativeMethod::safe("jsonDeserialize".to_string(), 1 << 12),
            // Memory operations
            NativeMethod::safe("memoryCompare".to_string(), 1 << 5),
            NativeMethod::safe("memorySearch".to_string(), 1 << SECONDS_PER_BLOCK),
            NativeMethod::safe("stringSplit".to_string(), 1 << 13),
            NativeMethod::safe("stringLen".to_string(), 1 << 4),
        ];

        Self { hash, methods }
    }

    /// Invokes a method on the StdLib contract.
    pub fn invoke_method(
        &self,
        engine: &mut ApplicationEngine,
        method: &str,
        args: &[Vec<u8>],
    ) -> Result<Vec<u8>> {
        match method {
            "atoi" => self.atoi(args),
            "itoa" => self.itoa(args),
            "base64Encode" => self.base64_encode(args),
            "base64Decode" => self.base64_decode(args),
            "jsonSerialize" => self.json_serialize(args),
            "jsonDeserialize" => self.json_deserialize(args),
            "memoryCompare" => self.memory_compare(args),
            "memorySearch" => self.memory_search(args),
            "stringSplit" => self.string_split(args),
            "stringLen" => self.string_len(args),
            _ => Err(Error::NativeContractError(format!(
                "Unknown method: {}",
                method
            ))),
        }
    }

    /// Converts a string to an integer.
    fn atoi(&self, args: &[Vec<u8>]) -> Result<Vec<u8>> {
        if args.is_empty() {
            return Err(Error::NativeContractError(
                "atoi requires string argument".to_string(),
            ));
        }

        let string_data = String::from_utf8(args[0].clone())
            .map_err(|_| Error::NativeContractError("Invalid UTF-8 string".to_string()))?;

        let number = i64::from_str(&string_data)
            .map_err(|_| Error::NativeContractError("Invalid number format".to_string()))?;

        Ok(number.to_le_bytes().to_vec())
    }

    /// Converts an integer to a string.
    fn itoa(&self, args: &[Vec<u8>]) -> Result<Vec<u8>> {
        if args.is_empty() || args[0].len() != 8 {
            return Err(Error::NativeContractError(
                "itoa requires integer argument".to_string(),
            ));
        }

        let number = i64::from_le_bytes([
            args[0][0], args[0][1], args[0][2], args[0][3], args[0][4], args[0][5], args[0][6],
            args[0][7],
        ]);

        Ok(number.to_string().into_bytes())
    }

    /// Encodes data to base64.
    fn base64_encode(&self, args: &[Vec<u8>]) -> Result<Vec<u8>> {
        if args.is_empty() {
            return Err(Error::NativeContractError(
                "base64Encode requires data argument".to_string(),
            ));
        }

        let encoded = general_purpose::STANDARD.encode(&args[0]);
        Ok(encoded.into_bytes())
    }

    /// Decodes data from base64.
    fn base64_decode(&self, args: &[Vec<u8>]) -> Result<Vec<u8>> {
        if args.is_empty() {
            return Err(Error::NativeContractError(
                "base64Decode requires string argument".to_string(),
            ));
        }

        let string_data = String::from_utf8(args[0].clone())
            .map_err(|_| Error::NativeContractError("Invalid UTF-8 string".to_string()))?;

        let decoded = general_purpose::STANDARD
            .decode(&string_data)
            .map_err(|_| Error::NativeContractError("Invalid base64 data".to_string()))?;

        Ok(decoded)
    }

    /// Serializes data to JSON.
    fn json_serialize(&self, args: &[Vec<u8>]) -> Result<Vec<u8>> {
        if args.is_empty() {
            return Err(Error::NativeContractError(
                "jsonSerialize requires data argument".to_string(),
            ));
        }

        let string_data =
            String::from_utf8(args[0].clone()).unwrap_or_else(|_| hex::encode(&args[0]));

        let json_value = JsonValue::String(string_data);
        let json_string = serde_json::to_string(&json_value)
            .map_err(|e| Error::NativeContractError(format!("JSON serialization error: {}", e)))?;

        Ok(json_string.into_bytes())
    }

    /// Deserializes data from JSON.
    fn json_deserialize(&self, args: &[Vec<u8>]) -> Result<Vec<u8>> {
        if args.is_empty() {
            return Err(Error::NativeContractError(
                "jsonDeserialize requires JSON string argument".to_string(),
            ));
        }

        let json_string = String::from_utf8(args[0].clone())
            .map_err(|_| Error::NativeContractError("Invalid UTF-8 string".to_string()))?;

        let json_value: JsonValue = serde_json::from_str(&json_string).map_err(|e| {
            Error::NativeContractError(format!("JSON deserialization error: {}", e))
        })?;

        // Convert JSON value back to bytes
        match json_value {
            JsonValue::String(s) => Ok(s.into_bytes()),
            JsonValue::Number(n) => {
                if let Some(i) = n.as_i64() {
                    Ok(i.to_le_bytes().to_vec())
                } else {
                    Ok(n.to_string().into_bytes())
                }
            }
            JsonValue::Bool(b) => Ok(vec![if b { 1 } else { 0 }]),
            JsonValue::Null => Ok(vec![]),
            _ => Ok(json_value.to_string().into_bytes()),
        }
    }

    /// Compares two memory regions.
    fn memory_compare(&self, args: &[Vec<u8>]) -> Result<Vec<u8>> {
        if args.len() < 2 {
            return Err(Error::NativeContractError(
                "memoryCompare requires two data arguments".to_string(),
            ));
        }

        let result = match args[0].cmp(&args[1]) {
            std::cmp::Ordering::Less => -1i32,
            std::cmp::Ordering::Equal => 0i32,
            std::cmp::Ordering::Greater => 1i32,
        };

        Ok(result.to_le_bytes().to_vec())
    }

    /// Searches for a pattern in memory.
    fn memory_search(&self, args: &[Vec<u8>]) -> Result<Vec<u8>> {
        if args.len() < 2 {
            return Err(Error::NativeContractError(
                "memorySearch requires data and pattern arguments".to_string(),
            ));
        }

        let data = &args[0];
        let pattern = &args[1];

        if pattern.is_empty() {
            return Ok(0i32.to_le_bytes().to_vec());
        }

        // Find the first occurrence of pattern in data
        for i in 0..=data.len().saturating_sub(pattern.len()) {
            if data[i..i + pattern.len()] == *pattern {
                return Ok((i as i32).to_le_bytes().to_vec());
            }
        }

        // Not found
        Ok((-1i32).to_le_bytes().to_vec())
    }

    /// Splits a string by a delimiter.
    fn string_split(&self, args: &[Vec<u8>]) -> Result<Vec<u8>> {
        if args.len() < 2 {
            return Err(Error::NativeContractError(
                "stringSplit requires string and delimiter arguments".to_string(),
            ));
        }

        let string_data = String::from_utf8(args[0].clone())
            .map_err(|_| Error::NativeContractError("Invalid UTF-8 string".to_string()))?;

        let delimiter = String::from_utf8(args[1].clone())
            .map_err(|_| Error::NativeContractError("Invalid UTF-8 delimiter".to_string()))?;

        let parts: Vec<&str> = string_data.split(&delimiter).collect();

        // Serialize as a simple array format: [count][length1][data1][length2][data2]/* implementation */;
        let mut result = Vec::new();
        result.extend_from_slice(&(parts.len() as u32).to_le_bytes());

        for part in parts {
            let part_bytes = part.as_bytes();
            result.extend_from_slice(&(part_bytes.len() as u32).to_le_bytes());
            result.extend_from_slice(part_bytes);
        }

        Ok(result)
    }

    /// Gets the length of a string.
    fn string_len(&self, args: &[Vec<u8>]) -> Result<Vec<u8>> {
        if args.is_empty() {
            return Err(Error::NativeContractError(
                "stringLen requires string argument".to_string(),
            ));
        }

        let string_data = String::from_utf8(args[0].clone())
            .map_err(|_| Error::NativeContractError("Invalid UTF-8 string".to_string()))?;

        let length = string_data.chars().count() as u32;
        Ok(length.to_le_bytes().to_vec())
    }
}

impl NativeContract for StdLib {
    fn hash(&self) -> UInt160 {
        self.hash
    }

    fn name(&self) -> &str {
        "StdLib"
    }

    fn methods(&self) -> &[NativeMethod] {
        &self.methods
    }

    fn invoke(
        &self,
        engine: &mut ApplicationEngine,
        method: &str,
        args: &[Vec<u8>],
    ) -> Result<Vec<u8>> {
        self.invoke_method(engine, method, args)
    }
}

impl Default for StdLib {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    #[test]
    fn test_std_lib_creation() {
        let std_lib = StdLib::new();
        assert_eq!(std_lib.name(), "StdLib");
        assert!(!std_lib.methods().is_empty());
    }

    #[test]
    fn test_atoi() {
        let std_lib = StdLib::new();
        let args = vec![b"12345".to_vec()];
        let result = std_lib.atoi(&args).unwrap();
        let number = i64::from_le_bytes([
            result[0], result[1], result[2], result[3], result[4], result[5], result[6], result[7],
        ]);
        assert_eq!(number, 12345);
    }

    #[test]
    fn test_itoa() {
        let std_lib = StdLib::new();
        let args = vec![12345i64.to_le_bytes().to_vec()];
        let result = std_lib.itoa(&args).unwrap();
        let string_result =
            String::from_utf8(result).map_err(|e| anyhow::anyhow!("Invalid UTF-8: {}", e))?;
        assert_eq!(string_result, "12345");
    }

    #[test]
    fn test_base64_encode_decode() {
        let std_lib = StdLib::new();
        let original_data = b"Hello, World!".to_vec();

        // Encode
        let encoded_result = std_lib.base64_encode(&[original_data.clone()]).unwrap();
        let encoded_string = String::from_utf8(encoded_result)
            .map_err(|e| anyhow::anyhow!("Invalid UTF-8: {}", e))?;

        // Decode
        let decoded_result = std_lib
            .base64_decode(&[encoded_string.into_bytes()])
            .unwrap();
        assert_eq!(decoded_result, original_data);
    }

    #[test]
    fn test_memory_compare() {
        let std_lib = StdLib::new();
        let data1 = b"abc".to_vec();
        let data2 = b"abc".to_vec();
        let data3 = b"def".to_vec();

        // Equal
        let result = std_lib.memory_compare(&[data1.clone(), data2]).unwrap();
        let comparison = i32::from_le_bytes([result[0], result[1], result[2], result[3]]);
        assert_eq!(comparison, 0);

        // Not equal
        let result = std_lib.memory_compare(&[data1, data3]).unwrap();
        let comparison = i32::from_le_bytes([result[0], result[1], result[2], result[3]]);
        assert_ne!(comparison, 0);
    }

    #[test]
    fn test_memory_search() {
        let std_lib = StdLib::new();
        let data = b"Hello, World!".to_vec();
        let pattern = b"World".to_vec();

        let result = std_lib.memory_search(&[data, pattern]).unwrap();
        let index = i32::from_le_bytes([result[0], result[1], result[2], result[3]]);
        assert_eq!(index, 7); // "World" starts at index 7
    }

    #[test]
    fn test_string_len() {
        let std_lib = StdLib::new();
        let args = vec![b"Hello".to_vec()];
        let result = std_lib.string_len(&args).unwrap();
        let length = u32::from_le_bytes([result[0], result[1], result[2], result[3]]);
        assert_eq!(length, 5);
    }
}
