use error::DataStackError;
use std::vec::Vec;

pub(crate) struct DataStack {
    stack: Vec<u8>,
}

impl DataStack {
    pub(in crate::vm) fn new() -> Self {
        DataStack {
            stack: Vec::new(),
        }
    }

    pub(in crate::vm) fn len(&self) -> usize {
        self.stack.len()
    }

    pub(in crate::vm) fn pop8(&mut self) -> Result<u8, DataStackError> {
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

    pub(in crate::vm) fn push8(&mut self, byte: u8) -> () {
        self.stack.push(byte);
    }

    pub(in crate::vm) fn pop16(&mut self) -> Result<u16, DataStackError> {
        if self.len() < 2 {
            return Err(DataStackError::NotEnoughBytes {
                attempt: 2,
                remaining: self.len(),
            });
        }

        let result = u16::from(self.stack.pop().unwrap());
        let result = (u16::from(self.stack.pop().unwrap()) << 8) | result;

        Ok(result)
    }

    pub(in crate::vm) fn push16(&mut self, word: u16) -> () {
        let bytes = word.to_be_bytes();
        self.stack.push(bytes[0]);
        self.stack.push(bytes[1]);
    }
}

pub(in crate::vm) mod error {
    use std::{error::Error, fmt};

    /// Errors involving the data stack.
    #[derive(Debug, PartialEq, Eq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Positive test for pop16.
    #[test]
    fn pop16_positive() {
        let mut data = DataStack::new();
        data.push8(0xbe);
        data.push8(0xef);
        assert_eq!(data.pop16().unwrap(), 0xbeef);
        assert_eq!(data.len(), 0);
    }

    /// Negative test for pop16 where there's not enough bytes to pop a 16-bit value.
    #[test]
    fn pop16_not_enough_bytes() {
        let mut data = DataStack::new();
        let error = data.pop16().unwrap_err();
        assert_eq!(error, DataStackError::NotEnoughBytes { attempt: 2, remaining: 0, });
    }

    #[test]
    fn push16() {
        let mut data = DataStack::new();
        data.push16(0x1ea5);
        assert_eq!(data.stack[0], 0x1e);
        assert_eq!(data.stack[1], 0xa5);
    }
}
