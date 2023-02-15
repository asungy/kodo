use crate::vm::op::{
    error::{
        InstructionFormatError,
        OpExecuteError,
        OperationError,
    },
    self,
};
use crate::vm::thread::data_stack::DataStack;
use std::collections::VecDeque;

/// Contains unit tests for 8-bit operations.
#[cfg(test)]
mod byte_op {
    use super::*;

    /// Positive test for Add8 opcode.
    #[test]
    fn add8_positive() {
        let bytes = VecDeque::<u8>::from([
            // 21 + 3 (unsigned)
            0x00, 0x00, 0x15, 0x03,
            // extra random bytes
            0xe1, 0xff, 0x53, 0xbf,
        ]);
        let mut data = DataStack::new();
        let option = op::execute(&bytes, &mut data);
        assert!(option.is_ok());

        // Test Result value
        let delta = option.unwrap();
        assert_eq!(delta.inst_bytes_consumed, 4);
        assert_eq!(delta.data_bytes_pushed, 1);

        // Check data stack
        // 21 + 3 = 24 (unsigned)
        assert_eq!(data.pop8().unwrap(), 0x18);
    }

    /// Negative test for Add8 NotEnoughBytes error.
    #[test]
    fn add8_not_enough_bytes() {
        let bytes = VecDeque::<u8>::from([0x00, 0x00, 0x15]);
        let mut data = DataStack::new();
        let error = match op::execute(&bytes, &mut data).unwrap_err() {
            OpExecuteError::InstructionFormatError { error } => error,
            _ => { unreachable!("Expected OpExecuteError::InstructionFormatError.") },
        };
        assert_eq!(error, InstructionFormatError::NotEnoughBytes {
            raw_opcode: 0x00,
            min: 4,
            actual: 3,
        });
    }

    /// Negative test for Add8 overflow error.
    #[test]
    fn add8_overflow() {
        let bytes = VecDeque::<u8>::from([0x00, 0x00, 0xff, 0x02]);
        let mut data = DataStack::new();
        let error = match op::execute(&bytes, &mut data).unwrap_err() {
            OpExecuteError::OperationError { error } => error,
            _ => { unreachable!("Expected OpExecuteError::OperationError.") },
        };
        assert_eq!(error, OperationError::Add8Overflow{addend1: 0xff, addend2: 0x02});
    }
}

/// Contains unit tests for 16-bit operations.
#[cfg(test)]
mod word_op {
    use super::*;

    /// Positive test for Add16 opcode.
    #[test]
    fn add16() {
        let bytes = VecDeque::<u8>::from([
            // Add16 opcode
            0x00, 0x01,
            // 845 (0x034d)
            0x03, 0x4d,
            // 1112 (0x0458)
            0x04, 0x58,
        ]);
        let mut data = DataStack::new();
        let option = op::execute(&bytes, &mut data);
        assert!(option.is_ok());

        // Test Result value
        let delta = option.unwrap();
        assert_eq!(delta.inst_bytes_consumed, 6);
        assert_eq!(delta.data_bytes_pushed, 2);

        // Check data stack
        // 845 + 1112 = 1957 (unsigned)
        assert_eq!(data.pop16().unwrap(), 0x07a5);
    }
}

/// Contains unit tests for 32-bit operations.
#[cfg(test)]
mod double_word_op {
    use super::*;

    /// Positive test for Add32 opcode.
    #[test]
    fn add32() {
        let bytes = VecDeque::<u8>::from([
            // Add32 opcode
            0x00, 0x02,
            // 1,125,364,592 (43 13 B3 70)
            0x43, 0x13, 0xb3, 0x70,
            // 124,928,165 (07 72 40 A5)
            0x07, 0x72, 0x40, 0xa5,
        ]);
        let mut data = DataStack::new();
        let option = op::execute(&bytes, &mut data);
        assert!(option.is_ok());

        // Test Result value
        let delta = option.unwrap();
        assert_eq!(delta.inst_bytes_consumed, 10);
        assert_eq!(delta.data_bytes_pushed, 4);

        // Check data stack
        // 1,125,364,592 + 124,928,165 = 1,250,292,757 (unsigned)
        assert_eq!(data.pop32().unwrap(), 0x4a85f415);
    }
}
