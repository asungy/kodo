use std::{error::Error, fmt};

/// Errors involving opcode execution.
#[derive(Debug)]
pub enum ExecuteError {
    /// Occurs when insufficient byte length is provided.
    NotEnoughBytes {
        /// Minimum number of bytes expected.
        min: usize,
        /// Actual length of bytes provided.
        actual: usize,
    },
}

impl Error for ExecuteError{}

impl fmt::Display for ExecuteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            ExecuteError::NotEnoughBytes { min, actual } => {
                write!(
                    f,
                    "Expected minimum bytes: {min}. Actual: {actual}.",
                    min = min,
                    actual = actual,
                )
            },
        }
    }
}
