use fml_ast;
use crate::bytecode::OpCode;
use fml_ast::{AST, Identifier};
use crate::program::Program;
use crate::objects::ProgramObject;
use crate::types::{LocalFrameIndex, ConstantPoolIndex, Arity, Size, AddressRange};
use std::collections::HashMap;

#[derive(PartialEq,Debug,Clone)]
pub struct Bookkeeping { // TODO rename
    locals: Vec<HashMap<String, LocalFrameIndex>>,
//    labels: HashMap<String, ConstantPoolIndex>,
}

impl Bookkeeping {
    pub fn empty() -> Bookkeeping {
        Bookkeeping { locals: vec!(HashMap::new()) }
    }

    pub fn from(locals: Vec<String>) -> Bookkeeping {
        let mut local_map: HashMap<String, LocalFrameIndex> = HashMap::new();

        for (i, local) in locals.into_iter().enumerate() {
            local_map.insert(local, LocalFrameIndex::from_usize(i));
        }

        Bookkeeping { locals: vec!(local_map) }
    }

    fn register_local(&mut self, id: &str) -> LocalFrameIndex {
        let mut locals = self.locals.last_mut()
            .expect("Bookkeeping: cannot register local, no frame on stack");

        if let Some(index) = locals.get(id) {
            return *index;
        }

        let index = LocalFrameIndex::from_usize(locals.len());
        locals.insert(id.to_string(), index);
        index
    }

    fn count_locals(&self) -> usize {
        let mut locals = self.locals.last()
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
        let result = locals.insert(format!("${}_{}", name, locals.len()), index);
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

            AST::LocalDefinition { local: name, value } => {
                let index: LocalFrameIndex = environment.register_local(name.to_str()).clone();
                (**value).compile_into(program, environment);    // FIXME scoping!!!
                program.emit_code(OpCode::SetLocal { index });
            }

            AST::LocalAccess { local: name } => {
                let index: LocalFrameIndex = environment.register_local(name.to_str()).clone();
                program.emit_code(OpCode::GetLocal { index });
            }

            AST::LocalMutation { local: name, value } => {
                let index: LocalFrameIndex = environment.register_local(name.to_str()).clone();
                (**value).compile_into(program, environment);    // FIXME scoping!!!
                program.emit_code(OpCode::SetLocal { index })
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
                match **value {
                    AST::Boolean(_) | AST::Number(_) | AST::Unit |
                    AST::LocalAccess { local:_ } | AST::FieldAccess { object:_, field:_ } => {
                        (**size).compile_into(program, environment);
                        (**value).compile_into(program, environment);
                        program.emit_code(OpCode::Array);
                    }
                    _ => {
                        unimplemented!()
                        /*
                         * evaluate <size>          // stack: [SIZE]
                         * local $sz                // stack: []                          $sz = SIZE
                         * push $sz to stack        // stack: [SIZE]
                         * null                     // stack: [SIZE, null]
                         * array                    // stack: [array(null, null, ...)]
                         * integer 0                // stack: [array(null, null, ...), 0]
                         * local $i                 // stack: [array(null, null, ...)]    $i = 0

                         * local $arr               // stack: []       $arr = array(null, null, ...)
                         * label $array_init_start:
                         *
                         *   push $i to stack       // stack: [$i]
                         *   push $sz to stack      // stack: [$i, $sz]
                         *   method call ge         // stack: [R]        R = ($i >= $sz)
                         *   branch $array_init_end // stack: []
                         *
                         *   evaluate <value>       // stack: [VALUE]
                         *   push $i to stack       // stack: [VALUE, $i]
                         *   push $arr to stack     // stack: [VALUE, $i, $arr]
                         *   method call set        // stack: [null]               $arr[$i] <- VALUE
                         *   drop                   // stack: []
                         *   jump  $array_init_start
                         *
                         * label $array_init_end:
                         */
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

            AST::FunctionApplication { function: Identifier(name), arguments } => {
                let index = program.register_constant(ProgramObject::String(name.to_string()));
                for argument in arguments.iter() {
                    argument.compile_into(program, environment);
                }
                let arity = Arity::from_usize(arguments.len());
                program.emit_code(OpCode::CallFunction { name: index, arguments: arity });
            }

            AST::ObjectDefinition { extends: _, members: _ } => {

            }

            AST::FieldMutation { field_path: _, value: _ } => { unimplemented!() }
            AST::OperatorDefinition { operator: _, parameters: _, body: _ } => { unimplemented!() }
            AST::MethodCall { method_path: _, arguments: _ } => { unimplemented!() }
            AST::FieldAccess { object: _, field: _ } => { unimplemented!() }
            AST::OperatorAccess { object: _, operator: _ } => { unimplemented!() }
            AST::Block(_) => { unimplemented!() }
            AST::Operation { operator: _, left: _, right: _ } => { unimplemented!() }
        }
    }
}
