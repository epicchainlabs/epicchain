use crate::{JToken, JsonError, JsonResult};

/// JSON Path token types
/// This matches the C# JPathTokenType enum
#[derive(Debug, Clone, PartialEq)]
pub enum JPathTokenType {
    Root,
    Property,
    ArrayIndex,
    ArraySlice,
    Wildcard,
    RecursiveDescent,
    FilterExpression,
}

/// Represents a JSON path token
/// This matches the C# JPathToken class
#[derive(Debug, Clone, PartialEq)]
pub struct JPathToken {
    pub token_type: JPathTokenType,
    pub value: Option<String>,
    pub index: Option<usize>,
    pub start: Option<usize>,
    pub end: Option<usize>,
}

impl JPathToken {
    /// Creates a new root token
    pub fn root() -> Self {
        Self {
            token_type: JPathTokenType::Root,
            value: None,
            index: None,
            start: None,
            end: None,
        }
    }

    /// Creates a new property token
    pub fn property(name: String) -> Self {
        Self {
            token_type: JPathTokenType::Property,
            value: Some(name),
            index: None,
            start: None,
            end: None,
        }
    }

    /// Creates a new array index token
    pub fn array_index(index: usize) -> Self {
        Self {
            token_type: JPathTokenType::ArrayIndex,
            value: None,
            index: Some(index),
            start: None,
            end: None,
        }
    }

    /// Creates a new array slice token
    pub fn array_slice(start: Option<usize>, end: Option<usize>) -> Self {
        Self {
            token_type: JPathTokenType::ArraySlice,
            value: None,
            index: None,
            start,
            end,
        }
    }

    /// Creates a new wildcard token
    pub fn wildcard() -> Self {
        Self {
            token_type: JPathTokenType::Wildcard,
            value: None,
            index: None,
            start: None,
            end: None,
        }
    }

    /// Creates a new recursive descent token
    pub fn recursive_descent() -> Self {
        Self {
            token_type: JPathTokenType::RecursiveDescent,
            value: None,
            index: None,
            start: None,
            end: None,
        }
    }

    /// Parses a JSON path expression
    pub fn parse(expr: &str) -> JsonResult<Vec<JPathToken>> {
        let mut tokens = Vec::new();
        let mut chars = expr.chars().peekable();

        // Always start with root
        if expr.starts_with('$') {
            tokens.push(JPathToken::root());
            chars.next(); // consume '$'
        } else {
            return Err(JsonError::ParseError(
                "JSON path must start with '$'".to_string(),
            ));
        }

        while let Some(ch) = chars.next() {
            match ch {
                '.' => {
                    if chars.peek() == Some(&'.') {
                        chars.next(); // consume second '.'
                        tokens.push(JPathToken::recursive_descent());
                    } else {
                        // Property access - read property name
                        let mut prop_name = String::new();
                        while let Some(&next_ch) = chars.peek() {
                            if next_ch == '.' || next_ch == '[' {
                                break;
                            }
                            prop_name.push(chars.next().ok_or_else(|| {
                                crate::error::JsonError::ParseError(
                                    "Iterator exhausted".to_string(),
                                )
                            })?);
                        }
                        if !prop_name.is_empty() {
                            if prop_name == "*" {
                                tokens.push(JPathToken::wildcard());
                            } else {
                                tokens.push(JPathToken::property(prop_name));
                            }
                        } else {
                            return Err(JsonError::ParseError(
                                "Empty property name after '.'".to_string(),
                            ));
                        }
                    }
                }
                '[' => {
                    // Array access or slice
                    let mut bracket_content = String::new();
                    let mut bracket_depth = 1;

                    for next_ch in chars.by_ref() {
                        if next_ch == '[' {
                            bracket_depth += 1;
                        } else if next_ch == ']' {
                            bracket_depth -= 1;
                            if bracket_depth == 0 {
                                break;
                            }
                        }
                        bracket_content.push(next_ch);
                    }

                    if bracket_depth != 0 {
                        return Err(JsonError::ParseError(
                            "Unclosed bracket in JSON path".to_string(),
                        ));
                    }

                    if bracket_content == "*" {
                        tokens.push(JPathToken::wildcard());
                    } else if bracket_content.contains(':') {
                        // Array slice
                        let parts: Vec<&str> = bracket_content.split(':').collect();
                        let start = if parts[0].is_empty() {
                            None
                        } else {
                            parts[0].parse().ok()
                        };
                        let end = if parts.len() > 1 && !parts[1].is_empty() {
                            parts[1].parse().ok()
                        } else {
                            None
                        };
                        tokens.push(JPathToken::array_slice(start, end));
                    } else if let Ok(index) = bracket_content.parse::<usize>() {
                        tokens.push(JPathToken::array_index(index));
                    } else {
                        let prop_name = bracket_content.trim_matches(|c| c == '"' || c == '\'');
                        tokens.push(JPathToken::property(prop_name.to_string()));
                    }
                }
                '*' => {
                    tokens.push(JPathToken::wildcard());
                }
                _ => {
                    // Unexpected character
                    return Err(JsonError::ParseError(format!(
                        "Unexpected character '{ch}' in JSON path"
                    )));
                }
            }
        }

        Ok(tokens)
    }

    /// Evaluates the path against a JSON token
    pub fn evaluate<'a>(tokens: &[JPathToken], root: &'a JToken) -> JsonResult<Vec<&'a JToken>> {
        let mut results = vec![root];

        for token in tokens.iter().skip(1) {
            // Skip root token
            let mut new_results = Vec::new();

            for current in results {
                match &token.token_type {
                    JPathTokenType::Property => {
                        if let Some(prop_name) = &token.value {
                            if let JToken::Object(obj) = current {
                                if let Some(Some(token)) = obj.get(prop_name) {
                                    new_results.push(token);
                                }
                            }
                        }
                    }
                    JPathTokenType::ArrayIndex => {
                        if let Some(index) = token.index {
                            if let JToken::Array(arr) = current {
                                if let Some(Some(token)) = arr.get(index) {
                                    new_results.push(token);
                                }
                            }
                        }
                    }
                    JPathTokenType::Wildcard => match current {
                        JToken::Array(arr) => {
                            for token in arr.iter().flatten() {
                                new_results.push(token);
                            }
                        }
                        JToken::Object(obj) => {
                            for token in obj.values().flatten() {
                                new_results.push(token);
                            }
                        }
                        _ => {}
                    },
                    JPathTokenType::ArraySlice => {
                        if let JToken::Array(arr) = current {
                            let start = token.start.unwrap_or(0);
                            let end = token.end.unwrap_or(arr.len());
                            for i in start..end.min(arr.len()) {
                                if let Some(Some(token)) = arr.get(i) {
                                    new_results.push(token);
                                }
                            }
                        }
                    }
                    JPathTokenType::RecursiveDescent => {
                        // Recursively find all matching nodes
                        Self::recursive_search(current, &mut new_results);
                    }
                    _ => {}
                }
            }

            results = new_results;
        }

        Ok(results)
    }

    /// Recursively searches for all nodes
    fn recursive_search<'a>(node: &'a JToken, results: &mut Vec<&'a JToken>) {
        results.push(node);

        match node {
            JToken::Array(arr) => {
                for token in arr.iter().flatten() {
                    Self::recursive_search(token, results);
                }
            }
            JToken::Object(obj) => {
                for token in obj.values().flatten() {
                    Self::recursive_search(token, results);
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;
    use crate::OrderedDictionary;

    #[test]
    fn test_jpath_token_creation() {
        let root = JPathToken::root();
        assert_eq!(root.token_type, JPathTokenType::Root);

        let prop = JPathToken::property("name".to_string());
        assert_eq!(prop.token_type, JPathTokenType::Property);
        assert_eq!(prop.value, Some("name".to_string()));

        let index = JPathToken::array_index(5);
        assert_eq!(index.token_type, JPathTokenType::ArrayIndex);
        assert_eq!(index.index, Some(5));
    }

    #[test]
    fn test_jpath_parse_simple() {
        let tokens = JPathToken::parse("$.name").unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].token_type, JPathTokenType::Root);
        assert_eq!(tokens[1].token_type, JPathTokenType::Property);
        assert_eq!(tokens[1].value, Some("name".to_string()));
    }

    #[test]
    fn test_jpath_parse_array() {
        let tokens = JPathToken::parse("$.items[0]").unwrap();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[1].token_type, JPathTokenType::Property);
        assert_eq!(tokens[2].token_type, JPathTokenType::ArrayIndex);
        assert_eq!(tokens[2].index, Some(0));
    }

    #[test]
    fn test_jpath_parse_wildcard() {
        let tokens = JPathToken::parse("$.items[*]").unwrap();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[2].token_type, JPathTokenType::Wildcard);
    }

    #[test]
    fn test_jpath_parse_slice() {
        let tokens = JPathToken::parse("$.items[1:3]").unwrap();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[2].token_type, JPathTokenType::ArraySlice);
        assert_eq!(tokens[2].start, Some(1));
        assert_eq!(tokens[2].end, Some(3));
    }

    #[test]
    fn test_jpath_evaluate_property() {
        let mut obj = OrderedDictionary::new();
        obj.insert("name".to_string(), Some(JToken::String("test".to_string())));
        let root = JToken::Object(obj);

        let tokens = JPathToken::parse("$.name").unwrap();
        let results = JPathToken::evaluate(&tokens, &root).unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0], &JToken::String("test".to_string()));
    }

    #[test]
    fn test_jpath_evaluate_array() {
        let arr = vec![
            Some(JToken::String("first".to_string())),
            Some(JToken::String("second".to_string())),
        ];
        let root = JToken::Array(arr);

        let tokens = JPathToken::parse("$[0]").unwrap();
        let results = JPathToken::evaluate(&tokens, &root).unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0], &JToken::String("first".to_string()));
    }

    #[test]
    fn test_jpath_invalid() {
        let result = JPathToken::parse("invalid");
        assert!(result.is_err());
    }
}
