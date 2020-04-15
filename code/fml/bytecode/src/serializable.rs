use std::io::{Read, Write};

pub trait Serializable {
    fn serialize<W: Write> (&self, sink: &mut W) -> ();
    fn from_bytes<R: Read>(input: &mut R) -> Self;
}

pub fn read_u8<R: Read>(reader: &mut R) -> u8 {
    let mut buf = [0u8; 1];
    reader.read_exact(&mut buf).expect("Problem reading u8 from data stream");
    println!("read_u8 {:?}", buf);
    u8::from_be_bytes(buf)
}

pub fn read_u32<R: Read>(reader: &mut R) -> u32 {
    let mut buf = [0u8; 4];
    reader.read_exact(&mut buf).expect("Problem reading u32 from data stream");
    println!("read_u32 {:?}", buf);
    u32::from_be_bytes(buf)
}

pub fn read_u64<R: Read>(reader: &mut R) -> u64 {
    let mut buf = [0u8; 8];
    reader.read_exact(&mut buf).expect("Problem reading u64 from data stream");
    println!("read_u64 {:?}", buf);
    u64::from_be_bytes(buf)
}

pub fn write_u8<W: Write>(writer: &mut W, value: u8) -> () {
    writer.write(&[value]).expect("Problem writing u8 to data sink");
}

pub fn write_u32<W: Write>(writer: &mut W, value: u32) -> () {
    let buf = value.to_be_bytes();
    writer.write(&buf).expect("Problem writing u32 to data sink");
}

pub fn write_u64<W: Write>(writer: &mut W, value: u64) -> () {
    let buf = value.to_be_bytes();
    writer.write(&buf).expect("Problem writing u64 to data sink");
}


