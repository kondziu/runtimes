#![crate_name = "doc"]
use crate::types::{ConstantPoolIndex, LocalFrameIndex, Size};
use crate::serializable::Serializable;
use std::fs::File;

/**
 * # Bytecode operation
 *
 */
pub enum OpCode {
    /**
     * ## Push literal onto stack
     *
     * Retrieves the [ProgramObject] at the given `index` from the [ConstantPool] and pushes it onto
     * the [OperandStack].
     *
     * The [ProgramObject] retrieved from the [ConstantPool] is *guaranteed* to be one of:
     *  - [ProgramObject::Integer],
     *  - [ProgramObject::Boolean], or
     *  - [ProgramObject::Null].
     *
     * Serialized as opcode `0x01`.
     *
     * [ConstantPool]: ../interpreter/struct.ConstantPool.html
     * [OperandStack]: ../interpreter/struct.OperandStack.html
     * [ProgramObject]: ../objects/enum.ProgramObject.html
     * [ProgramObject::Boolean]: ../objects/enum.ProgramObject.html#variant.Boolean
     * [ProgramObject::Integer]: ../objects/enum.ProgramObject.html#variant.Integer
     * [ProgramObject::Null]: ../objects/enum.ProgramObject.html#variant.Null
     */
    Literal { index: /*Integer|Null|Boolean*/ ConstantPoolIndex },

    /**
     * ## Push the value of local variable onto stack
     *
     * Retrieves a slot in the current [LocalFrame] at the given index and pushes it onto the
     * [OperandStack].
     *
     * Serialized as opcode `0x0A`.
     *
     * [LocalFrame]: ../interpreter/struct.LocalFrame.html
     * [OperandStack]: ../interpreter/struct.OperandStack.html
     */
     // FIXME writing out all those links in each variant is not sustainable...
    GetLocal { index: LocalFrameIndex },

    /**
     * ## Set the value local variable to top value from stack
     *
     * Sets the slot in the current `LocalFrame` at the given index to the top value in the
     * `OperandStack`.
     *
     * Serialized as opcode `0x09`.
     */
    SetLocal { index: LocalFrameIndex },

    /**
     * ## Push the value of global variable onto stack
     *
     * Retrieves the value of the global variable with name specified by the `ProgramObject::String`
     * object at the given index and pushes it onto the `OperandStack`.
     *
     * Serialized as opcode `0x0C`.
     */
    GetGlobal { name: /*String*/ ConstantPoolIndex },

    /**
     * ## Set the value of global variable to the top value from stack
     *
     * Sets the global variable with the name specified by the `ProgramObject::String` object at the
     * given index to the top value in the `OperandStack`.
     *
     * Serialized as opcode `0x0B`.
     */
    SetGlobal { name: /*String*/ ConstantPoolIndex },

    /**
     * ## Create a new (runtime) object
     *
     * Retrieves the `ProgramObject::Class` object at the given index.Suppose that the
     * `ProgramObject::Class` object contains n `ProgramObject::Slot` objects and m
     * `ProgramObject::Method` objects. This instruction will pop n values from the `OperandStack`
     * for use as initial values of the variable slots in the object, then an additional value for
     * use as the parent of the object.
     *
     * The first variable slot is initialized to the deepest value on the `OperandStack` (last
     * popped). The last variable slot is initialized to the shallowest value on the `OperandStack`
     * (first popped).
     *
     * A new `RuntimeObject` is created with the variable slots and method slots indicated
     * by the Class object with the given parent object. The `RuntimeObject` is pushed onto the
     * `OperandStack`.
     *
     * Serialized as opcode `0x04`.
     */
    Object { class: /*ProgramObject::Class*/ ConstantPoolIndex },

    /**
     * ## Create a new array (runtime) object
     *
     * First pops the initializing value from the `OperandStack`. Creates a new array with the given
     * `size``, with each element initialized to the initializing value. Then, pushes the array onto
     * the `OperandStack`.
     *
     * **Warning**: this is different from the semantics of the `array` operation in FML, which
     * evaluates the initializing value separately for each element.
     *
     * Serialized as opcode `0x03`.
     */
    Array { size: Size },

    /**
     * ## Push the value of an object's field member to stack
     *
     *  Pops a value from the `OperandStack` assuming it is a `RuntimeObject`. Then, retrieves a
     *  `ProgramObject::String` object at the index specified by `name`. The `ProgramObject::String`
     *  object is then used to reference a field member of the `RuntimeObject` by name, producing
     *  a value that is also a `RuntimeObject`. The value is pushed onto the operand stack.
     *
     * Serialized as opcode `0x05`.
     */
    GetSlot { name: ConstantPoolIndex },

    /**
     * ## Set the value of an object's field member variable to the top value from stack
     *
     * Pops the value to store, call it x, from the operand stack. Then pops the object to store it
     * into. Stores x into the object at the variable slot with name given by the String object at
     * index i. Then, x is pushed onto the operand stack.
     *
     * Serialized as opcode `0x06`.
     */
    SetSlot { index: ConstantPoolIndex },

    /**
     * ## Call a member method
     *
     * Pops `arguments` values from the `OperandStack` for the arguments to the call. Then pops the
     * a `RuntimeObject` from the `OperandStack` to be used as the method call's receiver.
     * Afterwards, a `ProgramObject::String` object represnting the name of the method to call is
     * retrieved from the `ConstantPool` from the index specified by `name`.
     *
     * If the receiver is a `RuntimeObject::Integer` or `RuntimeObject::Array`, then the result of
     * the method call (as specified by the semantics of Feeny/FML) is pushed onto the stack.
     *
     * If the receiver is a `RuntimeObject::Object`, then a new `LocalFrame` is created for the
     * context of the call. Slot 0 in the new `LocalFrame` holds the receiver object, and the
     * following n slots hold the argument values starting with the deepest value on the stack (last
     * popped) and ending with the shallowest value on the stack (first popped). The new
     * `LocalFrame` has the current frame as its parent and the current `InstructionPointer` as the
     * return `Address`.
     *
     * Execution proceeds by registering the newly created frame as the current `LocalFrame`, and
     * setting the `InstructionPointer` to the `Address` of the body of the method.
     *
     * Serialized as opcode `0x07`.
     */
    CallMethod { name: ConstantPoolIndex, arguments: Size },

    /**
     * ## Call a global function
     *
     * Pops `arguments` values from the `OperandStack` for the arguments to the call. Then, a
     * `ProgramObject::Method` object representing the function to call is retrieved from the
     * `ConstantPool` from the index specified by `function`.
     *
     * The first `arguments` slots in the frame hold argument values starting with the deepest value
     * on the stack (last popped) and ending with the shallowest value on the stack (first popped).
     * The new `LocalFrame` has the current frame as its parent, and the current
     * `InstructionPointer` as the return address. Execution proceeds by registering the newly
     * created `LocalFrame` as the current frame, and setting the `InstructionPointer` to the
     * `Address` of the body of the function.
     *
     * Serialized as opcode `0x08`.
     */
    CallFunction { function: ConstantPoolIndex, arguments: Size },

    /**
     * ## Print a formatted string
     *
     * Pops `arity` values from the `OperandStack`. Then retrieves a `ProgramObject::String` object
     * referenced by the given `format` index. Then, prints out all the values retrieved from the
     * `OperandStack` out according to the given retrieved format string. `Null` is then pushed onto
     * the `OperandStack`.
     *
     * Arguments are spliced in from the deepest value in the stack (last popped) to the
     * shallowest value in the stack (first popped).
     *
     * Serialized as opcode `0x02`.
     */
    Print { format: /*String*/ ConstantPoolIndex, arity: Size },

    /**
     * ## Define a new label here
     *
     * Associates `name` with the address of this instruction. The name is given by the
     * `ProgramObject::String`object at the specified index.
     *
     * Serialized as opcode `0x00`.
     */
    Label { name: /*String*/ ConstantPoolIndex },

    /**
     * ## Jump to a label
     *
     * Sets the `InstructionPointer` to the instruction `Address` associated with the name given
     * by the `ProgramObject::String` at the given index in the `ConstantPool`.
     *
     * Serialized as opcode `0x0E`.
     */
    Jump { label: /*String*/ ConstantPoolIndex },

    /**
     * ## Conditionally jump to a label
     *
     * Pops a value from the `OperandStack`. If this value is not `Null`, then sets the
     * `InstructionPointer` to the instruction `Address` associated with the name given by the
     * `ProgramObject::String` object at the given index.
     *
     * Serialized as opcode `0x0D`.
     */
    Branch { label: /*String*/ ConstantPoolIndex },

    /**
     * ## Return from the current function or method
     *
     * Registers the parent frame of the current `LocalFrame` as the current frame. Execution
     * proceeds by setting the `InstructionPointer` to the return `Address` stored in the current
     * `LocalFrame`.
     *
     * The `LocalFrame` is no longer used after a `Return` instruction and any storage allocated
     * for it may be reclaimed.
     *
     * Serialized as opcode `0x0F`.
     */
    Return,

    /**
     * ## Discard top of stack
     *
     * Pops and discards the top value from the `OperandStack`.
     *
     * Serialized as opcode `0x10`.
     */
    Drop,
}

impl OpCode {
    pub fn to_hex(&self) -> u8 {
        use OpCode::*;
        match self {
            Label { name: _ } => 0x00,
            Literal { index: _ } => 0x01,
            Print { format: _, arity: _ } => 0x02,
            Array { size: _ } => 0x03,
            Object { class: _ } => 0x04,
            GetSlot { name: _ } => 0x05,
            SetSlot { index: _ } => 0x06,
            CallMethod { name: _, arguments: _ } => 0x07,
            CallFunction { function: _, arguments: _ } => 0x08,
            SetLocal { index: _ } => 0x09,
            GetLocal { index: _ } => 0x0A,
            SetGlobal { name: _ } => 0x0B,
            GetGlobal { name: _ } => 0x0C,
            Branch { label: _ } => 0x0D,
            Jump { label: _ } => 0x0E,
            Return => 0x0F,
            Drop => 0x10,
        }
    }
}

impl Serializable for OpCode {
    fn serialize (&self) -> Vec<u8> {
        let mut result: Vec<u8> = vec!(self.to_hex());

        use OpCode::*;
        match self {
            Label { name } => {
                result.extend(name.serialize())
            }
            Literal { index } => {
                result.extend(index.serialize())
            },
            Print { format, arity } => {
                result.extend(format.serialize());
                result.extend(arity.serialize())
            },
            Array { size } => {
                result.extend(size.serialize())
            },
            Object { class } => {
                result.extend(class.serialize())
            },
            GetSlot { name } => {
                result.extend(name.serialize())
            },
            SetSlot { index } => {
                result.extend(index.serialize())
            },
            CallMethod { name, arguments } => {
                result.extend(name.serialize());
                result.extend(arguments.serialize())
            },
            CallFunction { function, arguments } => {
                result.extend(function.serialize());
                result.extend(arguments.serialize())
            },
            SetLocal { index } => {
                result.extend(index.serialize())
            }
            GetLocal { index } => {
                result.extend(index.serialize())
            },
            SetGlobal { name } => {
                result.extend(name.serialize())
            },
            GetGlobal { name } => {
                result.extend(name.serialize())
            },
            Branch { label } => {
                result.extend(label.serialize())
            },
            Jump { label } => {
                result.extend(label.serialize())
            },
            Return => (),
            Drop => (),
        }

        result
    }
}