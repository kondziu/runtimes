use std::collections::HashMap;

use crate::types::ConstantPoolIndex;
use crate::objects::{RuntimeObject, ProgramObject};

/**
 * A name-to-value table that holds the current value of all the global variables used in the
 * program.
 *
 * Operations:
 *  - a value associated with a given name can be retrieved,
 *  - a new value can be assigned to a given name.
 */
pub struct GlobalVariables {
    table: HashMap<String, RuntimeObject>
}

/**
 * The current local frame represents the context in which a function or method is executing.

 * It contains the following slots:
 *  - the values of the arguments to the function,
 *  - the values of all local variables defined in the function,
 *
 * In total, the local frame has as many slots as the sum of the number of the functions arguments
 * and the number of the locals defined within it.
 *
 * The local frame also contains:
 *  - the address of instruction that called the current function,
 *  - the index of the parent frame, ie. the local frame of the calling instruction.
 */
pub struct LocalFrame {
    slots: Vec<RuntimeObject>, /* ProgramObject::Slot */
    call_site: u64, /* address */
    parent_frame: u64, /* index to local frame stack */
}

/**
 * The stack of `LocalFrame`s.
 *
 * Note: this is a structure used to track parenthood which I added to avoid having a
 * self-referential `LocalFrame` struct type.
 */
pub struct LocalFrameStack {

}

/**
 * A single  stack that holds the temporary values of all intermediate results needed during the
 * evaluation of a compound expression.
 *
 * It supports the following operations:
 *  - pushing a value to the stack,
 *  - popping a value from the stack,
 *  - peeking at the top value of the stack.
 */
pub struct OperandStack {
    stack: Vec<Operand>,
}
enum Operand {
    ProgramObject(ProgramObject),
    RuntimeObject(RuntimeObject),
}

/**
 * The instruction pointer contains the address of the instruction that will be executed next.
 */
pub struct InstructionPointer(u64);

/**
 * A listing of program objects. They can be referred to by their numerical index.
 */
pub struct ConstantPool {
    constants: Vec<ProgramObject>,
}


/**
 * A listing of global variables and functions in the program. Each listing points to an object in
 * the constant pool and is **guaranteed** to refer to either:
 *   - a `ProgramObject::Slot` object, or
 *   - a `ProgramObject:Method` object.
 */
pub struct GlobalSlots {
    slots: Vec<ConstantPoolIndex>,
}


