//! Implementation of the SharedStates structure for execution contexts.
//!
//! This module provides the SharedStates structure, which is used to share state across
//! multiple execution contexts. This matches the C# implementation's approach.

use crate::evaluation_stack::EvaluationStack;
use crate::script::Script;
use crate::reference_counter::ReferenceCounter;

/// Represents shared states across multiple contexts.
/// This matches the C# implementation's SharedStates class exactly.
#[derive(Clone)]
pub struct SharedStates {
    /// The script being executed
    pub(crate) script: Script,

    /// The evaluation stack for this context
    pub(crate) evaluation_stack: EvaluationStack,

    /// The static fields for this context
    pub(crate) static_fields: Option<crate::execution_context::Slot>,

    /// Reference counter for managing object lifetimes
    pub(crate) reference_counter: ReferenceCounter,

    /// Additional states
    pub(crate) states: std::collections::HashMap<std::any::TypeId, Box<dyn std::any::Any>>,
}

impl SharedStates {
    /// Creates a new SharedStates instance.
    pub fn new(script: Script, reference_counter: ReferenceCounter) -> Self {
        Self {
            script,
            evaluation_stack: EvaluationStack::new(reference_counter.clone()),
            static_fields: None,
            reference_counter,
            states: std::collections::HashMap::new(),
        }
    }

    /// Gets the script.
    pub fn script(&self) -> &Script {
        &self.script
    }

    /// Gets the evaluation stack.
    pub fn evaluation_stack(&self) -> &EvaluationStack {
        &self.evaluation_stack
    }

    /// Gets the evaluation stack (mutable).
    pub fn evaluation_stack_mut(&mut self) -> &mut EvaluationStack {
        &mut self.evaluation_stack
    }

    /// Gets the static fields.
    pub fn static_fields(&self) -> Option<&crate::execution_context::Slot> {
        self.static_fields.as_ref()
    }

    /// Gets the static fields (mutable).
    pub fn static_fields_mut(&mut self) -> &mut Option<crate::execution_context::Slot> {
        &mut self.static_fields
    }

    /// Sets the static fields.
    /// This matches the C# implementation's StaticFields property setter.
    pub fn set_static_fields(&mut self, static_fields: Option<crate::execution_context::Slot>) {
        self.static_fields = static_fields;
    }

    /// Gets the reference counter.
    pub fn reference_counter(&self) -> &ReferenceCounter {
        &self.reference_counter
    }

    /// Gets a shared state by type, creating it if it doesn't exist.
    /// This matches the C# implementation's GetState<T> method.
    pub fn get_state<T: 'static + Default>(&mut self) -> &mut T {
        let type_id = std::any::TypeId::of::<T>();

        if !self.states.contains_key(&type_id) {
            self.states.insert(type_id, Box::new(T::default()));
        }

        self.states
            .get_mut(&type_id)
            .and_then(|value| value.downcast_mut::<T>())
            .unwrap_or_else(|| {
                self.states.insert(type_id, Box::new(T::default()));
                self.states
                    .get_mut(&type_id)
                    .and_then(|value| value.downcast_mut::<T>())
                    .expect("Operation failed")
            })
    }

    /// Gets a shared state by type, creating it with the provided factory if it doesn't exist.
    /// This matches the C# implementation's GetState<T>(Func<T>) method.
    pub fn get_state_with_factory<T: 'static, F: FnOnce() -> T>(&mut self, factory: F) -> &mut T {
        let type_id = std::any::TypeId::of::<T>();

        if !self.states.contains_key(&type_id) {
            self.states.insert(type_id, Box::new(factory()));
        }

        self.states
            .get_mut(&type_id)
            .and_then(|value| value.downcast_mut::<T>())
            .unwrap_or_else(|| {
                self.states.insert(type_id, Box::new(factory()));
                self.states
                    .get_mut(&type_id)
                    .and_then(|value| value.downcast_mut::<T>())
                    .expect("Operation failed")
            })
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;
    use crate::script::Script;
    use crate::stack_item::reference_counter::ReferenceCounter;

    #[test]
    fn test_shared_states_creation() {
        let script = Script::new(vec![0x00, 0x01, 0x02], false).unwrap();
        let reference_counter = ReferenceCounter::new();

        let shared_states = SharedStates::new(script.clone(), reference_counter);

        assert_eq!(shared_states.script().bytes(), script.bytes());
        assert!(shared_states.evaluation_stack().is_empty());
        assert!(shared_states.static_fields().is_none());
    }
}
