use std::collections::HashMap;

use crate::types::{Address, LocalFrameIndex};
use crate::objects::{RuntimeObject, ProgramObject, SharedRuntimeObject};
use crate::bytecode::OpCode;
use crate::program::Program;
use std::rc::Rc;
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::fmt::Write;

/**
 * A name-to-value table that holds the current value of all the global variables used in the
 * program.
 *
 * Operations:
 *  - a value associated with a given name can be retrieved,
 *  - a new value can be assigned to a given name.
 */
//pub struct GlobalVariables {
//    table: HashMap<String, RuntimeObject>
//}

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
#[derive(PartialEq,Debug)]
pub struct LocalFrame {
    slots: Vec<SharedRuntimeObject>, /* ProgramObject::Slot */
    return_address: Option<Address>, /* address */
    //parent_frame: u64, /* index to local frame stack */
}

impl LocalFrame {
    pub fn empty() -> Self {
        LocalFrame {
            slots: vec!(),
            return_address: None,
        }
    }

    pub fn from(return_address: Option<Address>, slots: Vec<SharedRuntimeObject>) -> Self {
        LocalFrame {
            return_address,
            slots,
        }
    }

    pub fn return_address(&self) -> &Option<Address> {
        &self.return_address
    }

    pub fn get_local(&self, index: &LocalFrameIndex) -> Option<SharedRuntimeObject> {
        match index.value() {
            index if index as usize >= self.slots.len() => None,
            index => Some(self.slots[index as usize].clone()), // new ref
        }
    }

    pub fn update_local(&mut self, index: &LocalFrameIndex, local: SharedRuntimeObject) -> Result<(), String> {
        match index.value() {
            index if index as usize >= self.slots.len() =>
                Err(format!("No local at index {} in frame", index)),
            index => {
                self.slots[index as usize] = local;
                Ok(())
            },
        }
    }

    pub fn push_local(&mut self, local: SharedRuntimeObject) -> LocalFrameIndex {
        self.slots.push(local);
        assert!(self.slots.len() <= 65_535usize);
        LocalFrameIndex::new(self.slots.len() as u16 - 1u16)
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

pub struct State {
    pub instruction_pointer: Option<Address>,
    pub frames: Vec<LocalFrame>,
    pub operands: Vec<SharedRuntimeObject>,
    pub globals: HashMap<String, SharedRuntimeObject>,
    pub functions: HashMap<String, ProgramObject>,
}

impl State {
    pub fn from(program: &Program) -> Self {
        let entry_index = program.entry();
        let entry_method = program.get_constant(entry_index)
            .expect(&format!("State init error: entry method is not in the constant pool \
                              at index {:?}", entry_index));

        let instruction_pointer = *match entry_method {
            ProgramObject::Method { name:_, arguments:_, locals:_, code } => code.start(),
            _ => panic!("State init error: entry method is not a Method {:?}", entry_method),
        };

        let (globals, functions) = {
            let mut globals: HashMap<String, SharedRuntimeObject> = HashMap::new();
            let mut functions: HashMap<String, ProgramObject> = HashMap::new();

            for index in program.globals() {
                let thing = program.get_constant(index)
                    .expect(&format!("State init error: no such entry at index pool: {:?}", index));

                match thing {
                    ProgramObject::Slot { name: index } => {
                        let constant = program.get_constant(index)
                            .expect(&format!("State init error: no such entry at index pool: {:?} \
                                     (expected by slot: {:?})", index, thing));
                        let name = match constant {
                            ProgramObject::String(string) => string,
                            constant => panic!("State init error: name of global at index {:?} is \
                                                not a String {:?}", index, constant),
                        };
                        if globals.contains_key(name) {
                            panic!("State init error: duplicate name for global {:?}", name)
                        }
                        globals.insert(name.to_string(), RuntimeObject::null());
                    }

                    ProgramObject::Method { name: index, arguments:_, locals:_, code:_ } => {
                        let constant = program.get_constant(index)
                            .expect(&format!("State init error: no such entry at index pool: {:?} \
                                     (expected by method: {:?})", index, thing));
                        let name = match constant {
                            ProgramObject::String(string) => string,
                            constant => panic!("State init error: name of function at index {:?} \
                                                is not a String {:?}", index, constant),
                        };
                        if functions.contains_key(name) {
                            panic!("State init error: duplicate name for function {:?}", name)
                        }
                        functions.insert(name.to_string(), thing.clone());
                    }
                    _ => panic!("State init error: name of global at index {:?} is not a String \
                                 {:?}", index, thing),
                };

            }
            (globals, functions)
        };

        let frames = vec!(LocalFrame::empty());

        State {
            instruction_pointer: Some(instruction_pointer),
            frames,
            operands: Vec::new(),
//            labels: HashMap::new(),
            globals,
            functions,
        }
    }

    pub fn empty() -> Self {
        State {
            instruction_pointer: None,
            frames: Vec::new(),
            operands: Vec::new(),
//            labels: HashMap::new(),
            globals: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    pub fn minimal() -> Self {
        State {
            instruction_pointer: Some(Address::from_usize(0)),
            frames: vec!(LocalFrame::empty()),
            operands: Vec::new(),
//            labels: HashMap::new(),
            globals: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    pub fn instruction_pointer(&self) -> &Option<Address> {
        &self.instruction_pointer
    }

    pub fn bump_instruction_pointer(&mut self, program: &Program) -> &Option<Address> {
        let address = program.code().next_address(self.instruction_pointer);
        self.instruction_pointer = address;
        &self.instruction_pointer
    }

    pub fn set_instruction_pointer(&mut self, address: Option<Address>) -> () {
        self.instruction_pointer = address;
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

    pub fn new_frame(&mut self, return_address: Option<Address>, slots: Vec<SharedRuntimeObject>,) {
        self.frames.push(LocalFrame{ slots, return_address });
    }

    pub fn peek_operand(&mut self) -> Option<&SharedRuntimeObject> {
        self.operands.last()
    }

    pub fn pop_operand(&mut self) -> Option<SharedRuntimeObject> {
        self.operands.pop()
    }

    pub fn push_operand(&mut self, object: SharedRuntimeObject) -> () {
        self.operands.push(object)
    }

    pub fn get_function(&self, name: &str) -> Option<&ProgramObject> {
        self.functions.get(name)
    }

    pub fn get_global(&self, name: &str) -> Option<&SharedRuntimeObject> {
        self.globals.get(name)
    }

    pub fn register_global(&mut self, name: String, object: SharedRuntimeObject) -> Result<(), String> {
        if self.globals.contains_key(&name) {
            Err(format!("Global {} already registered (with value {:?})",
                        &name, self.globals.get(&name).unwrap()))
        } else {
            self.globals.insert(name, object);
            Ok(())
        }
    }

    pub fn update_global(&mut self, name: String, object: SharedRuntimeObject) -> Result<(), String> {
        if self.globals.contains_key(&name) {
            self.globals.insert(name, object);
            Ok(())
        } else {
            Err(format!("Global {} does not exist and cannot be updated", &name))
        }
    }

//    pub fn get_label_address(&self, name: &str) -> Option<&Address> {
//        self.labels.get(name)
//    }

//    pub fn add_label(&mut self, name: String, address: Address) -> Result<(), String> {
//        if self.labels.contains_key(&name) {
//            Err(format!("Label {} already registered (with value {:?})",
//                        &name, self.labels.get(&name).unwrap()))
//        } else {
//            self.labels.insert(name, address);
//            Ok(())
//        }
//    }

//    pub fn create_label_at_instruction_pointer(&mut self, name: String) -> Result<(), String> {
//        let address: Option<Address> = self.instruction_pointer;
//        if self.labels.contains_key(&name) {
//            Err(format!("Label {} already registered (with value {:?})",
//                        &name, self.labels.get(&name).unwrap()))
//        } else {
//            match address {
//                Some(address) => {
//                    self.labels.insert(name, address);
//                    Ok(())
//                },
//                None => panic!("Cannot set label address to nothing"),
//            }
//        }
//    }

    pub fn set_instruction_pointer_from_label(&mut self, program: &Program, name: &str) -> Result<(), String> {
        match program.get_label(name) {
            None => Err(format!("Label {} does not exist", name)),
            Some(address) => {self.instruction_pointer = Some(*address); Ok(())}
        }
    }

    pub fn push_global_to_operand_stack(&mut self, name: &str) -> Result<(), String> {
        let global = self.get_global(name).map(|e| e.clone());
        match global {
            Some(global) => {
                self.push_operand(global); Ok(())
            },
            None => {
                Err(format!("No such global {}", name))
            }
        }
    }
}

pub fn interpret<Output>(state: &mut State, output: &mut Output, program: &Program)
    where /*Input : Read,*/ Output : Write {
    let opcode: &OpCode = {
        let address = state.instruction_pointer()
            .expect("Interpreter error: cannot reference opcode at instruction pointer: nothing");

        let opcode = program.get_opcode(&address)
            .expect(&format!("Interpreter error: cannot reference opcode at instruction pointer: \
                              {:?}", address));

        opcode
    };

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
            state.push_operand(object);

            state.bump_instruction_pointer(program);
//                .expect("Literal error: cannot bump instruction pointer");
        }

        OpCode::GetLocal { index } => {
            let frame: &LocalFrame = state.current_frame()
                .expect("Get local error: no frame on stack.");

            let local: SharedRuntimeObject = frame.get_local(&index)
                .expect(&format!("Get local error: there is no local at index {:?} in the current \
                                  frame", index));

            state.push_operand(local);

            state.bump_instruction_pointer(program);
//                .expect("Get local error: cannot bump instruction pointer");
        }

        OpCode::SetLocal { index } => {
            let operand: SharedRuntimeObject = state.peek_operand()
                .expect("Set local error: cannot pop from empty operand stack")
                .clone();

            let frame: &mut LocalFrame = state.current_frame_mut()
                .expect("Set local error: no frame on stack.");

            frame.update_local(index, operand)
                .expect(&format!("Set local error: there is no local at index {:?} in the current \
                                  frame", index));

            state.bump_instruction_pointer(program);
//                .expect("Set local error: cannot bump instruction pointer");
        }

        OpCode::GetGlobal { name: index } => {
            let constant: &ProgramObject = program.get_constant(index)
                .expect(&format!("Get global error: no constant at index {:?}", index.value()));

            let name: &str = match constant {
                ProgramObject::String(s) => s,
                _ => panic!("Get global error: constant at index {:?} must be a String, \
                             but it is {:?}", index.value(), constant),
            };

            let global = state.get_global(name).map(|g| g.clone())
                .expect(&format!("Get global error: no such global: {}", name));

            state.push_operand(global);

            state.bump_instruction_pointer(program);
//                .expect("Get global error: cannot bump instruction pointer");
        }

        OpCode::SetGlobal { name: index } => {
            let constant: &ProgramObject = program.get_constant(index)
                .expect(&format!("Set global error: no constant at index {:?}", index.value()));

            let name: &str = match constant {
                ProgramObject::String(s) => s,
                _ => panic!("Set global error: constant at index {:?} must be a String, \
                             but it is {:?}", index.value(), constant),
            };

            let operand: SharedRuntimeObject = state.peek_operand().map(|o| o.clone())
                .expect("Set global: cannot pop operand from empty operand stack");

            state.update_global(name.to_string(), operand)
                .expect(&format!("Set global: cannot update global {}, no such global", name));

            state.bump_instruction_pointer(program);
//                .expect("Set global error: cannot bump instruction pointer");
        }

        OpCode::Object { class: index } => {
            let constant: &ProgramObject = program.get_constant(index)
                .expect(&format!("Object error: no constant at index {:?}", index.value()));

            let member_definitions: Vec<&ProgramObject> = match constant {
                ProgramObject::Class(v) => v,
                _ => panic!("Object error: constant at index {:?} must be a String, \
                             but it is {:?}", index.value(), constant),
            }.iter().map(| index | program.get_constant(index)
                .expect(&format!("Object error: no constant at index {:?} for member_ object",
                                 index.value()))).collect();

            let (slots, methods): (Vec<&ProgramObject>, Vec<&ProgramObject>) =
                member_definitions.iter().partition(|c| match c {
                    ProgramObject::Method { code:_, locals:_, arguments:_, name:_ } => false,
                    ProgramObject::Slot { name:_ } => true,
                    member =>
                        panic!("Object error: class members may be either Methods or Slots, \
                                 but this member is {:?}", member),
            }); // XXX this will work even if the member definitions are not sorted, which is
                // contrary to the spec

            let fields: HashMap<String, SharedRuntimeObject> = {
                let mut map: HashMap<String, SharedRuntimeObject> = HashMap::new();
                for slot in slots {
                    if let ProgramObject::Slot {name: index} = slot {
                        let object = state.pop_operand()
                            .expect("Object error: cannot pop operand (member) from empty operand \
                                     stack");

                        let constant: &ProgramObject = program.get_constant(index)
                            .expect(&format!("Object error: no constant at index {:?}",
                                             index.value()));

                        let name: &str = match constant {
                            ProgramObject::String(s) => s,
                            _ => panic!("Object error: constant at index {:?} must be a String, \
                                         but it is {:?}", index.value(), constant),
                        };

                        let result = map.insert(name.to_string(), object);
                        if let Some(_) = result {
                            panic!("Object error: member fields must have unique names, but \
                                    {} is used by to name more than one field", name)
                        }
                    } else {
                        unreachable!()
                    }
                }
                map
            };

            let method_map: HashMap<String, ProgramObject> = {
                let mut map: HashMap<String, ProgramObject> = HashMap::new();
                for method in methods {
                    match method {
                        ProgramObject::Method { name: index, arguments:_, locals:_, code:_ } => {
                            let constant: &ProgramObject = program.get_constant(index)
                                .expect(&format!("Object error: no constant at index {:?}",
                                                 index.value()));

                            let name: &str = match constant {
                                ProgramObject::String(s) => s,
                                _ => panic!("Object error: constant at index {:?} must be a String, \
                                         but it is {:?}", index.value(), constant),
                            };
                            let result = map.insert(name.to_string(), method.clone());

                            match result {
                                Some (other_method) =>
                                    panic!("Object error: method {} has a non-unique name in \
                                            object: {:?} v {:?}", name, method, other_method),
                                None => ()
                            }
                        },
                        _ => unreachable!(),
                    }
                }
                map
            };

            let parent = state.pop_operand()
                .expect("Object error: cannot pop operand (parent) from empty operand stack");

            let object = Rc::new(RefCell::new(RuntimeObject::Object {
                parent, fields, methods: method_map
            }));

            state.push_operand(object);

            state.bump_instruction_pointer(program);
//                .expect("Object error: cannot bump instruction pointer");
        }

        OpCode::Array => {
            let initializer = state.pop_operand()
                .expect(&format!("Array error: cannot pop initializer from empty operand stack"));

            let size_object = state.pop_operand()
                .expect(&format!("Array error: cannot pop size from empty operand stack"));

            let size: usize = match size_object.as_ref().borrow().deref() {
                RuntimeObject::Integer(n) => {
                    if *n < 0 {
                        panic!("Array error: negative value cannot be used to specify the size of \
                                an array {:?}", size_object);
                    } else {
                        *n as usize
                    }
                }
                _ => panic!("Array error: object cannot be used to specify the size of an array \
                             {:?}", size_object),
            };

            let elements = {
                let mut elements: Vec<SharedRuntimeObject> = Vec::new();
                for _ in 0..size {
                    elements.push(initializer.clone());
                }
                elements
            };

            let object = Rc::new(RefCell::new(RuntimeObject::Array(elements)));

            state.push_operand(object);

            state.bump_instruction_pointer(program);
//                .expect("Array error: cannot bump instruction pointer");
        }

//        OpCode::ArrayN { size } => {
//            let elements = {
//                let mut elements: Vec<SharedRuntimeObject> = Vec::new();
//                for index in 0..size.value() {
//                    let element = state.pop_operand()
//                        .expect(&format!("Array error: cannot pop operand {} from empty operand \
//                                          stack", index));
//                    elements.push(element);
//                }
//                elements
//            };
//
//            let object = Rc::new(RefCell::new(RuntimeObject::Array(elements)));
//
//            state.push_operand(object);
//
//            state.bump_instruction_pointer(program);
////                .expect("Array error: cannot bump instruction pointer");
//        }

        OpCode::GetSlot { name: index } => {
            let constant: &ProgramObject = program.get_constant(index)
                .expect(&format!("Get slot error: no constant to serve as label name at index {:?}",
                                 index.value()));

            let name: &str = match constant {
                ProgramObject::String(s) => s,
                _ => panic!("Get slot error: constant at index {:?} must be a String, but it is {:?}",
                            index, constant),
            };

            let operand: SharedRuntimeObject = state.pop_operand()
                .expect(&format!("Get slot error: cannot pop operand from empty operand stack"));

            match operand.as_ref().borrow().deref() {
                RuntimeObject::Object { parent:_, fields, methods:_ } => {
                    let slot: &SharedRuntimeObject = fields.get(name)
                        .expect(&format!("Get slot error: no field {} in object {:?}",
                                         name, operand));

                    state.push_operand(slot.clone())
                }
                _ => panic!("Get slot error: attempt to access field of a non-object {:?}", operand)
            }; // this semicolon turns the expression into a statement and is *important* because of
               // how temporaries work https://github.com/rust-lang/rust/issues/22449

            state.bump_instruction_pointer(program);
//                .expect("Get slot error: cannot bump instruction pointer");
        }

        OpCode::SetSlot { name: index } => {
            let constant: &ProgramObject = program.get_constant(index)
                .expect(&format!("Set slot error: no constant to serve as label name at index {:?}",
                                 index.value()));

            let name: &str = match constant {
                ProgramObject::String(s) => s,
                _ => panic!("Set slot error: constant at index {:?} must be a String, but it is {:?}",
                            index, constant),
            };

            let value: SharedRuntimeObject = state.pop_operand()
                .expect(&format!("Set slot error: cannot pop operand (value) from empty operand \
                                  stack"));

            let host: SharedRuntimeObject = state.pop_operand().clone()
                .expect(&format!("Set slot error: cannot pop operand (host) from empty operand \
                                  stack"));

            match host.as_ref().borrow_mut().deref_mut() {
                RuntimeObject::Object { parent:_, fields, methods:_ } => {
                    if !(fields.contains_key(name)) {
                        panic!("Set slot error: no field {} in object {:?}", name, host)
                    }

                    fields.insert(name.to_string(), value.clone());
                    state.push_operand(value)
                }
                _ => panic!("Get slot error: attempt to access field of a non-object {:?}", host)
            }; // this semicolon turns the expression into a statement and is *important* because of
               // how temporaries work https://github.com/rust-lang/rust/issues/22449

            state.bump_instruction_pointer(program);
//                .expect("Get slot error: cannot bump instruction pointer");
        }

        OpCode::CallMethod { name: index, arguments: parameters } => {
            if parameters.value() == 0 {
                panic!("Call method error: method must have at least one parameter (receiver)");
            }

            let mut arguments: Vec<SharedRuntimeObject> = Vec::with_capacity(parameters.value() as usize);
            for index in 0..(parameters.value() - 1) {
                let element = state.pop_operand()
                    .expect(&format!("Call method error: cannot pop argument {} from empty operand \
                                      stack", index));
                arguments.push(element);
            }

            let object: SharedRuntimeObject = state.pop_operand()
                .expect(&format!("Call method error: cannot pop host object from empty operand \
                                  stack"));

            let constant: &ProgramObject = program.get_constant(index)
                .expect(&format!("Call method error: no constant to serve as format index {:?}",
                                 index));

            let name: &str = match constant {
                ProgramObject::String(s) => s,
                _ => panic!("Call method error: constant at index {:?} must be a String, but it is \
                             {:?}", index, constant),
            };

            match object.as_ref().borrow_mut().deref_mut() {
                RuntimeObject::Null => {
                    if arguments.len() != 1 {
                        panic!("Call method error: Null method {} takes 1 argument, but {} were \
                                supplied", name, arguments.len())
                    }

                    let operand_is_null: bool = match arguments[0].as_ref().borrow().deref() {
                        RuntimeObject::Null => true,
                        _ => false,
                    };
                    let result = match name {
                        "eq"  | "==" => RuntimeObject::from_bool(operand_is_null),
                        "neq" | "!=" => RuntimeObject::from_bool(!operand_is_null),
                        _ => panic!("Call method error: Null type does not implement method {}",
                                    name),
                    };
                    state.push_operand(result);
                    state.bump_instruction_pointer(program);
                },
                RuntimeObject::Integer(i) => {
                    if arguments.len() != 1 {
                        panic!("Call method error: Integer method {} takes 1 argument, but {} were \
                                supplied", name, arguments.len())
                    }
                    let result = match (name, arguments[0].as_ref().borrow().deref()) {
                        ("+",   RuntimeObject::Integer(j)) => RuntimeObject::from_i32 (*i +  *j),
                        ("-",   RuntimeObject::Integer(j)) => RuntimeObject::from_i32 (*i -  *j),
                        ("*",   RuntimeObject::Integer(j)) => RuntimeObject::from_i32 (*i *  *j),
                        ("/",   RuntimeObject::Integer(j)) => RuntimeObject::from_i32 (*i /  *j),
                        ("%",   RuntimeObject::Integer(j)) => RuntimeObject::from_i32 (*i %  *j),
                        ("<=",  RuntimeObject::Integer(j)) => RuntimeObject::from_bool(*i <= *j),
                        (">=",  RuntimeObject::Integer(j)) => RuntimeObject::from_bool(*i >= *j),
                        ("<",   RuntimeObject::Integer(j)) => RuntimeObject::from_bool(*i <  *j),
                        (">",   RuntimeObject::Integer(j)) => RuntimeObject::from_bool(*i >  *j),
                        ("==",  RuntimeObject::Integer(j)) => RuntimeObject::from_bool(*i == *j),
                        ("!=",  RuntimeObject::Integer(j)) => RuntimeObject::from_bool(*i != *j),
                        ("==",  _)                         => RuntimeObject::from_bool(false),
                        ("!=",  _)                         => RuntimeObject::from_bool(true),

                        ("add", RuntimeObject::Integer(j)) => RuntimeObject::from_i32 (*i +  *j),
                        ("sub", RuntimeObject::Integer(j)) => RuntimeObject::from_i32 (*i -  *j),
                        ("mul", RuntimeObject::Integer(j)) => RuntimeObject::from_i32 (*i *  *j),
                        ("div", RuntimeObject::Integer(j)) => RuntimeObject::from_i32 (*i /  *j),
                        ("mod", RuntimeObject::Integer(j)) => RuntimeObject::from_i32 (*i %  *j),
                        ("le",  RuntimeObject::Integer(j)) => RuntimeObject::from_bool(*i <= *j),
                        ("ge",  RuntimeObject::Integer(j)) => RuntimeObject::from_bool(*i >= *j),
                        ("lt",  RuntimeObject::Integer(j)) => RuntimeObject::from_bool(*i <  *j),
                        ("gt",  RuntimeObject::Integer(j)) => RuntimeObject::from_bool(*i >  *j),
                        ("eq",  RuntimeObject::Integer(j)) => RuntimeObject::from_bool(*i == *j),
                        ("neq", RuntimeObject::Integer(j)) => RuntimeObject::from_bool(*i != *j),
                        ("eq",  _)                         => RuntimeObject::from_bool(false),
                        ("neq", _)                         => RuntimeObject::from_bool(true),

                        _ => panic!("Call method error: Integer type does not implement method {} for operand {:?}",
                                    name, arguments[0]),
                    };

                    state.push_operand(result);
                    state.bump_instruction_pointer(program);
                }
                RuntimeObject::Boolean(p) => {
                    if arguments.len() != 1 {
                        panic!("Call method error: Boolean method {} takes 1 argument, but {} were \
                                supplied", name, arguments.len())
                    }
                    let result = match (name, arguments[0].as_ref().borrow().deref()) {
                        ("and", RuntimeObject::Boolean(q)) => RuntimeObject::from_bool(*p && *q),
                        ("or",  RuntimeObject::Boolean(q)) => RuntimeObject::from_bool(*p || *q),
                        ("eq",  RuntimeObject::Boolean(q)) => RuntimeObject::from_bool(*p == *q),
                        ("neq", RuntimeObject::Boolean(q)) => RuntimeObject::from_bool(*p != *q),
                        ("eq",  _)                         => RuntimeObject::from_bool(false),
                        ("neq", _)                         => RuntimeObject::from_bool(true),

                        ("&",   RuntimeObject::Boolean(q)) => RuntimeObject::from_bool(*p && *q),
                        ("|",   RuntimeObject::Boolean(q)) => RuntimeObject::from_bool(*p || *q),
                        ("==",  RuntimeObject::Boolean(q)) => RuntimeObject::from_bool(*p == *q),
                        ("!=",  RuntimeObject::Boolean(q)) => RuntimeObject::from_bool(*p != *q),
                        ("==",  _)                         => RuntimeObject::from_bool(false),
                        ("!=",  _)                         => RuntimeObject::from_bool(true),

                        _ => panic!("Call method error: Boolean type does not implement method {}",
                                    name),
                    };
                    state.push_operand(result);
                    state.bump_instruction_pointer(program);
                }
                RuntimeObject::Array(elements) => {
                    if arguments.len() != (parameters.value() - 1) as usize {
                        panic!("Call method error: Array method {} takes {} argument, but {} were \
                                supplied", name, parameters.value(), arguments.len())
                    }

                    let result: SharedRuntimeObject = match name {
                        "get"  => {
                            if parameters.value() as usize != 2 {
                                panic!("Call method error: Array method get takes {} argument, but \
                                        it should take 1", parameters.value() - 1)
                            }
                            match arguments[0].as_ref().borrow().deref() {
                                RuntimeObject::Integer(n) => {
                                    if (*n as usize) >= elements.len() {
                                        panic!("Call method error: array index {} is out of bounds",
                                               n)
                                    }

                                    elements[*n as usize].clone()
                                },
                                _ => panic!("Call method error: cannot apply Array method {} with \
                                     argument {:?}", name, arguments[0]),
                            }
                        },
                        "set"  => {
                            if parameters.value() as usize != 3 {
                                panic!("Call method error: Array method set takes {} argument, but \
                                        it should take 2", parameters.value() - 1)
                            }
                            match arguments[0].as_ref().borrow().deref() {
                                RuntimeObject::Integer(n) => {
                                    if (*n as usize) >= elements.len() {
                                        panic!("Call method error: array index {} is out of bounds",
                                               n)
                                    }

                                    elements[*n as usize] = arguments[1].clone();
                                    RuntimeObject::null()
                                },
                                _ => panic!("Call method error: cannot apply Array method {} with \
                                     argument {:?}", name, arguments[0]),
                            }
                        }
                        _ => panic!("Call method error: Array type does not implement method {}",
                                    name),
                    };
                    state.push_operand(result);
                    state.bump_instruction_pointer(program);
                }
                RuntimeObject::Object { parent, fields:_, methods } => {
                    let constant = methods.get(name)                                                // FIXME dispatch though
                        .expect(&format!("Call method error: there is no method {} in object{:?}",
                                          name, object));
                    match constant {
                        ProgramObject::Method { name:_, arguments: parameters, locals, code } => {
                            if arguments.len() != (parameters.value() - 1) as usize {
                                panic!("Call method error: method {} from object {:?} takes {} \
                                        arguments, but {} were supplied",
                                        name, object, parameters.value(), arguments.len())
                            }

                            let slots = {
                                let mut slots: Vec<SharedRuntimeObject> =
                                    Vec::with_capacity(1 + parameters.value() as usize
                                                         + locals.value() as usize);
                                slots.push(object.clone());
                                slots.extend(arguments);
                                for _ in 0..locals.value() {
                                    slots.push(RuntimeObject::null())
                                }
                                slots
                            };

                            state.bump_instruction_pointer(program);
                            state.new_frame(*state.instruction_pointer(), slots);
                            state.set_instruction_pointer(Some(*code.start()));
                        },
                        thing => panic!("Call method error: member {} in object definition {:?}
                                         should have type Method, but it is {:?}",
                                         name, object, thing),
                    };
                }
            };
        }

        OpCode::CallFunction { name: index, arguments } => {

            let constant: &ProgramObject = program.get_constant(index)
                .expect(&format!("Call function error: no constant to serve as function name at \
                                  index {:?}", index));

            let name = match constant {
                ProgramObject::String(string) => string,
                _ => panic!("Call function error: function name must be specified by a String \
                             object, but instead it is: {:?}", constant),
            };

            let function: ProgramObject = {
                state.get_function(name)
                    .expect(&format!("Call function error: no such function {}", name))
                    .clone()
            };

            match function {
                ProgramObject::Method { name:_, arguments: parameters, locals, code: range } => {
                    if arguments.value() != parameters.value() {
                        panic!("Call function error: function definition requires {} arguments, \
                               but {} were supplied", parameters.value(), arguments.value())
                    }

                    let mut slots: Vec<SharedRuntimeObject> =
                        Vec::with_capacity(parameters.value() as usize + locals.value() as usize);

                    for index in 0..arguments.value() {
                        let element = state.pop_operand()
                            .expect(&format!("Call function error: cannot pop argument {} from \
                                              empty operand stack", index));
                        slots.push(element);
                    }

                    for _ in 0..locals.value() {
                        slots.push(RuntimeObject::null())
                    }

                    state.bump_instruction_pointer(program);
                    state.new_frame(*state.instruction_pointer(), slots);
                    state.set_instruction_pointer(Some(*range.start()));
                },
                _ => panic!("Call function error: constant at index {:?} must be a Method, but it \
                             is {:?}", index, constant),
            }
        }

        OpCode::Print { format: index, arguments } => {
            let mut argument_values = {
                let mut argument_values: Vec<SharedRuntimeObject> = Vec::new();
                for index in 0..arguments.value() {
                    let element = state.pop_operand()
                        .expect(&format!("Print error: cannot pop argument {} from empty operand \
                                          stack", index));
                    argument_values.push(element);
                }
                argument_values
            };

            let constant: &ProgramObject = program.get_constant(index)
                .expect(&format!("Print error: no constant to serve as format index {:?}", index));

            let format: &str = match constant {
                ProgramObject::String(s) => s,
                _ => panic!("Print error: constant at index {:?} must be a String, but it is {:?}",
                            index, constant),
            };

            for character in format.chars() {
                match character {
                    '~' => {
                        let string = &argument_values.pop()
                            .map(|e| RuntimeObject::to_string(&e))
                            .expect(&format!("Print error: Not enough arguments for format {}",
                                             format));

                        output.write_str(string)
                            .expect("Print error: Could not write to output stream.")
                    },
                    character => {
                        output.write_char(character)
                            .expect("Print error: Could not write to output stream.")
                    }
                }
            }

            if !argument_values.is_empty() {
                panic!("Print error: Unused arguments for format {}", format)
            }

            state.push_operand(RuntimeObject::from_constant(&ProgramObject::Null));

            state.bump_instruction_pointer(program);
//                .expect("Print error: cannot bump instruction pointer");
        }

        OpCode::Label { name: _ } => {
//            let constant: &ProgramObject = program.get_constant(label)
//                .expect(&format!("Label error: no constant to serve as label name at index {:?}",
//                                 label.value()));
//
//            let name: &str = match constant {
//                ProgramObject::String(s) => s,
//                _ => panic!("Label error: constant at index {:?} must be a String, but it is {:?}",
//                            label, constant),
//            };

            state.bump_instruction_pointer(program);
//                .expect("Label error: cannot bump instruction pointer");

//            state.create_label_at_instruction_pointer(name.to_string())
//                .expect(&format!("Label error: a label with name {} already exists", name));
        }

        OpCode::Jump { label } => {
            let constant: &ProgramObject = program.get_constant(label)
                .expect(&format!("Jump error: no label name at index {:?}", label.value()));

            let name: &str = match constant {
                ProgramObject::String(s) => s,
                _ => panic!("Jump error: constant at index {:?} must be a String, but it is {:?}",
                            label, constant),
            };

            state.set_instruction_pointer_from_label(program, name)
                .expect(&format!("Jump error: no such label {:?}", name));
        }

        OpCode::Branch { label } => {
            let operand = state.pop_operand()
                .expect("Branch error: cannot pop operand from empty operand stack");

            let jump_condition = {
                match *operand.as_ref().borrow() {
                    RuntimeObject::Boolean(value) => value,
                    RuntimeObject::Null => false,
                    _ => true,
                }
            };

            if !jump_condition {
                state.bump_instruction_pointer(program);
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

            state.set_instruction_pointer_from_label(program, name)
                .expect(&format!("Branch error: no such label {:?}", name));
        }

        OpCode::Return => {
            let current_frame: LocalFrame = state.pop_frame()
                .expect("Return error: cannot pop local frame from empty frame stack");
            let address: &Option<Address> = current_frame.return_address();
            state.set_instruction_pointer(*address);
            // current_frame is reclaimed here
        }

        OpCode::Drop => {
            state.pop_operand()
                .expect("Drop error: cannot pop operand from empty operand stack");
            state.bump_instruction_pointer(program);
//                .expect("Drop error: cannot bump instruction pointer");
        },

        OpCode::Skip => {
            state.bump_instruction_pointer(program);
//                .expect("Skip error: cannot bump instruction pointer");
        }
    }
}

