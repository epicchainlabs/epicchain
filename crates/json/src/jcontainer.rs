use crate::JToken;

/// Base trait for JSON containers (objects and arrays)
/// This matches the C# JContainer abstract class
pub trait JContainer {
    /// Clears all children from the container
    fn clear_container(&mut self);

    /// Gets the children of the container
    fn children(&self) -> Vec<Option<&JToken>>;

    /// Gets the number of children in the container
    fn count(&self) -> usize {
        self.children().len()
    }

    /// Checks if the container is empty
    fn is_empty_container(&self) -> bool {
        self.count() == 0
    }
}

impl JContainer for crate::JArray {
    fn clear_container(&mut self) {
        self.clear();
    }

    fn children(&self) -> Vec<Option<&JToken>> {
        self.iter().collect()
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::{JArray, JToken};

    #[test]
    fn test_jarray_container() {
        let mut array = JArray::new();
        array.add(Some(JToken::String("test".to_string())));
        array.add(Some(JToken::Number(42.0)));

        assert_eq!(array.len(), 2);
        assert!(!array.is_empty());

        // Test accessing elements
        assert_eq!(array.get(0), Some(&JToken::String("test".to_string())));
        assert_eq!(array.get(1), Some(&JToken::Number(42.0)));

        array.clear();
        assert!(array.is_empty());
        assert_eq!(array.len(), 0);
    }
}
