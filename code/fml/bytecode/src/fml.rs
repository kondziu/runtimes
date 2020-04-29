use fml_ast;
use crate::bytecode::OpCode;
use fml_ast::{AST, Identifier};
use crate::program::Program;
use crate::objects::ProgramObject;
use crate::types::{LocalFrameIndex, ConstantPoolIndex, Arity};
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

#[derive(PartialEq,Debug,Clone)]
pub struct Bookkeeping { // TODO rename
    locals: HashMap<String, LocalFrameIndex>,
//    labels: HashMap<String, ConstantPoolIndex>,
}

impl Bookkeeping {
    pub fn empty() -> Bookkeeping {
        Bookkeeping { locals: HashMap::new() }
    }

    pub fn from(locals: Vec<String>) -> Bookkeeping {
        let mut local_map: HashMap<String, LocalFrameIndex> = HashMap::new();
        for (i, local) in locals.into_iter().enumerate() {
            local_map.insert(local, LocalFrameIndex::from_usize(i));
        }
        Bookkeeping { locals: local_map }
    }

//    pub fn from_labels(labels: Vec<String>) -> Bookkeeping {
//        let mut labels_map: HashMap<String, ConstantPoolIndex> = HashMap::new();
//        for (i, label) in labels.into_iter().enumerate() {
//            labels_map.insert(label, ConstantPoolIndex::from_usize(i));
//        }
//        Bookkeeping { locals: HashMap::new(), labels: labels_map }
//    }

//    pub fn from(locals: Vec<String>, labels: Vec<String>) -> Bookkeeping {
//
//        let mut local_map: HashMap<String, LocalFrameIndex> = HashMap::new();
//        for (i, local) in locals.into_iter().enumerate() {
//            local_map.insert(local, LocalFrameIndex::from_usize(i));
//        }

//        let mut labels_map: HashMap<String, ConstantPoolIndex> = HashMap::new();
//        for (i, label) in labels.into_iter().enumerate() {
//            labels_map.insert(label, ConstantPoolIndex::from_usize(i));
//        }

//        Bookkeeping { locals: local_map }
//    }

    fn register_local(&mut self, id: Identifier) -> LocalFrameIndex {
        if let Some(index) = self.locals.get(id.to_str()) {
            return *index;
        }
        let index = LocalFrameIndex::from_usize(self.locals.len());
        self.locals.insert(id.to_string(), index);
        index
    }

    fn generate_new_local(&mut self, name: &str) -> LocalFrameIndex {
        let index = LocalFrameIndex::from_usize(self.locals.len());
        let result = self.locals.insert(format!("${}_{}", name, self.locals.len()), index);
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
                let index: LocalFrameIndex = environment.register_local(name.clone()).clone();
                (**value).compile_into(program, environment);    // FIXME scoping!!!
                program.emit_code(OpCode::SetLocal { index });
            }

            AST::LocalAccess { local: name } => {
                let index: LocalFrameIndex = environment.register_local(name.clone()).clone();
                program.emit_code(OpCode::GetLocal { index });
            }

            AST::LocalMutation { local: name, value } => {
                let index: LocalFrameIndex = environment.register_local(name.clone()).clone();
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

            }

            AST::FunctionApplication { function: Identifier(name), arguments } => {
                let index = program.register_constant(ProgramObject::String(name.to_string()));
                for argument in arguments.iter() {
                    argument.compile_into(program, environment);
                }
                let arity = Arity::from_usize(arguments.len());
                program.emit_code(OpCode::CallMethod { name: index, arguments: arity });
            }

            AST::ObjectDefinition { extends: _, members: _ } => { unimplemented!() }
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
