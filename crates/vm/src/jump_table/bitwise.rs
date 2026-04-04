//! Bitwise operations for the Neo Virtual Machine.
//!
//! This module provides the bitwise operation handlers for the Neo VM.

use crate::error::VmError;
use crate::error::VmResult;
use crate::execution_engine::ExecutionEngine;
use crate::instruction::Instruction;
use crate::jump_table::JumpTable;
use crate::op_code::OpCode;
use crate::stack_item::StackItem;

/// Registers the bitwise operation handlers.
pub fn register_handlers(jump_table: &mut JumpTable) {
    jump_table.register(OpCode::INVERT, invert);
    jump_table.register(OpCode::AND, and);
    jump_table.register(OpCode::OR, or);
    jump_table.register(OpCode::XOR, xor);
    jump_table.register(OpCode::EQUAL, equal);
    jump_table.register(OpCode::NOTEQUAL, not_equal);
}

/// Implements the INVERT operation.
fn invert(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Pop the value from the stack
    let value = context.pop()?;

    // Invert the value
    let result = match value {
        StackItem::Integer(i) => {
            // Bitwise NOT
            StackItem::from_int(!i)
        }
        StackItem::Boolean(b) => {
            // Logical NOT
            StackItem::from_bool(!b)
        }
        _ => {
            // Convert to boolean and invert
            StackItem::from_bool(!value.as_bool()?)
        }
    };

    context.push(result)?;

    Ok(())
}

/// Implements the AND operation.
fn and(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Pop the values from the stack
    let b = context.pop()?;
    let a = context.pop()?;

    // Perform the AND operation
    let result = match (&a, &b) {
        (StackItem::Integer(a), StackItem::Integer(b)) => {
            // Bitwise AND
            StackItem::from_int(a & b)
        }
        (StackItem::Boolean(a), StackItem::Boolean(b)) => StackItem::from_bool(*a && *b),
        (StackItem::ByteString(_), StackItem::ByteString(_)) => {
            // Convert ByteStrings to integers and perform bitwise AND
            let int_a = a.as_int()?;
            let int_b = b.as_int()?;
            StackItem::from_int(int_a & int_b)
        }
        _ => {
            return Err(VmError::invalid_operation_msg(format!(
                "AND operation not supported for types: {:?} and {:?}",
                a.stack_item_type(),
                b.stack_item_type()
            )));
        }
    };

    context.push(result)?;

    Ok(())
}

/// Implements the OR operation.
fn or(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Pop the values from the stack
    let b = context.pop()?;
    let a = context.pop()?;

    // Perform the OR operation
    let result = match (&a, &b) {
        (StackItem::Integer(a), StackItem::Integer(b)) => {
            // Bitwise OR
            StackItem::from_int(a | b)
        }
        (StackItem::Boolean(a), StackItem::Boolean(b)) => StackItem::from_bool(*a || *b),
        (StackItem::ByteString(_), StackItem::ByteString(_)) => {
            // Convert ByteStrings to integers and perform bitwise OR
            let int_a = a.as_int()?;
            let int_b = b.as_int()?;
            StackItem::from_int(int_a | int_b)
        }
        _ => {
            return Err(VmError::invalid_operation_msg(format!(
                "OR operation not supported for types: {:?} and {:?}",
                a.stack_item_type(),
                b.stack_item_type()
            )));
        }
    };

    context.push(result)?;

    Ok(())
}

/// Implements the XOR operation.
fn xor(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Pop the values from the stack
    let b = context.pop()?;
    let a = context.pop()?;

    // Perform the XOR operation
    let result = match (&a, &b) {
        (StackItem::Integer(a), StackItem::Integer(b)) => {
            // Bitwise XOR
            StackItem::from_int(a ^ b)
        }
        (StackItem::Boolean(a), StackItem::Boolean(b)) => StackItem::from_bool(*a != *b),
        (StackItem::ByteString(_), StackItem::ByteString(_)) => {
            // Convert ByteStrings to integers and perform bitwise XOR
            let int_a = a.as_int()?;
            let int_b = b.as_int()?;
            StackItem::from_int(int_a ^ int_b)
        }
        _ => {
            return Err(VmError::invalid_operation_msg(format!(
                "XOR operation not supported for types: {:?} and {:?}",
                a.stack_item_type(),
                b.stack_item_type()
            )));
        }
    };

    context.push(result)?;

    Ok(())
}

/// Implements the EQUAL operation.
fn equal(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Check stack has at least 2 items
    if context.evaluation_stack().len() < 2 {
        return Err(VmError::insufficient_stack_items(
            2,
            context.evaluation_stack().len(),
        ));
    }

    // Pop the values from the stack
    let b = context.pop()?;
    let a = context.pop()?;

    // Compare the values
    let result = a.equals(&b)?;

    context.push(StackItem::from_bool(result))?;

    Ok(())
}

/// Implements the NOTEQUAL operation.
fn not_equal(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Pop the values from the stack
    let b = context.pop()?;
    let a = context.pop()?;

    // Compare the values
    let result = !a.equals(&b)?;

    context.push(StackItem::from_bool(result))?;

    Ok(())
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    // Bitwise operation tests with proper Result handling
    use super::*;
    use crate::execution_engine::ExecutionEngine;
    use crate::instruction::Instruction;
    use crate::op_code::OpCode;
    use crate::script::Script;
    use crate::stack_item::StackItem;
    use num_bigint::BigInt;

    #[test]
    fn test_invert() {
        let mut engine = ExecutionEngine::new(None);
        let script = Script::new_relaxed(vec![]);
        let _context = engine
            .load_script(script, -1, 0)
            .map_err(|_| VmError::invalid_operation_msg("operation failed"))?;

        // Test integer inversion
        engine
            .current_context_mut()
            .ok_or_else(|| VmError::invalid_operation_msg("operation failed"))?
            .push(StackItem::from_int(42))
            .map_err(|_| VmError::invalid_operation_msg("operation failed"))?;
        invert(&mut engine, &Instruction::new(OpCode::INVERT, &[]))
            .map_err(|_| VmError::invalid_operation_msg("operation failed"))?;
        let result = engine
            .current_context_mut()
            .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?
            .evaluation_stack_mut()
            .pop()?;
        assert_eq!(
            result
                .as_int()
                .ok_or_else(|| VmError::invalid_operation_msg("operation failed"))?,
            BigInt::from(-43)
        );

        // Test boolean inversion
        engine
            .current_context_mut()
            .ok_or_else(|| VmError::invalid_operation_msg("operation failed"))?
            .push(StackItem::from_bool(true))
            .map_err(|_| VmError::invalid_operation_msg("operation failed"))?;
        invert(&mut engine, &Instruction::new(OpCode::INVERT, &[]))
            .map_err(|_| VmError::invalid_operation_msg("operation failed"))?;
        let result = engine
            .current_context_mut()
            .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?
            .evaluation_stack_mut()
            .pop()?;
        assert_eq!(
            result
                .as_bool()
                .ok_or_else(|| VmError::invalid_type_simple("Expected boolean"))?,
            false
        );
    }

    #[test]
    fn test_and() {
        let mut engine = ExecutionEngine::new(None);
        let script = Script::new_relaxed(vec![]);
        let _context = engine
            .load_script(script, -1, 0)
            .map_err(|_| VmError::invalid_operation_msg("operation failed"))?;

        // Test integer AND
        engine
            .current_context_mut()
            .ok_or_else(|| VmError::invalid_operation_msg("operation failed"))?
            .push(StackItem::from_int(0b1010))
            .map_err(|_| VmError::invalid_operation_msg("operation failed"))?;
        engine
            .current_context_mut()
            .ok_or_else(|| VmError::invalid_operation_msg("operation failed"))?
            .push(StackItem::from_int(0b1100))
            .map_err(|_| VmError::invalid_operation_msg("operation failed"))?;
        and(&mut engine, &Instruction::new(OpCode::AND, &[]))
            .map_err(|_| VmError::invalid_operation_msg("operation failed"))?;
        let result = engine
            .current_context_mut()
            .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?
            .evaluation_stack_mut()
            .pop()?;
        assert_eq!(
            result
                .as_int()
                .ok_or_else(|| VmError::invalid_operation_msg("operation failed"))?,
            BigInt::from(0b1000)
        );

        // Test boolean AND
        engine
            .current_context_mut()
            .ok_or_else(|| VmError::invalid_operation_msg("operation failed"))?
            .push(StackItem::from_bool(true))
            .expect("Operation failed");
        engine
            .current_context_mut()
            .expect("Operation failed")
            .push(StackItem::from_bool(false))
            .expect("Operation failed");
        and(&mut engine, &Instruction::new(OpCode::AND, &[])).expect("Operation failed");
        let result = engine
            .current_context_mut()
            .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?
            .evaluation_stack_mut()
            .pop()?;
        assert_eq!(
            result
                .as_bool()
                .ok_or_else(|| VmError::invalid_type_simple("Expected boolean"))?,
            false
        );
    }

    #[test]
    fn test_or() {
        let mut engine = ExecutionEngine::new(None);
        let script = Script::new_relaxed(vec![]);
        let _context = engine
            .load_script(script, -1, 0)
            .map_err(|_| VmError::invalid_operation_msg("operation failed"))?;

        // Test integer OR
        engine
            .current_context_mut()
            .ok_or_else(|| VmError::invalid_operation_msg("operation failed"))?
            .push(StackItem::from_int(0b1010))
            .map_err(|_| VmError::invalid_operation_msg("operation failed"))?;
        engine
            .current_context_mut()
            .ok_or_else(|| VmError::invalid_operation_msg("operation failed"))?
            .push(StackItem::from_int(0b1100))
            .map_err(|_| VmError::invalid_operation_msg("operation failed"))?;
        or(&mut engine, &Instruction::new(OpCode::OR, &[]))
            .map_err(|_| VmError::invalid_operation_msg("operation failed"))?;
        let result = engine
            .current_context_mut()
            .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?
            .evaluation_stack_mut()
            .pop()?;
        assert_eq!(
            result
                .as_int()
                .ok_or_else(|| VmError::invalid_operation_msg("operation failed"))?,
            BigInt::from(0b1110)
        );

        // Test boolean OR
        engine
            .current_context_mut()
            .ok_or_else(|| VmError::invalid_operation_msg("operation failed"))?
            .push(StackItem::from_bool(true))
            .expect("Operation failed");
        engine
            .current_context_mut()
            .expect("Operation failed")
            .push(StackItem::from_bool(false))
            .expect("Operation failed");
        or(&mut engine, &Instruction::new(OpCode::OR, &[])).expect("Operation failed");
        let result = engine
            .current_context_mut()
            .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?
            .evaluation_stack_mut()
            .pop()?;
        assert_eq!(
            result
                .as_bool()
                .ok_or_else(|| VmError::invalid_type_simple("Expected boolean"))?,
            true
        );
    }

    #[test]
    fn test_xor() {
        let mut engine = ExecutionEngine::new(None);
        let script = Script::new_relaxed(vec![]);
        let _context = engine
            .load_script(script, -1, 0)
            .map_err(|_| VmError::invalid_operation_msg("operation failed"))?;

        // Test integer XOR
        engine
            .current_context_mut()
            .ok_or_else(|| VmError::invalid_operation_msg("operation failed"))?
            .push(StackItem::from_int(0b1010))
            .map_err(|_| VmError::invalid_operation_msg("operation failed"))?;
        engine
            .current_context_mut()
            .ok_or_else(|| VmError::invalid_operation_msg("operation failed"))?
            .push(StackItem::from_int(0b1100))
            .map_err(|_| VmError::invalid_operation_msg("operation failed"))?;
        xor(&mut engine, &Instruction::new(OpCode::XOR, &[]))
            .map_err(|_| VmError::invalid_operation_msg("operation failed"))?;
        let result = engine
            .current_context_mut()
            .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?
            .evaluation_stack_mut()
            .pop()?;
        assert_eq!(result.as_int().unwrap(), BigInt::from(0b0110));

        // Test boolean XOR
        engine
            .current_context_mut()
            .unwrap()
            .push(StackItem::from_bool(true))
            .expect("Operation failed");
        engine
            .current_context_mut()
            .expect("Operation failed")
            .push(StackItem::from_bool(false))
            .expect("Operation failed");
        xor(&mut engine, &Instruction::new(OpCode::XOR, &[])).expect("Operation failed");
        let result = engine
            .current_context_mut()
            .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?
            .evaluation_stack_mut()
            .pop()?;
        assert_eq!(
            result
                .as_bool()
                .ok_or_else(|| VmError::invalid_type_simple("Expected boolean"))?,
            true
        );
    }

    #[test]
    fn test_equal() {
        let mut engine = ExecutionEngine::new(None);
        let script = Script::new_relaxed(vec![]);
        let _context = engine.load_script(script, -1, 0).unwrap();

        // Test equal integers
        engine
            .current_context_mut()
            .unwrap()
            .push(StackItem::from_int(42))
            .unwrap();
        engine
            .current_context_mut()
            .unwrap()
            .push(StackItem::from_int(42))
            .unwrap();
        equal(&mut engine, &Instruction::new(OpCode::EQUAL, &[])).unwrap();
        let result = engine
            .current_context_mut()
            .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?
            .evaluation_stack_mut()
            .pop()?;
        assert_eq!(
            result
                .as_bool()
                .ok_or_else(|| VmError::invalid_type_simple("Expected boolean"))?,
            true
        );

        // Test unequal integers
        engine
            .current_context_mut()
            .expect("Operation failed")
            .push(StackItem::from_int(42))
            .expect("Operation failed");
        engine
            .current_context_mut()
            .expect("Operation failed")
            .push(StackItem::from_int(43))
            .expect("Operation failed");
        equal(&mut engine, &Instruction::new(OpCode::EQUAL, &[])).expect("Operation failed");
        let result = engine
            .current_context_mut()
            .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?
            .evaluation_stack_mut()
            .pop()?;
        assert_eq!(
            result
                .as_bool()
                .ok_or_else(|| VmError::invalid_type_simple("Expected boolean"))?,
            false
        );
    }

    #[test]
    fn test_not_equal() {
        let mut engine = ExecutionEngine::new(None);
        let script = Script::new_relaxed(vec![]);
        let _context = engine.load_script(script, -1, 0).unwrap();

        // Test equal integers
        engine
            .current_context_mut()
            .unwrap()
            .push(StackItem::from_int(42))
            .unwrap();
        engine
            .current_context_mut()
            .unwrap()
            .push(StackItem::from_int(42))
            .unwrap();
        not_equal(&mut engine, &Instruction::new(OpCode::NOTEQUAL, &[])).unwrap();
        let result = engine
            .current_context_mut()
            .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?
            .evaluation_stack_mut()
            .pop()?;
        assert_eq!(
            result
                .as_bool()
                .ok_or_else(|| VmError::invalid_type_simple("Expected boolean"))?,
            false
        );

        // Test unequal integers
        engine
            .current_context_mut()
            .expect("Operation failed")
            .push(StackItem::from_int(42))
            .expect("Operation failed");
        engine
            .current_context_mut()
            .expect("Operation failed")
            .push(StackItem::from_int(43))
            .expect("Operation failed");
        not_equal(&mut engine, &Instruction::new(OpCode::NOTEQUAL, &[])).expect("Operation failed");
        let result = engine
            .current_context_mut()
            .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?
            .evaluation_stack_mut()
            .pop()?;
        assert_eq!(
            result
                .as_bool()
                .ok_or_else(|| VmError::invalid_type_simple("Expected boolean"))?,
            true
        );
    }
}
