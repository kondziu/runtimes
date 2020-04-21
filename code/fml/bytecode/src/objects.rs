use crate::types::{ConstantPoolIndex, Arity, Size, AddressRange};
use crate::bytecode::OpCode;
use crate::serializable::{Serializable, SerializableWithContext};
use crate::serializable;
use std::io::{Read, Write};
use std::rc::Rc;
use std::collections::HashMap;
use std::cell::RefCell;
use std::ops::Deref;
use crate::program::Code;

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

pub type SharedRuntimeObject = Rc<RefCell<RuntimeObject>>;

#[derive(PartialEq,Debug)]
pub enum RuntimeObject {
    Null,
    Integer(i32),
    Boolean(bool),
    Array(Vec<SharedRuntimeObject>),
    Object {
        parent: SharedRuntimeObject,
        fields: HashMap<String, SharedRuntimeObject>,
        methods: HashMap<String, ProgramObject>,
    },
}

impl RuntimeObject {
    pub fn from_i32(n :i32) -> Rc<RefCell<RuntimeObject>> {
        Rc::new(RefCell::new(RuntimeObject::Integer(n)))
    }

    pub fn from_bool(b: bool) -> Rc<RefCell<RuntimeObject>> {
        Rc::new(RefCell::new(RuntimeObject::Boolean(b)))
    }

    pub fn from_constant(constant: &ProgramObject) -> Rc<RefCell<RuntimeObject>> {
        match constant {
            ProgramObject::Null => Rc::new(RefCell::new(RuntimeObject::Null)),
            ProgramObject::Integer(value) => Rc::new(RefCell::new(RuntimeObject::Integer(*value))),
            ProgramObject::Boolean(value) => Rc::new(RefCell::new(RuntimeObject::Boolean(*value))),
            _ => unimplemented!(),
        }
    }

    pub fn null() -> Rc<RefCell<RuntimeObject>> {
        Rc::new(RefCell::new(RuntimeObject::Null))
    }

    pub fn to_string(object: &SharedRuntimeObject) -> String {
        match object.as_ref().borrow().deref() {
            RuntimeObject::Null => "null".to_string(),
            RuntimeObject::Integer(n) => n.to_string(),
            RuntimeObject::Boolean(b) => b.to_string(),
            RuntimeObject::Array(elements) => {
                let mut buffer = String::new();
                buffer.push('[');
                for (i, e) in elements.iter().enumerate() {
                    buffer.push_str(&RuntimeObject::to_string(e));
                    if i < elements.len() {
                        buffer.push_str(", ")
                    }
                }
                buffer.push(']');
                buffer
            },
            RuntimeObject::Object { parent, fields, methods:_ } => {
                let mut buffer = String::from("object(");

                buffer.push_str("..=");
                buffer.push_str(&RuntimeObject::to_string(parent));
                buffer.push_str(", ");

                for (i, (name, field)) in fields.iter().enumerate() {
                    buffer.push_str(name);
                    buffer.push_str("=");
                    buffer.push_str(&RuntimeObject::to_string(field));
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