pub mod error;

#[cfg(test)]
mod tests;

use crate::vm::thread::data_stack::DataStack;
use error::ExecuteError;
use num_enum::TryFromPrimitive;
use std::collections::VecDeque;

#[derive(TryFromPrimitive)]
#[repr(u16)]
enum OpCode {
    Add8 = 0, // 8-bit addition
}

pub type OpCodeRepr = u16;
const OPCODE_LENGTH: usize = 2;

/// Contains information regarding changes to VM after executing an instruction.
pub struct Delta {
    /// Instruction bytes consumed.
    inst_bytes_consumed: usize,
    /// Data bytes pushed on stack.
    data_bytes_pushed: usize,
}

/// Executes an opcode instruction.
pub fn execute(bytes: &VecDeque<u8>, data: &mut DataStack) -> Result<Delta, ExecuteError> {
    if bytes.len() < OPCODE_LENGTH {
        return Err(ExecuteError::NotEnoughBytes {
            min: OPCODE_LENGTH,
            actual: bytes.len(),
        })
    }

    todo!("Implement me!");
}
