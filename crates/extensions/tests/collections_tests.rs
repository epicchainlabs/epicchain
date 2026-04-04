//! Extensions Collections C# Compatibility Tests
//!
//! These tests ensure full compatibility with C# Neo.Extensions collection utilities.
//! Tests are based on the C# Neo.Extensions.Collections test suite.

use neo_extensions::collections::*;
use std::collections::{HashMap, HashSet};

#[cfg(test)]
#[allow(dead_code)]
mod collections_tests {
    use super::*;

    /// Test Vec extensions compatibility (matches C# IEnumerable extensions exactly)
    #[test]
    fn test_vec_dedup_preserve_order_compatibility() {
        let mut vec = vec![1, 2, 2, 3, 1, 4, 3, 5];
        vec.dedup_preserve_order();
        assert_eq!(vec, vec![1, 2, 3, 4, 5]);

        let mut string_vec = vec!["apple", "banana", "apple", "cherry", "banana"];
        string_vec.dedup_preserve_order();
        assert_eq!(string_vec, vec!["apple", "banana", "cherry"]);

        // Test empty vector
        let mut empty_vec: Vec<i32> = vec![];
        empty_vec.dedup_preserve_order();
        assert_eq!(empty_vec, vec![] as Vec<i32>);

        // Test single element
        let mut single_vec = vec![42];
        single_vec.dedup_preserve_order();
        assert_eq!(single_vec, vec![42]);

        // Test already deduplicated
        let mut clean_vec = vec![1, 2, 3, 4, 5];
        clean_vec.dedup_preserve_order();
        assert_eq!(clean_vec, vec![1, 2, 3, 4, 5]);
    }

    /// Test Vec chunking compatibility (matches C# Chunk() method exactly)
    #[test]
    fn test_vec_chunks_exact_compatibility() {
        let vec = vec![1, 2, 3, 4, 5, 6];
        let chunks = vec.chunks_exact_vec(2);
        assert_eq!(chunks, vec![vec![1, 2], vec![3, 4], vec![5, 6]]);

        let vec = vec![1, 2, 3, 4, 5];
        let chunks = vec.chunks_exact_vec(2);
        assert_eq!(chunks, vec![vec![1, 2], vec![3, 4], vec![5]]);

        // Test chunk size larger than vector
        let vec = vec![1, 2, 3];
        let chunks = vec.chunks_exact_vec(5);
        assert_eq!(chunks, vec![vec![1, 2, 3]]);

        let vec = vec![1, 2, 3, 4];
        let chunks = vec.chunks_exact_vec(0);
        assert_eq!(chunks, Vec::<Vec<i32>>::new());

        // Test empty vector
        let vec: Vec<i32> = vec![];
        let chunks = vec.chunks_exact_vec(3);
        assert_eq!(chunks, Vec::<Vec<i32>>::new());
    }

    /// Test Vec argmax/argmin compatibility (matches C# LINQ extensions exactly)
    #[test]
    fn test_vec_argmax_argmin_compatibility() {
        let vec = vec![3, 1, 4, 1, 5, 9, 2, 6];
        assert_eq!(vec.argmax(), Some(5)); // Index of 9

        assert_eq!(vec.argmin(), Some(1)); // Index of first 1

        // Test with floats
        let float_vec = vec![3.14, 2.71, 1.41, 4.47, 2.23];
        assert_eq!(float_vec.argmax(), Some(3)); // Index of 4.47
        assert_eq!(float_vec.argmin(), Some(2)); // Index of 1.41

        // Test empty vector
        let empty_vec: Vec<i32> = vec![];
        assert_eq!(empty_vec.argmax(), None);
        assert_eq!(empty_vec.argmin(), None);

        // Test single element
        let single_vec = vec![42];
        assert_eq!(single_vec.argmax(), Some(0));
        assert_eq!(single_vec.argmin(), Some(0));

        let equal_vec = vec![5, 5, 5, 5];
        assert_eq!(equal_vec.argmax(), Some(0));
        assert_eq!(equal_vec.argmin(), Some(0));
    }

    /// Test Vec sorting check compatibility (matches C# IsSorted extension exactly)
    #[test]
    fn test_vec_is_sorted_compatibility() {
        let sorted_vec = vec![1, 2, 3, 4, 5];
        assert!(sorted_vec.is_sorted());

        // Test unsorted
        let unsorted_vec = vec![1, 3, 2, 4, 5];
        assert!(!unsorted_vec.is_sorted());

        let reverse_vec = vec![5, 4, 3, 2, 1];
        assert!(!reverse_vec.is_sorted());

        let empty_vec: Vec<i32> = vec![];
        assert!(empty_vec.is_sorted());

        let single_vec = vec![42];
        assert!(single_vec.is_sorted());

        let dup_vec = vec![1, 2, 2, 3, 3, 3, 4];
        assert!(dup_vec.is_sorted());

        // Test with strings
        let string_vec = vec!["apple", "banana", "cherry"];
        assert!(string_vec.is_sorted());

        let unsorted_strings = vec!["banana", "apple", "cherry"];
        assert!(!unsorted_strings.is_sorted());
    }

    /// Test Vec first_n/last_n compatibility (matches C# Take/TakeLast exactly)
    #[test]
    fn test_vec_first_last_n_compatibility() {
        let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        assert_eq!(vec.first_n(3), &[1, 2, 3]);
        assert_eq!(vec.first_n(0), &[] as &[i32]);
        assert_eq!(vec.first_n(15), &vec[..]); // Request more than available

        assert_eq!(vec.last_n(3), &[8, 9, 10]);
        assert_eq!(vec.last_n(0), &[] as &[i32]);
        assert_eq!(vec.last_n(15), &vec[..]); // Request more than available

        // Test with empty vector
        let empty_vec: Vec<i32> = vec![];
        assert_eq!(empty_vec.first_n(3), &[] as &[i32]);
        assert_eq!(empty_vec.last_n(3), &[] as &[i32]);

        // Test with single element
        let single_vec = vec![42];
        assert_eq!(single_vec.first_n(1), &[42]);
        assert_eq!(single_vec.last_n(1), &[42]);
        assert_eq!(single_vec.first_n(0), &[] as &[i32]);
        assert_eq!(single_vec.last_n(0), &[] as &[i32]);
    }

    /// Test HashMap extensions compatibility (matches C# Dictionary extensions exactly)
    #[test]
    fn test_hashmap_get_or_insert_default_compatibility() {
        let mut map: HashMap<String, i32> = HashMap::new();

        let value = map.get_or_insert_default("key1".to_string());
        *value = 42;
        assert_eq!(map.get("key1"), Some(&42));

        // Test accessing existing key
        let existing_value = map.get_or_insert_default("key1".to_string());
        assert_eq!(*existing_value, 42);

        let DEFAULT_VALUE = map.get_or_insert_default("key2".to_string());
        assert_eq!(*DEFAULT_VALUE, 0); // Default for i32

        // Test with custom types that implement Default
        let mut string_map: HashMap<i32, String> = HashMap::new();
        let default_string = string_map.get_or_insert_default(1);
        assert_eq!(default_string, ""); // Default for String

        *default_string = "Hello".to_string();
        assert_eq!(string_map.get(&1), Some(&"Hello".to_string()));
    }

    /// Test HashMap merge compatibility (matches C# Dictionary merge patterns exactly)
    #[test]
    fn test_hashmap_merge_compatibility() {
        let mut map1 = HashMap::new();
        map1.insert("a", 1);
        map1.insert("b", 2);

        let mut map2 = HashMap::new();
        map2.insert("c", 3);
        map2.insert("d", 4);
        map2.insert("a", 10); // Should overwrite existing key

        map1.merge(map2);

        assert_eq!(map1.get("a"), Some(&10)); // Overwritten value
        assert_eq!(map1.get("b"), Some(&2));
        assert_eq!(map1.get("c"), Some(&3));
        assert_eq!(map1.get("d"), Some(&4));
        assert_eq!(map1.len(), 4);

        // Test merging empty map
        let empty_map = HashMap::new();
        let original_len = map1.len();
        map1.merge(empty_map);
        assert_eq!(map1.len(), original_len);

        // Test merging into empty map
        let mut empty_target = HashMap::new();
        let mut source = HashMap::new();
        source.insert("x", 100);
        source.insert("y", 200);

        empty_target.merge(source);
        assert_eq!(empty_target.len(), 2);
        assert_eq!(empty_target.get("x"), Some(&100));
        assert_eq!(empty_target.get("y"), Some(&200));
    }

    /// Test HashMap get_many compatibility (matches C# batch lookup patterns exactly)
    #[test]
    fn test_hashmap_get_many_compatibility() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 2);
        map.insert("c", 3);

        let keys = vec!["a", "b", "x", "c", "y"];
        let values = map.get_many(&keys);

        assert_eq!(values.len(), 5);
        assert_eq!(values[0], Some(&1));
        assert_eq!(values[1], Some(&2));
        assert_eq!(values[2], None);
        assert_eq!(values[3], Some(&3));
        assert_eq!(values[4], None);

        // Test with empty keys
        let empty_keys: Vec<&str> = vec![];
        let empty_values = map.get_many(&empty_keys);
        assert_eq!(empty_values.len(), 0);

        // Test with all non-existent keys
        let missing_keys = vec!["x", "y", "z"];
        let missing_values = map.get_many(&missing_keys);
        assert_eq!(missing_values, vec![None, None, None]);

        // Test with all existing keys
        let existing_keys = vec!["a", "b", "c"];
        let existing_values = map.get_many(&existing_keys);
        assert_eq!(existing_values, vec![Some(&1), Some(&2), Some(&3)]);
    }

    /// Test HashMap filter_values compatibility (matches C# Where/ToDictionary exactly)
    #[test]
    fn test_hashmap_filter_values_compatibility() {
        let mut map = HashMap::new();
        map.insert("one", 1);
        map.insert("two", 2);
        map.insert("three", 3);
        map.insert("four", 4);
        map.insert("five", 5);

        let even_values = map.filter_values(|&v| v % 2 == 0);
        assert_eq!(even_values.len(), 2);
        assert_eq!(even_values.get("two"), Some(&2));
        assert_eq!(even_values.get("four"), Some(&4));

        // Test filter with no matches
        let large_values = map.filter_values(|&v| v > 10);
        assert_eq!(large_values.len(), 0);

        // Test filter with all matches
        let all_values = map.filter_values(|&v| v > 0);
        assert_eq!(all_values.len(), 5);

        let mut string_map = HashMap::new();
        string_map.insert(1, "hello".to_string());
        string_map.insert(2, "world".to_string());
        string_map.insert(3, "hi".to_string());
        string_map.insert(4, "universe".to_string());

        let long_strings = string_map.filter_values(|s| s.len() > 4);
        assert_eq!(long_strings.len(), 2);
        assert_eq!(long_strings.get(&1), Some(&"hello".to_string()));
        assert_eq!(long_strings.get(&4), Some(&"universe".to_string()));
    }

    /// Test HashSet extensions compatibility (matches C# HashSet operations exactly)
    #[test]
    fn test_hashset_subset_superset_compatibility() {
        let set1: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
        let set2: HashSet<i32> = [1, 2, 3, 4, 5].iter().cloned().collect();
        let set3: HashSet<i32> = [2, 3].iter().cloned().collect();
        let set4: HashSet<i32> = [6, 7, 8].iter().cloned().collect();

        assert!(set3.is_subset_of(&set1)); // {2,3} ⊆ {1,2,3}
        assert!(set1.is_subset_of(&set2)); // {1,2,3} ⊆ {1,2,3,4,5}
        assert!(!set1.is_subset_of(&set3)); // {1,2,3} ⊄ {2,3}
        assert!(!set1.is_subset_of(&set4)); // {1,2,3} ⊄ {6,7,8}

        assert!(set1.is_superset_of(&set3)); // {1,2,3} ⊇ {2,3}
        assert!(set2.is_superset_of(&set1)); // {1,2,3,4,5} ⊇ {1,2,3}
        assert!(!set3.is_superset_of(&set1)); // {2,3} ⊅ {1,2,3}
        assert!(!set1.is_superset_of(&set4)); // {1,2,3} ⊅ {6,7,8}

        // Test with empty sets
        let empty_set: HashSet<i32> = HashSet::new();
        assert!(empty_set.is_subset_of(&set1));
        assert!(set1.is_superset_of(&empty_set));
        assert!(empty_set.is_subset_of(&empty_set));
        assert!(empty_set.is_superset_of(&empty_set));

        // Test with identical sets
        let set1_copy = set1.clone();
        assert!(set1.is_subset_of(&set1_copy));
        assert!(set1.is_superset_of(&set1_copy));
    }

    /// Test HashSet set operations compatibility (matches C# HashSet operations exactly)
    #[test]
    fn test_hashset_set_operations_compatibility() {
        let set1: HashSet<i32> = [1, 2, 3, 4].iter().cloned().collect();
        let set2: HashSet<i32> = [3, 4, 5, 6].iter().cloned().collect();

        let intersection = set1.intersection_with(&set2);
        let expected_intersection: HashSet<i32> = [3, 4].iter().cloned().collect();
        assert_eq!(intersection, expected_intersection);

        let union = set1.union_with(&set2);
        let expected_union: HashSet<i32> = [1, 2, 3, 4, 5, 6].iter().cloned().collect();
        assert_eq!(union, expected_union);

        let difference = set1.difference_with(&set2);
        let expected_difference: HashSet<i32> = [1, 2].iter().cloned().collect();
        assert_eq!(difference, expected_difference);

        // Test with empty sets
        let empty_set: HashSet<i32> = HashSet::new();

        let intersection_empty = set1.intersection_with(&empty_set);
        assert_eq!(intersection_empty, empty_set);

        let union_empty = set1.union_with(&empty_set);
        assert_eq!(union_empty, set1);

        let difference_empty = set1.difference_with(&empty_set);
        assert_eq!(difference_empty, set1);

        let reverse_difference = set2.difference_with(&set1);
        let expected_reverse: HashSet<i32> = [5, 6].iter().cloned().collect();
        assert_eq!(reverse_difference, expected_reverse);

        // Test with disjoint sets
        let set3: HashSet<i32> = [7, 8, 9].iter().cloned().collect();

        let disjoint_intersection = set1.intersection_with(&set3);
        assert_eq!(disjoint_intersection, empty_set);

        let disjoint_union = set1.union_with(&set3);
        let expected_disjoint_union: HashSet<i32> = [1, 2, 3, 4, 7, 8, 9].iter().cloned().collect();
        assert_eq!(disjoint_union, expected_disjoint_union);

        let disjoint_difference = set1.difference_with(&set3);
        assert_eq!(disjoint_difference, set1);
    }

    /// Test Collections utility functions compatibility (matches C# LINQ helpers exactly)
    #[test]
    fn test_collections_hashmap_from_pairs_compatibility() {
        let pairs = vec![("apple", 5), ("banana", 3), ("cherry", 8), ("date", 2)];

        let map = Collections::hashmap_from_pairs(pairs);
        assert_eq!(map.len(), 4);
        assert_eq!(map.get("apple"), Some(&5));
        assert_eq!(map.get("banana"), Some(&3));
        assert_eq!(map.get("cherry"), Some(&8));
        assert_eq!(map.get("date"), Some(&2));

        // Test with empty pairs
        let empty_pairs: Vec<(String, i32)> = vec![];
        let empty_map = Collections::hashmap_from_pairs(empty_pairs);
        assert_eq!(empty_map.len(), 0);

        let duplicate_pairs = vec![("key", 1), ("key", 2), ("key", 3)];
        let dup_map = Collections::hashmap_from_pairs(duplicate_pairs);
        assert_eq!(dup_map.len(), 1);
        assert_eq!(dup_map.get("key"), Some(&3));
    }

    /// Test Collections hashset creation compatibility (matches C# HashSet constructor exactly)
    #[test]
    fn test_collections_hashset_from_values_compatibility() {
        let values = vec![1, 2, 3, 2, 4, 1, 5];
        let set = Collections::hashset_from_values(values);

        assert_eq!(set.len(), 5);
        assert!(set.contains(&1));
        assert!(set.contains(&2));
        assert!(set.contains(&3));
        assert!(set.contains(&4));
        assert!(set.contains(&5));

        // Test with empty values
        let empty_values: Vec<i32> = vec![];
        let empty_set = Collections::hashset_from_values(empty_values);
        assert_eq!(empty_set.len(), 0);

        // Test with all unique values
        let unique_values = vec![10, 20, 30, 40];
        let unique_set = Collections::hashset_from_values(unique_values);
        assert_eq!(unique_set.len(), 4);

        let string_values = vec!["a".to_string(), "b".to_string(), "a".to_string()];
        let string_set = Collections::hashset_from_values(string_values);
        assert_eq!(string_set.len(), 2);
        assert!(string_set.contains("a"));
        assert!(string_set.contains("b"));
    }

    /// Test Collections group_by compatibility (matches C# GroupBy exactly)
    #[test]
    fn test_collections_group_by_compatibility() {
        let items = vec![
            ("apple", 5),
            ("banana", 6),
            ("apricot", 7),
            ("blueberry", 9),
            ("avocado", 7),
        ];

        // Group by first letter
        let groups = Collections::group_by(items, |(name, _)| name.chars().next().unwrap());

        assert_eq!(groups.len(), 2);
        assert_eq!(groups.get(&'a').unwrap().len(), 3);
        assert_eq!(groups.get(&'b').unwrap().len(), 2);

        // Verify specific groupings
        let a_group = groups.get(&'a').unwrap();
        assert!(a_group.contains(&("apple", 5)));
        assert!(a_group.contains(&("apricot", 7)));
        assert!(a_group.contains(&("avocado", 7)));

        let b_group = groups.get(&'b').unwrap();
        assert!(b_group.contains(&("banana", 6)));
        assert!(b_group.contains(&("blueberry", 9)));

        // Test with numbers - group by even/odd
        let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let number_groups = Collections::group_by(numbers, |&n| n % 2);

        assert_eq!(number_groups.len(), 2);
        assert_eq!(number_groups.get(&0).unwrap().len(), 5); // Even numbers
        assert_eq!(number_groups.get(&1).unwrap().len(), 5); // Odd numbers

        // Test with empty input
        let empty_items: Vec<i32> = vec![];
        let empty_groups = Collections::group_by(empty_items, |&x| x % 2);
        assert_eq!(empty_groups.len(), 0);
    }

    /// Test Collections count_occurrences compatibility (matches C# GroupBy/Count exactly)
    #[test]
    fn test_collections_count_occurrences_compatibility() {
        let items = vec![1, 2, 3, 2, 1, 4, 2, 1, 3, 5];
        let counts = Collections::count_occurrences(&items);

        assert_eq!(counts.len(), 5);
        assert_eq!(counts.get(&1), Some(&3));
        assert_eq!(counts.get(&2), Some(&3));
        assert_eq!(counts.get(&3), Some(&2));
        assert_eq!(counts.get(&4), Some(&1));
        assert_eq!(counts.get(&5), Some(&1));

        // Test with strings
        let words = vec!["apple", "banana", "apple", "cherry", "banana", "apple"];
        let word_counts = Collections::count_occurrences(&words);

        assert_eq!(word_counts.len(), 3);
        assert_eq!(word_counts.get(&"apple"), Some(&3));
        assert_eq!(word_counts.get(&"banana"), Some(&2));
        assert_eq!(word_counts.get(&"cherry"), Some(&1));

        // Test with empty slice
        let empty_items: Vec<i32> = vec![];
        let empty_counts = Collections::count_occurrences(&empty_items);
        assert_eq!(empty_counts.len(), 0);

        // Test with single element
        let single_item = vec!["unique"];
        let single_counts = Collections::count_occurrences(&single_item);
        assert_eq!(single_counts.len(), 1);
        assert_eq!(single_counts.get(&"unique"), Some(&1));
    }

    /// Test Collections most_common compatibility (matches C# MaxBy(Count) exactly)
    #[test]
    fn test_collections_most_common_compatibility() {
        let items = vec![1, 2, 3, 2, 1, 4, 2, 1, 3, 2];
        let most_common = Collections::most_common(&items);
        assert_eq!(most_common, Some(2)); // Appears 4 times

        let tied_items = vec![1, 1, 2, 2, 3];
        let tied_common = Collections::most_common(&tied_items);
        assert!(tied_common == Some(1) || tied_common == Some(2)); // Both appear twice

        // Test with empty slice
        let empty_items: Vec<i32> = vec![];
        let empty_common = Collections::most_common(&empty_items);
        assert_eq!(empty_common, None);

        // Test with single element
        let single_item = vec![42];
        let single_common = Collections::most_common(&single_item);
        assert_eq!(single_common, Some(42));

        // Test with all unique elements
        let unique_items = vec![1, 2, 3, 4, 5];
        let unique_common = Collections::most_common(&unique_items);
        assert!(unique_common.is_some()); // One of the elements (implementation defined)

        // Test with strings
        let words = vec!["apple", "banana", "apple", "apple", "cherry"];
        let most_common_word = Collections::most_common(&words);
        assert_eq!(most_common_word, Some("apple"));
    }

    /// Test Collections partition compatibility (matches C# Where/Where(Not) exactly)
    #[test]
    fn test_collections_partition_compatibility() {
        let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let (evens, odds) = Collections::partition(numbers, |&x| x % 2 == 0);

        assert_eq!(evens, vec![2, 4, 6, 8, 10]);
        assert_eq!(odds, vec![1, 3, 5, 7, 9]);

        // Test with strings - partition by length
        let words = vec!["a", "hello", "hi", "world", "x", "universe"];
        let (short, long) = Collections::partition(words, |&s| s.len() <= 2);

        assert_eq!(short, vec!["a", "hi", "x"]);
        assert_eq!(long, vec!["hello", "world", "universe"]);

        // Test with all true predicate
        let items = vec![2, 4, 6, 8];
        let (all_match, none_match) = Collections::partition(items, |&x| x % 2 == 0);
        assert_eq!(all_match, vec![2, 4, 6, 8]);
        assert_eq!(none_match, Vec::<i32>::new());

        // Test with all false predicate
        let items = vec![1, 3, 5, 7];
        let (none_match, all_match) = Collections::partition(items, |&x| x % 2 == 0);
        assert_eq!(none_match, Vec::<i32>::new());
        assert_eq!(all_match, vec![1, 3, 5, 7]);

        // Test with empty input
        let empty_items: Vec<i32> = vec![];
        let (empty_true, empty_false) = Collections::partition(empty_items, |&x| x > 0);
        assert_eq!(empty_true, Vec::<i32>::new());
        assert_eq!(empty_false, Vec::<i32>::new());
    }

    /// Test Collections performance characteristics (matches C# performance exactly)
    #[test]
    fn test_collections_performance_compatibility() {
        // Test with large datasets to ensure performance is reasonable
        let large_size = 10000;

        // Test deduplication performance
        let mut large_vec: Vec<i32> = (0..large_size as i32)
            .cycle()
            .take(large_size * 2)
            .collect();
        let start = std::time::Instant::now();
        large_vec.dedup_preserve_order();
        let dedup_duration = start.elapsed();

        assert_eq!(large_vec.len(), large_size);
        assert!(dedup_duration.as_millis() < 1000); // Should be fast

        // Test group_by performance
        let large_items: Vec<i32> = (0..large_size as i32).collect();
        let start = std::time::Instant::now();
        let groups = Collections::group_by(large_items, |&x| x % 100);
        let group_duration = start.elapsed();

        assert_eq!(groups.len(), 100);
        assert!(group_duration.as_millis() < 1000); // Should be fast

        // Test count_occurrences performance
        let repeated_items: Vec<i32> = (0..100).cycle().take(large_size).collect();
        let start = std::time::Instant::now();
        let counts = Collections::count_occurrences(&repeated_items);
        let count_duration = start.elapsed();

        assert_eq!(counts.len(), 100);
        assert!(count_duration.as_millis() < 1000); // Should be fast
    }
}
