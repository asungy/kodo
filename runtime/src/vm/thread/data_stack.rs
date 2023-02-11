use std::vec::Vec;
use error::DataStackError;

pub struct DataStack {
    stack: Vec<u8>,
}

impl DataStack {
    pub fn new() -> Self {
        DataStack {
            stack: Vec::new(),
        }
    }

    pub fn pop8(&mut self) -> Result<u8, DataStackError> {
        let byte = self.stack.pop();
        match byte {
            Some(b) => { Ok(b) },
            None => {
                Err(DataStackError::NotEnoughBytes {
                    attempt: 1,
                    remaining: 0,
                })
            },
        }
    }
}

pub mod error {
    use std::{error::Error, fmt};

    /// Errors involving the data stack.
    #[derive(Debug)]
    pub enum DataStackError {
        /// Error for attempting to pop off more bytes than stack contains.
        NotEnoughBytes {
            attempt: usize,
            remaining: usize,
        },
    }

    impl Error for DataStackError{}

    impl fmt::Display for DataStackError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match &self {
                DataStackError::NotEnoughBytes { attempt, remaining } => {
                    write!(
                        f,
                        concat!(
                            "Attempting to pop {attempt} byte(s) off of stack when only {remaining} ",
                            "byte(s) are remaining.",
                        ),
                        attempt = attempt,
                        remaining = remaining,
                    )
                },
            }
        }
    }

}
