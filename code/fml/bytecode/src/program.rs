use crate::objects::ProgramObject;
use crate::types::{ConstantPoolIndex, Address};
use crate::serializable::Serializable;
use std::io::{Write, Read};
use crate::serializable;

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
pub struct Program {
    constants: Vec<ProgramObject>,
    globals: Vec<ConstantPoolIndex>,
    entry: ConstantPoolIndex,
}

impl Program {
    pub fn new(constants: Vec<ProgramObject>,
               globals: Vec<ConstantPoolIndex>,
               entry: ConstantPoolIndex) -> Program {

        Program {constants, globals, entry}
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
}

impl Serializable for Program {
    fn serialize<W: Write>(&self, sink: &mut W) -> () {

        serializable::write_usize_as_u16(sink, self.constants.len());
        for constant in self.constants.iter() {
            constant.serialize(sink)
        }

        ConstantPoolIndex::write_cpi_vector(sink, &self.globals);

        self.entry.serialize(sink);
    }

    fn from_bytes<R: Read>(input: &mut R) -> Self {
        println!("Program::from_bytes");

        let size = serializable::read_u16_as_usize(input);
        let mut constants: Vec<ProgramObject> = Vec::new();
        for _ in 0..size {
            constants.push(ProgramObject::from_bytes(input))
        }

        Program { constants,
            globals: ConstantPoolIndex::read_cpi_vector(input),
            entry: ConstantPoolIndex::from_bytes(input),
        }
    }
}
