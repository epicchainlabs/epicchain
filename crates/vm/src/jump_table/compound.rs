//! Compound operations for the Neo Virtual Machine.
//!
//! This module provides the compound operation handlers for the Neo VM.

use crate::error::VmError;
use crate::error::VmResult;
use crate::execution_engine::ExecutionEngine;
use crate::instruction::Instruction;
use crate::jump_table::JumpTable;
use crate::op_code::OpCode;
use crate::stack_item::StackItem;
use num_bigint::BigInt;
use num_traits::ToPrimitive;
use std::collections::BTreeMap;

/// Registers the compound operation handlers.
pub fn register_handlers(jump_table: &mut JumpTable) {
    jump_table.register(OpCode::NEWARRAY0, new_array0);
    jump_table.register(OpCode::NEWARRAY, new_array);
    jump_table.register(OpCode::NewarrayT, new_array_t);
    jump_table.register(OpCode::NEWSTRUCT0, new_struct0);
    jump_table.register(OpCode::NEWSTRUCT, new_struct);
    jump_table.register(OpCode::NEWMAP, new_map);
    jump_table.register(OpCode::APPEND, append);
    jump_table.register(OpCode::REVERSEITEMS, reverse);
    jump_table.register(OpCode::REMOVE, remove);
    jump_table.register(OpCode::CLEARITEMS, clear_items);
    jump_table.register(OpCode::POPITEM, pop_item);
    jump_table.register(OpCode::HASKEY, has_key);
    jump_table.register(OpCode::KEYS, keys);
    jump_table.register(OpCode::VALUES, values);
    jump_table.register(OpCode::PACKMAP, pack_map);
    jump_table.register(OpCode::PACKSTRUCT, pack_struct);
    jump_table.register(OpCode::PACK, pack);
    jump_table.register(OpCode::UNPACK, unpack);
    jump_table.register(OpCode::PICKITEM, pick_item);
    jump_table.register(OpCode::SETITEM, set_item);
    jump_table.register(OpCode::SIZE, size);
}

/// Implements the NEWARRAY0 operation.
fn new_array0(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Create a new empty array
    let array = StackItem::from_array(Vec::new());

    // Push the array onto the stack
    context.push(array)?;

    Ok(())
}

/// Implements the NEWARRAY operation.
fn new_array(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Pop the count from the stack
    let count = context
        .pop()?
        .as_int()?
        .to_usize()
        .ok_or_else(|| VmError::invalid_operation_msg("Invalid array size"))?;

    // Create a new array with the specified count
    let mut items = Vec::with_capacity(count);
    for _ in 0..count {
        items.push(StackItem::Null);
    }

    // Push the array onto the stack
    context.push(StackItem::from_array(items))?;

    Ok(())
}

/// Implements the NewarrayT operation.
fn new_array_t(engine: &mut ExecutionEngine, instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Pop the count from the stack
    let count = context
        .pop()?
        .as_int()?
        .to_usize()
        .ok_or_else(|| VmError::invalid_operation_msg("Invalid array size"))?;

    // Get the type from the instruction
    let type_byte = instruction
        .operand()
        .first()
        .copied()
        .ok_or_else(|| VmError::invalid_instruction_msg("Missing type operand"))?;

    // Create a new array with the specified count and type
    let mut items = Vec::with_capacity(count);
    for _ in 0..count {
        // Create a default value based on the type
        let default_value = match type_byte {
            0x00 => StackItem::Boolean(false),
            0x01 => StackItem::Integer(BigInt::from(0)),
            0x02 => StackItem::ByteString(Vec::new()),
            0x03 => StackItem::Buffer(Vec::new()),
            0x04 => StackItem::Array(Vec::new()),
            0x05 => StackItem::Struct(Vec::new()),
            0x06 => StackItem::Map(BTreeMap::new()),
            _ => {
                return Err(VmError::invalid_instruction_msg(format!(
                    "Invalid type: {type_byte}"
                )));
            }
        };

        items.push(default_value);
    }

    // Push the array onto the stack
    context.push(StackItem::from_array(items))?;

    Ok(())
}

/// Implements the NEWSTRUCT0 operation.
fn new_struct0(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    let struct_item = StackItem::from_struct(Vec::new());

    context.push(struct_item)?;

    Ok(())
}

/// Implements the NEWSTRUCT operation.
fn new_struct(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Pop the count from the stack
    let count = context
        .pop()?
        .as_int()?
        .to_usize()
        .ok_or_else(|| VmError::invalid_operation_msg("Invalid struct size"))?;

    let mut items = Vec::with_capacity(count);
    for _ in 0..count {
        items.push(StackItem::Null);
    }

    context.push(StackItem::from_struct(items))?;

    Ok(())
}

/// Implements the NEWMAP operation.
fn new_map(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Create a new map
    let map = StackItem::from_map(BTreeMap::new());

    // Push the map onto the stack
    context.push(map)?;

    Ok(())
}

/// Implements the APPEND operation.
fn append(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Pop the item and array from the stack
    let item = context.pop()?;
    let array = context.pop()?;

    // Append the item to the array
    match array {
        StackItem::Array(mut items) => {
            items.push(item);
            context.push(StackItem::from_array(items))?;
        }
        StackItem::Struct(mut items) => {
            items.push(item);
            context.push(StackItem::from_struct(items))?;
        }
        _ => return Err(VmError::invalid_type_simple("Expected Array or Struct")),
    }

    Ok(())
}

/// Implements the REVERSE operation.
fn reverse(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Pop the array from the stack
    let array = context.pop()?;

    // Reverse the array
    match array {
        StackItem::Array(mut items) => {
            items.reverse();
            context.push(StackItem::from_array(items))?;
        }
        StackItem::Struct(mut items) => {
            items.reverse();
            context.push(StackItem::from_struct(items))?;
        }
        _ => return Err(VmError::invalid_type_simple("Expected Array or Struct")),
    }

    Ok(())
}

/// Implements the REMOVE operation.
fn remove(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Pop the key and collection from the stack
    let key = context.pop()?;
    let collection = context.pop()?;

    // Remove the item from the collection
    match collection {
        StackItem::Array(mut items) => {
            let index = key
                .as_int()?
                .to_usize()
                .ok_or_else(|| VmError::invalid_operation_msg("Invalid array index"))?;
            if index >= items.len() {
                return Err(VmError::invalid_operation_msg(format!(
                    "Index out of range: {index}"
                )));
            }
            items.remove(index);
            context.push(StackItem::from_array(items))?;
        }
        StackItem::Struct(mut items) => {
            let index = key
                .as_int()?
                .to_usize()
                .ok_or_else(|| VmError::invalid_operation_msg("Invalid struct index"))?;
            if index >= items.len() {
                return Err(VmError::invalid_operation_msg(format!(
                    "Index out of range: {index}"
                )));
            }
            items.remove(index);
            context.push(StackItem::from_struct(items))?;
        }
        StackItem::Map(mut items) => {
            items.remove(&key);
            context.push(StackItem::from_map(items))?;
        }
        _ => {
            return Err(VmError::invalid_type_simple(
                "Expected Array, Struct, or Map",
            ));
        }
    }

    Ok(())
}

/// Implements the CLEARITEMS operation.
fn clear_items(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Pop the collection from the stack
    let collection = context.pop()?;

    // Clear the collection
    match collection {
        StackItem::Array(mut items) => {
            items.clear();
            context.push(StackItem::from_array(items))?;
        }
        StackItem::Struct(mut items) => {
            items.clear();
            context.push(StackItem::from_struct(items))?;
        }
        StackItem::Map(mut items) => {
            items.clear();
            context.push(StackItem::from_map(items))?;
        }
        _ => {
            return Err(VmError::invalid_type_simple(
                "Expected Array, Struct, or Map",
            ));
        }
    }

    Ok(())
}

/// Implements the POPITEM operation.
fn pop_item(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Pop the collection from the stack
    let collection = context.pop()?;

    // Pop an item from the collection
    match collection {
        StackItem::Array(mut items) => {
            if items.is_empty() {
                return Err(VmError::invalid_operation_msg(
                    "Cannot pop from empty array",
                ));
            }
            let popped_item = items
                .pop()
                .ok_or_else(|| VmError::invalid_operation_msg("Collection is empty"))?;
            context.push(StackItem::from_array(items))?;
            context.push(popped_item)?;
        }
        StackItem::Struct(mut items) => {
            if items.is_empty() {
                return Err(VmError::invalid_operation_msg(
                    "Cannot pop from empty struct",
                ));
            }
            let popped_item = items
                .pop()
                .ok_or_else(|| VmError::invalid_operation_msg("Collection is empty"))?;
            context.push(StackItem::from_struct(items))?;
            context.push(popped_item)?;
        }
        _ => return Err(VmError::invalid_type_simple("Expected Array or Struct")),
    }

    Ok(())
}

/// Implements the HASKEY operation.
fn has_key(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Pop the key and collection from the stack
    let key = context.pop()?;
    let collection = context.pop()?;

    let result = match collection {
        StackItem::Array(items) => {
            let index = key
                .as_int()?
                .to_usize()
                .ok_or_else(|| VmError::invalid_operation_msg("Invalid array index"))?;
            index < items.len()
        }
        StackItem::Struct(items) => {
            let index = key
                .as_int()?
                .to_usize()
                .ok_or_else(|| VmError::invalid_operation_msg("Invalid struct index"))?;
            index < items.len()
        }
        StackItem::Map(items) => items.contains_key(&key),
        _ => {
            return Err(VmError::invalid_type_simple(
                "Expected Array, Struct, or Map",
            ));
        }
    };

    context.push(StackItem::from_bool(result))?;

    Ok(())
}

/// Implements the KEYS operation.
fn keys(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Pop the map from the stack
    let map = context.pop()?;

    // Get the keys from the map
    match map {
        StackItem::Map(items) => {
            let keys: Vec<StackItem> = items.keys().cloned().collect();
            context.push(StackItem::from_array(keys))?;
        }
        _ => return Err(VmError::invalid_type_simple("Expected Map")),
    }

    Ok(())
}

/// Implements the VALUES operation.
fn values(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Pop the map from the stack
    let map = context.pop()?;

    // Get the values from the map
    match map {
        StackItem::Map(items) => {
            let values: Vec<StackItem> = items.values().cloned().collect();
            context.push(StackItem::from_array(values))?;
        }
        _ => return Err(VmError::invalid_type_simple("Expected Map")),
    }

    Ok(())
}

/// Implements the PACKMAP operation.
fn pack_map(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Pop the count from the stack
    let count = context
        .pop()?
        .as_int()?
        .to_usize()
        .ok_or_else(|| VmError::invalid_operation_msg("Invalid map size"))?;

    // Create a new map
    let mut map = BTreeMap::new();

    // Pop key-value pairs from the stack
    for _ in 0..count {
        let value = context.pop()?;
        let key = context.pop()?;
        map.insert(key, value);
    }

    // Push the map onto the stack
    context.push(StackItem::from_map(map))?;

    Ok(())
}

/// Implements the PACKSTRUCT operation.
fn pack_struct(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Pop the count from the stack
    let count = context
        .pop()?
        .as_int()?
        .to_usize()
        .ok_or_else(|| VmError::invalid_operation_msg("Invalid struct size"))?;

    let mut items = Vec::with_capacity(count);

    // Pop items from the stack
    for _ in 0..count {
        items.push(context.pop()?);
    }

    items.reverse();

    context.push(StackItem::from_struct(items))?;

    Ok(())
}

/// Implements the PACK operation.
fn pack(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Pop the count from the stack
    let count = context
        .pop()?
        .as_int()?
        .to_usize()
        .ok_or_else(|| VmError::invalid_operation_msg("Invalid array size"))?;

    // Create a new array
    let mut items = Vec::with_capacity(count);

    // Pop items from the stack
    for _ in 0..count {
        items.push(context.pop()?);
    }

    items.reverse();

    // Push the array onto the stack
    context.push(StackItem::from_array(items))?;

    Ok(())
}

/// Implements the UNPACK operation.
fn unpack(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Pop the array from the stack
    let array = context.pop()?;

    // Unpack the array
    match array {
        StackItem::Array(items) | StackItem::Struct(items) => {
            // Push the items onto the stack
            for item in items.iter() {
                context.push(item.clone())?;
            }

            // Push the count onto the stack
            context.push(StackItem::from_int(items.len()))?;
        }
        _ => return Err(VmError::invalid_type_simple("Expected Array or Struct")),
    }

    Ok(())
}

/// Implements the PICKITEM operation.
fn pick_item(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Pop the key and collection from the stack
    let key = context.pop()?;
    let collection = context.pop()?;

    // Get the item from the collection
    let result = match collection {
        StackItem::Array(items) => {
            let index = key
                .as_int()?
                .to_usize()
                .ok_or_else(|| VmError::invalid_operation_msg("Invalid array index"))?;
            if index >= items.len() {
                return Err(VmError::invalid_operation_msg(format!(
                    "Index out of range: {index}"
                )));
            }
            items[index].clone()
        }
        StackItem::Struct(items) => {
            let index = key
                .as_int()?
                .to_usize()
                .ok_or_else(|| VmError::invalid_operation_msg("Invalid struct index"))?;
            if index >= items.len() {
                return Err(VmError::invalid_operation_msg(format!(
                    "Index out of range: {index}"
                )));
            }
            items[index].clone()
        }
        StackItem::Map(items) => items
            .get(&key)
            .cloned()
            .ok_or_else(|| VmError::invalid_operation_msg(format!("Key not found: {key:?}")))?,
        _ => {
            return Err(VmError::invalid_type_simple(
                "Expected Array, Struct, or Map",
            ));
        }
    };

    context.push(result)?;

    Ok(())
}

/// Implements the SETITEM operation.
fn set_item(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Pop the value, key, and collection from the stack
    let value = context.pop()?;
    let key = context.pop()?;
    let collection = context.pop()?;

    // Set the item in the collection
    match collection {
        StackItem::Array(mut items) => {
            let index = key
                .as_int()?
                .to_usize()
                .ok_or_else(|| VmError::invalid_operation_msg("Invalid array index"))?;
            if index >= items.len() {
                return Err(VmError::invalid_operation_msg(format!(
                    "Index out of range: {index}"
                )));
            }
            items[index] = value;
            context.push(StackItem::from_array(items))?;
        }
        StackItem::Struct(mut items) => {
            let index = key
                .as_int()?
                .to_usize()
                .ok_or_else(|| VmError::invalid_operation_msg("Invalid struct index"))?;
            if index >= items.len() {
                return Err(VmError::invalid_operation_msg(format!(
                    "Index out of range: {index}"
                )));
            }
            items[index] = value;
            context.push(StackItem::from_struct(items))?;
        }
        StackItem::Map(mut items) => {
            items.insert(key, value);
            context.push(StackItem::from_map(items))?;
        }
        _ => {
            return Err(VmError::invalid_type_simple(
                "Expected Array, Struct, or Map",
            ));
        }
    }

    Ok(())
}

/// Implements the SIZE operation.
fn size(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Pop the collection from the stack
    let collection = context.pop()?;

    // Get the size of the collection
    let size = match collection {
        StackItem::Array(items) => items.len(),
        StackItem::Struct(items) => items.len(),
        StackItem::Map(items) => items.len(),
        StackItem::ByteString(data) => data.len(),
        StackItem::Buffer(data) => data.len(),
        _ => {
            return Err(VmError::invalid_type_simple(
                "Expected Array, Struct, Map, ByteString, or Buffer",
            ));
        }
    };

    // Push the size onto the stack
    context.push(StackItem::from_int(size))?;

    Ok(())
}
