//! Exception handling for the Neo Virtual Machine.
//!
//! This module provides exception handling functionality for the Neo VM.

use crate::stack_item::StackItem;

/// Indicates the state of the ExceptionHandlingContext.
/// This matches the C# implementation's ExceptionHandlingState enum exactly.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ExceptionHandlingState {
    /// No exception handling state (initial state).
    None,

    /// Indicates that the try block is being executed.
    Try,

    /// Indicates that the catch block is being executed.
    Catch,

    /// Indicates that the finally block is being executed.
    Finally,
}

/// Represents an exception handling context in the VM.
/// This matches the C# implementation's ExceptionHandlingContext class exactly.
#[derive(Debug, Clone)]
pub struct ExceptionHandlingContext {
    /// The start position of the try block.
    pub try_start: usize,

    /// The end position of the try block.
    pub try_end: usize,

    /// The start position of the catch block.
    pub catch_start: usize,

    /// The start position of the finally block.
    pub finally_start: usize,

    /// The end position of the try-catch-finally block.
    pub end_offset: usize,

    /// The position of the catch block.
    /// This matches the C# implementation's CatchPointer property.
    pub catch_pointer: i32,

    /// The position of the finally block.
    /// This matches the C# implementation's FinallyPointer property.
    pub finally_pointer: i32,

    /// The end position of the try-catch-finally block.
    /// This matches the C# implementation's EndPointer property.
    pub end_pointer: i32,

    /// The current state of exception handling.
    /// This matches the C# implementation's State property.
    pub state: ExceptionHandlingState,

    /// The exception being handled (Rust-specific extension).
    /// This field doesn't exist in the C# implementation but is useful for our implementation.
    /// It's kept private to maintain API compatibility.
    exception: Option<StackItem>,
}

impl ExceptionHandlingContext {
    /// Creates a new exception handling context with try/catch/finally positions.
    /// This matches the expected API from the tests.
    pub fn new(
        try_start: usize,
        try_end: usize,
        catch_start: usize,
        finally_start: usize,
        end_offset: usize,
    ) -> Self {
        Self {
            try_start,
            try_end,
            catch_start,
            finally_start,
            end_offset,
            catch_pointer: catch_start as i32,
            finally_pointer: finally_start as i32,
            end_pointer: end_offset as i32,
            state: ExceptionHandlingState::None,
            exception: None,
        }
    }

    /// Creates a new exception handling context (C# compatibility constructor).
    /// This matches the C# implementation's constructor exactly.
    pub fn new_simple(catch_pointer: i32, finally_pointer: i32) -> Self {
        Self {
            try_start: 0,
            try_end: 0,
            catch_start: if catch_pointer >= 0 {
                catch_pointer as usize
            } else {
                0
            },
            finally_start: if finally_pointer >= 0 {
                finally_pointer as usize
            } else {
                0
            },
            end_offset: 0,
            catch_pointer,
            finally_pointer,
            end_pointer: -1,
            state: ExceptionHandlingState::Try,
            exception: None,
        }
    }

    /// Gets the try start position.
    pub fn try_start(&self) -> usize {
        self.try_start
    }

    /// Gets the try end position.
    pub fn try_end(&self) -> usize {
        self.try_end
    }

    /// Gets the catch start position.
    pub fn catch_start(&self) -> usize {
        self.catch_start
    }

    /// Gets the finally start position.
    pub fn finally_start(&self) -> usize {
        self.finally_start
    }

    /// Gets the end offset.
    pub fn end_offset(&self) -> usize {
        self.end_offset
    }

    /// Sets the end offset.
    pub fn set_end_offset(&mut self, end_offset: usize) {
        self.end_offset = end_offset;
        self.end_pointer = end_offset as i32;
    }

    /// Checks if a position is within the try block.
    pub fn is_within_try(&self, position: usize) -> bool {
        position >= self.try_start && position < self.try_end
    }

    /// Checks if a position is within the catch block.
    pub fn is_within_catch(&self, position: usize) -> bool {
        position >= self.catch_start && position < self.finally_start
    }

    /// Checks if a position is within the finally block.
    pub fn is_within_finally(&self, position: usize) -> bool {
        position >= self.finally_start && position < self.end_offset
    }

    /// Gets the next instruction pointer based on exception state.
    pub fn get_next_instruction_pointer(&self, has_exception: bool) -> Result<usize, &'static str> {
        match self.state {
            ExceptionHandlingState::None => {
                if has_exception {
                    if self.catch_pointer >= 0 {
                        Ok(self.catch_start)
                    } else if self.finally_pointer >= 0 {
                        Ok(self.finally_start)
                    } else {
                        Err("No exception handler available")
                    }
                } else if self.finally_pointer >= 0 {
                    Ok(self.finally_start)
                } else {
                    Ok(self.end_offset)
                }
            }
            ExceptionHandlingState::Try => {
                if has_exception {
                    if self.catch_pointer >= 0 {
                        Ok(self.catch_start)
                    } else if self.finally_pointer >= 0 {
                        Ok(self.finally_start)
                    } else {
                        Err("No exception handler available")
                    }
                } else if self.finally_pointer >= 0 {
                    Ok(self.finally_start)
                } else {
                    Ok(self.end_offset)
                }
            }
            ExceptionHandlingState::Catch => {
                if self.finally_pointer >= 0 {
                    Ok(self.finally_start)
                } else {
                    Ok(self.end_offset)
                }
            }
            ExceptionHandlingState::Finally => Ok(self.end_offset),
        }
    }

    /// Checks if the context has a catch block.
    /// This matches the C# implementation's HasCatch property.
    pub fn has_catch(&self) -> bool {
        self.catch_pointer >= 0
    }

    /// Checks if the context has a finally block.
    /// This matches the C# implementation's HasFinally property.
    pub fn has_finally(&self) -> bool {
        self.finally_pointer >= 0
    }

    /// Gets the current state of exception handling.
    /// This is a Rust-specific helper method not in the C# implementation.
    pub fn state(&self) -> ExceptionHandlingState {
        self.state
    }

    /// Gets the exception being handled.
    pub fn exception(&self) -> Option<&StackItem> {
        self.exception.as_ref()
    }

    /// Sets the exception being handled.
    pub fn set_exception(&mut self, exception: Option<StackItem>) {
        self.exception = exception;
    }

    /// Gets the catch pointer.
    /// This matches the C# implementation's CatchPointer property.
    pub fn catch_pointer(&self) -> i32 {
        self.catch_pointer
    }

    /// Gets the finally pointer.
    /// This matches the C# implementation's FinallyPointer property.
    pub fn finally_pointer(&self) -> i32 {
        self.finally_pointer
    }

    /// Gets the end pointer.
    /// This matches the C# implementation's EndPointer property.
    pub fn end_pointer(&self) -> i32 {
        self.end_pointer
    }

    /// Sets the end pointer.
    /// This matches the C# implementation's EndPointer property setter.
    pub fn set_end_pointer(&mut self, end_pointer: i32) {
        self.end_pointer = end_pointer;
        self.end_offset = if end_pointer >= 0 {
            end_pointer as usize
        } else {
            0
        };
    }

    /// Sets the state.
    /// This matches the C# implementation's State property setter.
    pub fn set_state(&mut self, state: ExceptionHandlingState) {
        self.state = state;
    }

    /// Checks if this exception handling context is currently in an exception state.
    /// This matches the C# implementation's exception state tracking.
    pub fn is_in_exception(&self) -> bool {
        // 1. The state is Catch (actively handling an exception)
        // 2. OR the state is Finally (executing finally block, possibly due to exception)
        // 3. OR there's an exception stored in this context
        matches!(
            self.state,
            ExceptionHandlingState::Catch | ExceptionHandlingState::Finally
        ) || self.exception.is_some()
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    #[test]
    fn test_exception_handling_context_creation() {
        let context = ExceptionHandlingContext::new_simple(30, 40);

        assert_eq!(context.catch_pointer(), 30);
        assert_eq!(context.finally_pointer(), 40);
        assert_eq!(context.end_pointer(), -1);
        assert_eq!(context.state(), ExceptionHandlingState::Try);
        assert!(context.exception().is_none());
    }

    #[test]
    fn test_exception_handling_context_full_creation() {
        use neo_config::ADDRESS_SIZE;
        let context = ExceptionHandlingContext::new(10, ADDRESS_SIZE, 30, 40, 50);

        assert_eq!(context.try_start(), 10);
        assert_eq!(context.try_end(), ADDRESS_SIZE);
        assert_eq!(context.catch_start(), 30);
        assert_eq!(context.finally_start(), 40);
        assert_eq!(context.end_offset(), 50);
        assert_eq!(context.state(), ExceptionHandlingState::None);
        assert!(context.exception().is_none());
    }

    #[test]
    fn test_exception_handling_context_state() {
        let mut context = ExceptionHandlingContext::new_simple(30, 40);

        context.set_state(ExceptionHandlingState::Catch);
        assert_eq!(context.state(), ExceptionHandlingState::Catch);

        context.set_state(ExceptionHandlingState::Finally);
        assert_eq!(context.state(), ExceptionHandlingState::Finally);

        context.set_state(ExceptionHandlingState::Try);
        assert_eq!(context.state(), ExceptionHandlingState::Try);
    }

    #[test]
    fn test_exception_handling_context_exception() {
        let mut context = ExceptionHandlingContext::new_simple(30, 40);

        let exception = StackItem::from_byte_string("Test exception".as_bytes().to_vec());
        context.set_exception(Some(exception.clone()));

        assert!(context.exception().is_some());
        assert_eq!(
            context.exception().unwrap().as_bytes().unwrap(),
            exception.as_bytes().unwrap()
        );

        context.set_exception(None);
        assert!(context.exception().is_none());
    }

    #[test]
    fn test_exception_handling_context_end_pointer() {
        let mut context = ExceptionHandlingContext::new_simple(30, 40);

        assert_eq!(context.end_pointer(), -1);

        context.set_end_pointer(50);
        assert_eq!(context.end_pointer(), 50);
    }

    #[test]
    fn test_exception_handling_context_has_catch_finally() {
        // Context with catch and finally
        let context = ExceptionHandlingContext::new_simple(30, 40);

        assert!(context.has_catch());
        assert!(context.has_finally());

        // Context with catch but no finally
        let context = ExceptionHandlingContext::new_simple(30, -1);

        assert!(context.has_catch());
        assert!(!context.has_finally());

        // Context with finally but no catch
        let context = ExceptionHandlingContext::new_simple(-1, 40);

        assert!(!context.has_catch());
        assert!(context.has_finally());

        // Context with no catch and no finally
        let context = ExceptionHandlingContext::new_simple(-1, -1);

        assert!(!context.has_catch());
        assert!(!context.has_finally());
    }
}
