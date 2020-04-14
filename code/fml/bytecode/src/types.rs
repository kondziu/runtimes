use crate::serializable::Serializable;
use byteorder::{BigEndian, ReadBytesExt};

pub struct Size(usize);
pub struct Address(u64);
pub struct ConstantPoolIndex(u64);
pub struct LocalFrameIndex(u64);

macro_rules! to_u8s {
    ($n: expr) => {
        {
            let mut vector: Vec<u8> = Vec::new();
            vector.extend_from_slice(&$n.0.to_be_bytes());
            vector
        }
    };
}

impl Serializable for Size              { fn serialize(&self) -> Vec<u8> { to_u8s!(self) } }
impl Serializable for Address           { fn serialize(&self) -> Vec<u8> { to_u8s!(self) } }
impl Serializable for ConstantPoolIndex { fn serialize(&self) -> Vec<u8> { to_u8s!(self) } }
impl Serializable for LocalFrameIndex   { fn serialize(&self) -> Vec<u8> { to_u8s!(self) } }