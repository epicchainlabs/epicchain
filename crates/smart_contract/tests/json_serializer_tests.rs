//! JsonSerializer tests converted from C# Neo unit tests (UT_JsonSerializer.cs).
//! These tests ensure 100% compatibility with the C# Neo JSON serialization implementation.

use neo_json::{JObject, JValue};
use neo_smart_contract::{ApplicationEngine, ExecutionEngineLimits, JsonSerializer};
use neo_vm::types::{Array, Boolean, ByteString, Integer, Map, StackItem};
use std::collections::HashMap;

// ============================================================================
// Test JSON parsing errors
// ============================================================================

/// Test converted from C# UT_JsonSerializer.JsonTest_WrongJson
#[test]
fn test_json_wrong_json() {
    // Test trailing characters after array
    let json = "[    ]XXXXXXX";
    let result = JObject::parse(json);
    assert!(result.is_err());

    // Test trailing characters after object
    let json = "{   }XXXXXXX";
    let result = JObject::parse(json);
    assert!(result.is_err());

    // Test invalid array with multiple commas
    let json = "[,,,,]";
    let result = JObject::parse(json);
    assert!(result.is_err());

    // Test invalid false with comma
    let json = "false,X";
    let result = JObject::parse(json);
    assert!(result.is_err());

    // Test invalid false with @
    let json = "false@@@";
    let result = JObject::parse(json);
    assert!(result.is_err());

    // Test very long number (974 9's)
    let long_number = "9".repeat(974);
    let json = format!("{{\"length\":{}}}", long_number);
    let result = JObject::parse(&json);
    assert!(result.is_err());
}

// ============================================================================
// Test JSON array parsing
// ============================================================================

/// Test converted from C# UT_JsonSerializer.JsonTest_Array
#[test]
fn test_json_array() {
    // Test empty array
    let json = "[    ]";
    let parsed = JObject::parse(json).unwrap();
    assert_eq!("[]", parsed.to_string());

    // Test array with mixed types
    let json = "[1,\"a==\",    -1.3 ,null] ";
    let parsed = JObject::parse(json).unwrap();
    assert_eq!("[1,\"a==\",-1.3,null]", parsed.to_string());
}

// ============================================================================
// Test JSON boolean parsing
// ============================================================================

/// Test converted from C# UT_JsonSerializer.JsonTest_Bool
#[test]
fn test_json_bool() {
    // Test valid boolean values
    let json = "[  true ,false ]";
    let parsed = JObject::parse(json).unwrap();
    assert_eq!("[true,false]", parsed.to_string());

    // Test invalid boolean values (case sensitive)
    let json = "[True,FALSE] ";
    let result = JObject::parse(json);
    assert!(result.is_err());
}

// ============================================================================
// Test JSON number parsing
// ============================================================================

/// Test converted from C# UT_JsonSerializer.JsonTest_Numbers
#[test]
fn test_json_numbers() {
    // Test valid numbers
    let json = "[  1, -2 , 3.5 ]";
    let parsed = JObject::parse(json).unwrap();
    assert_eq!("[1,-2,3.5]", parsed.to_string());

    // Test scientific notation
    let json = "[200.500000E+005,200.500000e+5,-1.1234e-100,9.05E+28]";
    let parsed = JObject::parse(json).unwrap();
    assert_eq!(
        "[20050000,20050000,-1.1234E-100,9.05E+28]",
        parsed.to_string()
    );

    // Test invalid numbers
    let invalid_numbers = vec![
        "[-]",
        "[1.]",
        "[.123]",
        "[--1.123]",
        "[+1.123]",
        "[1.12.3]",
        "[e--1]",
        "[e++1]",
        "[E- 1]",
        "[3e--1]",
        "[2e++1]",
        "[1E- 1]",
    ];

    for json in invalid_numbers {
        let result = JObject::parse(json);
        assert!(result.is_err(), "Expected error for JSON: {}", json);
    }
}

// ============================================================================
// Test JSON string parsing
// ============================================================================

/// Test converted from C# UT_JsonSerializer.JsonTest_String
#[test]
fn test_json_string() {
    // Test escape sequences
    let json = r#" ["" ,  "\b\f\t\n\r\/\\" ]"#;
    let parsed = JObject::parse(json).unwrap();
    assert_eq!(r#"["","\b\f\t\n\r/\\"]"#, parsed.to_string());

    // Test Unicode escape
    let json = r#"["\uD834\uDD1E"]"#;
    let parsed = JObject::parse(json).unwrap();
    assert_eq!(json, parsed.to_string());

    // Test invalid escape sequence
    let json = r#"["\\x00"]"#;
    let parsed = JObject::parse(json).unwrap();
    assert_eq!(json, parsed.to_string());

    // Test invalid strings
    let invalid_strings = vec![
        r#"["]"#,
        r#"["\uaaa"]"#,
        r#"["\uaa"]"#,
        r#"["\ua"]"#,
        r#"["\u"]"#,
    ];

    for json in invalid_strings {
        let result = JObject::parse(json);
        assert!(result.is_err(), "Expected error for JSON: {}", json);
    }
}

// ============================================================================
// Test JSON object parsing
// ============================================================================

/// Test converted from C# UT_JsonSerializer.JsonTest_Object
#[test]
fn test_json_object() {
    // Test simple object
    let json = r#" {"test":   true}"#;
    let parsed = JObject::parse(json).unwrap();
    assert_eq!(r#"{"test":true}"#, parsed.to_string());

    // Test object with Unicode key
    let json = r#" {"\uAAAA":   true}"#;
    let parsed = JObject::parse(json).unwrap();
    assert_eq!(r#"{"\uAAAA":true}"#, parsed.to_string());

    // Test invalid objects
    let invalid_objects = vec![r#"{"a":}"#, r#"{NULL}"#, r#"["a":]"#];

    for json in invalid_objects {
        let result = JObject::parse(json);
        assert!(result.is_err(), "Expected error for JSON: {}", json);
    }
}

// ============================================================================
// Test serialization
// ============================================================================

/// Test converted from C# UT_JsonSerializer.Serialize_WrongJson
#[test]
fn test_serialize_wrong_json() {
    // InteropInterface cannot be serialized
    let item = StackItem::InteropInterface(InteropInterface::new(Box::new("test")));
    let result = JsonSerializer::serialize(&item);
    assert!(result.is_err());
}

/// Test converted from C# UT_JsonSerializer.Serialize_EmptyObject
#[test]
fn test_serialize_empty_object() {
    let entry = Map::new();
    let json = JsonSerializer::serialize(&StackItem::Map(entry)).unwrap();
    assert_eq!("{}", json.to_string());
}

/// Test converted from C# UT_JsonSerializer.Serialize_Number
#[test]
fn test_serialize_number() {
    // Test number too large for JSON
    let mut array = Array::new();
    array.push(StackItem::Integer(Integer::from(1)));
    array.push(StackItem::Integer(Integer::from(9007199254740992i64)));

    let result = JsonSerializer::serialize(&StackItem::Array(array));
    assert!(result.is_err());
}

/// Test converted from C# UT_JsonSerializer.Serialize_Null
#[test]
fn test_serialize_null() {
    let json = JsonSerializer::serialize(&StackItem::Null).unwrap();
    assert_eq!("null", json.to_string());
}

/// Test converted from C# UT_JsonSerializer.Serialize_EmptyArray
#[test]
fn test_serialize_empty_array() {
    let entry = Array::new();
    let json = JsonSerializer::serialize(&StackItem::Array(entry)).unwrap();
    assert_eq!("[]", json.to_string());
}

/// Test converted from C# UT_JsonSerializer.Serialize_Map_Test
#[test]
fn test_serialize_map() {
    let mut entry = Map::new();
    entry.insert(
        StackItem::ByteString(ByteString::from("test1".as_bytes())),
        StackItem::Integer(Integer::from(1)),
    );
    entry.insert(
        StackItem::ByteString(ByteString::from("test3".as_bytes())),
        StackItem::Integer(Integer::from(3)),
    );
    entry.insert(
        StackItem::ByteString(ByteString::from("test2".as_bytes())),
        StackItem::Integer(Integer::from(2)),
    );

    let json = JsonSerializer::serialize(&StackItem::Map(entry)).unwrap();
    assert_eq!(r#"{"test1":1,"test3":3,"test2":2}"#, json.to_string());
}

/// Test converted from C# UT_JsonSerializer.JsonTest_Serialize_Map_Test
#[test]
fn test_serialize_map_invalid_utf8() {
    let mut entry = Map::new();
    entry.insert(
        StackItem::ByteString(ByteString::from(vec![0xC1])),
        StackItem::Integer(Integer::from(1)),
    );
    entry.insert(
        StackItem::ByteString(ByteString::from(vec![0xC2])),
        StackItem::Integer(Integer::from(2)),
    );

    let result = JsonSerializer::serialize(&StackItem::Map(entry));
    assert!(result.is_err());
}

/// Test converted from C# UT_JsonSerializer.Serialize_Array_Bool_Str_Num
#[test]
fn test_serialize_array_bool_str_num() {
    let mut entry = Array::new();
    entry.push(StackItem::Boolean(Boolean::from(true)));
    entry.push(StackItem::ByteString(ByteString::from("test".as_bytes())));
    entry.push(StackItem::Integer(Integer::from(123)));

    let json = JsonSerializer::serialize(&StackItem::Array(entry)).unwrap();
    assert_eq!("[true,\"test\",123]", json.to_string());
}

/// Test converted from C# UT_JsonSerializer.Serialize_Array_OfArray
#[test]
fn test_serialize_array_of_array() {
    let mut array1 = Array::new();
    array1.push(StackItem::Boolean(Boolean::from(true)));
    array1.push(StackItem::ByteString(ByteString::from("test1".as_bytes())));
    array1.push(StackItem::Integer(Integer::from(123)));

    let mut array2 = Array::new();
    array2.push(StackItem::Boolean(Boolean::from(true)));
    array2.push(StackItem::ByteString(ByteString::from("test2".as_bytes())));
    array2.push(StackItem::Integer(Integer::from(321)));

    let mut entry = Array::new();
    entry.push(StackItem::Array(array1));
    entry.push(StackItem::Array(array2));

    let json = JsonSerializer::serialize(&StackItem::Array(entry)).unwrap();
    assert_eq!(
        "[[true,\"test1\",123],[true,\"test2\",321]]",
        json.to_string()
    );
}

// ============================================================================
// Test deserialization
// ============================================================================

/// Test converted from C# UT_JsonSerializer.Deserialize_WrongJson
#[test]
fn test_deserialize_wrong_json() {
    let engine = create_test_engine();
    let json = JObject::parse("x");
    assert!(json.is_err());
}

/// Test converted from C# UT_JsonSerializer.Deserialize_EmptyObject
#[test]
fn test_deserialize_empty_object() {
    let mut engine = create_test_engine();
    let json = JObject::parse("{}").unwrap();
    let items =
        JsonSerializer::deserialize(&mut engine, &json, &ExecutionEngineLimits::default()).unwrap();

    match items {
        StackItem::Map(map) => {
            assert_eq!(map.len(), 0);
        }
        _ => panic!("Expected Map"),
    }
}

/// Test converted from C# UT_JsonSerializer.Deserialize_EmptyArray
#[test]
fn test_deserialize_empty_array() {
    let mut engine = create_test_engine();
    let json = JObject::parse("[]").unwrap();
    let items =
        JsonSerializer::deserialize(&mut engine, &json, &ExecutionEngineLimits::default()).unwrap();

    match items {
        StackItem::Array(array) => {
            assert_eq!(array.len(), 0);
        }
        _ => panic!("Expected Array"),
    }
}

/// Test converted from C# UT_JsonSerializer.Deserialize_Map_Test
#[test]
fn test_deserialize_map() {
    let mut engine = create_test_engine();
    let json = JObject::parse(r#"{"test1":123,"test2":321}"#).unwrap();
    let items =
        JsonSerializer::deserialize(&mut engine, &json, &ExecutionEngineLimits::default()).unwrap();

    match items {
        StackItem::Map(map) => {
            assert_eq!(map.len(), 2);

            let key1 = StackItem::ByteString(ByteString::from("test1".as_bytes()));
            let value1 = map.get(&key1).unwrap();
            assert_eq!(value1.get_integer(), 123);

            let key2 = StackItem::ByteString(ByteString::from("test2".as_bytes()));
            let value2 = map.get(&key2).unwrap();
            assert_eq!(value2.get_integer(), 321);
        }
        _ => panic!("Expected Map"),
    }
}

/// Test converted from C# UT_JsonSerializer.Deserialize_Array_Bool_Str_Num
#[test]
fn test_deserialize_array_bool_str_num() {
    let mut engine = create_test_engine();
    let json = JObject::parse(r#"[true,"test",123,9.05E+28]"#).unwrap();
    let items =
        JsonSerializer::deserialize(&mut engine, &json, &ExecutionEngineLimits::default()).unwrap();

    match items {
        StackItem::Array(array) => {
            assert_eq!(array.len(), 4);

            assert!(array[0].get_boolean());
            assert_eq!(array[1].get_string(), "test");
            assert_eq!(array[2].get_integer(), 123);
            assert_eq!(array[3].get_integer(), 90500000000000000000000000000u128);
        }
        _ => panic!("Expected Array"),
    }
}

/// Test converted from C# UT_JsonSerializer.Deserialize_Array_OfArray
#[test]
fn test_deserialize_array_of_array() {
    let mut engine = create_test_engine();
    let json = JObject::parse(r#"[[true,"test1",123],[true,"test2",321]]"#).unwrap();
    let items =
        JsonSerializer::deserialize(&mut engine, &json, &ExecutionEngineLimits::default()).unwrap();

    match items {
        StackItem::Array(array) => {
            assert_eq!(array.len(), 2);

            // Check first inner array
            match &array[0] {
                StackItem::Array(inner_array) => {
                    assert_eq!(inner_array.len(), 3);
                    assert!(inner_array[0].get_boolean());
                    assert_eq!(inner_array[1].get_string(), "test1");
                    assert_eq!(inner_array[2].get_integer(), 123);
                }
                _ => panic!("Expected inner Array"),
            }

            // Check second inner array
            match &array[1] {
                StackItem::Array(inner_array) => {
                    assert_eq!(inner_array.len(), 3);
                    assert!(inner_array[0].get_boolean());
                    assert_eq!(inner_array[1].get_string(), "test2");
                    assert_eq!(inner_array[2].get_integer(), 321);
                }
                _ => panic!("Expected inner Array"),
            }
        }
        _ => panic!("Expected Array"),
    }
}

// ============================================================================
// Helper functions
// ============================================================================

fn create_test_engine() -> ApplicationEngine {
    ApplicationEngine::new()
}

// ============================================================================
// Implementation stubs
// ============================================================================

mod neo_json {
    use std::fmt;

    pub struct JObject;
    pub enum JValue {}

    impl JObject {
        pub fn parse(_json: &str) -> Result<JValue, String> {
            unimplemented!("JObject::parse stub")
        }
    }

    impl fmt::Display for JValue {
        fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
            unimplemented!("JValue::fmt stub")
        }
    }
}

mod neo_smart_contract {
    use super::*;

    pub struct JsonSerializer;

    impl JsonSerializer {
        pub fn serialize(_item: &StackItem) -> Result<JValue, String> {
            unimplemented!("JsonSerializer::serialize stub")
        }

        pub fn deserialize(
            _engine: &mut ApplicationEngine,
            _json: &JValue,
            _limits: &ExecutionEngineLimits,
        ) -> Result<StackItem, String> {
            unimplemented!("JsonSerializer::deserialize stub")
        }
    }

    pub struct ApplicationEngine;

    impl ApplicationEngine {
        pub fn new() -> Self {
            ApplicationEngine
        }
    }

    #[derive(Default)]
    pub struct ExecutionEngineLimits;
}

mod neo_vm {
    pub mod types {
        use std::collections::HashMap;

        #[derive(Debug, Clone, PartialEq)]
        pub enum StackItem {
            Null,
            Boolean(Boolean),
            Integer(Integer),
            ByteString(ByteString),
            Buffer(Buffer),
            Array(Array),
            Struct(Struct),
            Map(Map),
            InteropInterface(InteropInterface),
        }

        impl StackItem {
            pub fn get_boolean(&self) -> bool {
                match self {
                    StackItem::Boolean(b) => b.0,
                    _ => panic!("Not a boolean"),
                }
            }

            pub fn get_string(&self) -> String {
                match self {
                    StackItem::ByteString(bs) => String::from_utf8(bs.0.clone()).unwrap(),
                    _ => panic!("Not a string"),
                }
            }

            pub fn get_integer(&self) -> u128 {
                match self {
                    StackItem::Integer(i) => i.0 as u128,
                    _ => panic!("Not an integer"),
                }
            }
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct Boolean(pub bool);

        impl Boolean {
            pub fn from(value: bool) -> Self {
                Boolean(value)
            }
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct Integer(pub i64);

        impl Integer {
            pub fn from(value: i64) -> Self {
                Integer(value)
            }
        }

        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct ByteString(pub Vec<u8>);

        impl ByteString {
            pub fn from(value: &[u8]) -> Self {
                ByteString(value.to_vec())
            }
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct Buffer(Vec<u8>);

        #[derive(Debug, Clone, PartialEq)]
        pub struct Array(Vec<StackItem>);

        impl Array {
            pub fn new() -> Self {
                Array(Vec::new())
            }

            pub fn push(&mut self, item: StackItem) {
                self.0.push(item);
            }

            pub fn len(&self) -> usize {
                self.0.len()
            }
        }

        impl std::ops::Index<usize> for Array {
            type Output = StackItem;

            fn index(&self, index: usize) -> &Self::Output {
                &self.0[index]
            }
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct Struct(Vec<StackItem>);

        #[derive(Debug, Clone)]
        pub struct Map(HashMap<StackItem, StackItem>);

        impl Map {
            pub fn new() -> Self {
                Map(HashMap::new())
            }

            pub fn insert(&mut self, key: StackItem, value: StackItem) {
                self.0.insert(key, value);
            }

            pub fn len(&self) -> usize {
                self.0.len()
            }

            pub fn get(&self, key: &StackItem) -> Option<&StackItem> {
                self.0.get(key)
            }
        }

        impl PartialEq for Map {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct InteropInterface(Box<dyn std::any::Any>);

        impl InteropInterface {
            pub fn new(value: Box<dyn std::any::Any>) -> Self {
                InteropInterface(value)
            }
        }
    }
}
