#![crate_name = "bytecode"]

mod interpreter;
mod bytecode;
mod objects;
mod types;
mod serializable;
mod program;

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

#[cfg(test)]
mod program_deserialization_and_deserialization_tests {

    use crate::program::{ConstantPool, GlobalSlots, Program};
    use crate::objects::ProgramObject;
    use crate::types::{Arity, Size, ConstantPoolIndex, Address, LocalFrameIndex};
    use crate::bytecode::OpCode;
    use std::io::Cursor;
    use crate::serializable::Serializable;

    macro_rules! test_deserialization {
        ($expected: expr, $input: expr) => {{
            let input: Vec<u8> = $input;
            let actual = Program::from_bytes(&mut Cursor::new(input));
            assert_eq!($expected, actual)
        }}
    }

    /* Feeny BC program (hello world)

        Constants :
        #0: String("Hello World\n")
        #1: String("main")
        #2: Method(#1, nargs:0, nlocals:0) :
            printf #0 0
            return
        #3: Null
        #4: String("entry35")
        #5: Method(#4, nargs:0, nlocals:0) :
            call #1 0
            drop
            lit #3
            return
        Globals :
        #2
        Entry : #5

        06 00 02 0C  00 00 00 48  65 6C 6C 6F  20 57 6F 72  6C 64 0A 02  04 00 00 00
        6D 61 69 6E  03 01 00 00  00 00 02 00  00 00 02 00  00 00 0F 01  02 07 00 00
        00 65 6E 74  72 79 33 35  03 04 00 00  00 00 04 00  00 00 08 01  00 00 10 01
        03 00 0F 01  00 02 00 05  00
    */

    #[test] fn hello_world () {
        let constants = vec!(
            /* #0 */ ProgramObject::String("Hello World\n".to_string()),
            /* #1 */ ProgramObject::String("main".to_string()),
            /* #2 */ ProgramObject::Method {
                name: ConstantPoolIndex::new(1),
                arguments: Arity::new(0),
                locals: Size::new(0),
                code: vec!(
                    OpCode::Print { format: ConstantPoolIndex::new(0),
                                    arguments: Arity::new(0) },
                    OpCode::Return
                )
            },
            /* #3 */ ProgramObject::Null,
            /* #4 */ ProgramObject::String("entry35".to_string()),
            /* #5 */ ProgramObject::Method {
                name: ConstantPoolIndex::new(4),
                arguments: Arity::new(0),
                locals: Size::new(0),
                code: vec!(
                    OpCode::CallFunction { function: ConstantPoolIndex::new(1),
                                           arguments: Arity::new(0) },
                    OpCode::Drop,
                    OpCode::Literal { index: ConstantPoolIndex::new(3) },
                    OpCode::Return
                )
            },
        );

        let globals = vec!(ConstantPoolIndex::new(2));
        let entry = ConstantPoolIndex::new(5);

        let program = Program::new (
            ConstantPool::new(constants),
            GlobalSlots::new(globals),
            entry
        );

        let bytes = vec!(
            0x06, 0x00, 0x02, 0x0C, 0x00, 0x00, 0x00, 0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x20, 0x57,
            0x6F, 0x72, 0x6C, 0x64, 0x0A, 0x02, 0x04, 0x00, 0x00, 0x00, 0x6D, 0x61, 0x69, 0x6E,
            0x03, 0x01, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00,
            0x0F, 0x01, 0x02, 0x07, 0x00, 0x00, 0x00, 0x65, 0x6E, 0x74, 0x72, 0x79, 0x33, 0x35,
            0x03, 0x04, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x08, 0x01, 0x00, 0x00,
            0x10, 0x01, 0x03, 0x00, 0x0F, 0x01, 0x00, 0x02, 0x00, 0x05, 0x00,
        );

        test_deserialization!(program, bytes);
    }

    /*  Another Feeny BC program (fibonacci)
        Constants :
           #0: String("conseq39")
           #1: String("end40")
           #2: Int(0)
           #3: String("eq")
           #4: String("conseq41")
           #5: String("end42")
           #6: Int(1)
           #7: String("test43")
           #8: String("loop44")
           #9: String("add")
           #10: String("sub")
           #11: Int(2)
           #12: String("ge")
           #13: Null
           #14: String("fib")
           #15: Method(#14, nargs:1, nlocals:3) :
                 get local 0
                 lit #2
                 call-slot #3 2
                 branch #0
                 get local 0
                 lit #6
                 call-slot #3 2
                 branch #4
                 lit #6
                 set local 1
                 drop
                 lit #6
                 set local 2
                 drop
                 goto #7
              label #8
                 get local 1
                 get local 2
                 call-slot #9 2
                 set local 3
                 drop
                 get local 2
                 set local 1
                 drop
                 get local 3
                 set local 2
                 drop
                 get local 0
                 lit #6
                 call-slot #10 2
                 set local 0
                 drop
              label #7
                 get local 0
                 lit #11
                 call-slot #12 2
                 branch #8
                 lit #13
                 drop
                 get local 2
                 goto #5
              label #4
                 lit #6
              label #5
                 goto #1
              label #0
                 lit #6
              label #1
                 return
           #16: String("test45")
           #17: String("loop46")
           #18: String("Fib(~) = ~\n")
           #19: Int(20)
           #20: String("lt")
           #21: String("main")
           #22: Method(#21, nargs:0, nlocals:1) :
                 lit #2
                 set local 0
                 drop
                 goto #16
              label #17
                 get local 0
                 get local 0
                 call #14 1
                 printf #18 2
                 drop
                 get local 0
                 lit #6
                 call-slot #9 2
                 set local 0
                 drop
              label #16
                 get local 0
                 lit #19
                 call-slot #20 2
                 branch #17
                 lit #13
                 return
           #23: String("entry47")
           #24: Method(#23, nargs:0, nlocals:0) :
                 call #21 0
                 drop
                 lit #13
                 return
        Globals :
           #15
           #22
        Entry : #24

        19 00 02 08 00 00 00 63 6F 6E 73 65 71 33 39 02 05 00 00 00 65 6E 64 34 30 00 00 00 00 00
        02 02 00 00 00 65 71 02 08 00 00 00 63 6F 6E 73 65 71 34 31 02 05 00 00 00 65 6E 64 34 32
        00 01 00 00 00 02 06 00 00 00 74 65 73 74 34 33 02 06 00 00 00 6C 6F 6F 70 34 34 02 03 00
        00 00 61 64 64 02 03 00 00 00 73 75 62 00 02 00 00 00 02 02 00 00 00 67 65 01 02 03 00 00
        00 66 69 62 03 0E 00 01 03 00 31 00 00 00 0A 00 00 01 02 00 07 03 00 02 0D 00 00 0A 00 00
        01 06 00 07 03 00 02 0D 04 00 01 06 00 09 01 00 10 01 06 00 09 02 00 10 0E 07 00 00 08 00
        0A 01 00 0A 02 00 07 09 00 02 09 03 00 10 0A 02 00 09 01 00 10 0A 03 00 09 02 00 10 0A 00
        00 01 06 00 07 0A 00 02 09 00 00 10 00 07 00 0A 00 00 01 0B 00 07 0C 00 02 0D 08 00 01 0D
        00 10 0A 02 00 0E 05 00 00 04 00 01 06 00 00 05 00 0E 01 00 00 00 00 01 06 00 00 01 00 0F
        02 06 00 00 00 74 65 73 74 34 35 02 06 00 00 00 6C 6F 6F 70 34 36 02 0B 00 00 00 46 69 62
        28 7E 29 20 3D 20 7E 0A 00 14 00 00 00 02 02 00 00 00 6C 74 02 04 00 00 00 6D 61 69 6E 03
        15 00 00 01 00 16 00 00 00 01 02 00 09 00 00 10 0E 10 00 00 11 00 0A 00 00 0A 00 00 08 0E
        00 01 02 12 00 02 10 0A 00 00 01 06 00 07 09 00 02 09 00 00 10 00 10 00 0A 00 00 01 13 00
        07 14 00 02 0D 11 00 01 0D 00 0F 02 07 00 00 00 65 6E 74 72 79 34 37 03 17 00 00 00 00 04
        00 00 00 08 15 00 00 10 01 0D 00 0F 02 00 0F 00 16 00 18 00
    */
    #[test] fn fibonacci () {

        let constants = vec!(
        /* #0  0x00 */ ProgramObject::String("conseq39".to_string()),
        /* #1  0x01 */ ProgramObject::String("end40".to_string()),
        /* #2  0x02 */ ProgramObject::Integer(0),
        /* #3  0x03 */ ProgramObject::String("eq".to_string()),
        /* #4  0x04 */ ProgramObject::String("conseq41".to_string()),
        /* #5  0x05 */ ProgramObject::String("end42".to_string()),
        /* #6  0x06 */ ProgramObject::Integer(1),
        /* #7  0x07 */ ProgramObject::String("test43".to_string()),
        /* #8  0x08 */ ProgramObject::String("loop44".to_string()),
        /* #9  0x09 */ ProgramObject::String("add".to_string()),
        /* #10 0x0A */ ProgramObject::String("sub".to_string()),
        /* #11 0x0B */ ProgramObject::Integer(2),
        /* #12 0x0C */ ProgramObject::String("ge".to_string()),
        /* #13 0x0D */ ProgramObject::Null,
        /* #14 0x0E */ ProgramObject::String("fib".to_string()),
        /* #15 0x0F */ ProgramObject::Method {                             // fib
                name: ConstantPoolIndex::new(14),
                arguments: Arity::new(1),
                locals: Size::new(3),
                code: vec!(
                    OpCode::GetLocal { index: LocalFrameIndex::new(0) },   // arg0
                    OpCode::Literal { index: ConstantPoolIndex::new(2) },  // 0
                    OpCode::CallMethod { name: ConstantPoolIndex::new(3),  // 0.eq(arg0)
                                         arguments: Arity::new(2) },
                    OpCode::Branch { label: ConstantPoolIndex::new(0) },   // branch conseq39
                    OpCode::GetLocal { index: LocalFrameIndex::new(0) },   // also x
                    OpCode::Literal { index: ConstantPoolIndex::new(6) },  // 1
                    OpCode::CallMethod { name: ConstantPoolIndex::new(3),  // arg0.eq(1)
                                         arguments: Arity::new(2) },
                    OpCode::Branch { label: ConstantPoolIndex::new(4) },   // branch conseq41
                    OpCode::Literal { index: ConstantPoolIndex::new(6) },  // 1
                    OpCode::SetLocal { index: LocalFrameIndex::new(1) },   // var1 = 1
                    OpCode::Drop,
                    OpCode::Literal { index: ConstantPoolIndex::new(6) },  // 1
                    OpCode::SetLocal { index: LocalFrameIndex::new(2) },   // var2 = 1
                    OpCode::Drop,
                    OpCode::Jump { label: ConstantPoolIndex::new(7) },     // goto test43

                    OpCode::Label { name: ConstantPoolIndex::new(8) },     // label loop44
                    OpCode::GetLocal { index: LocalFrameIndex::new(1) },   // var1
                    OpCode::GetLocal { index: LocalFrameIndex::new(2) },   // var2
                    OpCode::CallMethod { name: ConstantPoolIndex::new(9),  // var1.add(var2) -> result1
                                         arguments: Arity::new(2) },
                    OpCode::SetLocal { index: LocalFrameIndex::new(3) },   // var3 = result1
                    OpCode::Drop,
                    OpCode::GetLocal { index: LocalFrameIndex::new(2) },   // var2
                    OpCode::SetLocal { index: LocalFrameIndex::new(1) },   // var1 = var2
                    OpCode::Drop,
                    OpCode::GetLocal { index: LocalFrameIndex::new(3) },   // var3
                    OpCode::SetLocal { index: LocalFrameIndex::new(2) },   // var2 = var3
                    OpCode::Drop,
                    OpCode::GetLocal { index: LocalFrameIndex::new(0) },   // arg0
                    OpCode::Literal { index: ConstantPoolIndex::new(6) },  // 1
                    OpCode::CallMethod { name: ConstantPoolIndex::new(10), // arg0.sub(1) -> result2
                                         arguments: Arity::new(2) },
                    OpCode::SetLocal { index: LocalFrameIndex::new(0) },   // arg0 = result2
                    OpCode::Drop,
                    OpCode::Label { name: ConstantPoolIndex::new(7) },     // label test43
                    OpCode::GetLocal { index: LocalFrameIndex::new(0) },   // arg0
                    OpCode::Literal { index: ConstantPoolIndex::new(11) }, // 2
                    OpCode::CallMethod { name: ConstantPoolIndex::new(12), // arg0.ge(2) -> result3
                                         arguments: Arity::new(2) },
                    OpCode::Branch { label: ConstantPoolIndex::new(8) },   // loop44
                    OpCode::Literal { index: ConstantPoolIndex::new(13) }, // null
                    OpCode::Drop,
                    OpCode::GetLocal { index: LocalFrameIndex::new(2) },   // arg2 (return arg2)
                    OpCode::Jump { label: ConstantPoolIndex::new(5) },     // goto end42
                    OpCode::Label { name: ConstantPoolIndex::new(4) },     // label conseq41
                    OpCode::Literal { index: ConstantPoolIndex::new(6) },  // 1 (return 1)
                    OpCode::Label { name: ConstantPoolIndex::new(5) },     // label end42
                    OpCode::Jump { label: ConstantPoolIndex::new(1) },     // goto end40
                    OpCode::Label { name: ConstantPoolIndex::new(0) },     // label conseq39
                    OpCode::Literal { index: ConstantPoolIndex::new(6) },  // 1 (return 1)
                    OpCode::Label { name: ConstantPoolIndex::new(1) },     // label end40
                    OpCode::Return
                )
            },
        /* #16 0x10 */ ProgramObject::String("test45".to_string()),
        /* #17 0x11 */ ProgramObject::String("loop46".to_string()),
        /* #18 0x11 */ ProgramObject::String("Fib(~) = ~\n".to_string()),
        /* #19 0x12 */ ProgramObject::Integer(20),
        /* #20 0x13 */ ProgramObject::String("lt".to_string()),
        /* #21 0x14 */ ProgramObject::String("main".to_string()),
        /* #22 0x15 */ ProgramObject::Method {                             // main
                name: ConstantPoolIndex::new(21),
                arguments: Arity::new(0),
                locals: Size::new(1),
                code: vec!(
                    OpCode::Literal { index: ConstantPoolIndex::new(2) },  // 0
                    OpCode::SetLocal { index: LocalFrameIndex::new(0) },   // var0 = 0
                    OpCode::Drop,
                    OpCode::Jump { label: ConstantPoolIndex::new(16) },    // goto loop45
                    OpCode::Label { name: ConstantPoolIndex::new(17) },    // label loop46
                    OpCode::GetLocal { index: LocalFrameIndex::new(0) },   // var0
                    OpCode::GetLocal { index: LocalFrameIndex::new(0) },   // var0 ... again?
                    OpCode::CallFunction {                                 // fib(var0) -> result1
                        function: ConstantPoolIndex::new(14),
                        arguments: Arity::new(1) },
                    OpCode::Print {                                        // printf "Fib(~) = ~\n" var0 result1
                        format: ConstantPoolIndex::new(18),
                        arguments: Arity::new(2) },
                    OpCode::Drop,
                    OpCode::GetLocal { index: LocalFrameIndex::new(0) },    // var0
                    OpCode::Literal { index: ConstantPoolIndex::new(6) },   // 1
                    OpCode::CallMethod {                                    // var0.add(1) -> result2
                        name: ConstantPoolIndex::new(9),
                        arguments: Arity::new(2) },
                    OpCode::SetLocal { index: LocalFrameIndex::new(0) },    // var0 = result2
                    OpCode::Drop,
                    OpCode::Label { name: ConstantPoolIndex::new(16) },     // label test45
                    OpCode::GetLocal { index: LocalFrameIndex::new(0) },    // var0
                    OpCode::Literal { index: ConstantPoolIndex::new(19) },  // 20
                    OpCode::CallMethod {                                    // var0.lt(20) -> result3
                        name: ConstantPoolIndex::new(20),
                        arguments: Arity::new(2) },
                    OpCode::Branch { label: ConstantPoolIndex::new(17) },   // branch loop46
                    OpCode::Literal { index: ConstantPoolIndex::new(13) },  // null
                    OpCode::Return,
                )
            },
        /* #23 0x15 */ ProgramObject::String("entry47".to_string()),
        /* #24 0x16 */ ProgramObject::Method {                             // entry47
                name: ConstantPoolIndex::new(23),
                arguments: Arity::new(0),
                locals: Size::new(0),
                code: vec!(
                    OpCode::CallFunction {                                 // main() -> result0
                        function: ConstantPoolIndex::new(21),
                        arguments: Arity::new(0) },
                    OpCode::Drop,
                    OpCode::Literal { index: ConstantPoolIndex::new(13) }, // null
                    OpCode::Return
                )
            }
        );

        let globals = vec!(
            ConstantPoolIndex::new(15),
            ConstantPoolIndex::new(22)
        );
        let entry = ConstantPoolIndex::new(24);

        let program = Program::new (
            ConstantPool::new(constants),
            GlobalSlots::new(globals),
            entry
        );

        let bytes = vec!(
            0x19, 0x00, 0x02, 0x08, 0x00, 0x00, 0x00, 0x63, 0x6F, 0x6E, 0x73, 0x65, 0x71, 0x33,
            0x39, 0x02, 0x05, 0x00, 0x00, 0x00, 0x65, 0x6E, 0x64, 0x34, 0x30, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x02, 0x02, 0x00, 0x00, 0x00, 0x65, 0x71, 0x02, 0x08, 0x00, 0x00, 0x00,
            0x63, 0x6F, 0x6E, 0x73, 0x65, 0x71, 0x34, 0x31, 0x02, 0x05, 0x00, 0x00, 0x00, 0x65,
            0x6E, 0x64, 0x34, 0x32, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02, 0x06, 0x00, 0x00, 0x00,
            0x74, 0x65, 0x73, 0x74, 0x34, 0x33, 0x02, 0x06, 0x00, 0x00, 0x00, 0x6C, 0x6F, 0x6F,
            0x70, 0x34, 0x34, 0x02, 0x03, 0x00, 0x00, 0x00, 0x61, 0x64, 0x64, 0x02, 0x03, 0x00,
            0x00, 0x00, 0x73, 0x75, 0x62, 0x00, 0x02, 0x00, 0x00, 0x00, 0x02, 0x02, 0x00, 0x00,
            0x00, 0x67, 0x65, 0x01, 0x02, 0x03, 0x00, 0x00, 0x00, 0x66, 0x69, 0x62, 0x03, 0x0E,
            0x00, 0x01, 0x03, 0x00, 0x31, 0x00, 0x00, 0x00, 0x0A, 0x00, 0x00, 0x01, 0x02, 0x00,
            0x07, 0x03, 0x00, 0x02, 0x0D, 0x00, 0x00, 0x0A, 0x00, 0x00, 0x01, 0x06, 0x00, 0x07,
            0x03, 0x00, 0x02, 0x0D, 0x04, 0x00, 0x01, 0x06, 0x00, 0x09, 0x01, 0x00, 0x10, 0x01,
            0x06, 0x00, 0x09, 0x02, 0x00, 0x10, 0x0E, 0x07, 0x00, 0x00, 0x08, 0x00, 0x0A, 0x01,
            0x00, 0x0A, 0x02, 0x00, 0x07, 0x09, 0x00, 0x02, 0x09, 0x03, 0x00, 0x10, 0x0A, 0x02,
            0x00, 0x09, 0x01, 0x00, 0x10, 0x0A, 0x03, 0x00, 0x09, 0x02, 0x00, 0x10, 0x0A, 0x00,
            0x00, 0x01, 0x06, 0x00, 0x07, 0x0A, 0x00, 0x02, 0x09, 0x00, 0x00, 0x10, 0x00, 0x07,
            0x00, 0x0A, 0x00, 0x00, 0x01, 0x0B, 0x00, 0x07, 0x0C, 0x00, 0x02, 0x0D, 0x08, 0x00,
            0x01, 0x0D, 0x00, 0x10, 0x0A, 0x02, 0x00, 0x0E, 0x05, 0x00, 0x00, 0x04, 0x00, 0x01,
            0x06, 0x00, 0x00, 0x05, 0x00, 0x0E, 0x01, 0x00, 0x00, 0x00, 0x00, 0x01, 0x06, 0x00,
            0x00, 0x01, 0x00, 0x0F, 0x02, 0x06, 0x00, 0x00, 0x00, 0x74, 0x65, 0x73, 0x74, 0x34,
            0x35, 0x02, 0x06, 0x00, 0x00, 0x00, 0x6C, 0x6F, 0x6F, 0x70, 0x34, 0x36, 0x02, 0x0B,
            0x00, 0x00, 0x00, 0x46, 0x69, 0x62, 0x28, 0x7E, 0x29, 0x20, 0x3D, 0x20, 0x7E, 0x0A,
            0x00, 0x14, 0x00, 0x00, 0x00, 0x02, 0x02, 0x00, 0x00, 0x00, 0x6C, 0x74, 0x02, 0x04,
            0x00, 0x00, 0x00, 0x6D, 0x61, 0x69, 0x6E, 0x03, 0x15, 0x00, 0x00, 0x01, 0x00, 0x16,
            0x00, 0x00, 0x00, 0x01, 0x02, 0x00, 0x09, 0x00, 0x00, 0x10, 0x0E, 0x10, 0x00, 0x00,
            0x11, 0x00, 0x0A, 0x00, 0x00, 0x0A, 0x00, 0x00, 0x08, 0x0E, 0x00, 0x01, 0x02, 0x12,
            0x00, 0x02, 0x10, 0x0A, 0x00, 0x00, 0x01, 0x06, 0x00, 0x07, 0x09, 0x00, 0x02, 0x09,
            0x00, 0x00, 0x10, 0x00, 0x10, 0x00, 0x0A, 0x00, 0x00, 0x01, 0x13, 0x00, 0x07, 0x14,
            0x00, 0x02, 0x0D, 0x11, 0x00, 0x01, 0x0D, 0x00, 0x0F, 0x02, 0x07, 0x00, 0x00, 0x00,
            0x65, 0x6E, 0x74, 0x72, 0x79, 0x34, 0x37, 0x03, 0x17, 0x00, 0x00, 0x00, 0x00, 0x04,
            0x00, 0x00, 0x00, 0x08, 0x15, 0x00, 0x00, 0x10, 0x01, 0x0D, 0x00, 0x0F, 0x02, 0x00,
            0x0F, 0x00, 0x16, 0x00, 0x18, 0x00,
        );

        test_deserialization!(program, bytes);
    }


}

fn main() {

}


