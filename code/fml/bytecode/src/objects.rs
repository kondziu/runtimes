use crate::types::ConstantPoolIndex;
use crate::bytecode::OpCode;

pub enum ProgramObject {
    /**
     * Represents a 32 bit integer. Used by the `Literal` instruction.
     */
    /*0*/ Integer(i32),

    /**
     * Represents a boolean. Used by the `Literal` instruction.
     */
    /*6*/ Boolean(bool),

    /**
     * Represents the unit value. Used by the `Literal` instruction.
     */
    /*1*/ Null,

    /**
     * Represents a character string. Strings are used to:
     *   - represent the names of functions, slots, methods, and labels,
     *   - as format strings in the `Print`.
     */
    /*2*/ String(String),

    /**
     * Represents one of two things:
     *   - a field member (aka slot) of an object when it is referred to from a `Class` object, or
     *   - a global variable when referred to from the list of Global slots.
     *
     * Contains an index that refers to a `ProgramObject::String` object. The string object
     * represents this slot's name.
     */
    /*3*/ Slot { name: ConstantPoolIndex },

    /**
     * Represents one of two things:
     *   - a method member of an object, or
     *   - a global function.
     *
     * Contains:
     *   - `name`:  an index that refers to a `ProgramObject::String` object, which represents this method's name,
     *   - `arguments`: the number of arguments this function takes,
     *   - `locals`: the number of local variables defined in this method,
     *   - `code`: a vector containing all the instructions in this method.
     */
    /*4*/ Method {
        name: ConstantPoolIndex,
        arguments: usize,
        locals: usize,
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
     */
    /*5*/ Class(Vec<ConstantPoolIndex>),
}

pub enum RuntimeObject {
    Null,
    Integer,
    Array,
    Object,
}
