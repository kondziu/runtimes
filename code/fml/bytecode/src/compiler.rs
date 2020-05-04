use fml_ast;
use crate::bytecode::OpCode;
use fml_ast::{AST, Identifier};
use crate::program::Program;
use crate::objects::ProgramObject;
use crate::types::{LocalFrameIndex, ConstantPoolIndex, Arity, Size, AddressRange};
use std::collections::{HashMap, HashSet};
use crate::bytecode::OpCode::Literal;
use std::ops::Deref;

#[derive(PartialEq,Debug,Clone)]
pub struct Bookkeeping { // TODO rename
    locals: Vec<HashMap<String, LocalFrameIndex>>,
    globals: HashSet<String>,
//    labels: HashMap<String, ConstantPoolIndex>,
}

enum VariableIndex {
    Global(ConstantPoolIndex),
    Local(LocalFrameIndex),
}

impl Bookkeeping {
    pub fn with_frame() -> Bookkeeping {
        Bookkeeping { locals: vec!(HashMap::new()), globals: HashSet::new() }
    }

    pub fn without_frame() -> Bookkeeping {
        Bookkeeping { locals: vec!(), globals: HashSet::new() }
    }

    pub fn from(locals: Vec<String>, globals: Vec<String>) -> Bookkeeping {
        let mut local_map: HashMap<String, LocalFrameIndex> = HashMap::new();

        for (i, local) in locals.into_iter().enumerate() {
            local_map.insert(local, LocalFrameIndex::from_usize(i));
        }

        Bookkeeping { locals: vec!(local_map), globals: globals.into_iter().collect() }
    }

    pub fn from_locals(locals: Vec<String>) -> Bookkeeping {
        let mut local_map: HashMap<String, LocalFrameIndex> = HashMap::new();

        for (i, local) in locals.into_iter().enumerate() {
            local_map.insert(local, LocalFrameIndex::from_usize(i));
        }

        Bookkeeping { locals: vec!(local_map), globals: HashSet::new() }
    }

    pub fn from_globals(globals: Vec<String>) -> Bookkeeping {
        Bookkeeping { locals: vec!(), globals: globals.into_iter().collect() }
    }

    fn has_frame(&self) -> bool {
        !self.locals.is_empty()
    }

    fn has_local(&self, id: &str) -> bool {
        if self.locals.is_empty() {
            false
        } else {
            self.locals.last().unwrap().contains_key(id)
        }
    }

//    fn register_variable(&mut self, id: &str) -> VariableIndex {
//        match self.locals.last_mut() {
//            Some(locals) => VariableIndex::Local(self.register_local(id)),
//            None => VariableIndex::Global(self.register_global(id)),
//        }
//    }

    fn register_global(&mut self, id: &str) {
        self.globals.insert(id.to_string());
//        match self.globals.get(id) {
//            Some(index) => *index,
//            None => {
//                let index = ConstantPoolIndex::from_usize(self.globals.len());
//                self.globals.insert(id.to_string(), index);
//                index
//            },
//        }
    }

    fn register_local(&mut self, id: &str) -> LocalFrameIndex {
        let locals = self.locals.last_mut()
            .expect("Bookkeeping: cannot register local, no frame on stack");

        if let Some(index) = locals.get(id) {
            return *index;
        }

        let index = LocalFrameIndex::from_usize(locals.len());
        locals.insert(id.to_string(), index);
        index
    }

    fn count_locals(&self) -> usize {
        let locals = self.locals.last()
            .expect("Bookkeeping: cannot count locals, no frame on stack");

        locals.len()
    }

    fn add_frame(&mut self) {
        self.locals.push(HashMap::new())
    }

    fn remove_frame(&mut self)  {
        self.locals.pop().expect("Bookkeeping: cannot pop frame from an empty stack");
    }

    fn generate_new_local(&mut self, name: &str) -> LocalFrameIndex {
        let locals = self.locals.last_mut()
            .expect("Bookkeeping: cannot generate local, no frame on stack");

        let index = LocalFrameIndex::from_usize(locals.len());
        let result = locals.insert(format!("?{}_{}", name, locals.len()), index);
        assert!(result.is_none());
        index
    }
}

pub trait Compiled {
    fn compile_into(&self, program: &mut Program, environment: &mut Bookkeeping);
}

impl Compiled for AST {
    fn compile_into(&self, program: &mut Program, environment: &mut Bookkeeping) {
        match self {
            AST::Number(value) => {
                let constant = ProgramObject::Integer(*value);
                let index = program.register_constant(constant);
                program.emit_code(OpCode::Literal { index });
            }

            AST::Boolean(value) => {
                let constant = ProgramObject::Boolean(*value);
                let index = program.register_constant(constant);
                program.emit_code(OpCode::Literal { index });
            }

            AST::Unit => {
                let constant = ProgramObject::Null;
                let index = program.register_constant(constant);
                program.emit_code(OpCode::Literal { index });
            }

            AST::VariableDefinition { name: Identifier(name), value } => {
                if environment.has_frame() {
                    let index = environment.register_local(name).clone();   // FIXME error if not new
                    value.deref().compile_into(program, environment);    // FIXME scoping!!!
                    program.emit_code(OpCode::SetLocal { index });
                } else {
                    let index = program.register_constant(ProgramObject::from_str(name));
                    environment.register_global(name);                  // TODO necessary?
                    value.deref().compile_into(program, environment);
                    program.emit_code(OpCode::SetGlobal { name: index });
                }
            }

            AST::VariableAccess { name: Identifier(name) } => {
                if environment.has_local(name) {
                    let index = environment.register_local(name).clone();   // FIXME error if does not exists
                    program.emit_code(OpCode::GetLocal { index });      // FIXME scoping!!!
                } else {
                    let index = program.register_constant(ProgramObject::from_str(name));
                    environment.register_global(name);                  // TODO necessary?
                    program.emit_code(OpCode::GetGlobal { name: index });
                }
            }

            AST::VariableMutation { name: Identifier(name), value } => {
                if environment.has_frame() {
                    let index = environment.register_local(name).clone(); // FIXME error if does not exists
                    value.deref().compile_into(program, environment);    // FIXME scoping!!!
                    program.emit_code(OpCode::SetLocal { index });
                } else {
                    let index = program.register_constant(ProgramObject::from_str(name));
                    environment.register_global(name);                  // TODO necessary?
                    value.deref().compile_into(program, environment);
                    program.emit_code(OpCode::SetGlobal { name: index });
                }
            }

            AST::Conditional { condition, consequent, alternative } => {
                let consequent_label_index = program.generate_new_label_name("if_consequent");
                let end_label_index = program.generate_new_label_name("if_end");

                (**condition).compile_into(program, environment);
                program.emit_code(OpCode::Branch { label: consequent_label_index} );
                (**alternative).compile_into(program, environment);
                program.emit_code(OpCode::Jump { label: end_label_index} );
                program.emit_code(OpCode::Label { name: consequent_label_index });
                (**consequent).compile_into(program, environment);
                program.emit_code(OpCode::Label { name: end_label_index });
            }

            AST::Loop { condition, body } => {
                let body_label_index = program.generate_new_label_name("loop_body");
                let condition_label_index = program.generate_new_label_name("loop_condition");

                program.emit_code(OpCode::Jump { label: condition_label_index });
                program.emit_code(OpCode::Label { name: body_label_index });
                (**body).compile_into(program, environment);
                program.emit_code(OpCode::Label { name: condition_label_index });
                (**condition).compile_into(program, environment);
                program.emit_code(OpCode::Branch { label: body_label_index });
            }

            AST::ArrayDefinition { size, value } => {
                match value.deref() {
                    AST::Boolean(_) | AST::Number(_) | AST::Unit |
                    AST::VariableAccess { name:_ } | AST::FieldAccess { object:_, field:_ } => {
                        size.deref().compile_into(program, environment);
                        value.deref().compile_into(program, environment);
                        program.emit_code(OpCode::Array);
                    },
                    _ => {
                        let body_label_index = program.generate_new_label_name("array_init_start"); //                                                                                                              constants:[null,array_init_start_0]
                        let end_label_index = program.generate_new_label_name("array_init_end");    //                                                                                                              constants:[null,array_init_start_0,ge,array_init_end_0]

                        size.deref().compile_into(program, environment);                            // <compile SIZE>           stack:[SIZE]                                    locals:[]                           constants:[]
                        let size_local_index = environment.generate_new_local("size");
                        program.emit_code(OpCode::SetLocal { index: size_local_index });            // set local 0              stack:[SIZE]                                    locals:[SIZE]                       constants:[]

                        let null_index = program.register_constant(ProgramObject::Null);            //                                                                                                              constants:[null]
                        program.emit_code(OpCode::Literal { index: null_index });                   // literal 0                stack:[SIZE,null]                               locals:[SIZE]                       constants:[null]

                        let array_local_index = environment.generate_new_local("array");
                        program.emit_code(OpCode::Array);                                           // array                    stack:[array(SIZE,null)]                        locals:[SIZE]                       constants:[null]
                        program.emit_code(OpCode::SetLocal { index: array_local_index });           // set local 1              stack:[array(SIZE,null)]                        locals:[SIZE,array(SIZE,null)]      constants:[null]

                        let zero_index = program.register_constant(ProgramObject::Integer(0));      //                                                                                                              constants:[null,0]
                        program.emit_code(OpCode::Literal { index: zero_index });                   // literal 0                stack:[array(SIZE,null),0]                      locals:[SIZE,array(SIZE,null)]      constants:[null,0]

                        let iterator_local_index = environment.generate_new_local("i");
                        program.emit_code(OpCode::SetLocal { index: iterator_local_index });        // set local 2              stack:[array(SIZE,null),0]                      locals:[SIZE,array(SIZE,null),0]    constants:[null,0]


                        program.emit_code(OpCode::Label { name: body_label_index });                // label array_init_start   stack:[array(SIZE,null),0]                      locals:[SIZE,array(SIZE,null),0]    constants:[null,0,array_init_start_0]

                        program.emit_code(OpCode::GetLocal { index: size_local_index });            // get local 0              stack:[array(SIZE,null),0,SIZE]                 locals:[SIZE,array(SIZE,null),0]    constants:[null,0,array_init_start_0]

                        let ge_label_index =
                            program.register_constant(ProgramObject::from_str("ge"));               //                                                                                                              constants:[null,array_init_start_0,ge]
                        program.emit_code(OpCode::CallMethod { name: ge_label_index ,
                                                               arguments: Arity::new(2) });         // call method 3 2          stack:[array(SIZE,null),false]                  locals:[SIZE,array(SIZE,null),0]    constants:[null,0,array_init_start_0,ge]

                        program.emit_code(OpCode::Branch { label: end_label_index });               // branch 4                 stack:[array(SIZE,null)]                        locals:[SIZE,array(SIZE,null),0]    constants:[null,0,array_init_start_0,ge,array_init_end_0]

                        program.emit_code(OpCode::GetLocal { index: iterator_local_index });        // get local 2              stack:[array(SIZE,null),0]                      locals:[SIZE,array(SIZE,null),0]    constants:[null,0,array_init_start_0,ge,array_init_end_0]
                        value.deref().compile_into(program, environment);                           // <compile VALUE>          stack:[array(SIZE,null),0,VALUE]                locals:[SIZE,array(SIZE,null),0]    constants:[null,0,array_init_start_0,ge,array_init_end_0]

                        let set_index =
                            program.register_constant(ProgramObject::from_str("set"));              //                                                                                                              constants:[null,array_init_start_0,ge,array_init_end_0,set]
                        program.emit_code(OpCode::CallMethod { name: set_index ,
                                                               arguments: Arity::new(3) });         // call method 4 3          stack:[null]                                     locals:[SIZE,array(SIZE,null),0]   constants:[null,0,array_init_start_0,ge,array_init_end_0,set]
                        program.emit_code(OpCode::Drop);                                            // drop                     stack:[]                                         locals:[SIZE,array(SIZE,null),0]   constants:[null,0,array_init_start_0,ge,array_init_end_0,set]

                        let one_index = program.register_constant(ProgramObject::from_i32(1));      //                                                                                                              constants:[null,0,array_init_start_0,ge,array_init_end_0,set,1]
                        program.emit_code(OpCode::Literal { index: one_index });                    // literal 5                stack:[1]                                        locals:[SIZE,array(SIZE,null),0]   constants:[null,0,array_init_start_0,ge,array_init_end_0,set,1]
                        program.emit_code(OpCode::GetLocal { index: iterator_local_index });        // get local 2              stack:[1,0]                                      locals:[SIZE,array(SIZE,null),0]   constants:[null,0,array_init_start_0,ge,array_init_end_0,set,1]
                        let add_index = program.register_constant(ProgramObject::from_str("add"));  //                                                                                                              constants:[null,0,array_init_start_0,ge,array_init_end_0,set,1,add]
                        program.emit_code(OpCode::CallMethod { name: add_index,
                                                               arguments: Arity::new(2) });         // call method 7 2          stack:[1=1+0]                                    locals:[SIZE,array(SIZE,null),0]   constants:[null,0,array_init_start_0,ge,array_init_end_0,set,1,add]
                        program.emit_code(OpCode::SetLocal { index: iterator_local_index });        // set local 2              stack:[1=1+0]                                    locals:[SIZE,array(SIZE,null),1]   constants:[null,0,array_init_start_0,ge,array_init_end_0,set,1,add]
                        program.emit_code(OpCode::Drop);                                            // drop                     stack:[]                                         locals:[SIZE,array(SIZE,null),1]   constants:[null,0,array_init_start_0,ge,array_init_end_0,set,1,add]

                        program.emit_code(OpCode::GetLocal { index: array_local_index });           // get local 1              stack:[array(SIZE,null,0)]                       locals:[SIZE,array(SIZE,null),1]   constants:[null,0,array_init_start_0,ge,array_init_end_0,set,1,add]
                        program.emit_code(OpCode::Jump { label: body_label_index });                // jump 2
                        program.emit_code(OpCode::Label { name: end_label_index} );                 // label 4
                    }
                }
            }

            AST::ArrayAccess { array, index } => {
                (**array).compile_into(program, environment);
                (**index).compile_into(program, environment);
                let name = program.register_constant(ProgramObject::String("get".to_string()));
                program.emit_code(OpCode::CallMethod { name, arguments: Arity::new(2) });
            }

            AST::ArrayMutation { array, index, value } => {
                (**array).compile_into(program, environment);
                (**index).compile_into(program, environment);
                (**value).compile_into(program, environment);
                let name = program.register_constant(ProgramObject::String("set".to_string()));
                program.emit_code(OpCode::CallMethod { name, arguments: Arity::new(3) });
            }

            AST::Print { format, arguments } => {
                let format: ConstantPoolIndex =
                    program.register_constant(ProgramObject::String(format.to_string()));

                for argument in arguments.iter() {
                    argument.compile_into(program, environment);
                }

                let arguments = Arity::from_usize(arguments.len());
                program.emit_code(OpCode::Print { format, arguments });
            }

            AST::OperatorDefinition { operator, parameters, body } => {
                let name = operator.to_str();
                let end_label_index = program.generate_new_label_name("function_guard"); // FIXME merge with FunctionDefinition

                program.emit_code(OpCode::Jump { label: end_label_index });
                let start_address = program.get_upcoming_address();

                environment.add_frame();
                for parameter in parameters.into_iter() {
                    environment.register_local(parameter.to_str());
                }

                (**body).compile_into(program, environment);

                let locals_in_frame = environment.count_locals();
                environment.remove_frame();

                program.emit_code(OpCode::Return);
                program.emit_code(OpCode::Label { name: end_label_index });
                let end_address = program.get_current_address();

                let name = ProgramObject::String(name.to_string());
                let name_index = program.register_constant(name);

                let method = ProgramObject::Method {
                    name: name_index,
                    locals: Size::from_usize(locals_in_frame - parameters.len()),
                    arguments: Arity::from_usize(parameters.len()),
                    code: AddressRange::from_addresses(start_address, end_address),
                };
                program.register_constant(method);
            }

            AST::FunctionDefinition { function: Identifier(name), parameters, body } => {
                let end_label_index = program.generate_new_label_name("function_guard");

                program.emit_code(OpCode::Jump { label: end_label_index });
                let start_address = program.get_upcoming_address();

                environment.add_frame();
                for parameter in parameters.into_iter() {
                    environment.register_local(parameter.to_str());
                }

                (**body).compile_into(program, environment);

                let locals_in_frame = environment.count_locals();
                environment.remove_frame();

                program.emit_code(OpCode::Return);
                program.emit_code(OpCode::Label { name: end_label_index });
                let end_address = program.get_current_address();

                let name = ProgramObject::String(name.to_string());
                let name_index = program.register_constant(name);

                let method = ProgramObject::Method {
                    name: name_index,
                    locals: Size::from_usize(locals_in_frame - parameters.len()),
                    arguments: Arity::from_usize(parameters.len()),
                    code: AddressRange::from_addresses(start_address, end_address),
                };
                program.register_constant(method);
            }

            AST::FunctionCall { function: Identifier(name), arguments } => {
                let index = program.register_constant(ProgramObject::String(name.to_string()));
                for argument in arguments.iter() {
                    argument.compile_into(program, environment);
                }
                let arity = Arity::from_usize(arguments.len());
                program.emit_code(OpCode::CallFunction { name: index, arguments: arity });
            }

            AST::ObjectDefinition { extends, members } => {

                let slots: Vec<ConstantPoolIndex> = members.iter().map(|m| m.deref()).map(|m| match m {
                    AST::FunctionDefinition { function, parameters, body } => {
                        compile_function_definition(function.to_str(), parameters, body.deref(),
                                                    program, environment)

                    }
                    AST::OperatorDefinition { operator, parameters, body } => {
                        compile_function_definition(operator.to_str(), parameters, body.deref(),
                                                    program, environment)

                    }
                    AST::VariableDefinition { name: Identifier(name), value } => {
                        (*value).compile_into(program, environment);
                        let index = program.register_constant(ProgramObject::from_str(name));
                        program.register_constant(ProgramObject::slot_from_index(index))
                    },
                    _ => panic!("Object definition: cannot define a member from {:?}", m)
                }).collect();

                let class = ProgramObject::Class(slots);
                let class_index = program.register_constant(class);

                match extends {
                    Some(parent) => {
                        (**parent).compile_into(program, environment)
                    },
                    None => {
                        let index = program.register_constant(ProgramObject::Null);
                        program.emit_code(Literal { index })
                    },
                }

                program.emit_code(OpCode::Object { class: class_index })
            }

            AST::Block(children) => {
                for child in children {
                    child.deref().compile_into(program, environment)
                }
            }

            AST::FieldAccess { object, field: Identifier(name) } => {
                object.deref().compile_into(program, environment);
                let index = program.register_constant(ProgramObject::from_str(name));
                program.emit_code(OpCode::GetSlot { name: index })
            }

            AST::FieldMutation { object, field: Identifier(name), value } => {
                value.deref().compile_into(program, environment);
                object.deref().compile_into(program, environment);
                let index = program.register_constant(ProgramObject::from_str(name));
                program.emit_code(OpCode::SetSlot { name: index })
            }

            AST::MethodCall { object, method: Identifier(name), arguments } => {
                let index = program.register_constant(ProgramObject::from_str(name));
                for argument in arguments.iter() {
                    argument.compile_into(program, environment);
                }
                object.deref().compile_into(program, environment);
                let arity = Arity::from_usize(arguments.len() + 1);
                program.emit_code(OpCode::CallMethod { name: index, arguments: arity });
            }

            AST::OperatorCall { object, operator, arguments } => {
                let index = program.register_constant(ProgramObject::from_str(operator.to_str()));
                for argument in arguments.iter() {
                    argument.compile_into(program, environment);
                }
                object.deref().compile_into(program, environment);
                let arity = Arity::from_usize(arguments.len() + 1);
                program.emit_code(OpCode::CallMethod { name: index, arguments: arity });
            }

            AST::Operation { operator, left, right } => {
                let index = program.register_constant(ProgramObject::from_str(operator.to_str()));
                right.deref().compile_into(program, environment);
                left.deref().compile_into(program, environment);
                let arity = Arity::from_usize(2);
                program.emit_code(OpCode::CallMethod { name: index, arguments: arity });
            }
        }
    }
}

fn compile_function_definition(name: &str,
                               parameters: &Vec<Identifier>,
                               body: &AST,
                               program: &mut Program,
                               environment: &mut Bookkeeping) -> ConstantPoolIndex {

    let end_label_index = program.generate_new_label_name("function_guard");
    program.emit_code(OpCode::Jump { label: end_label_index });

    let expected_arguments = parameters.len();

    let start_address = program.get_upcoming_address();

    environment.add_frame();
    for parameter in parameters.into_iter() {
        environment.register_local(parameter.to_str());
    }

    body.compile_into(program, environment);

    let locals_in_frame = environment.count_locals();
    environment.remove_frame();

    program.emit_code(OpCode::Return);
    let end_address = program.get_current_address();

    program.emit_code(OpCode::Label { name: end_label_index });

    let name = ProgramObject::String(name.to_string());
    let name_index = program.register_constant(name);

    let method = ProgramObject::Method {
        name: name_index,
        locals: Size::from_usize(locals_in_frame - expected_arguments),
        arguments: Arity::from_usize(expected_arguments),
        code: AddressRange::from_addresses(start_address, end_address),
    };

    program.register_constant(method)
}
