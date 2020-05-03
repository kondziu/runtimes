use fml_ast::{AST, Identifier};
use crate::bytecode::OpCode;
use crate::types::{Address, ConstantPoolIndex, LocalFrameIndex};
use std::collections::HashMap;

#[derive(PartialEq,Debug,Clone)]
pub struct Blueprint {
    code: Vec<OpCode>,
    constants: HashMap<String, ConstantPoolIndex>,
    labels: HashMap<String, Address>,
}

impl Blueprint {
    pub fn new() -> Self {
        Blueprint {
            code: Vec::new(),
            constants: HashMap::new(),
            labels: HashMap::new(),
        }
    }
}

#[derive(PartialEq,Debug,Clone)]
struct LocalState {
    locals: HashMap<String, LocalFrameIndex>,
}

impl LocalState {
    fn new() -> Self {
        LocalState { locals: HashMap::new() }
    }
}

#[derive(PartialEq,Debug,Clone)]
struct GlobalState {
    locals: HashMap<String, LocalFrameIndex>,
}

#[derive(PartialEq,Debug,Clone)]
struct CompilationUnit {
    name: String,
    path: String,
    root: AST,
    locals: Vec<String>,
}

pub fn compile(ast: AST, blueprint: &mut Blueprint) {
    let top = CompilationUnit {
        name: String::new(),
        path: String::new(),
        root: ast,
        locals: Vec::new(),
    };

    let mut queue: Vec<CompilationUnit> = vec!(top);
    while !queue.is_empty() {
        println!("Before: {}/{:?}", queue.len(), queue);
        let compilation_unit = queue.pop().unwrap();
        let mut local_state = LocalState::new();
        compile_tree(compilation_unit.root,
                     &compilation_unit.path,
                     blueprint,
                     &mut local_state,
                     &mut queue);
        println!("After: {}/{:?}", queue.len(), queue);
    }
}

fn compile_tree(ast: AST,
                path: &str,
                blueprint: &mut Blueprint,
                local_state: &mut LocalState,
                queue: &mut Vec<CompilationUnit>) {

    println!("compile_tree:");
    println!("    {:?}", ast);
    println!("    {:?}", path);
    println!("    {:?}", blueprint);
    println!("    {:?}", local_state);
    println!("    {:?}", queue);
    println!();

    match ast {
        AST::Number(_) => {  }
        AST::Boolean(_) => {  }
        AST::Unit => {  }
        AST::VariableDefinition { name: _, value: _ } => {  }
        AST::ArrayDefinition { size: _, value: _ } => {  }
        AST::VariableMutation { name: _, value: _ } => {  }
        AST::FieldMutation { object: _, field: _, value: _ } => {  }
        AST::ArrayMutation { array: _, index: _, value: _ } => {  }
        AST::FunctionCall { function: _, arguments: _ } => {    }
        AST::MethodCall { object: _, method: _, arguments: _ } => {  }
        AST::OperatorCall { object: _, operator: _, arguments: _ } => {  }
        AST::Print { format: _, arguments: _ } => {  }
        AST::VariableAccess { name: _ } => {  }
        AST::FieldAccess { object: _, field: _ } => {  }
        AST::ArrayAccess { array: _, index: _ } => {  }
        AST::Block(children) => {
            for child in children {
                compile_tree(*child, path, blueprint, local_state, queue);
            }
        }
        AST::Operation { operator: _, left: _, right: _ } => {  }
        AST::Loop { condition: _, body: _ } => {  }
        AST::Conditional { condition: _, consequent: _, alternative: _ } => {  }
        AST::FunctionDefinition { function: Identifier(name), parameters, body } => {
            let path = if path.is_empty() {
                name.to_string()
            } else {
                format!("{}/{}", path, name)
            };
            let locals = parameters.into_iter().map(|p| p.to_string()).collect();
            let root = *body;
            queue.push(CompilationUnit { name, path, root, locals });
        }
        AST::OperatorDefinition { operator, parameters, body } => {
            let name = operator.to_string();
            let path = if path.is_empty() {
                name.to_string()
            } else {
                format!("{}/{}", path, name)
            };
            let locals = parameters.into_iter().map(|p| p.to_string()).collect();
            let root = *body;
            queue.push(CompilationUnit { name, path, root, locals });
        }
        AST::ObjectDefinition { extends: _, members: _ } => {
            // all the slots
            // all the methods
        }
    }
}

