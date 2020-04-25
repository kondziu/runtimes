use crate::objects::ProgramObject;
use crate::types::{ConstantPoolIndex, Address, AddressRange};
use crate::serializable::{Serializable, SerializableWithContext};
use std::io::{Write, Read};
use crate::serializable;
use crate::bytecode::OpCode;

/**
 * The instruction pointer contains the address of the instruction that will be executed next.
 */
//pub struct InstructionPointer(u64);

/**
 * A listing of program objects. They can be referred to by their numerical index.
 */
//#[derive(PartialEq,Debug,Clone)]
//pub struct ConstantPool {
//    constants: Vec<ProgramObject>,
//}

//impl ConstantPool {
//    pub fn new(constants: Vec<ProgramObject>) -> ConstantPool {
//        ConstantPool {constants}
//    }
//}

//impl Serializable for ConstantPool {
//    fn serialize<W: Write>(&self, sink: &mut W) -> () {
//        serializable::write_usize_as_u16(sink, self.constants.len());
//        for constant in self.constants.iter() {
//            constant.serialize(sink)
//        }
//    }
//
//    fn from_bytes<R: Read>(input: &mut R) -> Self {
//        println!("ConstantPool::from_bytes");
//        let size = serializable::read_u16_as_usize(input);
//        let mut constants: Vec<ProgramObject> = Vec::new();
//        for _ in 0..size {
//            constants.push(ProgramObject::from_bytes(input))
//        }
//        ConstantPool { constants }
//    }
//}

/**
 * A listing of global variables and functions in the program. Each listing points to an object in
 * the constant pool and is **guaranteed** to refer to either:
 *   - a `ProgramObject::Slot` object, or
 *   - a `ProgramObject:Method` object.
 */
//#[derive(PartialEq,Debug,Clone)]
//pub struct GlobalSlots {
//    slots: Vec<ConstantPoolIndex>,
//}

//impl GlobalSlots {
//    pub fn new(slots: Vec<ConstantPoolIndex>) -> GlobalSlots {
//        GlobalSlots {slots}
//    }
//}
//
//impl Serializable for GlobalSlots {
//    fn serialize<W: Write>(&self, sink: &mut W) -> () {
//        ConstantPoolIndex::write_cpi_vector(sink, &self.slots)
//    }
//
//    fn from_bytes<R: Read>(input: &mut R) -> Self {
//        println!("GlobalSlots::from_bytes");
//        GlobalSlots { slots: ConstantPoolIndex::read_cpi_vector(input) }
//    }
//}

#[derive(PartialEq,Debug,Clone)]
pub struct Code {
    opcodes: Vec<OpCode>
}

impl Code {
    pub fn new() -> Code {
        Code { opcodes: Vec::new() }
    }

    pub fn from(opcodes: Vec<OpCode>) -> Code {
        Code { opcodes }
    }

    pub fn register_opcodes(&mut self, opcodes: Vec<OpCode>) -> AddressRange {
        let start = self.opcodes.len();
        let length = opcodes.len();
        self.opcodes.extend(opcodes);
        AddressRange::new(Address::from_usize(start), length)
    }

    pub fn addresses_to_code_vector(&self, range: &AddressRange) -> Vec<&OpCode> {
        let start = range.start().value_usize();
        let end = start + range.length();
        let mut result: Vec<&OpCode> = Vec::new();
        for i in start..end {
            result.push(&self.opcodes[i]);
        }
        result
    }

    pub fn next_address(&self, address: Option<Address>) -> Option<Address> {
        match address {
            Some(address) => {
                let new_address = Address::from_usize(address.value_usize() + 1);
                if self.opcodes.len() > new_address.value_usize() {
                    Some(new_address)
                } else {
                    None
                }
            }
            None => panic!("Cannot advance a nothing address.")
        }
    }

    pub fn get_opcode(&self, address: Address) -> Option<&OpCode> {
        //self.table[address.value() as usize]
        self.opcodes.get(address.value_usize())
    }
}

#[derive(PartialEq,Debug,Clone)]
pub struct Program {
    code: Code,
    constants: Vec<ProgramObject>,
    globals: Vec<ConstantPoolIndex>,
    entry: ConstantPoolIndex,
}

impl Program {
    pub fn new(code: Code,
               constants: Vec<ProgramObject>,
               globals: Vec<ConstantPoolIndex>,
               entry: ConstantPoolIndex) -> Program {

        Program { code, constants, globals, entry }
    }

    pub fn code(&self) -> &Code {
        &self.code
    }

    pub fn constants(&self) -> &Vec<ProgramObject> {
        &self.constants
    }

    pub fn globals(&self) -> &Vec<ConstantPoolIndex> {
        &self.globals
    }

    pub fn entry(&self) -> &ConstantPoolIndex {
        &self.entry
    }

    pub fn get_constant(&self, index: &ConstantPoolIndex) -> Option<&ProgramObject> {
        self.constants.get(index.value() as usize)
    }

    pub fn get_opcode(&self, address: Address) -> Option<&OpCode> {
        self.code.get_opcode(address)
    }
}

impl Serializable for Program {
    fn serialize<W: Write>(&self, sink: &mut W) -> () {

        serializable::write_usize_as_u16(sink, self.constants.len());
        for constant in self.constants.iter() {
            constant.serialize(sink, self.code())
        }

        ConstantPoolIndex::write_cpi_vector(sink, &self.globals);

        self.entry.serialize(sink);
    }

    fn from_bytes<R: Read>(input: &mut R) -> Self {
        println!("Program::from_bytes");

        let mut code = Code::new();

        let size = serializable::read_u16_as_usize(input);
        let mut constants: Vec<ProgramObject> = Vec::new();
        for _ in 0..size {
            constants.push(ProgramObject::from_bytes(input, &mut code))
        }

        Program {
            code,
            constants,
            globals: ConstantPoolIndex::read_cpi_vector(input),
            entry: ConstantPoolIndex::from_bytes(input),
        }
    }
}
