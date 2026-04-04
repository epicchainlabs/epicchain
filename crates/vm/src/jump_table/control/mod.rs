//! Control operations module for the Neo Virtual Machine.
//!
//! This module contains all control flow operations, syscalls, and interop services
//! organized into logical submodules for better maintainability.

pub mod control_ops;
pub mod exception_handling;
pub mod interop_services;
pub mod oracle;
pub mod storage;
pub mod syscall;
pub mod types;
pub mod witness;

use crate::{jump_table::JumpTable, op_code::OpCode};

/// Registers all control operation handlers with the jump table.
pub fn register_handlers(jump_table: &mut JumpTable) {
    // Basic control flow operations
    jump_table.register(OpCode::NOP, control_ops::nop);
    jump_table.register(OpCode::JMP, control_ops::jmp);
    jump_table.register(OpCode::JmpL, control_ops::jmp_l);
    jump_table.register(OpCode::RET, control_ops::ret);

    // Conditional jump operations
    jump_table.register(OpCode::JMPIF, control_ops::jmpif);
    jump_table.register(OpCode::JmpifL, control_ops::jmpif_l);
    jump_table.register(OpCode::JMPIFNOT, control_ops::jmpifnot);
    jump_table.register(OpCode::JmpifnotL, control_ops::jmpifnot_l);
    jump_table.register(OpCode::JMPEQ, control_ops::jmpeq);
    jump_table.register(OpCode::JmpeqL, control_ops::jmpeq_l);
    jump_table.register(OpCode::JMPNE, control_ops::jmpne);
    jump_table.register(OpCode::JmpneL, control_ops::jmpne_l);
    jump_table.register(OpCode::JMPGT, control_ops::jmpgt);
    jump_table.register(OpCode::JmpgtL, control_ops::jmpgt_l);
    jump_table.register(OpCode::JMPGE, control_ops::jmpge);
    jump_table.register(OpCode::JmpgeL, control_ops::jmpge_l);
    jump_table.register(OpCode::JMPLT, control_ops::jmplt);
    jump_table.register(OpCode::JmpltL, control_ops::jmplt_l);
    jump_table.register(OpCode::JMPLE, control_ops::jmple);
    jump_table.register(OpCode::JmpleL, control_ops::jmple_l);

    // Call operations
    jump_table.register(OpCode::CALL, control_ops::call);
    jump_table.register(OpCode::CallL, control_ops::call_l);
    jump_table.register(OpCode::CALLA, control_ops::calla);
    jump_table.register(OpCode::CALLT, control_ops::callt);

    // Exception handling operations
    jump_table.register(OpCode::TRY, exception_handling::try_op);
    jump_table.register(OpCode::TryL, exception_handling::try_l);
    jump_table.register(OpCode::THROW, exception_handling::throw);
    jump_table.register(OpCode::ABORT, exception_handling::abort);
    jump_table.register(OpCode::ASSERT, exception_handling::assert);
    jump_table.register(OpCode::ENDTRY, exception_handling::endtry);
    jump_table.register(OpCode::EndtryL, exception_handling::endtry_l);
    jump_table.register(OpCode::ENDFINALLY, exception_handling::endfinally);

    // Syscall operation
    jump_table.register(OpCode::SYSCALL, syscall::syscall);
}

pub use oracle::{get_oracle_request_signers, get_oracle_response_attribute};
pub use storage::{calculate_storage_fee, construct_storage_key};
pub use syscall::syscall;
pub use types::{Block, ExceptionHandler, Transaction};
pub use witness::check_witness_internal;
