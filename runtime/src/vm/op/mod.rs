mod error;

#[cfg(test)]
mod tests;

use crate::vm::thread::data_stack::DataStack;
use error::{ OpExecuteError, InstructionFormatError, OperationError, };
use num_enum::TryFromPrimitive;
use std::collections::VecDeque;

#[derive(TryFromPrimitive)]
#[repr(u16)]
enum OpCode {
    Add8 = 0, // 8-bit addition
}

pub(self) type OpCodeRepr = u16;
const OPCODE_LENGTH: usize = 2;

/// Contains information regarding changes to VM after executing an instruction.
#[derive(Debug, Copy, Clone)]
pub(crate) struct Delta {
    /// Instruction bytes consumed.
    inst_bytes_consumed: usize,
    /// Data bytes pushed on stack.
    data_bytes_pushed: usize,
}

/// Executes an opcode instruction.
pub(crate) fn execute(bytes: &VecDeque<u8>, data: &mut DataStack) -> Result<Delta, OpExecuteError> {
    if bytes.len() < OPCODE_LENGTH {
        return Err(OpExecuteError::NotEnoughBytes {
            min: OPCODE_LENGTH,
            actual: bytes.len(),
        })
    }

    // Closure to check that the bytes provided is sufficiently long enough for the given opcode.
    let check_bytes_len = |bytes: &VecDeque<u8>, raw_opcode: OpCodeRepr, min: usize|
            -> Result<usize, OpExecuteError> {
        if bytes.len() < min {
            Err(OpExecuteError::InstructionFormatError {
                error: InstructionFormatError::NotEnoughBytes {
                    raw_opcode,
                    min,
                    actual: bytes.len(),
                }
            })
        } else {
            Ok(min)
        }
    };

    let raw_opcode: OpCodeRepr = OpCodeRepr::from(bytes[0]) << 8;
    let raw_opcode = raw_opcode | OpCodeRepr::from(bytes[1]);
    let data_len_before = data.len();

    let bytes_consumed = match OpCode::try_from(raw_opcode) {
        Ok(OpCode::Add8) => {
            let consumed = check_bytes_len(bytes, raw_opcode, 4);
            if consumed.is_err() {
                return Err(consumed.unwrap_err());
            }
            let consumed = consumed.unwrap();

            let (sum, overflow) = bytes[2].overflowing_add(bytes[3]);
            if overflow {
                return Err(OpExecuteError::OperationError {
                    error: OperationError::Add8Overflow {
                        addend1: bytes[2],
                        addend2: bytes[3],
                    },
                });
            }
            data.push8(sum);

            consumed
        },
        Err(_) => {
            return Err(OpExecuteError::InstructionFormatError {
                error: InstructionFormatError::UnknownOpcodeError {
                    raw_opcode,
                },
            });
        }
    };

    Ok(Delta {
        inst_bytes_consumed: bytes_consumed,
        data_bytes_pushed: data.len() - data_len_before,
    })
}
