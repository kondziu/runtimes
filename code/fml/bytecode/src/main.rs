#![crate_name = "bytecode"]

mod interpreter;
mod bytecode;
mod objects;
mod types;
mod serializable;
mod parser;

#[cfg(test)]
mod bytecode_deserialization_tests {
    use std::io::Cursor;
    use crate::bytecode::OpCode;
    use crate::serializable::Serializable;
    use crate::types::{ConstantPoolIndex, LocalFrameIndex, Size, Arity};

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
            vec!(0x00, 0x01, 0x00, 0x00, 0x00)
        )
    }

    #[test] fn literal () {
        test_deserialization!(
            OpCode::Literal { index: ConstantPoolIndex::new(1) },
            vec!(0x01, 0x01, 0x00, 0x00, 0x00)
        )
    }

    #[test] fn get_local () {
        test_deserialization!(
            OpCode::GetLocal { index: LocalFrameIndex::new(1) },
            vec!(0x0A, 0x01, 0x00, 0x00, 0x00)
        )
    }

    #[test] fn set_local () {
        test_deserialization!(
            OpCode::SetLocal { index: LocalFrameIndex::new(1) },
            vec!(0x09, 0x01, 0x00, 0x00, 0x00)
        )
    }

    #[test] fn get_global () {
        test_deserialization!(
            OpCode::GetGlobal { name: ConstantPoolIndex::new(1) },
            vec!(0x0C, 0x01, 0x00, 0x00, 0x00)
        )
    }

    #[test] fn set_global () {
        test_deserialization!(
            OpCode::SetGlobal { name: ConstantPoolIndex::new(1) },
            vec!(0x0B, 0x01, 0x00, 0x00, 0x00)
        )
    }

    #[test] fn object () {
        test_deserialization!(
            OpCode::Object { class: ConstantPoolIndex::new(1) },
            vec!(0x04, 0x01, 0x00, 0x00, 0x00)
        )
    }

    #[test] fn array () {
        test_deserialization!(
            OpCode::Array { size: Size::new(1) },
            vec!(0x03, 0x01, 0x00, 0x00, 0x00)
        )
    }

    #[test] fn get_slot () {
        test_deserialization!(
            OpCode::GetSlot { name: ConstantPoolIndex::new(1) },
            vec!(0x05, 0x01, 0x00, 0x00, 0x00)
        )
    }

    #[test] fn set_slot () {
        test_deserialization!(
            OpCode::SetSlot { name: ConstantPoolIndex::new(1) },
            vec!(0x06, 0x01, 0x00, 0x00, 0x00)
        )
    }

    #[test] fn call_method () {
        test_deserialization!(
            OpCode::CallMethod { name: ConstantPoolIndex::new(1), arguments: Arity::new(2) },
            vec!(0x07, 0x01, 0x00, 0x00, 0x00, 0x02)
        )
    }

    #[test] fn call_function () {
        test_deserialization!(
            OpCode::CallFunction { function: ConstantPoolIndex::new(1), arguments: Arity::new(2) },
            vec!(0x08, 0x01, 0x00, 0x00, 0x00, 0x02)
        )
    }

    #[test] fn print () {
        test_deserialization!(
            OpCode::Print { format: ConstantPoolIndex::new(1), arguments: Arity::new(2) },
            vec!(0x02, 0x01, 0x00, 0x00, 0x00, 0x02)
        )
    }

    #[test] fn jump () {
        test_deserialization!(
            OpCode::Jump { label: ConstantPoolIndex::new(1) },
            vec!(0x0E, 0x01, 0x00, 0x00, 0x00)
        )
    }

    #[test] fn branch () {
        test_deserialization!(
            OpCode::Branch { label: ConstantPoolIndex::new(1) },
            vec!(0x0D, 0x01, 0x00, 0x00, 0x00)
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

mod bytecode_serialization_tests {
    use crate::bytecode::OpCode;
    use crate::serializable::Serializable;
    use crate::types::{ConstantPoolIndex, LocalFrameIndex, Size, Arity};

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
            vec!(0x00, 0x01, 0x00, 0x00, 0x00),
            OpCode::Label { name: ConstantPoolIndex::new(1) }
        )
    }

    #[test] fn literal () {
        test_serialization!(
            vec!(0x01, 0x01, 0x00, 0x00, 0x00),
            OpCode::Literal { index: ConstantPoolIndex::new(1) }
        )
    }

    #[test] fn get_local () {
        test_serialization!(
            vec!(0x0A, 0x01, 0x00, 0x00, 0x00),
            OpCode::GetLocal { index: LocalFrameIndex::new(1) }
        )
    }

    #[test] fn set_local () {
        test_serialization!(
            vec!(0x09, 0x01, 0x00, 0x00, 0x00),
            OpCode::SetLocal { index: LocalFrameIndex::new(1) }
        )
    }

    #[test] fn get_global () {
        test_serialization!(
            vec!(0x0C, 0x01, 0x00, 0x00, 0x00),
            OpCode::GetGlobal { name: ConstantPoolIndex::new(1) }
        )
    }

    #[test] fn set_global () {
        test_serialization!(
            vec!(0x0B, 0x01, 0x00, 0x00, 0x00),
            OpCode::SetGlobal { name: ConstantPoolIndex::new(1) }
        )
    }

    #[test] fn object () {
        test_serialization!(
            vec!(0x04, 0x01, 0x00, 0x00, 0x00),
            OpCode::Object { class: ConstantPoolIndex::new(1) }
        )
    }

    #[test] fn array () {
        test_serialization!(
            vec!(0x03, 0x01, 0x00, 0x00, 0x00),
            OpCode::Array { size: Size::new(1) }
        )
    }

    #[test] fn get_slot () {
        test_serialization!(
            vec!(0x05, 0x01, 0x00, 0x00, 0x00),
            OpCode::GetSlot { name: ConstantPoolIndex::new(1) }
        )
    }

    #[test] fn set_slot () {
        test_serialization!(
            vec!(0x06, 0x01, 0x00, 0x00, 0x00),
            OpCode::SetSlot { name: ConstantPoolIndex::new(1) }
        )
    }

    #[test] fn call_method () {
        test_serialization!(
            vec!(0x07, 0x01, 0x00, 0x00, 0x00, 0x02),
            OpCode::CallMethod { name: ConstantPoolIndex::new(1), arguments: Arity::new(2) }
        )
    }

    #[test] fn call_function () {
        test_serialization!(
            vec!(0x08, 0x01, 0x00, 0x00, 0x00, 0x02),
            OpCode::CallFunction { function: ConstantPoolIndex::new(1), arguments: Arity::new(2) }
        )
    }

    #[test] fn print () {
        test_serialization!(
            vec!(0x02, 0x01, 0x00, 0x00, 0x00, 0x02),
            OpCode::Print { format: ConstantPoolIndex::new(1), arguments: Arity::new(2) }
        )
    }

    #[test] fn jump () {
        test_serialization!(
            vec!(0x0E, 0x01, 0x00, 0x00, 0x00),
            OpCode::Jump { label: ConstantPoolIndex::new(1) }
        )
    }

    #[test] fn branch () {
        test_serialization!(
            vec!(0x0D, 0x01, 0x00, 0x00, 0x00),
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

mod program_object_serialization_tests {
    use crate::bytecode::OpCode;
    use crate::serializable::Serializable;
    use crate::types::{ConstantPoolIndex, Size, Arity};
    use crate::objects::ProgramObject;

    macro_rules! test_serialization {
        ($expected: expr, $input: expr) => {{
            let mut output: Vec<u8> = Vec::new();
            let expected: Vec<u8> = $expected;
            $input.serialize(&mut output);
            assert_eq!(expected, output);
        }}
    }

    #[test] fn null () {
        test_serialization!(
            vec!(0x01),
            ProgramObject::Null
        )
    }

    #[test] fn integer () {
        test_serialization!(
            vec!(0x00, 0x2A, 0x00, 0x00, 0x00),
            ProgramObject::Integer(42)
        )
    }

    #[test] fn boolean () {
        test_serialization!(
            vec!(0x06, 0x01),
            ProgramObject::Boolean(true)
        )
    }

    #[test] fn string () {
        test_serialization!(
            vec!(0x02,
                 0x0C, 0x00, 0x00, 0x00,
                 0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x20, 0x57, 0x6F, 0x72, 0x6C, 0x64, 0x00),
            ProgramObject::String("Hello World\0".to_string())
        )
    }

    #[test] fn slot () {
        test_serialization!(
            vec!(0x03,
                 0x2A, 0x00, 0x00, 0x00),
            ProgramObject::Slot {name: ConstantPoolIndex::new(42)}
        )
    }

    #[test] fn class () {
        test_serialization!(
            vec!(0x05,
                 0x02, 0x00, 0x00, 0x00,
                 0x2A, 0x00, 0x00, 0x00,
                 0x9A, 0x02, 0x00, 0x00),
            ProgramObject::Class(vec!(ConstantPoolIndex::new(42), ConstantPoolIndex::new(666)))
        )
    }

    #[test] fn method () {
        test_serialization!(
            vec!(0x04,
                 0xFF, 0x00, 0x00, 0x00,
                 0x03,
                 0x0F, 0x00, 0x00, 0x00,
                 0x02, 0x00, 0x00, 0x00,
                 0x01,
                 0x2A, 0x00, 0x00, 0x00,
                 0x0F),

            ProgramObject::Method {
                name: ConstantPoolIndex::new(255),
                arguments: Arity::new(3),
                locals: Size::new(15),
                code: vec!(OpCode::Literal { index: ConstantPoolIndex::new(42) },
                           OpCode::Return),
            }
        )
    }
}

mod program_object_deserialization_tests {
    use crate::bytecode::OpCode;
    use crate::serializable::Serializable;
    use crate::types::{ConstantPoolIndex, Size, Arity};
    use crate::objects::ProgramObject;
    use std::io::Cursor;

    macro_rules! test_deserialization {
        ($expected: expr, $input: expr) => {{
            let input: Vec<u8> = $input;
            let actual = ProgramObject::from_bytes(&mut Cursor::new(input));
            assert_eq!($expected, actual)
        }}
    }

//    macro_rules! test_deserialization {
//        ($expected: expr, $input: expr) => {{
//            let mut output: Vec<u8> = Vec::new();
//            let expected: Vec<u8> = $expected;
//            $input.serialize(&mut output);
//            assert_eq!(expected, output);
//        }}
//    }

    #[test] fn null () {
        test_deserialization!(
            ProgramObject::Null,
            vec!(0x01)
        )
    }

    #[test] fn integer () {
        test_deserialization!(
            ProgramObject::Integer(42),
            vec!(0x00, 0x2A, 0x00, 0x00, 0x00)
        )
    }

    #[test] fn boolean () {
        test_deserialization!(
            ProgramObject::Boolean(true),
            vec!(0x06, 0x01)
        )
    }

    #[test] fn string () {
        test_deserialization!(
            ProgramObject::String("Hello World\0".to_string()),
            vec!(0x02,
                 0x0C, 0x00, 0x00, 0x00,
                 0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x20, 0x57, 0x6F, 0x72, 0x6C, 0x64, 0x00)
        )
    }

    #[test] fn slot () {
        test_deserialization!(
            ProgramObject::Slot {name: ConstantPoolIndex::new(42)},
            vec!(0x03,
                 0x2A, 0x00, 0x00, 0x00)
        )
    }

    #[test] fn class () {
        test_deserialization!(
            ProgramObject::Class(vec!(ConstantPoolIndex::new(42), ConstantPoolIndex::new(666))),
            vec!(0x05,
                 0x02, 0x00, 0x00, 0x00,
                 0x2A, 0x00, 0x00, 0x00,
                 0x9A, 0x02, 0x00, 0x00)
        )
    }

    #[test] fn method () {
        test_deserialization!(
            ProgramObject::Method {
                name: ConstantPoolIndex::new(255),
                arguments: Arity::new(3),
                locals: Size::new(15),
                code: vec!(OpCode::Literal { index: ConstantPoolIndex::new(42) },
                           OpCode::Return),
            },

            vec!(0x04,
                 0xFF, 0x00, 0x00, 0x00,
                 0x03,
                 0x0F, 0x00, 0x00, 0x00,
                 0x02, 0x00, 0x00, 0x00,
                 0x01,
                 0x2A, 0x00, 0x00, 0x00,
                 0x0F)
        )
    }
}

fn main() {

}


