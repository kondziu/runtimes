#![crate_name = "bytecode"]

mod interpreter;
mod bytecode;
mod objects;
mod types;
mod serializable;


#[cfg(test)]
mod deserialization_tests {
    use std::io::Cursor;
    use crate::bytecode::OpCode;
    use crate::serializable::Serializable;
    use crate::types::{ConstantPoolIndex, LocalFrameIndex, Size};

    macro_rules! test_deserialization {
        ($expected: expr, $input: expr) => {{
            let input: Vec<u8> = $input;
            let actual = OpCode::from_bytes(&mut Cursor::new(input));
            assert_eq!($expected, actual)
        }}
    }

    #[test] fn label () {
        test_deserialization!(
            OpCode::Label { name: ConstantPoolIndex::new(1) },
            vec!(0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01)
        )
    }

    #[test] fn literal () {
        test_deserialization!(
            OpCode::Literal { index: ConstantPoolIndex::new(1) },
            vec!(0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01)
        )
    }

    #[test] fn get_local () {
        test_deserialization!(
            OpCode::GetLocal { index: LocalFrameIndex::new(1) },
            vec!(0x0A, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01)
        )
    }

    #[test] fn set_local () {
        test_deserialization!(
            OpCode::SetLocal { index: LocalFrameIndex::new(1) },
            vec!(0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01)
        )
    }

    #[test] fn get_global () {
        test_deserialization!(
            OpCode::GetGlobal { name: ConstantPoolIndex::new(1) },
            vec!(0x0C, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01)
        )
    }

    #[test] fn set_global () {
        test_deserialization!(
            OpCode::SetGlobal { name: ConstantPoolIndex::new(1) },
            vec!(0x0B, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01)
        )
    }

    #[test] fn object () {
        test_deserialization!(
            OpCode::Object { class: ConstantPoolIndex::new(1) },
            vec!(0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01)
        )
    }

    #[test] fn array () {
        test_deserialization!(
            OpCode::Array { size: Size::new(1) },
            vec!(0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01)
        )
    }

    #[test] fn get_slot () {
        test_deserialization!(
            OpCode::GetSlot { name: ConstantPoolIndex::new(1) },
            vec!(0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01)
        )
    }

    #[test] fn set_slot () {
        test_deserialization!(
            OpCode::SetSlot { name: ConstantPoolIndex::new(1) },
            vec!(0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01)
        )
    }

    #[test] fn call_method () {
        test_deserialization!(
            OpCode::CallMethod { name: ConstantPoolIndex::new(1), arguments: Size::new(0) },
            vec!(0x07, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
                       0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00)
        )
    }

    #[test] fn call_function () {
        test_deserialization!(
            OpCode::CallFunction { function: ConstantPoolIndex::new(1), arguments: Size::new(0) },
            vec!(0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
                       0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00)
        )
    }

    #[test] fn print () {
        test_deserialization!(
            OpCode::Print { format: ConstantPoolIndex::new(1), arguments: Size::new(0) },
            vec!(0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
                       0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00)
        )
    }

    #[test] fn jump () {
        test_deserialization!(
            OpCode::Jump { label: ConstantPoolIndex::new(1) },
            vec!(0x0E, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01)
        )
    }

    #[test] fn branch () {
        test_deserialization!(
            OpCode::Branch { label: ConstantPoolIndex::new(1) },
            vec!(0x0D, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01)
        )
    }

    #[test] fn return_op () {
        test_deserialization!(
            OpCode::Return,
            vec!(0x0F)
        )
    }

    #[test] fn drop () {
        test_deserialization!(
            OpCode::Drop,
            vec!(0x10)
        )
    }
}

mod serialization_tests {
    use crate::bytecode::OpCode;
    use crate::serializable::Serializable;
    use crate::types::{ConstantPoolIndex, LocalFrameIndex, Size};

    macro_rules! test_serialization {
        ($expected: expr, $input: expr) => {{
            let mut output: Vec<u8> = Vec::new();
            let expected: Vec<u8> = $expected;
            $input.serialize(&mut output);
            assert_eq!(expected, output);
        }}
    }

    #[test] fn label () {
        test_serialization!(
            vec!(0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01),
            OpCode::Label { name: ConstantPoolIndex::new(1) }
        )
    }

    #[test] fn literal () {
        test_serialization!(
            vec!(0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01),
            OpCode::Literal { index: ConstantPoolIndex::new(1) }
        )
    }

    #[test] fn get_local () {
        test_serialization!(
            vec!(0x0A, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01),
            OpCode::GetLocal { index: LocalFrameIndex::new(1) }
        )
    }

    #[test] fn set_local () {
        test_serialization!(
            vec!(0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01),
            OpCode::SetLocal { index: LocalFrameIndex::new(1) }
        )
    }

    #[test] fn get_global () {
        test_serialization!(
            vec!(0x0C, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01),
            OpCode::GetGlobal { name: ConstantPoolIndex::new(1) }
        )
    }

    #[test] fn set_global () {
        test_serialization!(
            vec!(0x0B, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01),
            OpCode::SetGlobal { name: ConstantPoolIndex::new(1) }
        )
    }

    #[test] fn object () {
        test_serialization!(
            vec!(0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01),
            OpCode::Object { class: ConstantPoolIndex::new(1) }
        )
    }

    #[test] fn array () {
        test_serialization!(
            vec!(0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01),
            OpCode::Array { size: Size::new(1) }
        )
    }

    #[test] fn get_slot () {
        test_serialization!(
            vec!(0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01),
            OpCode::GetSlot { name: ConstantPoolIndex::new(1) }
        )
    }

    #[test] fn set_slot () {
        test_serialization!(
            vec!(0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01),
            OpCode::SetSlot { name: ConstantPoolIndex::new(1) }
        )
    }

    #[test] fn call_method () {
        test_serialization!(
            vec!(0x07, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
                       0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00),
            OpCode::CallMethod { name: ConstantPoolIndex::new(1), arguments: Size::new(0) }
        )
    }

    #[test] fn call_function () {
        test_serialization!(
            vec!(0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
                       0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00),
            OpCode::CallFunction { function: ConstantPoolIndex::new(1), arguments: Size::new(0) }
        )
    }

    #[test] fn print () {
        test_serialization!(
            vec!(0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
                       0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00),
            OpCode::Print { format: ConstantPoolIndex::new(1), arguments: Size::new(0) }
        )
    }

    #[test] fn jump () {
        test_serialization!(
            vec!(0x0E, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01),
            OpCode::Jump { label: ConstantPoolIndex::new(1) }
        )
    }

    #[test] fn branch () {
        test_serialization!(
            vec!(0x0D, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01),
            OpCode::Branch { label: ConstantPoolIndex::new(1) }
        )
    }

    #[test] fn return_op () {
        test_serialization!(
            vec!(0x0F),
            OpCode::Return
        )
    }

    #[test] fn drop () {
        test_serialization!(
            vec!(0x10),
            OpCode::Drop
        )
    }
}

fn main() {

}


