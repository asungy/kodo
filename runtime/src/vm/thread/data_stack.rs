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

        let result = u16::from(self.stack.pop().unwrap()) |
                    (u16::from(self.stack.pop().unwrap()) << 8);

        Ok(result)
    }

    pub(in crate::vm) fn push16(&mut self, word: u16) -> () {
        let bytes = word.to_be_bytes();
        for i in 0..2 {
            self.stack.push(bytes[i]);
        }
    }

    pub(in crate::vm) fn pop32(&mut self) -> Result<u32, DataStackError> {
        if self.len() < 4 {
            return Err(DataStackError::NotEnoughBytes {
                attempt: 4,
                remaining: self.len(),
            });
        }

        let result = u32::from(self.stack.pop().unwrap()) |
                    (u32::from(self.stack.pop().unwrap()) << 8) |
                    (u32::from(self.stack.pop().unwrap()) << 16) |
                    (u32::from(self.stack.pop().unwrap()) << 24);

        Ok(result)
    }

    pub(in crate::vm) fn push32(&mut self, double_word: u32) -> () {
        let bytes = double_word.to_be_bytes();
        for i in 0..4 {
            self.stack.push(bytes[i]);
        }
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

    /// Positive test for push16.
    #[test]
    fn push16_positive() {
        let mut data = DataStack::new();
        data.push16(0x1ea5);
        assert_eq!(data.stack[0], 0x1e);
        assert_eq!(data.stack[1], 0xa5);
    }

    /// Positive test for pop32.
    #[test]
    fn pop32_positive() {
        let mut data = DataStack::new();
        data.push8(0x12);
        data.push8(0x34);
        data.push8(0x56);
        data.push8(0x78);
        assert_eq!(data.pop32().unwrap(), 0x12345678);
        assert_eq!(data.len(), 0);
    }

    /// Positive test for push32.
    #[test]
    fn push32_positive() {
        let mut data = DataStack::new();
        data.push32(0x12345678);
        assert_eq!(data.stack[0], 0x12);
        assert_eq!(data.stack[1], 0x34);
        assert_eq!(data.stack[2], 0x56);
        assert_eq!(data.stack[3], 0x78);
   }
}
