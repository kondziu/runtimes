use crate::types::{ConstantPoolIndex, Arity, Size};
use crate::bytecode::OpCode;
use crate::serializable::Serializable;
use crate::serializable;
use std::io::{Read, Write};

#[derive(PartialEq,Debug,Clone)]
pub enum ProgramObject {
    /**
     * Represents a 32 bit integer. Used by the `Literal` instruction.
     *
     * Serialized with tag `0x00`.
     */
    /*0*/ Integer(i32),

    /**
     * Represents a boolean. Used by the `Literal` instruction.
     *
     * Serialized with tag `0x06`.
     */
    Boolean(bool),

    /**
     * Represents the unit value. Used by the `Literal` instruction.
     *
     * Serialized with tag `0x01`.
     */
    Null,

    /**
     * Represents a character string. Strings are used to:
     *   - represent the names of functions, slots, methods, and labels,
     *   - as format strings in the `Print`.
     *
     * Serialized with tag `0x02`.
     */
    String(String),

    /**
     * Represents one of two things:
     *   - a field member (aka slot) of an object when it is referred to from a `Class` object, or
     *   - a global variable when referred to from the list of Global slots.
     *
     * Contains an index that refers to a `ProgramObject::String` object. The string object
     * represents this slot's name.
     *
     * Serialized with tag `0x03`.
     */
    Slot { name: ConstantPoolIndex },

    /**
     * Represents one of two things:
     *   - a method member of an object, or
     *   - a global function.
     *
     * Contains:
     *   - `name`: an index that refers to a `ProgramObject::String` object, which represents this
     *             method's name,
     *   - `arguments`: the number of arguments this function takes,
     *   - `locals`: the number of local variables defined in this method,
     *   - `code`: a vector containing all the instructions in this method.
     *
     * Serialized with tag `0x04`.
     */
    Method {
        name: ConstantPoolIndex,
        arguments: Arity,
        locals: Size,
        code: Vec<OpCode>
    },

    /**
     * Represents an object structure consisting of field (aka slot) and method members for each
     * type of object created by `object`.
     *
     * It contains a vector containing indices to all the slots in the objects. Each index refers to
     * either:
     *   - a `ProgramObject::Slot` object representing a member field, or
     *   - a `ProgramObject::Method` object representing a member method.
     *
     * Serialized with tag `0x05`.
     */
    Class(Vec<ConstantPoolIndex>),
}

impl ProgramObject {
    fn tag(&self) -> u8 {
        use ProgramObject::*;
        match &self {
            Integer(_)                                         => 0x00,
            Null                                               => 0x01,
            String(_)                                          => 0x02,
            Slot {name:_}                                      => 0x03,
            Method {name: _, arguments: _, locals: _, code: _} => 0x04,
            Class(_)                                           => 0x05,
            Boolean(_)                                         => 0x06,
        }
    }
}

impl Serializable for ProgramObject {
    fn serialize<W: Write>(&self, sink: &mut W) -> () {
        serializable::write_u8(sink, self.tag());
        use ProgramObject::*;
        match &self {
            Null        => (),
            Integer(n)  => serializable::write_i32(sink, *n),
            Boolean(b)  => serializable::write_bool(sink, *b),
            String(s)   => serializable::write_utf8(sink, s),
            Class(v)    => ConstantPoolIndex::write_cpi_vector(sink, v),
            Slot {name} => name.serialize(sink),

            Method {name, arguments, locals, code} => {
                name.serialize(sink);
                arguments.serialize(sink);
                locals.serialize(sink);
                OpCode::write_opcode_vector(sink, code)
            }
        }
    }

    fn from_bytes<R: Read>(input: &mut R) -> Self {
        let tag = serializable::read_u8(input);
        match tag {
            0x00 => ProgramObject::Integer(serializable::read_i32(input)),
            0x01 => ProgramObject::Null,
            0x02 => ProgramObject::String(serializable::read_utf8(input)),
            0x03 => ProgramObject::Slot { name: ConstantPoolIndex::from_bytes(input) },
            0x04 => ProgramObject::Method { name: ConstantPoolIndex::from_bytes(input),
                                            arguments: Arity::from_bytes(input),
                                            locals: Size::from_bytes(input),
                                            code: OpCode::read_opcode_vector(input) },
            0x05 => ProgramObject::Class(ConstantPoolIndex::read_cpi_vector(input)),
            0x06 => ProgramObject::Boolean(serializable::read_bool(input)),
            _    => panic!("Cannot deserialize value: unrecognized value tag: {}", tag)
        }
    }
}

pub enum RuntimeObject {
    Null,
    Integer,
    Array,
    Object,
}
