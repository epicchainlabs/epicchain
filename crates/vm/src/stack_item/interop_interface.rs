//! InteropInterface stack item implementation for the Neo Virtual Machine.
//!
//! This module provides the InteropInterface stack item implementation used in the Neo VM.

use super::stack_item::InteropInterface;
use crate::error::{VmError, VmResult};
use crate::stack_item::stack_item_type::StackItemType;
use std::sync::Arc;

/// Represents an interop interface in the VM.
#[derive(Debug, Clone)]
pub struct InteropInterfaceItem {
    /// The wrapped interop interface.
    interface: Arc<dyn InteropInterface>,
}

impl InteropInterfaceItem {
    /// Creates a new interop interface with the specified interface.
    pub fn new<T: InteropInterface + 'static>(interface: T) -> Self {
        Self {
            interface: Arc::new(interface),
        }
    }

    /// Gets the wrapped interop interface.
    pub fn interface(&self) -> &Arc<dyn InteropInterface> {
        &self.interface
    }

    /// Gets the type of the stack item.
    pub fn stack_item_type(&self) -> StackItemType {
        StackItemType::InteropInterface
    }

    /// Gets the interface type.
    pub fn interface_type(&self) -> &str {
        self.interface.interface_type()
    }

    /// Attempts to downcast the interface to the specified type.
    /// Production implementation with proper type downcasting for C# compatibility.
    pub fn downcast<T: InteropInterface + 'static>(&self) -> VmResult<&T> {
        // Use Any trait for runtime type checking (matches C# reflection pattern)
        let interface_any = self.interface.as_any();

        // Attempt to downcast to the requested type
        interface_any.downcast_ref::<T>().ok_or_else(|| {
            VmError::invalid_type_simple(&format!(
                "Cannot cast InteropInterface to type {}",
                std::any::type_name::<T>()
            ))
        })
    }

    /// Converts the interop interface to a boolean.
    pub fn to_boolean(&self) -> bool {
        true
    }

    /// Creates a deep copy of the interop interface.
    pub fn deep_copy(&self) -> Self {
        Self {
            interface: self.interface.clone(),
        }
    }
}

impl PartialEq for InteropInterfaceItem {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.interface, &other.interface)
    }
}

impl Eq for InteropInterfaceItem {}

/// A simple implementation of InteropInterface for testing.
#[derive(Debug, Clone)]
pub struct TestInteropInterface {
    /// The interface type.
    interface_type: String,
    /// Some test data.
    data: String,
}

impl TestInteropInterface {
    /// Creates a new test interop interface.
    pub fn new(interface_type: &str, data: &str) -> Self {
        Self {
            interface_type: interface_type.to_string(),
            data: data.to_string(),
        }
    }

    /// Gets the test data.
    pub fn data(&self) -> &str {
        &self.data
    }
}

impl InteropInterface for TestInteropInterface {
    fn interface_type(&self) -> &str {
        &self.interface_type
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    #[test]
    fn test_interop_interface_creation() {
        let test_interface = TestInteropInterface::new("TestInterface", "test data");
        let interop = InteropInterfaceItem::new(test_interface);

        assert_eq!(interop.interface_type(), "TestInterface");
        assert_eq!(interop.stack_item_type(), StackItemType::InteropInterface);
    }

    #[test]
    fn test_interop_interface_to_boolean() {
        let test_interface = TestInteropInterface::new("TestInterface", "test data");
        let interop = InteropInterfaceItem::new(test_interface);

        assert_eq!(interop.to_boolean(), true);
    }

    #[test]
    fn test_interop_interface_deep_copy() {
        let test_interface = TestInteropInterface::new("TestInterface", "test data");
        let interop = InteropInterfaceItem::new(test_interface);
        let copied = interop.deep_copy();

        assert_eq!(copied.interface_type(), interop.interface_type());
    }

    #[test]
    fn test_interop_interface_equality() {
        let test_interface1 = TestInteropInterface::new("TestInterface", "test data");
        let interop1 = InteropInterfaceItem::new(test_interface1);

        let test_interface2 = TestInteropInterface::new("TestInterface", "test data");
        let interop2 = InteropInterfaceItem::new(test_interface2);

        // Different instances should not be equal
        assert_ne!(interop1, interop2);

        // Same instance should be equal
        let interop3 = interop1.clone();
        assert_eq!(interop1, interop3);
    }
}
