use std::{error::Error, fmt};
use super::OpCodeRepr;

/// Errors involving executing an opcode instruction.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum OperationError {
    Add8Overflow {
        addend1: u8,
        addend2: u8,
    },
}

impl Error for OperationError{}

impl fmt::Display for OperationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            OperationError::Add8Overflow { addend1, addend2 } => {
                write!(
                    f,
                    "Performing 8-bit addition on {addend1} and {addend2} causes overflow.",
                    addend1 = addend1,
                    addend2 = addend2,
                )
            },
        }
    }
}

/// Errors involving instruction format.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum InstructionFormatError {
    /// Error for when the provided instruction bytes is not long enough for a given opcode.
    NotEnoughBytes {
        raw_opcode: OpCodeRepr,
        min: usize,
        actual: usize,
    },
    /// Error for when an unrecognized opcode is provided.
    UnknownOpcodeError {
        raw_opcode: OpCodeRepr,
    },
}

impl Error for InstructionFormatError{}

impl fmt::Display for InstructionFormatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            InstructionFormatError::NotEnoughBytes { raw_opcode, min, actual } => {
                write!(
                    f,
                    "Opcode ({opcode}) expects at least {min} byte(s). Only {actual} byte(s) available.",
                    opcode = raw_opcode,
                    min = min,
                    actual = actual,
                )
            },
            InstructionFormatError::UnknownOpcodeError { raw_opcode } => {
                write!(f, "Unknown opcode: {opcode:#06x}", opcode = raw_opcode)
            },
        }
    }
}

/// Errors involving opcode execution.
#[derive(Debug)]
pub(crate) enum OpExecuteError {
    /// Occurs when insufficient byte length is provided.
    NotEnoughBytes {
        /// Minimum number of bytes expected.
        min: usize,
        /// Actual length of bytes provided.
        actual: usize,
    },
    InstructionFormatError { error: InstructionFormatError },
    OperationError { error: OperationError },
}

impl Error for OpExecuteError{}

impl fmt::Display for OpExecuteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            OpExecuteError::NotEnoughBytes { min, actual } => {
                write!(
                    f,
                    "Expected minimum bytes: {min}. Actual: {actual}.",
                    min = min,
                    actual = actual,
                )
            },
            OpExecuteError::InstructionFormatError { error } => { error.fmt(f) },
            OpExecuteError::OperationError { error } => { error.fmt(f) },
        }
    }
}
