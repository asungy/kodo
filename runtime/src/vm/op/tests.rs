use crate::vm::op::{
    error::{
        InstructionFormatError,
        OpExecuteError,
    },
    self,
};
use crate::vm::thread::data_stack::DataStack;
use std::collections::VecDeque;

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
        let mut data_stack = DataStack::new();
        let option = op::execute(&bytes, &mut data_stack);
        assert!(option.is_ok());

        // Test Result value
        let delta = option.unwrap();
        assert_eq!(delta.inst_bytes_consumed, 4);
        assert_eq!(delta.data_bytes_pushed, 1);

        // Check data stack
        // 21 + 3 = 24 (unsigned)
        assert_eq!(data_stack.pop8().unwrap(), 0x18);
    }

    /// Negative test for Add8 NotEnoughBytes error.
    #[test]
    fn add8_not_enough_bytes() {
        let bytes = VecDeque::<u8>::from([0x00, 0x00, 0x15]);
        let mut data = DataStack::new();
        let error = match op::execute(&bytes, &mut data).unwrap_err() {
            OpExecuteError::InstructionFormatError { error } => error,
            _ => { unreachable!("Expected OpExecuteError::OperationError.") },
        };
        assert_eq!(error, InstructionFormatError::NotEnoughBytes {
            raw_opcode: 0x00,
            min: 4,
            actual: 3,
        });
    }
}

