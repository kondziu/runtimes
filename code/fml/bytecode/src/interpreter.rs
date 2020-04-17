use std::collections::HashMap;

use crate::types::{ConstantPoolIndex, Address};
use crate::objects::{RuntimeObject, ProgramObject};
use crate::bytecode::OpCode;
use crate::program::Program;
use std::error::Error;

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
    call_site: Address, /* address */
    //parent_frame: u64, /* index to local frame stack */
}

impl LocalFrame {
    pub fn return_address(&self) -> &Address {
        &self.call_site
    }
}

/**
 * The stack of `LocalFrame`s.
 *
 * Note: this is a structure used to track parenthood which I added to avoid having a
 * self-referential `LocalFrame` struct type.
 */
//pub struct LocalFrameStack {
//
//}

/**
 * A single  stack that holds the temporary values of all intermediate results needed during the
 * evaluation of a compound expression.
 *
 * It supports the following operations:
 *  - pushing a value to the stack,
 *  - popping a value from the stack,
 *  - peeking at the top value of the stack.
 */
//pub struct OperandStack {
//    stack: Vec<Operand>,
//}
//enum Operand {
//    ProgramObject(ProgramObject),
//    RuntimeObject(RuntimeObject),
//}

//pub trait Interpretable {
//    fn interpret(&self);
//}

struct State {
    pub instruction_pointer: Address,
    pub frames: Vec<LocalFrame>,
    pub operands: Vec<RuntimeObject>,
    pub labels: HashMap<String, Address>,
}

impl State {
    pub fn instruction_pointer(&self) -> &Address {
        &self.instruction_pointer
    }

    pub fn set_instruction_pointer(&mut self, address: Address) -> () {
        self.instruction_pointer = address;
    }

    pub fn increment_instruction_pointer(&mut self) -> &Address {
        self.instruction_pointer = Address::increment(self.instruction_pointer);
        &self.instruction_pointer
    }

    pub fn current_frame(&self) -> Option<&LocalFrame> {
        self.frames.last()
    }

    pub fn current_frame_mut(&mut self, ) -> Option<&mut LocalFrame> {
        self.frames.last_mut()
    }

    pub fn pop_frame(&mut self) -> Option<LocalFrame> {
        self.frames.pop()
    }

    pub fn pop_operand(&mut self) -> Option<RuntimeObject> {
        self.operands.pop()
    }

    pub fn push_operand(&mut self, object: RuntimeObject) -> () {
        self.operands.push(object)
    }

    pub fn get_label_address(&self, name: &str) -> Option<&Address> {
        self.labels.get(name)
    }

    pub fn add_label_address(&mut self, name: String, address: Address) -> Result<(), String> {
        if self.labels.contains_key(&name) {
            Err(format!("Label {} already registered with value {:?}",
                        &name, self.labels.get(&name).unwrap()))
        } else {
            self.labels.insert(name, address);
            Ok(())
        }
    }
}

fn interpret(opcode: &OpCode, state: &mut State, program: &Program) {
    match opcode {
        OpCode::Literal { index } => {
            let constant: &ProgramObject = program.get_constant(index)
                 .expect(&format!("Literal error: no constant at index {:?}", index.value()));

            match constant {
                ProgramObject::Null => (),
                ProgramObject::Boolean(_) => (),
                ProgramObject::Integer(_) => (),
                _ => panic!("Literal error: constant at index {:?} must be either Null, Integer, \
                             or Boolean, but is {:?}", index, constant),
            }

            let object = RuntimeObject::from_constant(constant);
            state.push_operand(object)
        }

        OpCode::GetLocal { index: _ } => { unimplemented!() }
        OpCode::SetLocal { index: _ } => { unimplemented!() }
        OpCode::GetGlobal { name: _ } => { unimplemented!() }
        OpCode::SetGlobal { name: _ } => { unimplemented!() }
        OpCode::Object { class: _ } => { unimplemented!() }
        OpCode::Array { size: _ } => { unimplemented!() }
        OpCode::GetSlot { name: _ } => { unimplemented!() }
        OpCode::SetSlot { name: _ } => { unimplemented!() }
        OpCode::CallMethod { name: _, arguments: _ } => { unimplemented!() }
        OpCode::CallFunction { function: _, arguments: _ } => { unimplemented!() }
        OpCode::Print { format: _, arguments: _ } => { unimplemented!() }

        OpCode::Label { name: label } => {
            let constant: &ProgramObject = program.get_constant(label)
                .expect(&format!("Label error: no constant to serve as label name at index {:?}",
                                 label.value()));

            let name: &str = match constant {
                ProgramObject::String(s) => s,
                _ => panic!("Label error: constant at index {:?} must be a String, but it is {:?}",
                            label, constant),
            };

            let address: &Address = state.instruction_pointer();

            state.add_label_address(name.to_string(), *address)
                .expect(&format!("Label error: a label with name {} already exists", name))
        }

        OpCode::Jump { label } => {
            let constant: &ProgramObject = program.get_constant(label)
                .expect(&format!("Jump error: no label name at index {:?}", label.value()));

            let name: &str = match constant {
                ProgramObject::String(s) => s,
                _ => panic!("Jump error: constant at index {:?} must be a String, but it is {:?}",
                            label, constant),
            };

            let address: &Address = state.get_label_address(name)
                .expect(&format!("Jump error: no such label {:?}", name));

            state.set_instruction_pointer(*address);
        }

        OpCode::Branch { label } => {
            let operand = state.pop_operand()
                .expect("Branch error: cannot pop operand from empty operand stack");

            let jump_condition = {
                match operand {
                    RuntimeObject::Boolean(value) => value,
                    RuntimeObject::Null => false,
                    _ => true,
                }
            };

            if !jump_condition {
                return;
            }

            let constant: &ProgramObject = program.get_constant(label)
                .expect(&format!("Branch error: no label name at index {:?}",
                                 label.value()));

            let name: &str = match constant {
                ProgramObject::String(s) => s,
                _ => panic!("Branch error: constant at index {:?} must be a String, but it is {:?}",
                            label, constant),
            };

            let address: &Address = state.get_label_address(name)
                .expect(&format!("Branch error: no such label {:?}", name));

            state.set_instruction_pointer(*address);
        }

        OpCode::Return => {
            let current_frame: LocalFrame = state.pop_frame()
                .expect("Return error: cannot pop local frame from empty frame stack");
            let address: &Address = current_frame.return_address();
            state.set_instruction_pointer(*address);
            // current_frame is reclaimed here
        }

        OpCode::Drop => {
            state.pop_operand()
                .expect("Drop error: cannot pop operand from empty operand stack");
        },
    }
}

