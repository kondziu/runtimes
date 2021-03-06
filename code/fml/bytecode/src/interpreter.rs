use std::collections::{HashMap, VecDeque};

use crate::types::{Address, LocalFrameIndex, Arity};
use crate::objects::{Pointer, Object, ProgramObject};
use crate::bytecode::OpCode;
use crate::program::Program;
use std::fmt::{Write, Error};
use std::io::Write as IOWrite;

pub struct Output {}

impl Output {
    fn new() -> Output {
        Output {}
    }
}

impl Write for Output {
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        match std::io::stdout().write_all(s.as_bytes()) {
            Ok(_) => Ok(()),
            Err(_) => Err(Error),
        }
    }
}

pub fn evaluate(program: &Program) {
    let mut state = State::from(program);
    let mut output = Output::new();

    let (start_address, locals) = match program.get_constant(program.entry()) {
        Some(ProgramObject::Method { name:_, locals, arguments:_, code }) => (*code.start(), locals),
        None => panic!("No entry method at index {:?}", program.entry()),
        Some(constant) => panic!("Constant at index {:?} is not a method {:?}",
                                  program.entry(), constant),
    };

    let mut slots = Vec::new();
    for _ in 0..locals.to_usize() {
        slots.push(state.allocate(Object::Null));
    }

    state.new_frame(None, slots);
    state.set_instruction_pointer(Some(start_address));
    while state.has_next_instruction_pointer() {
        interpret(&mut state, &mut output, program);
    }
}

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
    slots: Vec<Pointer>, /* ProgramObject::Slot */
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

    #[allow(dead_code)]
    pub fn from(return_address: Option<Address>, slots: Vec<Pointer>) -> Self {
        LocalFrame {
            return_address,
            slots,
        }
    }

    pub fn return_address(&self) -> &Option<Address> {
        &self.return_address
    }

    pub fn get_local(&self, index: &LocalFrameIndex) -> Option<Pointer> {
        match index.value() {
            index if index as usize >= self.slots.len() => None,
            index => Some(self.slots[index as usize].clone()), // new ref
        }
    }

    pub fn update_local(&mut self, index: &LocalFrameIndex, local: Pointer) -> Result<(), String> {
        match index.value() {
            index if index as usize >= self.slots.len() =>
                Err(format!("No local at index {} in frame", index)),
            index => {
                self.slots[index as usize] = local;
                Ok(())
            },
        }
    }

    #[allow(dead_code)]
    pub fn push_local(&mut self, local: Pointer) -> LocalFrameIndex {
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

#[derive(PartialEq,Debug)]
pub struct Memory {
    objects: HashMap<Pointer, Object>, // FIXME vec
    sequence: usize,
}

impl Memory {
    pub fn new() -> Self {
        Memory { objects: HashMap::new(), sequence: 0 }
    }

    #[allow(dead_code)]
    pub fn from(objects: Vec<Object>) -> Self {
        let mut sequence = 0;
        let mut object_map = HashMap::new();
        for object in objects {
            let pointer = Pointer::from(sequence);
            sequence += 1;
            let result = object_map.insert(pointer, object);
            assert!(result.is_none());
        }
        Memory { sequence, objects: object_map }
    }

    pub fn allocate(&mut self, object: Object) -> Pointer {
        let pointer = Pointer::from(self.sequence);
        self.sequence += 1;
        let result = self.objects.insert(pointer.clone(), object);
        assert!(result.is_none());
        pointer
    }

    pub fn dereference(&self, pointer: &Pointer) -> Option<&Object> {
        self.objects.get(pointer)
    }

    pub fn dereference_mut(&mut self, pointer: &Pointer) -> Option<&mut Object> {
        self.objects.get_mut(pointer)
    }

    pub fn copy(&mut self, pointer: &Pointer) -> Option<Pointer> {
        let new_object = match self.objects.get(pointer) {
            Some(object) => Some(object.clone()),
            None => None,
        };

        match new_object {
            Some(object) => Some(self.allocate(object)),
            None => None,
        }
    }

    #[allow(dead_code)]
    pub fn write_over(&mut self, pointer: Pointer, object: Object) -> Result<(),String> {
        let previous_value = self.objects.insert(pointer, object);
        match previous_value {
            Some(_) => Ok(()),
            None =>
                Err(format!("Expected an object at {:?} to write over, but none was found",
                             pointer)),
        }
    }

    pub fn dereference_to_string(&self, pointer: &Pointer) -> String {
        let object = self.dereference(&pointer)
            .expect(&format!("Expected object at {:?} to convert to string, but none was found",
                              pointer));

        match object {
            Object::Null => "null".to_string(),
            Object::Integer(n) => n.to_string(),
            Object::Boolean(b) => b.to_string(),
            Object::Array(elements) => {
                let mut buffer = String::new();
                buffer.push('[');
                for (i, e) in elements.iter().enumerate() {
                    buffer.push_str(&self.dereference_to_string(e));
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
                buffer.push_str(&self.dereference_to_string(parent));
                buffer.push_str(", ");

                for (i, (name, field)) in fields.iter().enumerate() {
                    buffer.push_str(name);
                    buffer.push_str("=");
                    buffer.push_str(&self.dereference_to_string(field));
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

pub struct State {
    pub instruction_pointer: Option<Address>,
    pub frames: Vec<LocalFrame>,
    pub operands: Vec<Pointer>,
    pub globals: HashMap<String, Pointer>,
    pub functions: HashMap<String, ProgramObject>,
    pub memory: Memory,
}

impl State {
    pub fn from(program: &Program) -> Self {

        let entry_index = program.entry();
        let entry_method = program.get_constant(entry_index)
            .expect(&format!("State init error: entry method is not in the constant pool \
                              at index {:?}", entry_index));

        let instruction_pointer = *match entry_method {
            ProgramObject::Method { name: _, arguments: _, locals: _, code } => code.start(),
            _ => panic!("State init error: entry method is not a Method {:?}", entry_method),
        };

        let mut globals: HashMap<String, Pointer> = HashMap::new();
        let mut functions: HashMap<String, ProgramObject> = HashMap::new();
        let mut memory: Memory = Memory::new();

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

                    let pointer = memory.allocate(Object::Null);
                    globals.insert(name.to_string(), pointer);
                }

                ProgramObject::Method { name: index, arguments: _, locals: _, code: _ } => {
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
                _ => panic!("State init error: name of global at index {:?} is not a String {:?}",
                            index, thing),
            };
        }

        let frames = vec!(LocalFrame::empty());

        State {
            instruction_pointer: Some(instruction_pointer),
            frames,
            operands: Vec::new(),
            globals,
            functions,
            memory,
        }
    }

    #[allow(dead_code)]
    pub fn empty() -> Self {
        State {
            instruction_pointer: None,
            frames: Vec::new(),
            operands: Vec::new(),
            globals: HashMap::new(),
            functions: HashMap::new(),
            memory: Memory::new(),
        }
    }

    #[allow(dead_code)]
    pub fn minimal() -> Self {
        State {
            instruction_pointer: Some(Address::from_usize(0)),
            frames: vec!(LocalFrame::empty()),
            operands: Vec::new(),
            globals: HashMap::new(),
            functions: HashMap::new(),
            memory: Memory::new(),
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

    pub fn has_next_instruction_pointer(&mut self) -> bool {
        self.instruction_pointer != None
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

    pub fn new_frame(&mut self, return_address: Option<Address>, slots: Vec<Pointer>, ) {
        self.frames.push(LocalFrame { slots, return_address });
    }

    pub fn peek_operand(&mut self) -> Option<&Pointer> {
        self.operands.last()
    }

    pub fn pop_operand(&mut self) -> Option<Pointer> {
        self.operands.pop()
    }

    pub fn push_operand(&mut self, object: Pointer) {
        self.operands.push(object)
    }

    pub fn allocate_and_push_operand(&mut self, object: Object) {
        self.operands.push(self.memory.allocate(object))
    }

    pub fn get_function(&self, name: &str) -> Option<&ProgramObject> {
        self.functions.get(name)
    }

    #[allow(dead_code)]
    pub fn get_global(&self, name: &str) -> Option<&Pointer> {
        self.globals.get(name)
    }

    #[allow(dead_code)]
    pub fn register_global(&mut self, name: String, object: Pointer) -> Result<(), String> {
        if self.globals.contains_key(&name) {
            Err(format!("Global {} already registered (with value {:?})",
                        &name, self.globals.get(&name).unwrap()))
        } else {
            self.globals.insert(name, object);
            Ok(())
        }
    }

    #[allow(dead_code)]
    pub fn allocate_and_register_global(&mut self, name: String, object: Object) -> Result<(), String> {
        let pointer = self.allocate(object);
        self.register_global(name, pointer)
    }

    pub fn update_global(&mut self, name: String, object: Pointer) {
        self.globals.insert(name, object);
    }

    pub fn set_instruction_pointer_from_label(&mut self, program: &Program, name: &str) -> Result<(), String> {
        match program.get_label(name) {
            None => Err(format!("Label {} does not exist", name)),
            Some(address) => {
                self.instruction_pointer = Some(*address);
                Ok(())
            }
        }
    }

    #[allow(dead_code)]
    pub fn push_global_to_operand_stack(&mut self, name: &str) -> Result<(), String> {
        let global = self.get_global(name).map(|e| e.clone());
        match global {
            Some(global) => {
                self.push_operand(global);
                Ok(())
            },
            None => {
                Err(format!("No such global {}", name))
            }
        }
    }

    pub fn dereference_to_string(&self, pointer: &Pointer) -> String {
        self.memory.dereference_to_string(pointer)
    }

    pub fn dereference_mut(&mut self, pointer: &Pointer) -> Option<&mut Object> {
        self.memory.dereference_mut(pointer)
    }

    pub fn dereference(&self, pointer: &Pointer) -> Option<&Object> {
        self.memory.dereference(pointer)
    }

    pub fn allocate(&mut self, object: Object) -> Pointer {
        self.memory.allocate(object)
    }

    pub fn copy_memory(&mut self, pointer: &Pointer) -> Option<Pointer> {
        self.memory.copy(pointer)
    }

    #[allow(dead_code)]
    pub fn pass_by_value_or_reference(&mut self, pointer: &Pointer) -> Option<Pointer> {
        let object = self.dereference(pointer).map(|e| e.clone());

        if object.is_none() {
            return None
        }

        let pass_by_value = object.as_ref().map_or(false, |e| match e {
            Object::Object { parent:_, methods:_, fields:_ } => false,
            Object::Array(_) => false,
            Object::Integer(_) => true,
            Object::Boolean(_) => true,
            Object::Null => true,
        });

        if pass_by_value {
            Some(self.allocate(object.unwrap()))
        } else {
            Some(*pointer)
        }
    }
}

pub fn interpret<Output>(state: &mut State, output: &mut Output, /*memory: &mut Memory,*/ program: &Program)
    where /*Input : Read,*/ Output : Write {

    println!("Stack:");
    for pointer in state.operands.iter() {
        println!("  {:?}: {:?}", pointer, state.memory.objects.get(&pointer));
    }
    println!("Memory:");
    for (pointer, object) in state.memory.objects.iter() {
        println!("  {:?}: {:?}", pointer, object);
    }
    println!("Interpreting {:?}: {:?}", state.instruction_pointer(), state.instruction_pointer().map(|opcode| program.code().get_opcode(&opcode)));

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

            state.allocate_and_push_operand(Object::from_constant(constant));
            state.bump_instruction_pointer(program);
        }

        OpCode::GetLocal { index } => {
            let frame: &LocalFrame = state.current_frame()
                .expect("Get local error: no frame on stack.");

            let local: Pointer = frame.get_local(&index)
                .expect(&format!("Get local error: there is no local at index {:?} in the current \
                                  frame", index));

            state.push_operand(local);
            state.bump_instruction_pointer(program);
        }

        OpCode::SetLocal { index } => {
            let operand: Pointer = *state.peek_operand()
                .expect("Set local error: cannot pop from empty operand stack");

            let frame: &mut LocalFrame = state.current_frame_mut()
                .expect("Set local error: no frame on stack.");

            frame.update_local(index, operand)
                .expect(&format!("Set local error: there is no local at index {:?} in the current \
                                  frame", index));

            state.bump_instruction_pointer(program);
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
        }

        OpCode::SetGlobal { name: index } => {
            let constant: &ProgramObject = program.get_constant(index)
                .expect(&format!("Set global error: no constant at index {:?}", index.value()));

            let name: &str = match constant {
                ProgramObject::String(s) => s,
                _ => panic!("Set global error: constant at index {:?} must be a String, \
                             but it is {:?}", index.value(), constant),
            };

            let operand: Pointer = state.peek_operand().map(|o| o.clone())
                .expect("Set global: cannot pop operand from empty operand stack");

            state.update_global(name.to_string(), operand);

            state.bump_instruction_pointer(program);
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

            let fields: HashMap<String, Pointer> = {
                let mut map: HashMap<String, Pointer> = HashMap::new();
                for slot in slots.into_iter().rev() {
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

            state.allocate_and_push_operand(Object::from(parent, fields, method_map));
            state.bump_instruction_pointer(program);
        }

        OpCode::Array => {
            let initializer = state.pop_operand()
                .expect(&format!("Array error: cannot pop initializer from empty operand stack"));

            let size_pointer = state.pop_operand()
                .expect(&format!("Array error: cannot pop size from empty operand stack"));

            let size_object: &Object = state.dereference(&size_pointer)
                .expect(&format!("Array error: pointer does not reference an object in memory {:?}",
                                 size_pointer));

            let size: usize = match size_object {
                Object::Integer(n) => {
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

            let mut elements: Vec<Pointer> = Vec::new();
            for _ in 0..size {
                let pointer = state.copy_memory(&initializer)
                    .expect(&format!("Array error: no initializer to copy from at {:?}",
                                      initializer));
                elements.push(pointer);
            }

            state.allocate_and_push_operand(Object::from_pointers(elements));
            state.bump_instruction_pointer(program);
        }

        OpCode::GetSlot { name: index } => {
            let constant: &ProgramObject = program.get_constant(index)
                .expect(&format!("Get slot error: no constant to serve as label name at index {:?}",
                                 index.value()));

            let name: &str = match constant {
                ProgramObject::String(s) => s,
                _ => panic!("Get slot error: constant at index {:?} must be a String, but it is {:?}",
                            index, constant),
            };

            let operand_pointer: Pointer = state.pop_operand()
                .expect(&format!("Get slot error: cannot pop operand from empty operand stack"));

            let operand = state.dereference(&operand_pointer)
                .expect(&format!("Get slot error: no operand object at {:?}", operand_pointer));

            match operand {
                Object::Object { parent:_, fields, methods:_ } => {
                    let slot: Pointer = fields.get(name)
                        .expect(&format!("Get slot error: no field {} in object", name))
                        .clone();

                    state.push_operand(slot)
                }
                _ => panic!("Get slot error: attempt to access field of a non-object {:?}", operand)
            }; // this semicolon turns the expression into a statement and is *important* because of
               // how temporaries work https://github.com/rust-lang/rust/issues/22449

            state.bump_instruction_pointer(program);
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

            let value: Pointer = state.pop_operand()
                .expect(&format!("Set slot error: cannot pop operand (value) from empty operand \
                                  stack"));

            let host_pointer: Pointer = state.pop_operand().clone()
                .expect(&format!("Set slot error: cannot pop operand (host) from empty operand \
                                  stack"));

            let host = state.dereference_mut(&host_pointer)
                .expect(&format!("Set slot error: no operand object at {:?}", host_pointer));

            match host {
                Object::Object { parent:_, fields, methods:_ } => {
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
        }

        OpCode::CallMethod { name: index, arguments: parameters } => {
            if parameters.value() == 0 {
                panic!("Call method error: method must have at least one parameter (receiver)");
            }

            let mut arguments: VecDeque<Pointer> = VecDeque::with_capacity(parameters.value() as usize);
            for index in 0..(parameters.to_usize() - 1) {
                let element = state.pop_operand()
                    .expect(&format!("Call method error: cannot pop argument {} from empty operand \
                                      stack", index));
                arguments.push_front(element);
            }

            let object_pointer: Pointer = state.pop_operand()
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

            let object: &mut Object = state.dereference_mut(&object_pointer)
                .expect(&format!("Call method error: no operand object at {:?}", object_pointer));


            println!("Dispatch! {:?}.{}({:?})", object_pointer, name, arguments);

            match object {
                Object::Null =>
                    interpret_null_method(object_pointer, name, &Vec::from(arguments), state, program),
                Object::Integer(_) =>
                    interpret_integer_method(object_pointer, name, &Vec::from(arguments), state, program),
                Object::Boolean(_) =>
                    interpret_boolean_method(object_pointer, name, &Vec::from(arguments), state, program),
                Object::Array(_) =>
                    interpret_array_method(object_pointer, name, &Vec::from(arguments), *parameters, state, program),
                Object::Object { parent:_, fields:_, methods:_ } =>
                    dispatch_object_method(object_pointer, name, &Vec::from(arguments), *parameters, state, program),
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

                    let mut slots: VecDeque<Pointer> =
                        VecDeque::with_capacity(parameters.value() as usize + locals.value() as usize);

                    for index in 0..arguments.to_usize() {
                        let element = state.pop_operand()
                            .expect(&format!("Call function error: cannot pop argument {} from \
                                              empty operand stack", index));
                        slots.push_front(element);
                    }

                    for _ in 0..locals.value() {
                        slots.push_back(state.allocate(Object::Null))
                    }

                    state.bump_instruction_pointer(program);
                    state.new_frame(*state.instruction_pointer(), Vec::from(slots));
                    state.set_instruction_pointer(Some(*range.start()));
                },
                _ => panic!("Call function error: constant at index {:?} must be a Method, but it \
                             is {:?}", index, constant),
            }
        }

        OpCode::Print { format: index, arguments } => {
            let mut argument_values = {
                let mut argument_values: Vec<Pointer> = Vec::new();
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

            let mut escape = false;
            for character in format.chars() {
                match (character, escape) {
                    ('~', _) => {
                        let string = &argument_values.pop()
                            .map(|e| state.dereference_to_string(&e))
                            .expect(&format!("Print error: Not enough arguments for format {}",
                                             format));

                        output.write_str(string)
                            .expect("Print error: Could not write to output stream.")
                    },
                    ('\\', false) => {
                        escape = true;
                    }
                    ('\\', true)  => {
                        output.write_char('\\')
                            .expect("Print error: Could not write to output stream.");
                        escape = false;
                    }
                    ('n', true)  => {
                        output.write_char('\n')
                            .expect("Print error: Could not write to output stream.");
                        escape = false;
                    }
                    ('t', true)  => {
                        output.write_char('\t')
                            .expect("Print error: Could not write to output stream.");
                        escape = false;
                    }
                    (character, true)  => {
                        panic!("Print error: Unknown escape sequence: \\{}", character)
                    }
                    (character, false) => {
                        output.write_char(character)
                            .expect("Print error: Could not write to output stream.")
                    }
                }
            }

            if !argument_values.is_empty() {
                panic!("Print error: Unused arguments for format {}", format)
            }

            state.allocate_and_push_operand(Object::Null);
            state.bump_instruction_pointer(program);
        }

        OpCode::Label { name: _ } => {
            state.bump_instruction_pointer(program);
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
                .expect(&format!("Jump error: no such label {:?} (labels: {:?})", name, program.labels()));
        }

        OpCode::Branch { label } => {
            let operand = state.pop_operand()
                .expect("Branch error: cannot pop operand from empty operand stack");

            let jump_condition_object = state.dereference(&operand)
                .expect(&format!("Branch error: cannot find condition at {:?}", operand));

            let jump_condition = {
                match jump_condition_object {
                    Object::Boolean(value) => *value,
                    Object::Null => false,
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

        OpCode::Drop => { // FIXME balance stack
            state.pop_operand()
                .expect("Drop error: cannot pop operand from empty operand stack");
            state.bump_instruction_pointer(program);
        },

        OpCode::Skip => {
            state.bump_instruction_pointer(program);
        }
    }
}

macro_rules! check_arguments_one {
    ($pointer: expr, $arguments: expr, $name: expr, $state: expr) => {{
        if $arguments.len() != 1 {
            panic!("Call method error: method {} takes 1 argument, but {} were supplied",
                    $name, $arguments.len())
        }

        let argument_pointer: &Pointer = &$arguments[0];
        let argument = $state.dereference(argument_pointer)
            .expect(&format!("Call method error: no operand object at {:?}", argument_pointer));

        let object = $state.dereference(&$pointer).unwrap(); /*checked earlier*/
        (object, argument)
    }}
}

macro_rules! push_result_and_finish {
    ($result: expr, $state: expr, $program: expr) => {{
        $state.allocate_and_push_operand($result);
        $state.bump_instruction_pointer($program);
    }}
}

macro_rules! push_pointer_and_finish {
    ($result: expr, $state: expr, $program: expr) => {{
        $state.push_operand($result);
        $state.bump_instruction_pointer($program);
    }}
}

pub fn interpret_null_method(pointer: Pointer, name: &str, arguments: &Vec<Pointer>,
                             state: &mut State, program: &Program) {

    let (object, operand) = check_arguments_one!(pointer, arguments, name, state);
    let result = match (name, operand) {
        ("==", Object::Null)  => Object::from_bool(true),
        ("==", _)             => Object::from_bool(false),
        ("!=", Object::Null)  => Object::from_bool(false),
        ("!=", _)             => Object::from_bool(true),
        ("eq", Object::Null)  => Object::from_bool(true),
        ("eq", _)             => Object::from_bool(false),
        ("neq", Object::Null) => Object::from_bool(false),
        ("neq", _)            => Object::from_bool(true),

        _ => panic!("Call method error: object {:?} has no method {} for operand {:?}",
                     object, name, operand),
    };
    push_result_and_finish!(result, state, program);
}

pub fn interpret_integer_method(pointer: Pointer, name: &str, arguments: &Vec<Pointer>,
                                state: &mut State, program: &Program) {

    let (object, operand) = check_arguments_one!(pointer, arguments, name, state);
    let result = match (object, name, operand) {
        (Object::Integer(i), "+",   Object::Integer(j)) => Object::from_i32 (*i +  *j),
        (Object::Integer(i), "-",   Object::Integer(j)) => Object::from_i32 (*i -  *j),
        (Object::Integer(i), "*",   Object::Integer(j)) => Object::from_i32 (*i *  *j),
        (Object::Integer(i), "/",   Object::Integer(j)) => Object::from_i32 (*i /  *j),
        (Object::Integer(i), "%",   Object::Integer(j)) => Object::from_i32 (*i %  *j),
        (Object::Integer(i), "<=",  Object::Integer(j)) => Object::from_bool(*i <= *j),
        (Object::Integer(i), ">=",  Object::Integer(j)) => Object::from_bool(*i >= *j),
        (Object::Integer(i), "<",   Object::Integer(j)) => Object::from_bool(*i <  *j),
        (Object::Integer(i), ">",   Object::Integer(j)) => Object::from_bool(*i >  *j),
        (Object::Integer(i), "==",  Object::Integer(j)) => Object::from_bool(*i == *j),
        (Object::Integer(i), "!=",  Object::Integer(j)) => Object::from_bool(*i != *j),
        (Object::Integer(_), "==",  _)                  => Object::from_bool(false),
        (Object::Integer(_), "!=",  _)                  => Object::from_bool(true),

        (Object::Integer(i), "add", Object::Integer(j)) => Object::from_i32 (*i +  *j),
        (Object::Integer(i), "sub", Object::Integer(j)) => Object::from_i32 (*i -  *j),
        (Object::Integer(i), "mul", Object::Integer(j)) => Object::from_i32 (*i *  *j),
        (Object::Integer(i), "div", Object::Integer(j)) => Object::from_i32 (*i /  *j),
        (Object::Integer(i), "mod", Object::Integer(j)) => Object::from_i32 (*i %  *j),
        (Object::Integer(i), "le",  Object::Integer(j)) => Object::from_bool(*i <= *j),
        (Object::Integer(i), "ge",  Object::Integer(j)) => Object::from_bool(*i >= *j),
        (Object::Integer(i), "lt",  Object::Integer(j)) => Object::from_bool(*i <  *j),
        (Object::Integer(i), "gt",  Object::Integer(j)) => Object::from_bool(*i >  *j),
        (Object::Integer(i), "eq",  Object::Integer(j)) => Object::from_bool(*i == *j),
        (Object::Integer(i), "neq", Object::Integer(j)) => Object::from_bool(*i != *j),
        (Object::Integer(_), "eq",  _)                  => Object::from_bool(false),
        (Object::Integer(_), "neq", _)                  => Object::from_bool(true),

        _ => panic!("Call method error: object {:?} has no method {} for operand {:?}",
                     object, name, operand),
    };
    push_result_and_finish!(result, state, program);
}

pub fn interpret_boolean_method(pointer: Pointer, name: &str, arguments: &Vec<Pointer>,
                                state: &mut State, program: &Program) {

    let (object, operand) = check_arguments_one!(pointer, arguments, name, state);
    let result = match (object, name, operand) {
        (Object::Boolean(p), "and", Object::Boolean(q)) => Object::from_bool(*p && *q),
        (Object::Boolean(p), "or",  Object::Boolean(q)) => Object::from_bool(*p || *q),
        (Object::Boolean(p), "eq",  Object::Boolean(q)) => Object::from_bool(*p == *q),
        (Object::Boolean(p), "neq", Object::Boolean(q)) => Object::from_bool(*p != *q),
        (Object::Boolean(_), "eq",  _)                  => Object::from_bool(false),
        (Object::Boolean(_), "neq", _)                  => Object::from_bool(true),

        (Object::Boolean(p), "&",   Object::Boolean(q)) => Object::from_bool(*p && *q),
        (Object::Boolean(p), "|",   Object::Boolean(q)) => Object::from_bool(*p || *q),
        (Object::Boolean(p), "==",  Object::Boolean(q)) => Object::from_bool(*p == *q),
        (Object::Boolean(p), "!=",  Object::Boolean(q)) => Object::from_bool(*p != *q),
        (Object::Boolean(_), "==",  _)                  => Object::from_bool(false),
        (Object::Boolean(_), "!=",  _)                  => Object::from_bool(true),

        _ => panic!("Call method error: object {:?} has no method {} for operand {:?}",
                    object, name, operand),
    };
    push_result_and_finish!(result, state, program);
}

pub fn interpret_array_method(pointer: Pointer, name: &str, arguments: &Vec<Pointer>,
                              arity: Arity, state: &mut State, program: &Program) {

    if arguments.len() != arity.to_usize() - 1 {
        panic!("Call method error: Array method {} takes {} argument, but {} were supplied",
                name, arity.value() - 1, arguments.len())
    }

    // if name == "length" {
    //     let object = state.dereference(&pointer);
    //     match object {
    //         Some(Object::Array(element_pointers)) => {
    //             let length = element_pointers.len() as i32; // was originally converted from i32 to usize, so should be ok.
    //             let length_object = Object::from_i32(length);
    //             let pointer = state.allocate(length_object);
    //             push_pointer_and_finish!(pointer, state, program);
    //         },
    //         None => panic!("Cannot dereference pointer: {:?}, no such object in memory"),
    //         _ => panic!("Call method error: object {:?} has no method {} for zero operands",
    //                     object, name),
    //     }
    // }

    if name == "get" {
        let (object, operand) = check_arguments_one!(pointer, arguments, name, state);
        let result = match (object, operand) {
            (Object::Array(element_pointers), Object::Integer(index)) => {
                if (*index as usize) >= element_pointers.len() {
                    panic!("Call method error: array index {} is out of bounds (should be < {})",
                            index, element_pointers.len())
                }
                element_pointers.get(*index as usize)
                    .expect("Call method error: no array element object at {:?}")
            },
            _ => panic!("Call method error: array {:?} has no method {} for operand {:?}",
                         object, name, operand),
        }.clone();

        push_pointer_and_finish!(result, state, program);
    }

    if name == "set" {
        let operand_1_pointer: &Pointer = &arguments[0];
        let operand_2_pointer: &Pointer = &arguments[1];

        let index: usize = match state.dereference(operand_1_pointer) {
            Some(Object::Integer(index)) => *index as usize,
            Some(object) => panic!("Call method error: cannot index array with {:?}", object),
            None => panic!("Call method error: no operand (1) object at {:?}", operand_1_pointer),
        };

        let object : &mut Object = state.dereference_mut(&pointer).unwrap(); /* pre-checked elsewhere */
        let result = match object {
            Object::Array(element_pointers) => {
                if index >= element_pointers.len() {
                    panic!("Call method error: array index {} is out of bounds (should be < {})",
                           index, element_pointers.len())
                }
                element_pointers[index] = *operand_2_pointer;
                Object::Null
            },
            _ => panic!("Call method error: object {:?} has no method {}", object, name),
        };

        push_result_and_finish!(result, state, program)
    }
}

fn dispatch_object_method(pointer: Pointer, name: &str, arguments: &Vec<Pointer>, arity: Arity,
                          state: &mut State, program: &Program) {

    println!("Dispatch! {:?}.{}({:?})/{:?}", pointer, name, arguments,arity);

    let mut cursor: Pointer = pointer;
    loop {
        let object = state.dereference(&cursor)
            .expect("Call method error: no object at {:?}");

        let method: ProgramObject = match object {
            Object::Object { parent, fields: _, methods } => {
                if let Some(method) = methods.get(name) {
                    method.clone()
                } else {
                    cursor = *parent;
                    continue
                }
            },
            Object::Null => {
                interpret_null_method(cursor, name, arguments, state, program);
                break
            },
            Object::Boolean(_) => {
                interpret_boolean_method(cursor, name, arguments, state, program);
                break
            },
            Object::Integer(_) => {
                interpret_integer_method(cursor, name, arguments, state, program);
                break
            },
            Object::Array(_) => {
                interpret_array_method(cursor, name, arguments, arity, state, program);
                break
            },
        };

        interpret_object_method(method, cursor, name, arguments, state, program);
        break
    }
}

fn interpret_object_method(method: ProgramObject, pointer: Pointer, name: &str,
                           arguments: &Vec<Pointer>, state: &mut State, program: &Program) {

    match method {
        ProgramObject::Method { name: _, locals, arguments: arity, code } => {
            if arguments.len() != arity.to_usize() - 1 {
                panic!("Call method error: method {} takes {} arguments, but {} were supplied",
                        name, arity.value() - 1, arguments.len())
            }

            let mut slots: Vec<Pointer> =
                Vec::with_capacity(1 + arity.to_usize() + locals.to_usize());

            slots.push(pointer);

            slots.extend(arguments); // TODO passes by reference... correct?

            for _ in 0..locals.to_usize() {
                slots.push(state.allocate(Object::Null))
            }

            state.bump_instruction_pointer(program);
            state.new_frame(*state.instruction_pointer(), slots);
            state.set_instruction_pointer(Some(*code.start()));
        },

        thing => panic!("Call method error: member {} in object definition should have type \
                         Method, but it is {:?}", name, thing),
    }
}