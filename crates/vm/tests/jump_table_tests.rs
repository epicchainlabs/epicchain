//! Integration tests for the Neo VM jump table operations.

use neo_vm::execution_engine::{ExecutionEngine, VMState};
use neo_vm::instruction::Instruction;
use neo_vm::jump_table::JumpTable;
use neo_vm::op_code::OpCode;
use neo_vm::script::Script;
use neo_vm::stack_item::StackItem;
use num_bigint::BigInt;

#[test]
fn test_bitwise_operations() {
    // Create a jump table with default handlers
    let jump_table = JumpTable::new();

    // Create an execution engine
    let mut engine = ExecutionEngine::new(Some(jump_table));

    // Create a script with bitwise operations
    let script_bytes = vec![
        OpCode::PUSH1 as u8,  // Push 1 onto the stack
        OpCode::PUSH3 as u8,  // Push 3 onto the stack
        OpCode::AND as u8,    // 1 & 3 = 1
        OpCode::PUSH1 as u8,  // Push 1 onto the stack
        OpCode::PUSH2 as u8,  // Push 2 onto the stack
        OpCode::OR as u8,     // 1 | 2 = 3
        OpCode::PUSH3 as u8,  // Push 3 onto the stack
        OpCode::PUSH5 as u8,  // Push 5 onto the stack
        OpCode::XOR as u8,    // 3 ^ 5 = 6
        OpCode::PUSH5 as u8,  // Push 5 onto the stack
        OpCode::INVERT as u8, // ~5 = -6
    ];
    let script = Script::new_relaxed(script_bytes);

    // Load the script
    engine.load_script(script, -1, 0).unwrap();

    // Execute the script
    engine.execute_next().unwrap(); // PUSH1
    engine.execute_next().unwrap(); // PUSH3
    engine.execute_next().unwrap(); // AND

    let context = engine.current_context().unwrap();
    let stack = context.evaluation_stack();
    assert_eq!(stack.len(), 1);
    assert_eq!(stack.peek(0).unwrap().as_int().unwrap(), BigInt::from(1));

    // Continue execution
    engine.execute_next().unwrap(); // PUSH1
    engine.execute_next().unwrap(); // PUSH2
    engine.execute_next().unwrap(); // OR

    let context = engine.current_context().unwrap();
    let stack = context.evaluation_stack();
    assert_eq!(stack.len(), 2);
    assert_eq!(stack.peek(0).unwrap().as_int().unwrap(), BigInt::from(3));

    // Continue execution
    engine.execute_next().unwrap(); // PUSH3
    engine.execute_next().unwrap(); // PUSH5
    engine.execute_next().unwrap(); // XOR

    let context = engine.current_context().unwrap();
    let stack = context.evaluation_stack();
    assert_eq!(stack.len(), 3);
    assert_eq!(stack.peek(0).unwrap().as_int().unwrap(), BigInt::from(6));

    // Continue execution
    engine.execute_next().unwrap(); // PUSH5
    engine.execute_next().unwrap(); // INVERT

    let context = engine.current_context().unwrap();
    let stack = context.evaluation_stack();
    assert_eq!(stack.len(), 4);
    assert_eq!(stack.peek(0).unwrap().as_int().unwrap(), BigInt::from(-6));
}

#[test]
fn test_numeric_operations() {
    // Create a jump table with default handlers
    let jump_table = JumpTable::new();

    // Create an execution engine
    let mut engine = ExecutionEngine::new(Some(jump_table));

    // Create a script with numeric operations
    let script_bytes = vec![
        OpCode::PUSH5 as u8, // Push 5 onto the stack
        OpCode::INC as u8,   // 5 + 1 = 6
        OpCode::PUSH7 as u8, // Push 7 onto the stack
        OpCode::DEC as u8,   // 7 - 1 = 6
        OpCode::PUSH2 as u8, // Push 2 onto the stack
        OpCode::PUSH3 as u8, // Push 3 onto the stack
        OpCode::ADD as u8,   // 2 + 3 = 5
        OpCode::PUSH9 as u8, // Push 9 onto the stack
        OpCode::PUSH4 as u8, // Push 4 onto the stack
        OpCode::SUB as u8,   // 9 - 4 = 5
        OpCode::PUSH3 as u8, // Push 3 onto the stack
        OpCode::PUSH4 as u8, // Push 4 onto the stack
        OpCode::MUL as u8,   // 3 * 4 = 12
        OpCode::PUSH9 as u8, // Push 9 onto the stack
        OpCode::PUSH3 as u8, // Push 3 onto the stack
        OpCode::DIV as u8,   // 9 / 3 = 3
        OpCode::PUSH7 as u8, // Push 7 onto the stack
        OpCode::PUSH3 as u8, // Push 3 onto the stack
        OpCode::MOD as u8,   // 7 % 3 = 1
    ];
    let script = Script::new_relaxed(script_bytes);

    // Load the script
    engine.load_script(script, -1, 0).unwrap();

    // Execute the script
    engine.execute_next().unwrap(); // PUSH5
    engine.execute_next().unwrap(); // INC

    let context = engine.current_context().unwrap();
    let stack = context.evaluation_stack();
    assert_eq!(stack.len(), 1);
    assert_eq!(stack.peek(0).unwrap().as_int().unwrap(), BigInt::from(6));

    // Continue execution
    engine.execute_next().unwrap(); // PUSH7
    engine.execute_next().unwrap(); // DEC

    let context = engine.current_context().unwrap();
    let stack = context.evaluation_stack();
    assert_eq!(stack.len(), 2);
    assert_eq!(stack.peek(0).unwrap().as_int().unwrap(), BigInt::from(6));

    // Continue execution
    engine.execute_next().unwrap(); // PUSH2
    engine.execute_next().unwrap(); // PUSH3
    engine.execute_next().unwrap(); // ADD

    let context = engine.current_context().unwrap();
    let stack = context.evaluation_stack();
    assert_eq!(stack.len(), 3);
    assert_eq!(stack.peek(0).unwrap().as_int().unwrap(), BigInt::from(5));

    // Continue execution
    engine.execute_next().unwrap(); // PUSH9
    engine.execute_next().unwrap(); // PUSH4
    engine.execute_next().unwrap(); // SUB

    let context = engine.current_context().unwrap();
    let stack = context.evaluation_stack();
    assert_eq!(stack.len(), 4);
    assert_eq!(stack.peek(0).unwrap().as_int().unwrap(), BigInt::from(5));

    // Continue execution
    engine.execute_next().unwrap(); // PUSH3
    engine.execute_next().unwrap(); // PUSH4
    engine.execute_next().unwrap(); // MUL

    let context = engine.current_context().unwrap();
    let stack = context.evaluation_stack();
    assert_eq!(stack.len(), 5);
    assert_eq!(stack.peek(0).unwrap().as_int().unwrap(), BigInt::from(12));

    // Continue execution
    engine.execute_next().unwrap(); // PUSH9
    engine.execute_next().unwrap(); // PUSH3
    engine.execute_next().unwrap(); // DIV

    let context = engine.current_context().unwrap();
    let stack = context.evaluation_stack();
    assert_eq!(stack.len(), 6);
    assert_eq!(stack.peek(0).unwrap().as_int().unwrap(), BigInt::from(3));

    // Continue execution
    engine.execute_next().unwrap(); // PUSH7
    engine.execute_next().unwrap(); // PUSH3
    engine.execute_next().unwrap(); // MOD

    let context = engine.current_context().unwrap();
    let stack = context.evaluation_stack();
    assert_eq!(stack.len(), 7);
    assert_eq!(stack.peek(0).unwrap().as_int().unwrap(), BigInt::from(1));
}

#[test]
fn test_stack_operations() {
    // Create a jump table with default handlers
    let jump_table = JumpTable::new();

    // Create an execution engine
    let mut engine = ExecutionEngine::new(Some(jump_table));

    // Create a script with stack operations
    let script_bytes = vec![
        OpCode::PUSH1 as u8, // Push 1 onto the stack
        OpCode::PUSH2 as u8, // Push 2 onto the stack
        OpCode::PUSH3 as u8, // Push 3 onto the stack
        OpCode::DUP as u8,   // Duplicate the top item (3)
        OpCode::SWAP as u8,  // Swap the top two items (3, 3 -> 3, 3)
        OpCode::OVER as u8,  // Copy the second item to the top (3, 3 -> 3, 3, 3)
        OpCode::ROT as u8,   // Rotate the top three items (2, 3, 3, 3 -> 3, 3, 2, 3)
        OpCode::TUCK as u8, // Insert the top item before the second item (3, 3, 2, 3 -> 3, 3, 3, 2, 3)
        OpCode::DEPTH as u8, // Push the number of items onto the stack (5)
        OpCode::DROP as u8, // Remove the top item (5)
        OpCode::NIP as u8,  // Remove the second item (3, 3, 3, 2 -> 3, 3, 2)
        OpCode::PICK as u8, // Copy the item n back to the top (n=2, so copy the third item from the top)
    ];
    let script = Script::new_relaxed(script_bytes);

    // Load the script
    engine.load_script(script, -1, 0).unwrap();

    // Execute the script
    engine.execute_next().unwrap(); // PUSH1
    engine.execute_next().unwrap(); // PUSH2
    engine.execute_next().unwrap(); // PUSH3

    // Check the initial stack
    let context = engine.current_context().unwrap();
    let stack = context.evaluation_stack();
    assert_eq!(stack.len(), 3);
    assert_eq!(stack.peek(0).unwrap().as_int().unwrap(), BigInt::from(3));
    assert_eq!(stack.peek(1).unwrap().as_int().unwrap(), BigInt::from(2));
    assert_eq!(stack.peek(2).unwrap().as_int().unwrap(), BigInt::from(1));

    // Continue execution
    engine.execute_next().unwrap(); // DUP

    let context = engine.current_context().unwrap();
    let stack = context.evaluation_stack();
    assert_eq!(stack.len(), 4);
    assert_eq!(stack.peek(0).unwrap().as_int().unwrap(), BigInt::from(3));
    assert_eq!(stack.peek(1).unwrap().as_int().unwrap(), BigInt::from(3));

    // Continue execution
    engine.execute_next().unwrap(); // SWAP

    let context = engine.current_context().unwrap();
    let stack = context.evaluation_stack();
    assert_eq!(stack.len(), 4);
    assert_eq!(stack.peek(0).unwrap().as_int().unwrap(), BigInt::from(3));
    assert_eq!(stack.peek(1).unwrap().as_int().unwrap(), BigInt::from(3));

    // Continue execution
    engine.execute_next().unwrap(); // OVER

    let context = engine.current_context().unwrap();
    let stack = context.evaluation_stack();
    assert_eq!(stack.len(), 5);
    assert_eq!(stack.peek(0).unwrap().as_int().unwrap(), BigInt::from(3));
    assert_eq!(stack.peek(1).unwrap().as_int().unwrap(), BigInt::from(3));
    assert_eq!(stack.peek(2).unwrap().as_int().unwrap(), BigInt::from(3));

    // Continue execution
    engine.execute_next().unwrap(); // ROT

    let context = engine.current_context().unwrap();
    let stack = context.evaluation_stack();
    assert_eq!(stack.len(), 5);
    assert_eq!(stack.peek(0).unwrap().as_int().unwrap(), BigInt::from(3));
    assert_eq!(stack.peek(1).unwrap().as_int().unwrap(), BigInt::from(3));
    assert_eq!(stack.peek(2).unwrap().as_int().unwrap(), BigInt::from(2));
    assert_eq!(stack.peek(3).unwrap().as_int().unwrap(), BigInt::from(3));
    assert_eq!(stack.peek(4).unwrap().as_int().unwrap(), BigInt::from(1));

    // Continue execution
    engine.execute_next().unwrap(); // TUCK

    let context = engine.current_context().unwrap();
    let stack = context.evaluation_stack();
    assert_eq!(stack.len(), 6);
    assert_eq!(stack.peek(0).unwrap().as_int().unwrap(), BigInt::from(3));
    assert_eq!(stack.peek(1).unwrap().as_int().unwrap(), BigInt::from(3));
    assert_eq!(stack.peek(2).unwrap().as_int().unwrap(), BigInt::from(3));
    assert_eq!(stack.peek(3).unwrap().as_int().unwrap(), BigInt::from(2));
    assert_eq!(stack.peek(4).unwrap().as_int().unwrap(), BigInt::from(3));
    assert_eq!(stack.peek(5).unwrap().as_int().unwrap(), BigInt::from(1));

    // Continue execution
    engine.execute_next().unwrap(); // DEPTH

    let context = engine.current_context().unwrap();
    let stack = context.evaluation_stack();
    assert_eq!(stack.len(), 7);
    assert_eq!(stack.peek(0).unwrap().as_int().unwrap(), BigInt::from(6));

    // Continue execution
    engine.execute_next().unwrap(); // DROP

    let context = engine.current_context().unwrap();
    let stack = context.evaluation_stack();
    assert_eq!(stack.len(), 6);
    assert_eq!(stack.peek(0).unwrap().as_int().unwrap(), BigInt::from(3));

    // Continue execution
    engine.execute_next().unwrap(); // NIP

    let context = engine.current_context().unwrap();
    let stack = context.evaluation_stack();
    assert_eq!(stack.len(), 5);
    assert_eq!(stack.peek(0).unwrap().as_int().unwrap(), BigInt::from(3));
    assert_eq!(stack.peek(1).unwrap().as_int().unwrap(), BigInt::from(3));
    assert_eq!(stack.peek(2).unwrap().as_int().unwrap(), BigInt::from(2));
    assert_eq!(stack.peek(3).unwrap().as_int().unwrap(), BigInt::from(3));
    assert_eq!(stack.peek(4).unwrap().as_int().unwrap(), BigInt::from(1));

    // Continue execution
    engine.execute_next().unwrap(); // PICK

    let context = engine.current_context().unwrap();
    let stack = context.evaluation_stack();
    assert_eq!(stack.len(), 5);
    assert_eq!(stack.peek(0).unwrap().as_int().unwrap(), BigInt::from(3));
    assert_eq!(stack.peek(1).unwrap().as_int().unwrap(), BigInt::from(3));
    assert_eq!(stack.peek(2).unwrap().as_int().unwrap(), BigInt::from(2));
    assert_eq!(stack.peek(3).unwrap().as_int().unwrap(), BigInt::from(3));
    assert_eq!(stack.peek(4).unwrap().as_int().unwrap(), BigInt::from(1));
}
