use std::collections::VecDeque;
use crate::vm::thread::data_stack::DataStack;

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
    let option = super::execute(&bytes, &mut data_stack);
    assert!(option.is_ok());

    // Test Result value
    let delta = option.unwrap();
    assert_eq!(delta.inst_bytes_consumed, 4);
    assert_eq!(delta.data_bytes_pushed, 1);

    // Check data stack
    // 21 + 3 = 24 (unsigned)
    assert_eq!(data_stack.pop8().unwrap(), 0x18);
}
