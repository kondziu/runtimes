use crate::serializable::Serializable;
use crate::serializable;
use std::io::{Read, Write};

#[derive(PartialEq,Debug,Copy,Clone)] pub struct Arity(u8);
#[derive(PartialEq,Debug,Copy,Clone)] pub struct Size(u32);
#[derive(PartialEq,Debug,Copy,Clone)] pub struct Address(u32);
#[derive(PartialEq,Debug,Copy,Clone)] pub struct ConstantPoolIndex(u32);
#[derive(PartialEq,Debug,Copy,Clone)] pub struct LocalFrameIndex(u32);

impl Arity             { pub fn new(value: u8)  -> Arity             { Arity(value)             }}
impl Size              { pub fn new(value: u32) -> Size              { Size(value)              }}
impl Address           { pub fn new(value: u32) -> Address           { Address(value)           }}
impl LocalFrameIndex   { pub fn new(value: u32) -> LocalFrameIndex   { LocalFrameIndex(value)   }}
impl ConstantPoolIndex { pub fn new(value: u32) -> ConstantPoolIndex { ConstantPoolIndex(value) }}

impl ConstantPoolIndex {
    pub fn read_cpi_vector<R: Read>(input: &mut R) -> Vec<ConstantPoolIndex> {
        serializable::read_u32_vector(input)
            .into_iter()
            .map(ConstantPoolIndex::new)
            .collect()
    }

    pub fn write_cpi_vector<R: Write>(sink: &mut R, vector: &Vec<ConstantPoolIndex>) -> () {
        let vector_of_u32s: Vec<u32> = vector.iter().map(|cpi| cpi.0).collect();
        serializable::write_u32_vector(sink, &vector_of_u32s)
    }
}

impl Serializable for Arity {

    fn serialize<W: Write> (&self, sink: &mut W) -> () {
        serializable::write_u8(sink, self.0)
    }

    fn from_bytes<R: Read>(input: &mut R) -> Self {
        Arity(serializable::read_u8(input))
    }
}

impl Serializable for Size {

    fn serialize<W: Write> (&self, sink: &mut W) -> () {
//        let size_of_type = std::mem::size_of::<usize>();
//        match size_of_type {
//            1 => serializable::write_u8(sink, self.0 as u8),
//            4 => serializable::write_u32(sink, self.0 as u32),
//            8 => serializable::write_u64(sink, self.0 as u64),
//            _ => panic!("Cannot serialize: sizeof::<usize> == {} \
//                         but only 1, 4, and 8 are supported", size_of_type),
//        }
        serializable::write_u32(sink, self.0)
    }

    fn from_bytes<R: Read>(input: &mut R) -> Self {
//        let size_of_type = std::mem::size_of::<usize>();
//        let value: usize = match size_of_type {
//            1 => serializable::read_u8(input) as usize,
//            4 => serializable::read_u32(input) as usize,
//            8 => serializable::read_u64(input) as usize,
//            _ => panic!("Cannot deserialize: sizeof::<usize> == {} \
//                         but only 1, 4, and 8 are supported", size_of_type),
//        };
        Size(serializable::read_u32(input))
    }
}

impl Serializable for Address {
    fn serialize<W: Write> (&self, sink: &mut W) -> () {
        serializable::write_u32(sink, self.0)
    }
    fn from_bytes<R: Read>(input: &mut R) -> Self {
        Address(serializable::read_u32(input))
    }
}

impl Serializable for ConstantPoolIndex {
    fn serialize<W: Write> (&self, sink: &mut W) -> () {
        serializable::write_u32(sink, self.0)
    }
    fn from_bytes<R: Read>(input: &mut R) -> Self {
        ConstantPoolIndex(serializable::read_u32(input))
    }
}

impl Serializable for LocalFrameIndex {
    fn serialize<W: Write> (&self, sink: &mut W) -> () {
        serializable::write_u32(sink, self.0)
    }
    fn from_bytes<R: Read>(input: &mut R) -> Self {
        LocalFrameIndex(serializable::read_u32(input))
    }
}