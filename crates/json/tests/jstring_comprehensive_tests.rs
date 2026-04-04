//! Comprehensive JString Tests for C# Compatibility
//!
//! This module implements all 40 test methods from C# UT_JString.cs
//! to ensure complete behavioral compatibility between Neo-RS and Neo-CS.

use neo_json::JString;

/// Test string constants matching C# UT_JString exactly
mod test_data {
    use super::*;

    pub fn asic_string() -> JString {
        JString::from("hello world")
    }
    pub fn escape_string() -> JString {
        JString::from("\n\t\'\"")
    }
    pub fn bad_char() -> JString {
        JString::from((0xff as char).to_string())
    }
    pub fn integer_string() -> JString {
        JString::from("123")
    }
    pub fn empty_string() -> JString {
        JString::from("")
    }
    pub fn space_string() -> JString {
        JString::from("    ")
    }
    pub fn double_string() -> JString {
        JString::from("123.456")
    }
    pub fn unicode_string() -> JString {
        JString::from("😃😁")
    }
    pub fn emoji_string() -> JString {
        JString::from("ã🦆")
    }
    pub fn mixed_string() -> JString {
        JString::from("abc123!@# ")
    }
    pub fn long_string() -> JString {
        JString::from("x".repeat(5000))
    }
    pub fn multi_lang_string() -> JString {
        JString::from("Hello 你好 مرحبا")
    }
    pub fn json_string() -> JString {
        JString::from("{\"key\": \"value\"}")
    }
    pub fn html_entity_string() -> JString {
        JString::from("&amp; &lt; &gt;")
    }
    pub fn control_char_string() -> JString {
        JString::from("\t\n\r")
    }
    pub fn single_char_string() -> JString {
        JString::from("a")
    }
    pub fn long_word_string() -> JString {
        JString::from("Supercalifragilisticexpialidocious")
    }
    pub fn concatenated_string() -> JString {
        JString::from(format!("{}{}{}", "Hello", "123", "!@#"))
    }
    pub fn white_space_string() -> JString {
        JString::from("   leading and trailing spaces   ")
    }
    pub fn file_path_string() -> JString {
        JString::from(r"C:\Users\Example\file.txt")
    }
    pub fn large_number_string() -> JString {
        JString::from("12345678901234567890")
    }
    pub fn hexadecimal_string() -> JString {
        JString::from("0x1A3F")
    }
    pub fn palindrome_string() -> JString {
        JString::from("racecar")
    }
    pub fn sql_injection_string() -> JString {
        JString::from("SELECT * FROM users WHERE name = 'a'; DROP TABLE users;")
    }
    pub fn regex_string() -> JString {
        JString::from(r"^\d{3}-\d{2}-\d{4}$")
    }
    pub fn date_time_string() -> JString {
        JString::from("2023-01-01T00:00:00")
    }
    pub fn special_char_string() -> JString {
        JString::from("!?@#$%^&*()")
    }
    pub fn substring_string() -> JString {
        JString::from(&"Hello world"[0..5])
    }
    pub fn case_sensitive_string1() -> JString {
        JString::from("TestString")
    }
    pub fn case_sensitive_string2() -> JString {
        JString::from("teststring")
    }
    pub fn boolean_string() -> JString {
        JString::from("true")
    }
    pub fn format_specifier_string() -> JString {
        JString::from("{0:C}")
    }
    pub fn emoji_sequence_string() -> JString {
        JString::from("👨‍👩‍👦")
    }
    pub fn null_char_string() -> JString {
        JString::from("Hello\0World")
    }
    pub fn repeating_pattern_string() -> JString {
        JString::from("abcabcabc")
    }
}

/// Enum for testing - matches C# Woo enum
#[derive(Debug, PartialEq)]
enum Woo {
    James,
    Jerry,
}

impl std::str::FromStr for Woo {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "James" => Ok(Woo::James),
            "Jerry" => Ok(Woo::Jerry),
            _ => Err(()),
        }
    }
}

/// Enum for testing implicit operators - matches C# EnumExample
#[derive(Debug, PartialEq)]
enum EnumExample {
    Value,
}

impl std::fmt::Display for EnumExample {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EnumExample::Value => write!(f, "Value"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_data::*;

    /// Test constructor functionality (matches C# UT_JString.TestConstructor)
    #[test]
    fn test_constructor() {
        let s = "hello world";
        let jstring = JString::from(s);
        assert_eq!(jstring.value(), s);

        // Test null constructor should panic (Rust equivalent)
        // In Rust, we can't pass null, so we test with empty string
    }

    /// Test null constructor (matches C# UT_JString.TestConstructorNull)
    #[test]
    fn test_constructor_null() {
        // In Rust, we can't have null strings, so this test ensures
        // we handle the empty case properly
        let jstring = JString::from("");
        assert_eq!(jstring.value(), "");
    }

    /// Test empty constructor (matches C# UT_JString.TestConstructorEmpty)
    #[test]
    fn test_constructor_empty() {
        let s = "";
        let jstring = JString::from(s);
        assert_eq!(jstring.value(), s);
    }

    /// Test space constructor (matches C# UT_JString.TestConstructorSpace)
    #[test]
    fn test_constructor_space() {
        let s = "    ";
        let jstring = JString::from(s);
        assert_eq!(jstring.value(), s);
    }

    /// Test AsBoolean functionality (matches C# UT_JString.TestAsBoolean)
    #[test]
    fn test_as_boolean() {
        assert!(asic_string().as_boolean());
        assert!(escape_string().as_boolean());
        assert!(bad_char().as_boolean());
        assert!(integer_string().as_boolean());
        assert!(!empty_string().as_boolean());
        assert!(space_string().as_boolean());
        assert!(double_string().as_boolean());
        assert!(unicode_string().as_boolean());
        assert!(emoji_string().as_boolean());
        assert!(mixed_string().as_boolean());
        assert!(long_string().as_boolean());
        assert!(multi_lang_string().as_boolean());
        assert!(json_string().as_boolean());
        assert!(html_entity_string().as_boolean());
        assert!(control_char_string().as_boolean());
        assert!(single_char_string().as_boolean());
        assert!(long_word_string().as_boolean());
        assert!(concatenated_string().as_boolean());
        assert!(white_space_string().as_boolean());
        assert!(file_path_string().as_boolean());
        assert!(large_number_string().as_boolean());
        assert!(hexadecimal_string().as_boolean());
        assert!(palindrome_string().as_boolean());
        assert!(sql_injection_string().as_boolean());
        assert!(regex_string().as_boolean());
        assert!(date_time_string().as_boolean());
        assert!(special_char_string().as_boolean());
        assert!(substring_string().as_boolean());
        assert!(case_sensitive_string1().as_boolean());
        assert!(case_sensitive_string2().as_boolean());
        assert!(boolean_string().as_boolean());
        assert!(format_specifier_string().as_boolean());
        assert!(emoji_sequence_string().as_boolean());
        assert!(null_char_string().as_boolean());
        assert!(repeating_pattern_string().as_boolean());
    }

    /// Test AsNumber functionality (matches C# UT_JString.TestAsNumber)
    #[test]
    fn test_as_number() {
        assert!(asic_string().as_number().is_nan());
        assert!(escape_string().as_number().is_nan());
        assert!(bad_char().as_number().is_nan());
        assert_eq!(integer_string().as_number(), 123.0);
        assert_eq!(empty_string().as_number(), 0.0);
        assert!(space_string().as_number().is_nan());
        assert_eq!(double_string().as_number(), 123.456);
        assert!(unicode_string().as_number().is_nan());
        assert!(emoji_string().as_number().is_nan());
        assert!(mixed_string().as_number().is_nan());
        assert!(long_string().as_number().is_nan());
        assert!(multi_lang_string().as_number().is_nan());
        assert!(json_string().as_number().is_nan());
        assert!(html_entity_string().as_number().is_nan());
        assert!(control_char_string().as_number().is_nan());
        assert!(single_char_string().as_number().is_nan());
        assert!(long_word_string().as_number().is_nan());
        assert!(concatenated_string().as_number().is_nan());
        assert!(white_space_string().as_number().is_nan());
        assert!(file_path_string().as_number().is_nan());
        assert_eq!(large_number_string().as_number(), 12345678901234567890.0);
        assert!(hexadecimal_string().as_number().is_nan());
        assert!(palindrome_string().as_number().is_nan());
        assert!(sql_injection_string().as_number().is_nan());
        assert!(regex_string().as_number().is_nan());
        assert!(date_time_string().as_number().is_nan());
        assert!(special_char_string().as_number().is_nan());
        assert!(substring_string().as_number().is_nan());
        assert!(case_sensitive_string1().as_number().is_nan());
        assert!(case_sensitive_string2().as_number().is_nan());
        assert!(boolean_string().as_number().is_nan());
        assert!(format_specifier_string().as_number().is_nan());
        assert!(emoji_sequence_string().as_number().is_nan());
        assert!(null_char_string().as_number().is_nan());
        assert!(repeating_pattern_string().as_number().is_nan());
    }

    /// Test valid GetEnum functionality (matches C# UT_JString.TestValidGetEnum)
    #[test]
    fn test_valid_get_enum() {
        let valid_enum = JString::from("James");
        let woo: Result<Woo, String> = valid_enum.get_enum();
        assert_eq!(woo.unwrap(), Woo::James);

        let valid_enum = JString::from("");
        let woo = valid_enum.as_enum(Woo::Jerry, false);
        assert_eq!(woo, Woo::Jerry);
    }

    /// Test invalid GetEnum functionality (matches C# UT_JString.TestInValidGetEnum)
    #[test]
    fn test_invalid_get_enum() {
        let invalid_enum = JString::from("_James");
        let result: Result<Woo, String> = invalid_enum.get_enum();
        assert!(result.is_err());
    }

    /// Test mixed string (matches C# UT_JString.TestMixedString)
    #[test]
    fn test_mixed_string() {
        assert_eq!(mixed_string().value(), "abc123!@# ");
    }

    /// Test long string (matches C# UT_JString.TestLongString)
    #[test]
    fn test_long_string() {
        assert_eq!(long_string().value(), "x".repeat(5000));
    }

    /// Test multi-language string (matches C# UT_JString.TestMultiLangString)
    #[test]
    fn test_multi_lang_string() {
        assert_eq!(multi_lang_string().value(), "Hello 你好 مرحبا");
    }

    /// Test JSON string (matches C# UT_JString.TestJsonString)
    #[test]
    fn test_json_string() {
        assert_eq!(json_string().value(), "{\"key\": \"value\"}");
    }

    /// Test HTML entity string (matches C# UT_JString.TestHtmlEntityString)
    #[test]
    fn test_html_entity_string() {
        assert_eq!(html_entity_string().value(), "&amp; &lt; &gt;");
    }

    /// Test control character string (matches C# UT_JString.TestControlCharString)
    #[test]
    fn test_control_char_string() {
        assert_eq!(control_char_string().value(), "\t\n\r");
    }

    /// Test single character string (matches C# UT_JString.TestSingleCharString)
    #[test]
    fn test_single_char_string() {
        assert_eq!(single_char_string().value(), "a");
    }

    /// Test long word string (matches C# UT_JString.TestLongWordString)
    #[test]
    fn test_long_word_string() {
        assert_eq!(
            long_word_string().value(),
            "Supercalifragilisticexpialidocious"
        );
    }

    /// Test concatenated string (matches C# UT_JString.TestConcatenatedString)
    #[test]
    fn test_concatenated_string() {
        assert_eq!(concatenated_string().value(), "Hello123!@#");
    }

    /// Test whitespace string (matches C# UT_JString.TestWhiteSpaceString)
    #[test]
    fn test_white_space_string() {
        assert_eq!(
            white_space_string().value(),
            "   leading and trailing spaces   "
        );
    }

    /// Test file path string (matches C# UT_JString.TestFilePathString)
    #[test]
    fn test_file_path_string() {
        assert_eq!(file_path_string().value(), r"C:\Users\Example\file.txt");
    }

    /// Test large number string (matches C# UT_JString.TestLargeNumberString)
    #[test]
    fn test_large_number_string() {
        assert_eq!(large_number_string().value(), "12345678901234567890");
    }

    /// Test hexadecimal string (matches C# UT_JString.TestHexadecimalString)
    #[test]
    fn test_hexadecimal_string() {
        assert_eq!(hexadecimal_string().value(), "0x1A3F");
    }

    /// Test palindrome string (matches C# UT_JString.TestPalindromeString)
    #[test]
    fn test_palindrome_string() {
        assert_eq!(palindrome_string().value(), "racecar");
    }

    /// Test SQL injection string (matches C# UT_JString.TestSqlInjectionString)
    #[test]
    fn test_sql_injection_string() {
        assert_eq!(
            sql_injection_string().value(),
            "SELECT * FROM users WHERE name = 'a'; DROP TABLE users;"
        );
    }

    /// Test regex string (matches C# UT_JString.TestRegexString)
    #[test]
    fn test_regex_string() {
        assert_eq!(regex_string().value(), r"^\d{3}-\d{2}-\d{4}$");
    }

    /// Test date-time string (matches C# UT_JString.TestDateTimeString)
    #[test]
    fn test_date_time_string() {
        assert_eq!(date_time_string().value(), "2023-01-01T00:00:00");
    }

    /// Test special character string (matches C# UT_JString.TestSpecialCharString)
    #[test]
    fn test_special_char_string() {
        assert_eq!(special_char_string().value(), "!?@#$%^&*()");
    }

    /// Test substring string (matches C# UT_JString.TestSubstringString)
    #[test]
    fn test_substring_string() {
        assert_eq!(substring_string().value(), "Hello");
    }

    /// Test case-sensitive strings (matches C# UT_JString.TestCaseSensitiveStrings)
    #[test]
    fn test_case_sensitive_strings() {
        assert_ne!(
            case_sensitive_string1().value(),
            case_sensitive_string2().value()
        );
    }

    /// Test boolean string (matches C# UT_JString.TestBooleanString)
    #[test]
    fn test_boolean_string() {
        assert_eq!(boolean_string().value(), "true");
    }

    /// Test format specifier string (matches C# UT_JString.TestFormatSpecifierString)
    #[test]
    fn test_format_specifier_string() {
        assert_eq!(format_specifier_string().value(), "{0:C}");
    }

    /// Test emoji sequence string (matches C# UT_JString.TestEmojiSequenceString)
    #[test]
    fn test_emoji_sequence_string() {
        assert_eq!(emoji_sequence_string().value(), "👨‍👩‍👦");
    }

    /// Test null character string (matches C# UT_JString.TestNullCharString)
    #[test]
    fn test_null_char_string() {
        assert_eq!(null_char_string().value(), "Hello\0World");
    }

    /// Test repeating pattern string (matches C# UT_JString.TestRepeatingPatternString)
    #[test]
    fn test_repeating_pattern_string() {
        assert_eq!(repeating_pattern_string().value(), "abcabcabc");
    }

    /// Test equality operations (matches C# UT_JString.TestEqual)
    #[test]
    fn test_equal() {
        let str_val = "hello world";
        let str2_val = "hello world2";
        let jstring = JString::from(str_val);
        let jstring2 = JString::from(str2_val);

        assert_eq!(jstring.value(), str_val);
        assert_ne!(jstring.value(), str2_val);
        assert_eq!(jstring.get_string(), str_val);
        assert_eq!(jstring.value(), jstring.value());
        assert_ne!(jstring.value(), jstring2.value());

        let reference = &jstring;
        assert_eq!(jstring.value(), reference.value());
    }

    /// Test JSON write functionality (matches C# UT_JString.TestWrite)
    #[test]
    fn test_write() {
        let jstring = JString::from("hello world");
        let mut output = String::new();
        jstring.write(&mut output).expect("Write should succeed");
        assert_eq!(output, "\"hello world\"");
    }

    /// Test clone functionality (matches C# UT_JString.TestClone)
    #[test]
    fn test_clone() {
        let jstring = JString::from("hello world");
        let clone = jstring.clone_jstring();
        assert_eq!(jstring.value(), clone.value());
        // In Rust, clones are separate instances (not same reference)
    }

    /// Test equality with different types (matches C# UT_JString.TestEqualityWithDifferentTypes)
    #[test]
    fn test_equality_with_different_types() {
        let jstring = JString::from("hello world");
        // In Rust, we test type conversion and comparison
        assert_ne!(jstring.value(), "123");
        // Type safety in Rust prevents comparing with completely different types
    }

    /// Test implicit operators (matches C# UT_JString.TestImplicitOperators)
    #[test]
    fn test_implicit_operators() {
        let from_enum = JString::from(EnumExample::Value.to_string());
        assert_eq!(from_enum.value(), "Value");

        let from_string = JString::from("test string");
        assert_eq!(from_string.value(), "test string");

        // Rust doesn't have null strings, so we test empty
        let empty_string = JString::from("");
        assert_eq!(empty_string.value(), "");
    }

    /// Test boundary and special cases (matches C# UT_JString.TestBoundaryAndSpecialCases)
    #[test]
    fn test_boundary_and_special_cases() {
        let large_string = JString::from("a".repeat(65535));
        assert_eq!(large_string.value().len(), 65535);

        let special_unicode = JString::from("😀");
        assert_eq!(special_unicode.value(), "😀");

        let complex_json = JString::from("{\"nested\":{\"key\":\"value\"}}");
        assert_eq!(complex_json.value(), "{\"nested\":{\"key\":\"value\"}}");
    }

    /// Test exception handling (matches C# UT_JString.TestExceptionHandling)
    #[test]
    fn test_exception_handling() {
        let invalid_enum = JString::from("invalid_value");

        let result = invalid_enum.as_enum(Woo::Jerry, false);
        assert_eq!(result, Woo::Jerry);

        let enum_result: Result<Woo, String> = invalid_enum.get_enum();
        assert!(enum_result.is_err());
    }
}
