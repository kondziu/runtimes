use crate::fml_ast::AST;

use std::collections::HashMap;
use std::error::Error;


//static fml_null: Object = Object::PLACEHOLDER;

enum Object {
    Reference(),
    PLACEHOLDER
}

struct Environment {
    locals: HashMap<String, Object>,
    functions: HashMap<String, Object>,
    parent: Box<Environment>,
}

impl Environment {
    fn local_is_defined(&mut self, name: &str) -> bool {
        self.locals.contains_key(name)
    }

    fn define_local(&mut self, name: &str, value: Object) -> Result<(), dyn Error> {
        if self.local_is_defined(name) {
            Err(format!("Attempt to define an already-defined local variable {}", name))
        } else {
            self.locals.insert(name.to_string(), value);
            Ok(())
        }
    }

    fn redefine_local(&mut self, name: &str, value: Object) -> Result<(), dyn Error> {
        if self.local_is_defined(name) {
            self.locals.insert(name.to_string(), value);
            Ok(())
        } else {
            Err(format!("Attempt to redefine an undefined local variable {}", name))
        }
    }
}

fn extract_identifier_token(ast: AST) -> &str {
    match ast {
        AST::Identifier(token) => token,
        _ => panic!("Cannot extract identifier token from non-Identifier AST: {:?}", ast)
    }
}

fn evaluate(environment: &mut Environment, expression: AST) -> Object {
    match expression {
        AST::LocalDefinition {identifier, value} => define_local(environment, *identifier, *value),
        AST::LocalMutation {identifier, value}   => mutate_local(environment, *identifier, *value),

        _ => print!("Not implemented")
    }
}

fn define_local(environment: &mut Environment, identifier: AST, value: AST) -> Object {
    let identifier_token = extract_identifier_token(identifier);
    let value = evaluate(environment, value); // FIXME clone
    environment.add_local(identifier_token, value)?;
}

fn mutate_local(environment: &mut Environment, identifier: AST, value: AST) -> Object {
    let identifier_token = extract_identifier_token(identifier);
    let value = evaluate(environment,value); // FIXME clone
    environment.set_local(identifier_token, value)?;
}