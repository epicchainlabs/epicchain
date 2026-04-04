//! Integration tests for the Neo VM reference counter.

use neo_vm::reference_counter::ReferenceCounter;
use neo_vm::stack_item::StackItem;
use std::collections::BTreeMap;

#[test]
fn test_reference_counter_creation() {
    let counter = ReferenceCounter::new();

    assert_eq!(counter.count(), 0);
}

#[test]
fn test_reference_counter_add_reference() {
    let mut counter = ReferenceCounter::new();

    // Add a reference
    let id = counter.add_reference();

    assert_eq!(counter.count(), 1);
    assert_eq!(counter.get_reference_count(id), 1);

    // Add another reference to the same item
    counter.add_reference_to(id);

    assert_eq!(counter.count(), 2);
    assert_eq!(counter.get_reference_count(id), 2);
}

#[test]
fn test_reference_counter_remove_reference() {
    let mut counter = ReferenceCounter::new();

    // Add a reference
    let id = counter.add_reference();

    // Add another reference to the same item
    counter.add_reference_to(id);

    assert_eq!(counter.count(), 2);
    assert_eq!(counter.get_reference_count(id), 2);

    // Remove a reference
    counter.remove_reference(id);

    assert_eq!(counter.count(), 1);
    assert_eq!(counter.get_reference_count(id), 1);

    // Remove another reference
    counter.remove_reference(id);

    assert_eq!(counter.count(), 0);
    assert_eq!(counter.get_reference_count(id), 0);
}

#[test]
fn test_reference_counter_check_zero_referred() {
    let mut counter = ReferenceCounter::new();

    // Add a reference
    let id = counter.add_reference();

    // Add another reference to the same item
    counter.add_reference_to(id);

    assert_eq!(counter.count(), 2);

    // Remove a reference
    counter.remove_reference(id);

    assert_eq!(counter.count(), 1);

    // Remove another reference
    counter.remove_reference(id);

    assert_eq!(counter.count(), 0);

    // Check zero referred
    let count = counter.check_zero_referred();

    assert_eq!(count, 0);
}

#[test]
fn test_reference_counter_with_stack_items() {
    let mut counter = ReferenceCounter::new();

    // Create stack items
    let item1 = StackItem::from_int(1);
    let item2 = StackItem::from_int(2);
    let item3 = StackItem::from_int(3);

    // Add references
    let id1 = counter.add_reference();
    let id2 = counter.add_reference();
    let id3 = counter.add_reference();

    assert_eq!(counter.count(), 3);

    // Remove references
    counter.remove_reference(id1);
    counter.remove_reference(id2);
    counter.remove_reference(id3);

    assert_eq!(counter.count(), 0);
}

#[test]
fn test_reference_counter_with_circular_references() {
    let counter = ReferenceCounter::new();

    // Register objects
    let id1 = counter.register();
    let id2 = counter.register();

    // Add references
    counter.add_reference_to(id1);
    counter.add_reference_to(id2);

    counter.add_reference_to(id1);
    counter.add_reference_to(id2);

    assert_eq!(counter.count(), 4);

    // Remove references
    let _zero_ref1 = counter.remove_reference(id1);
    let _zero_ref2 = counter.remove_reference(id2);

    assert_eq!(counter.count(), 2);

    // Check zero referred
    let count = counter.check_zero_referred();

    // The remaining references should still be counted
    assert_eq!(count, 2);
}

#[test]
fn test_reference_counter_with_complex_references() {
    let counter = ReferenceCounter::new();

    // Create a complex reference structure
    let id1 = counter.register();
    let id2 = counter.register();
    let id3 = counter.register();
    let id4 = counter.register();

    // Add references
    counter.add_reference(id1);
    counter.add_reference(id2);
    counter.add_reference(id3);
    counter.add_reference(id4);

    // Add more references
    counter.add_reference(id1);
    counter.add_reference(id2);
    counter.add_reference(id3);
    counter.add_reference(id4);

    // Create additional references
    counter.add_reference(id1);
    counter.add_reference(id2);

    assert_eq!(counter.count(), 10);

    // Remove references
    let _zero_ref1 = counter.remove_reference(id1);
    let _zero_ref2 = counter.remove_reference(id2);
    let _zero_ref3 = counter.remove_reference(id3);
    let _zero_ref4 = counter.remove_reference(id4);

    assert_eq!(counter.count(), 6);

    // Check zero referred
    let count = counter.check_zero_referred();

    // The remaining references should still be counted
    assert_eq!(count, 6);
}
