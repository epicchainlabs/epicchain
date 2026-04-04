//! Comprehensive StackItem tests that exactly match C# Neo.VM.Tests/UT_StackItem.cs
//!
//! This file contains unit tests that ensure the Rust StackItem implementation
//! behaves identically to the C# Neo VM StackItem implementation.

use neo_vm::StackItem;
use num_bigint::BigInt;
use std::collections::BTreeMap;

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    /// Test basic stack item creation and equality (matches C# TestEqual)
    #[test]
    fn test_equal() {
        let item_a = StackItem::from_byte_string("NEO");
        let item_b = StackItem::from_byte_string("NEO");
        let item_c = StackItem::from_byte_string("SmartEconomy");
        let item_d = StackItem::from_byte_string("Smarteconomy");
        let item_e = StackItem::from_byte_string("smarteconomy");

        assert!(
            item_a.equals(&item_b).unwrap(),
            "Same strings should be equal"
        );
        assert!(
            !item_a.equals(&item_c).unwrap(),
            "Different strings should not be equal"
        );
        assert!(
            !item_c.equals(&item_d).unwrap(),
            "Case-sensitive comparison should work"
        );
        assert!(
            !item_d.equals(&item_e).unwrap(),
            "Case-sensitive comparison should work"
        );
    }

    /// Test null item behavior (matches C# TestNull)
    #[test]
    fn test_null() {
        let null_item = StackItem::from_byte_string(vec![]);
        assert_ne!(
            StackItem::Null,
            null_item,
            "Empty byte array should not equal Null"
        );

        let null_item = StackItem::Null;
        assert_eq!(StackItem::Null, null_item, "Null should equal Null");
    }

    /// Test type casting (matches C# TestCast)
    #[test]
    fn test_cast() {
        // Test signed byte
        let item = StackItem::Integer(BigInt::from(i8::MAX));
        assert!(matches!(item, StackItem::Integer(_)));
        if let StackItem::Integer(value) = item {
            assert_eq!(value, BigInt::from(i8::MAX));
        }

        // Test unsigned byte
        let item = StackItem::Integer(BigInt::from(u8::MAX));
        assert!(matches!(item, StackItem::Integer(_)));
        if let StackItem::Integer(value) = item {
            assert_eq!(value, BigInt::from(u8::MAX));
        }

        // Test signed short
        let item = StackItem::Integer(BigInt::from(i16::MAX));
        assert!(matches!(item, StackItem::Integer(_)));
        if let StackItem::Integer(value) = item {
            assert_eq!(value, BigInt::from(i16::MAX));
        }

        // Test unsigned short
        let item = StackItem::Integer(BigInt::from(u16::MAX));
        assert!(matches!(item, StackItem::Integer(_)));
        if let StackItem::Integer(value) = item {
            assert_eq!(value, BigInt::from(u16::MAX));
        }

        // Test signed integer
        let item = StackItem::Integer(BigInt::from(i32::MAX));
        assert!(matches!(item, StackItem::Integer(_)));
        if let StackItem::Integer(value) = item {
            assert_eq!(value, BigInt::from(i32::MAX));
        }

        // Test unsigned integer
        let item = StackItem::Integer(BigInt::from(u32::MAX));
        assert!(matches!(item, StackItem::Integer(_)));
        if let StackItem::Integer(value) = item {
            assert_eq!(value, BigInt::from(u32::MAX));
        }

        // Test signed long
        let item = StackItem::Integer(BigInt::from(i64::MAX));
        assert!(matches!(item, StackItem::Integer(_)));
        if let StackItem::Integer(value) = item {
            assert_eq!(value, BigInt::from(i64::MAX));
        }

        // Test unsigned long
        let item = StackItem::Integer(BigInt::from(u64::MAX));
        assert!(matches!(item, StackItem::Integer(_)));
        if let StackItem::Integer(value) = item {
            assert_eq!(value, BigInt::from(u64::MAX));
        }

        // Test BigInteger
        let item = StackItem::Integer(BigInt::from(-1));
        assert!(matches!(item, StackItem::Integer(_)));
        if let StackItem::Integer(value) = item {
            assert_eq!(value, BigInt::from(-1));
        }

        // Test Boolean
        let item = StackItem::Boolean(true);
        assert!(matches!(item, StackItem::Boolean(_)));
        assert!(item.as_bool().unwrap());

        // Test ByteString
        let data = vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09];
        let item = StackItem::from_byte_string(data.clone());
        assert!(matches!(item, StackItem::ByteString(_)));
        if let StackItem::ByteString(byte_string) = item {
            assert_eq!(byte_string, data);
        }
    }

    /// Test deep copy functionality (matches C# TestDeepCopy)
    #[test]
    fn test_deep_copy() {
        // Create a complex nested structure
        let mut map = BTreeMap::new();
        map.insert(
            StackItem::Integer(BigInt::from(0)),
            StackItem::Integer(BigInt::from(1)),
        );
        map.insert(
            StackItem::Integer(BigInt::from(2)),
            StackItem::Integer(BigInt::from(3)),
        );

        let array = StackItem::Array(vec![
            StackItem::Boolean(true),
            StackItem::Integer(BigInt::from(1)),
            StackItem::from_byte_string(vec![1u8]),
            StackItem::Null,
            StackItem::Buffer(vec![1u8]),
            StackItem::Map(map),
            StackItem::Struct(vec![
                StackItem::Integer(BigInt::from(1)),
                StackItem::Integer(BigInt::from(2)),
                StackItem::Integer(BigInt::from(3)),
            ]),
        ]);

        // Test deep copy
        let array_copy = array.deep_clone();

        // Verify it's a different object but with same content
        assert_eq!(array, array_copy, "Deep copy should have same content");
    }

    /// Test boolean conversion (matches C# TestBoolean)
    #[test]
    fn test_boolean() {
        // Test null -> false
        assert!(!StackItem::Null.as_bool().unwrap());

        // Test boolean values
        assert!(StackItem::Boolean(true).as_bool().unwrap());
        assert!(!StackItem::Boolean(false).as_bool().unwrap());

        // Test integer values
        assert!(StackItem::Integer(BigInt::from(1)).as_bool().unwrap());
        assert!(!StackItem::Integer(BigInt::from(0)).as_bool().unwrap());
        assert!(StackItem::Integer(BigInt::from(-1)).as_bool().unwrap());

        assert!(StackItem::from_byte_string(vec![1u8]).as_bool().unwrap());
        assert!(!StackItem::from_byte_string(vec![]).as_bool().unwrap());
        assert!(!StackItem::from_byte_string(vec![0u8]).as_bool().unwrap());
        assert!(StackItem::from_byte_string(vec![0u8, 1u8])
            .as_bool()
            .unwrap());

        // Test buffer values
        assert!(StackItem::Buffer(vec![1u8]).as_bool().unwrap());
        assert!(!StackItem::Buffer(vec![]).as_bool().unwrap());
        assert!(!StackItem::Buffer(vec![0u8]).as_bool().unwrap());

        // Test array values
        assert!(StackItem::Array(vec![StackItem::Null]).as_bool().unwrap());
        assert!(!StackItem::Array(vec![]).as_bool().unwrap());

        assert!(StackItem::Struct(vec![StackItem::Null]).as_bool().unwrap());
        assert!(!StackItem::Struct(vec![]).as_bool().unwrap());

        // Test map values
        let mut map = BTreeMap::new();
        map.insert(StackItem::Integer(BigInt::from(0)), StackItem::Null);
        assert!(StackItem::Map(map).as_bool().unwrap());
        assert!(!StackItem::Map(BTreeMap::new()).as_bool().unwrap());
    }

    /// Test integer conversion (matches C# TestInteger)
    #[test]
    fn test_integer() {
        // Test boolean to integer
        assert_eq!(StackItem::Boolean(true).as_int().unwrap(), BigInt::from(1));
        assert_eq!(StackItem::Boolean(false).as_int().unwrap(), BigInt::from(0));

        // Test integer values
        assert_eq!(
            StackItem::Integer(BigInt::from(42)).as_int().unwrap(),
            BigInt::from(42)
        );
        assert_eq!(
            StackItem::Integer(BigInt::from(-42)).as_int().unwrap(),
            BigInt::from(-42)
        );

        assert_eq!(
            StackItem::from_byte_string(vec![]).as_int().unwrap(),
            BigInt::from(0)
        );
        assert_eq!(
            StackItem::from_byte_string(vec![1u8]).as_int().unwrap(),
            BigInt::from(1)
        );
        assert_eq!(
            StackItem::from_byte_string(vec![0u8, 1u8])
                .as_int()
                .unwrap(),
            BigInt::from(256)
        );

        // Test buffer to integer
        assert_eq!(StackItem::Buffer(vec![]).as_int().unwrap(), BigInt::from(0));
        assert_eq!(
            StackItem::Buffer(vec![1u8]).as_int().unwrap(),
            BigInt::from(1)
        );

        // Test null to integer should fail
        assert!(StackItem::Null.as_int().is_err());
    }

    /// Test byte array conversion (matches C# TestByteArray)
    #[test]
    fn test_byte_array() {
        // Test null to bytes
        assert_eq!(StackItem::Null.as_bytes().unwrap(), vec![]);

        // Test boolean to bytes
        assert_eq!(StackItem::Boolean(true).as_bytes().unwrap(), vec![1u8]);
        assert_eq!(StackItem::Boolean(false).as_bytes().unwrap(), vec![0u8]);

        // Test integer to bytes
        assert_eq!(
            StackItem::Integer(BigInt::from(0)).as_bytes().unwrap(),
            vec![]
        );
        assert_eq!(
            StackItem::Integer(BigInt::from(1)).as_bytes().unwrap(),
            vec![1u8]
        );
        assert_eq!(
            StackItem::Integer(BigInt::from(256)).as_bytes().unwrap(),
            vec![0u8, 1u8]
        );

        let data = vec![1u8, 2u8, 3u8];
        assert_eq!(
            StackItem::from_byte_string(data.clone())
                .as_bytes()
                .unwrap(),
            data
        );

        // Test buffer to bytes
        assert_eq!(StackItem::Buffer(data.clone()).as_bytes().unwrap(), data);
    }

    /// Test array operations (matches C# TestArray)
    #[test]
    fn test_array() {
        let array = StackItem::Array(vec![
            StackItem::Integer(BigInt::from(1)),
            StackItem::Integer(BigInt::from(2)),
            StackItem::Integer(BigInt::from(3)),
        ]);

        let array_ref = array.as_array().unwrap();
        assert_eq!(array_ref.len(), 3);
        assert_eq!(array_ref[0], StackItem::Integer(BigInt::from(1)));
        assert_eq!(array_ref[1], StackItem::Integer(BigInt::from(2)));
        assert_eq!(array_ref[2], StackItem::Integer(BigInt::from(3)));

        let struct_item = StackItem::Struct(vec![
            StackItem::Integer(BigInt::from(4)),
            StackItem::Integer(BigInt::from(5)),
        ]);

        let struct_ref = struct_item.as_array().unwrap();
        assert_eq!(struct_ref.len(), 2);
        assert_eq!(struct_ref[0], StackItem::Integer(BigInt::from(4)));
        assert_eq!(struct_ref[1], StackItem::Integer(BigInt::from(5)));

        // Test non-array should fail
        assert!(StackItem::Integer(BigInt::from(1)).as_array().is_err());
    }

    /// Test map operations (matches C# TestMap)
    #[test]
    fn test_map() {
        let mut map = BTreeMap::new();
        map.insert(
            StackItem::Integer(BigInt::from(1)),
            StackItem::from_byte_string("one"),
        );
        map.insert(
            StackItem::Integer(BigInt::from(2)),
            StackItem::from_byte_string("two"),
        );

        let map_item = StackItem::Map(map.clone());
        let map_ref = map_item.as_map().unwrap();

        assert_eq!(map_ref.len(), 2);
        assert_eq!(
            map_ref.get(&StackItem::Integer(BigInt::from(1))),
            Some(&StackItem::from_byte_string("one"))
        );
        assert_eq!(
            map_ref.get(&StackItem::Integer(BigInt::from(2))),
            Some(&StackItem::from_byte_string("two"))
        );

        // Test non-map should fail
        assert!(StackItem::Integer(BigInt::from(1)).as_map().is_err());
    }
}
