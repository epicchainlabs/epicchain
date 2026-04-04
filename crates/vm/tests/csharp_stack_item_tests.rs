//! Comprehensive StackItem tests that exactly match C# Neo.VM.Tests/UT_StackItem.cs
//!
//! This file contains unit tests that ensure the Rust StackItem implementation
//! behaves identically to the C# Neo VM StackItem implementation.

use neo_vm::{script::Script, stack_item::StackItem};
use num_bigint::BigInt;
use std::collections::BTreeMap;

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    /// Test circular reference handling (matches C# TestCircularReference)
    #[test]
    fn test_circular_reference() {
        // Note: In Rust, creating true circular references is complex due to borrowing rules
        // We'll test the equality logic instead
        let item_a =
            StackItem::from_struct(vec![StackItem::Boolean(true), StackItem::Boolean(false)]);

        let item_b =
            StackItem::from_struct(vec![StackItem::Boolean(true), StackItem::Boolean(false)]);

        let item_c =
            StackItem::from_struct(vec![StackItem::Boolean(false), StackItem::Boolean(false)]);

        // Test hash codes - items with same structure should have same hash
        // Note: Rust doesn't have GetHashCode() like C#, but we can test equality
        assert_eq!(item_a, item_b, "Items A and B should be equal");
        assert_ne!(item_a, item_c, "Items A and C should not be equal");
    }

    /// Test hash code generation (matches C# TestHashCode)
    #[test]
    fn test_hash_code() {
        // Note: Rust doesn't have GetHashCode(), but we test equality which uses similar logic

        let item_a = StackItem::from_byte_string("NEO");
        let item_b = StackItem::from_byte_string("NEO");
        let item_c = StackItem::from_byte_string("SmartEconomy");

        assert_eq!(item_a, item_b, "Same strings should be equal");
        assert_ne!(item_a, item_c, "Different strings should not be equal");

        // Buffer comparison
        let item_a = StackItem::from_buffer(vec![0; 1]);
        let item_b = StackItem::from_buffer(vec![0; 1]);
        let item_c = StackItem::from_buffer(vec![0; 2]);

        assert_eq!(item_a, item_b, "Same buffers should be equal");
        assert_ne!(item_a, item_c, "Different buffers should not be equal");

        // Byte array comparison
        let item_a = StackItem::from_byte_string(vec![1, 2, 3]);
        let item_b = StackItem::from_byte_string(vec![1, 2, 3]);
        let item_c = StackItem::from_byte_string(vec![5, 6]);

        assert_eq!(item_a, item_b, "Same byte arrays should be equal");
        assert_ne!(item_a, item_c, "Different byte arrays should not be equal");

        // Boolean comparison
        let item_a = StackItem::Boolean(true);
        let item_b = StackItem::Boolean(true);
        let item_c = StackItem::Boolean(false);

        assert_eq!(item_a, item_b, "Same booleans should be equal");
        assert_ne!(item_a, item_c, "Different booleans should not be equal");

        // Integer comparison
        let item_a = StackItem::from_int(1);
        let item_b = StackItem::from_int(1);
        let item_c = StackItem::from_int(123);

        assert_eq!(item_a, item_b, "Same integers should be equal");
        assert_ne!(item_a, item_c, "Different integers should not be equal");

        // Null comparison
        let item_a = StackItem::Null;
        let item_b = StackItem::Null;

        assert_eq!(item_a, item_b, "Null items should be equal");

        // Array comparison
        let item_a = StackItem::from_array(vec![
            StackItem::Boolean(true),
            StackItem::Boolean(false),
            StackItem::from_int(0),
        ]);
        let item_b = StackItem::from_array(vec![
            StackItem::Boolean(true),
            StackItem::Boolean(false),
            StackItem::from_int(0),
        ]);
        let item_c = StackItem::from_array(vec![
            StackItem::Boolean(true),
            StackItem::Boolean(false),
            StackItem::from_int(1),
        ]);

        assert_eq!(item_a, item_b, "Same arrays should be equal");
        assert_ne!(item_a, item_c, "Different arrays should not be equal");

        let item_a = StackItem::from_struct(vec![
            StackItem::Boolean(true),
            StackItem::Boolean(false),
            StackItem::from_int(0),
        ]);
        let item_b = StackItem::from_struct(vec![
            StackItem::Boolean(true),
            StackItem::Boolean(false),
            StackItem::from_int(0),
        ]);
        let item_c = StackItem::from_struct(vec![
            StackItem::Boolean(true),
            StackItem::Boolean(false),
            StackItem::from_int(1),
        ]);

        assert_eq!(item_a, item_b, "Same structs should be equal");
        assert_ne!(item_a, item_c, "Different structs should not be equal");

        // Map comparison
        let mut map_a = BTreeMap::new();
        map_a.insert(StackItem::Boolean(true), StackItem::Boolean(false));
        map_a.insert(StackItem::from_int(0), StackItem::from_int(1));
        let item_a = StackItem::from_map(map_a);

        let mut map_b = BTreeMap::new();
        map_b.insert(StackItem::Boolean(true), StackItem::Boolean(false));
        map_b.insert(StackItem::from_int(0), StackItem::from_int(1));
        let item_b = StackItem::from_map(map_b);

        let mut map_c = BTreeMap::new();
        map_c.insert(StackItem::Boolean(true), StackItem::Boolean(false));
        map_c.insert(StackItem::from_int(0), StackItem::from_int(2));
        let item_c = StackItem::from_map(map_c);

        assert_eq!(item_a, item_b, "Same maps should be equal");
        assert_ne!(item_a, item_c, "Different maps should not be equal");

        // Test compound type with shared subitems
        let junk = StackItem::from_array(vec![
            StackItem::Boolean(true),
            StackItem::Boolean(false),
            StackItem::from_int(0),
        ]);

        let mut map_a = BTreeMap::new();
        map_a.insert(StackItem::Boolean(true), junk.clone());
        map_a.insert(StackItem::from_int(0), junk.clone());
        let item_a = StackItem::from_map(map_a);

        let mut map_b = BTreeMap::new();
        map_b.insert(StackItem::Boolean(true), junk.clone());
        map_b.insert(StackItem::from_int(0), junk.clone());
        let item_b = StackItem::from_map(map_b);

        let mut map_c = BTreeMap::new();
        map_c.insert(StackItem::Boolean(true), junk);
        map_c.insert(StackItem::from_int(0), StackItem::from_int(2));
        let item_c = StackItem::from_map(map_c);

        assert_eq!(item_a, item_b, "Maps with same subitems should be equal");
        assert_ne!(
            item_a, item_c,
            "Maps with different subitems should not be equal"
        );

        // Pointer comparison
        let item_a = StackItem::from_pointer(123);
        let item_b = StackItem::from_pointer(123);
        let item_c = StackItem::from_pointer(1234);

        assert_eq!(item_a, item_b, "Same pointers should be equal");
        assert_ne!(item_a, item_c, "Different pointers should not be equal");
    }

    /// Test null item behavior (matches C# TestNull)
    #[test]
    fn test_null() {
        let null_item = StackItem::from_byte_string(vec![]);
        assert_ne!(
            StackItem::Null,
            null_item,
            "Empty byte string should not equal null"
        );

        let null_item = StackItem::Null;
        assert_eq!(StackItem::Null, null_item, "Null should equal null");
    }

    /// Test equality comparison (matches C# TestEqual)
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
            "Case sensitive comparison should fail"
        );
        assert!(
            !item_d.equals(&item_e).unwrap(),
            "Case sensitive comparison should fail"
        );

        // We can't directly test this like C# since Rust is statically typed
    }

    /// Test type casting (matches C# TestCast)
    #[test]
    fn test_cast() {
        // Signed byte
        let item = StackItem::from_int(i8::MAX);
        assert!(
            matches!(item, StackItem::Integer(_)),
            "Should be Integer type"
        );
        assert_eq!(item.as_int().unwrap(), BigInt::from(i8::MAX));

        // Unsigned byte
        let item = StackItem::from_int(u8::MAX);
        assert!(
            matches!(item, StackItem::Integer(_)),
            "Should be Integer type"
        );
        assert_eq!(item.as_int().unwrap(), BigInt::from(u8::MAX));

        // Signed short
        let item = StackItem::from_int(i16::MAX);
        assert!(
            matches!(item, StackItem::Integer(_)),
            "Should be Integer type"
        );
        assert_eq!(item.as_int().unwrap(), BigInt::from(i16::MAX));

        // Unsigned short
        let item = StackItem::from_int(u16::MAX);
        assert!(
            matches!(item, StackItem::Integer(_)),
            "Should be Integer type"
        );
        assert_eq!(item.as_int().unwrap(), BigInt::from(u16::MAX));

        // Signed integer
        let item = StackItem::from_int(i32::MAX);
        assert!(
            matches!(item, StackItem::Integer(_)),
            "Should be Integer type"
        );
        assert_eq!(item.as_int().unwrap(), BigInt::from(i32::MAX));

        // Unsigned integer
        let item = StackItem::from_int(u32::MAX);
        assert!(
            matches!(item, StackItem::Integer(_)),
            "Should be Integer type"
        );
        assert_eq!(item.as_int().unwrap(), BigInt::from(u32::MAX));

        // Signed long
        let item = StackItem::from_int(i64::MAX);
        assert!(
            matches!(item, StackItem::Integer(_)),
            "Should be Integer type"
        );
        assert_eq!(item.as_int().unwrap(), BigInt::from(i64::MAX));

        // Unsigned long
        let item = StackItem::from_int(u64::MAX);
        assert!(
            matches!(item, StackItem::Integer(_)),
            "Should be Integer type"
        );
        assert_eq!(item.as_int().unwrap(), BigInt::from(u64::MAX));

        // BigInteger
        let item = StackItem::from_int(BigInt::from(-1));
        assert!(
            matches!(item, StackItem::Integer(_)),
            "Should be Integer type"
        );
        assert_eq!(item.as_int().unwrap(), BigInt::from(-1));

        // Boolean
        let item = StackItem::Boolean(true);
        assert!(
            matches!(item, StackItem::Boolean(_)),
            "Should be Boolean type"
        );
        assert!(item.as_bool().unwrap(), "Should be true");

        // ByteString
        let item =
            StackItem::from_byte_string(vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]);
        assert!(
            matches!(item, StackItem::ByteString(_)),
            "Should be ByteString type"
        );
        assert_eq!(
            item.as_bytes().unwrap(),
            vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
        );
    }

    /// Test deep copy functionality (matches C# TestDeepCopy)
    #[test]
    fn test_deep_copy() {
        let a = StackItem::from_array(vec![
            StackItem::Boolean(true),
            StackItem::from_int(1),
            StackItem::from_byte_string(vec![1]),
            StackItem::Null,
            StackItem::from_buffer(vec![1]),
            {
                let mut map = BTreeMap::new();
                map.insert(StackItem::from_int(0), StackItem::from_int(1));
                map.insert(StackItem::from_int(2), StackItem::from_int(3));
                StackItem::from_map(map)
            },
            StackItem::from_struct(vec![
                StackItem::from_int(1),
                StackItem::from_int(2),
                StackItem::from_int(3),
            ]),
        ]);

        // Note: We can't easily create circular references in Rust due to borrowing rules

        let aa = a.deep_clone();

        // Verify it's a different object
        assert_ne!(
            &a as *const StackItem, &aa as *const StackItem,
            "Deep copy should create different object"
        );

        // Verify content is the same
        assert_eq!(a, aa, "Deep copy should have same content");

        // Verify that the map item was deep copied correctly
        if let (StackItem::Array(ref items_a), StackItem::Array(ref items_aa)) = (&a, &aa) {
            assert_eq!(items_a[5], items_aa[5], "Map items should be equal");

            // Verify all items are equal
            for (i, (item_a, item_aa)) in items_a.iter().zip(items_aa.iter()).enumerate() {
                assert_eq!(
                    item_a, item_aa,
                    "Item {} should be equal after deep copy",
                    i
                );
            }
        }
    }

    #[test]
    fn test_boolean_conversions() {
        // Test boolean conversion behavior
        assert!(
            !StackItem::Null.as_bool().unwrap(),
            "Null should convert to false"
        );
        assert!(
            StackItem::Boolean(true).as_bool().unwrap(),
            "True should convert to true"
        );
        assert!(
            !StackItem::Boolean(false).as_bool().unwrap(),
            "False should convert to false"
        );
        assert!(
            StackItem::from_int(1).as_bool().unwrap(),
            "Non-zero integer should convert to true"
        );
        assert!(
            !StackItem::from_int(0).as_bool().unwrap(),
            "Zero integer should convert to false"
        );
        assert!(
            !StackItem::from_byte_string(vec![]).as_bool().unwrap(),
            "Empty byte string should convert to false"
        );
        assert!(
            StackItem::from_byte_string(vec![1]).as_bool().unwrap(),
            "Non-empty byte string should convert to true"
        );
        assert!(
            !StackItem::from_array(vec![]).as_bool().unwrap(),
            "Empty array should convert to false"
        );
        assert!(
            StackItem::from_array(vec![StackItem::Null])
                .as_bool()
                .unwrap(),
            "Non-empty array should convert to true"
        );
    }

    #[test]
    fn test_integer_conversions() {
        // Test integer conversion behavior
        assert!(
            StackItem::Null.as_int().is_err(),
            "Null should not convert to integer"
        );
        assert_eq!(
            StackItem::Boolean(true).as_int().unwrap(),
            BigInt::from(1),
            "True should convert to 1"
        );
        assert_eq!(
            StackItem::Boolean(false).as_int().unwrap(),
            BigInt::from(0),
            "False should convert to 0"
        );
        assert_eq!(
            StackItem::from_int(42).as_int().unwrap(),
            BigInt::from(42),
            "Integer should convert to itself"
        );
        assert_eq!(
            StackItem::from_byte_string(vec![]).as_int().unwrap(),
            BigInt::from(0),
            "Empty bytes should convert to 0"
        );
        assert_eq!(
            StackItem::from_byte_string(vec![42]).as_int().unwrap(),
            BigInt::from(42),
            "Single byte should convert correctly"
        );
    }

    #[test]
    fn test_byte_conversions() {
        // Test byte array conversion behavior
        assert_eq!(
            StackItem::Null.as_bytes().unwrap(),
            vec![],
            "Null should convert to empty bytes"
        );
        assert_eq!(
            StackItem::Boolean(true).as_bytes().unwrap(),
            vec![1],
            "True should convert to [1]"
        );
        assert_eq!(
            StackItem::Boolean(false).as_bytes().unwrap(),
            vec![0],
            "False should convert to [0]"
        );
        assert_eq!(
            StackItem::from_int(0).as_bytes().unwrap(),
            vec![],
            "Zero should convert to empty bytes"
        );
        assert_eq!(
            StackItem::from_int(42).as_bytes().unwrap(),
            vec![42],
            "Small integer should convert correctly"
        );
        assert_eq!(
            StackItem::from_byte_string(vec![1, 2, 3])
                .as_bytes()
                .unwrap(),
            vec![1, 2, 3],
            "Byte string should convert to itself"
        );
    }

    #[test]
    fn test_type_checking() {
        // Test type checking methods
        assert!(StackItem::Null.is_null(), "Null should be null");
        assert!(
            !StackItem::Boolean(true).is_null(),
            "Boolean should not be null"
        );
        assert!(
            !StackItem::from_int(0).is_null(),
            "Integer should not be null"
        );

        // Test stack item type detection
        use neo_vm::stack_item::stack_item_type::StackItemType;

        assert_eq!(StackItem::Null.stack_item_type(), StackItemType::Any);
        assert_eq!(
            StackItem::Boolean(true).stack_item_type(),
            StackItemType::Boolean
        );
        assert_eq!(
            StackItem::from_int(42).stack_item_type(),
            StackItemType::Integer
        );
        assert_eq!(
            StackItem::from_byte_string(vec![]).stack_item_type(),
            StackItemType::ByteString
        );
        assert_eq!(
            StackItem::from_buffer(vec![]).stack_item_type(),
            StackItemType::Buffer
        );
        assert_eq!(
            StackItem::from_array(vec![]).stack_item_type(),
            StackItemType::Array
        );
        assert_eq!(
            StackItem::from_struct(vec![]).stack_item_type(),
            StackItemType::Struct
        );
        assert_eq!(
            StackItem::from_map(BTreeMap::new()).stack_item_type(),
            StackItemType::Map
        );
        assert_eq!(
            StackItem::from_pointer(0).stack_item_type(),
            StackItemType::Pointer
        );
    }
}
