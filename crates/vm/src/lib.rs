//! # Neo Virtual Machine (NeoVM)
//!
//! A complete implementation of the Neo Virtual Machine in Rust.
//!
//! The Neo Virtual Machine (NeoVM) is a stack-based virtual machine that executes
//! smart contracts on the Neo blockchain. This crate provides a fully compatible
//! implementation of the NeoVM specification with advanced features for debugging,
//! interoperability, and performance monitoring.
//!
//! ## Features
//!
//! - **Complete OpCode Support**: All Neo VM opcodes with precise semantics
//! - **Stack-Based Execution**: Evaluation stack with type-safe operations
//! - **Interop Services**: Native contract integration and system calls
//! - **Exception Handling**: Comprehensive error handling and recovery
//! - **Debugging Support**: Breakpoints, step execution, and state inspection
//! - **Script Building**: Programmatic smart contract bytecode generation
//! - **Reference Counting**: Memory management for complex data structures
//!
//! ## Architecture
//!
//! The VM is organized into several core components:
//!
//! - **ExecutionEngine**: Main VM execution loop and state management
//! - **ApplicationEngine**: High-level engine with interop service integration
//! - **EvaluationStack**: Type-safe stack for VM operations
//! - **ExecutionContext**: Script execution context and local variables
//! - **JumpTable**: OpCode implementation and instruction dispatch
//! - **StackItem**: Polymorphic data types for VM values
//! - **ScriptBuilder**: Utility for constructing VM scripts
//!
//! ## Example
//!
//! ```rust,no_run
//! use neo_vm::{ApplicationEngine, Script, StackItem};
//!
//! # fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Create a simple script that pushes numbers and adds them
//! let script = Script::new(vec![
//!     0x51, // PUSH1
//!     0x52, // PUSH2
//!     0x9F, // ADD
//! ]);
//!
//! // Create and configure the VM engine
//! let mut engine = ApplicationEngine::new();
//! engine.load_script(&script, false)?;
//!
//! // Execute the script
//! let result = engine.execute()?;
//!
//! // Get the result from the stack
//! if let Some(result_item) = engine.result_stack().peek(0) {
//!     if let StackItem::Integer(value) = result_item {
//!         println!("Result: {}", value);
//!     }
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Interop Services
//!
//! The VM supports interop services for accessing blockchain state:
//!
//! ```rust,no_run
//! use neo_vm::{ApplicationEngine, InteropService};
//!
//! # fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let mut engine = ApplicationEngine::new();
//!
//! // Register custom interop service
//! engine.register_interop_service("MyService.Method", |engine, args| {
//!     // Custom interop implementation
//!     Ok(neo_vm::StackItem::Boolean(true))
//! });
//! # Ok(())
//! # }
//! ```
//!
//! ## Debugging
//!
//! The VM includes comprehensive debugging features:
//!
//! ```rust,no_run
//! use neo_vm::{ApplicationEngine, Debugger, Breakpoint};
//!
//! # fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let mut engine = ApplicationEngine::new();
//! let mut debugger = Debugger::new();
//!
//! // Set breakpoint at instruction position
//! debugger.add_breakpoint(Breakpoint::new(0, 10));
//!
//! // Execute with debugging
//! engine.set_debugger(debugger);
//! # Ok(())
//! # }
//! ```

//#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]

// Always import standard library types
extern crate std;

// Core VM modules
/// High-level VM engine with interop services
pub mod application_engine;
/// Call permission flags for interop services  
pub mod call_flags;
/// Debugging support with breakpoints and step execution
pub mod debugger;
/// VM error types and result handling
pub mod error;
/// Type-safe evaluation stack implementation
pub mod evaluation_stack;
/// Exception handling and try-catch support
pub mod exception_handling;
/// Script execution context and local variables
pub mod execution_context;
/// Low-level VM execution engine
pub mod execution_engine;
/// Gas calculation system matching C# exactly
pub mod gas_calculator;
/// VM instruction representation
pub mod instruction;
/// Interop service registry and native calls
pub mod interop_service;
/// OpCode implementation and instruction dispatch
pub mod jump_table;
/// Memory pool for optimizing allocations
pub mod memory_pool;
/// Performance metrics collection
pub mod metrics;
/// VM opcode definitions and utilities
pub mod op_code;
/// Performance optimization utilities
pub mod performance_opt;
/// Memory management for complex data structures
pub mod reference_counter;
/// Safe execution utilities for VM operations
pub mod safe_execution;
/// Safe type conversion utilities
pub mod safe_type_conversion;
/// VM script representation and validation
pub mod script;
/// Utility for constructing VM bytecode
pub mod script_builder;
/// Polymorphic data types for VM values
pub mod stack_item;
/// Graph algorithms for garbage collection
pub mod strongly_connected_components;

/// Test utilities and compatibility tests
#[cfg(test)]
#[allow(dead_code)]
pub mod tests;

pub use application_engine::{ApplicationEngine, NotificationEvent, TriggerType};
pub use call_flags::CallFlags;
pub use debugger::{Breakpoint, Debugger};
pub use error::{VmError, VmResult};
pub use evaluation_stack::EvaluationStack;
pub use exception_handling::{ExceptionHandlingContext, ExceptionHandlingState};
pub use execution_context::{ExecutionContext, Slot};
pub use execution_engine::{ExecutionEngine, ExecutionEngineLimits, VMState};
pub use instruction::Instruction;
pub use interop_service::{InteropDescriptor, InteropMethod, InteropService};
pub use jump_table::{InstructionHandler, JumpTable};
pub use op_code::OpCode;
pub use reference_counter::ReferenceCounter;
pub use script::Script;
pub use script_builder::ScriptBuilder;
pub use stack_item::{StackItem, StackItemType};
pub use strongly_connected_components::Tarjan;

// I/O abstraction layer
/// Test I/O implementation for unit tests
#[cfg(test)]
#[allow(dead_code)]
pub use crate::tests::real_io as io;

/// Production I/O implementation
#[cfg(not(test))]
pub extern crate neo_io;
/// Re-export of neo_io for production use
#[cfg(not(test))]
pub use neo_io as io;
