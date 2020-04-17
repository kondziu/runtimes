use crate::serializable::Serializable;
use crate::serializable;
use std::io::{Read, Write};

#[derive(PartialEq,Debug,Copy,Clone)] pub struct Arity(u8);
#[derive(PartialEq,Debug,Copy,Clone)] pub struct Size(u16);
#[derive(PartialEq,Debug,Copy,Clone)] pub struct Address(u32);
#[derive(PartialEq,Debug,Copy,Clone)] pub struct ConstantPoolIndex(u16);
#[derive(PartialEq,Debug,Copy,Clone)] pub struct LocalFrameIndex(u16);

impl Arity             { pub fn new(value: u8)  -> Arity             { Arity(value)             }}
impl Size              { pub fn new(value: u16) -> Size              { Size(value)              }}
impl Address           { pub fn new(value: u32) -> Address           { Address(value)           }}
impl LocalFrameIndex   { pub fn new(value: u16) -> LocalFrameIndex   { LocalFrameIndex(value)   }}
impl ConstantPoolIndex { pub fn new(value: u16) -> ConstantPoolIndex { ConstantPoolIndex(value) }}

impl ConstantPoolIndex {
    pub fn read_cpi_vector<R: Read>(input: &mut R) -> Vec<ConstantPoolIndex> {
        println!("ConstantPoolIndex::read_cpi_vector");
        serializable::read_u16_vector(input)
            .into_iter()
            .map(ConstantPoolIndex::new)
            .collect()
    }

    pub fn write_cpi_vector<R: Write>(sink: &mut R, vector: &Vec<ConstantPoolIndex>) -> () {
        let vector_of_u16s: Vec<u16> = vector.iter().map(|cpi| cpi.0).collect();
        serializable::write_u16_vector(sink, &vector_of_u16s)
    }
}

impl ConstantPoolIndex  { pub fn value(&self) -> u16 { self.0 } }
impl LocalFrameIndex    { pub fn value(&self) -> u16 { self.0 } }
impl Address            { pub fn value(&self) -> u32 { self.0 } }
impl Size               { pub fn value(&self) -> u16 { self.0 } }
impl Arity              { pub fn value(&self) -> u8  { self.0 } }

impl Serializable for Arity {

    fn serialize<W: Write> (&self, sink: &mut W) -> () {
        serializable::write_u8(sink, self.0)
    }

    fn from_bytes<R: Read>(input: &mut R) -> Self {
        println!("Arity::from_bytes");
        Arity(serializable::read_u8(input))
    }
}

impl Serializable for Size {

    fn serialize<W: Write> (&self, sink: &mut W) -> () {
        serializable::write_u16(sink, self.0)
    }

    fn from_bytes<R: Read>(input: &mut R) -> Self {
        println!("Size::from_bytes");
        Size(serializable::read_u16(input))
    }
}

impl Serializable for Address {
    fn serialize<W: Write> (&self, sink: &mut W) -> () {
        serializable::write_u32(sink, self.0)
    }
    fn from_bytes<R: Read>(input: &mut R) -> Self {
        println!("Address::from_bytes");
        Address(serializable::read_u32(input))
    }
}

impl Serializable for ConstantPoolIndex {
    fn serialize<W: Write> (&self, sink: &mut W) -> () {
        serializable::write_u16(sink, self.0)
    }
    fn from_bytes<R: Read>(input: &mut R) -> Self {
        println!("ConstantPoolIndex::from_bytes");
        ConstantPoolIndex(serializable::read_u16(input))
    }
}

impl Serializable for LocalFrameIndex {
    fn serialize<W: Write> (&self, sink: &mut W) -> () {
        serializable::write_u16(sink, self.0)
    }
    fn from_bytes<R: Read>(input: &mut R) -> Self {
        println!("LocalFrameIndex::from_bytes");
        LocalFrameIndex(serializable::read_u16(input))
    }
}

impl Address {
    pub fn increment(address: Address) -> Address {
        Address(address.0 + 1)
    }
}