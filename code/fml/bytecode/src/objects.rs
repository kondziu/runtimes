use crate::types::{ConstantPoolIndex, Arity, Size, AddressRange};
use crate::bytecode::OpCode;
use crate::serializable::{Serializable, SerializableWithContext};
use crate::serializable;
use std::io::{Read, Write};
use std::collections::HashMap;
use crate::program::Code;

#[derive(PartialEq,Debug,Clone)]
pub enum ProgramObject {
    /**
     * Represents a 32 bit integer. Used by the `Literal` instruction.
     *
     * Serialized with tag `0x00`.
     */
    Integer(i32),

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
     * Serialized with tag `0x04`.
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
     * Serialized with tag `0x03`.
     */
    Method {
        name: ConstantPoolIndex,
        arguments: Arity,
        locals: Size,
        code: AddressRange,
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
            Method {name: _, arguments: _, locals: _, code: _} => 0x03,
            Slot {name:_}                                      => 0x04,
            Class(_)                                           => 0x05,
            Boolean(_)                                         => 0x06,
        }
    }
}

impl SerializableWithContext for ProgramObject {
    fn serialize<W: Write>(&self, sink: &mut W, code: &Code) -> () {
        serializable::write_u8(sink, self.tag());
        use ProgramObject::*;
        match &self {
            Null        => (),
            Integer(n)  => serializable::write_i32(sink, *n),
            Boolean(b)  => serializable::write_bool(sink, *b),
            String(s)   => serializable::write_utf8(sink, s),
            Class(v)    => ConstantPoolIndex::write_cpi_vector(sink, v),
            Slot {name} => name.serialize(sink),

            Method {name, arguments, locals, code: range} => {
                name.serialize(sink);
                arguments.serialize(sink);
                locals.serialize(sink);
                OpCode::write_opcode_vector(sink, &code.addresses_to_code_vector(range))
            }
        }
    }

    fn from_bytes<R: Read>(input: &mut R, code: &mut Code) -> Self {
        println!("ProgramObject::from_bytes");
        let tag = serializable::read_u8(input);
        match tag {
            0x00 => ProgramObject::Integer(serializable::read_i32(input)),
            0x01 => ProgramObject::Null,
            0x02 => ProgramObject::String(serializable::read_utf8(input)),
            0x03 => ProgramObject::Method { name: ConstantPoolIndex::from_bytes(input),
                                            arguments: Arity::from_bytes(input),
                                            locals: Size::from_bytes(input),
                                            code: code.register_opcodes(OpCode::read_opcode_vector(input))},
            0x04 => ProgramObject::Slot { name: ConstantPoolIndex::from_bytes(input) },
            0x05 => ProgramObject::Class(ConstantPoolIndex::read_cpi_vector(input)),
            0x06 => ProgramObject::Boolean(serializable::read_bool(input)),
            _    => panic!("Cannot deserialize value: unrecognized value tag: {}", tag)
        }
    }
}

impl ProgramObject {
    #[allow(dead_code)]
    pub fn null() -> Self {
        ProgramObject::Null
    }

    #[allow(dead_code)]
    pub fn from_bool(b: bool) -> Self {
        ProgramObject::Boolean(b)
    }

    pub fn from_str(string: &str) -> Self {
        ProgramObject::String(string.to_string())
    }

    #[allow(dead_code)]
    pub fn from_string(string: String) -> Self {
        ProgramObject::String(string)
    }

    #[allow(dead_code)]
    pub fn from_i32(n: i32) -> Self {
        ProgramObject::Integer(n)
    }

    #[allow(dead_code)]
    pub fn from_usize(n: usize) -> Self {
        ProgramObject::Integer(n as i32)
    }

    pub fn slot_from_index(index: ConstantPoolIndex) -> Self {
        ProgramObject::Slot { name: index }
    }

    #[allow(dead_code)]
    pub fn slot_from_u16(index: u16) -> Self {
        ProgramObject::Slot { name: ConstantPoolIndex::new(index) }
    }

    #[allow(dead_code)]
    pub fn class_from_vec(indices: Vec<u16>) -> Self {
        ProgramObject::Class(indices.iter().map(|n| ConstantPoolIndex::new(*n)).collect())
    }
}

#[derive(PartialEq,Eq,Debug,Hash,Clone,Copy)]
pub struct Pointer(usize);

impl Pointer {
    pub fn from(p: usize) -> Self {
        Pointer(p)
    }

    pub fn to_string(&self) -> String {
        format!("{:x}", self.0)
    }
}

//pub type SharedRuntimeObject = Rc<RefCell<RuntimeObject>>;

#[derive(PartialEq,Debug,Clone)]
pub enum Object {
    Null,
    Integer(i32),
    Boolean(bool),
    Array(Vec<Pointer>),
    Object {
        parent: Pointer,
        fields: HashMap<String, Pointer>,
        methods: HashMap<String, ProgramObject>,
    },
}

impl Object {
    pub fn from_pointers(v: Vec<Pointer>) -> Self { Object::Array(v)   }
    pub fn from_i32(n :i32)               -> Self { Object::Integer(n) }
    pub fn from_bool(b: bool)             -> Self { Object::Boolean(b) }

    pub fn from_constant(constant: &ProgramObject) -> Self {
        match constant {
            ProgramObject::Null => Object::Null,
            ProgramObject::Integer(value) => Object::Integer(*value),
            ProgramObject::Boolean(value) => Object::Boolean(*value),
            _ => unimplemented!(),
        }
    }

    pub fn from(parent: Pointer, fields: HashMap<String, Pointer>, methods: HashMap<String, ProgramObject>) -> Self {
        Object::Object { parent, fields, methods }
    }

    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        match self {
            Object::Null => "null".to_string(),
            Object::Integer(n) => n.to_string(),
            Object::Boolean(b) => b.to_string(),
            Object::Array(elements) => {
                let mut buffer = String::new();
                buffer.push('[');
                for (i, e) in elements.iter().enumerate() {
                    buffer.push_str(&e.to_string());
                    if i < elements.len() {
                        buffer.push_str(", ")
                    }
                }
                buffer.push(']');
                buffer
            },
            Object::Object { parent, fields, methods:_ } => {
                let mut buffer = String::from("object(");

                buffer.push_str("..=");
                buffer.push_str(&parent.to_string());
                buffer.push_str(", ");

                for (i, (name, field)) in fields.iter().enumerate() {
                    buffer.push_str(name);
                    buffer.push_str("=");
                    buffer.push_str(&field.to_string());
                    if i < fields.len() {
                        buffer.push_str(", ")
                    }
                }

                buffer.push_str(")");
                buffer
            }
        }
    }
}