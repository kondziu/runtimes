use crate::ast::AST;
use crate::environment::Environment;
use crate::environment::Object;

macro_rules! extract_identifier_token {
    ($ast:expr) => {
        match $ast {
            AST::Identifier(token) => token,
            ast => panic!("Expected AST::Identifier, but found {:?}", ast),
        }
    }
}

pub fn evaluate(environment: &mut Environment, expression: AST) -> Object {
    match expression {
        // let x = ...
        AST::LocalDefinition {identifier, value} => {
            let value = evaluate(&mut environment.child(), *value);

            let token = extract_identifier_token!(*identifier);
            environment.define_binding(token, value).unwrap();

            Object::Unit
        },

        //AST::LocalMutation {identifier, value}   => mutate_local(environment, identifier, value),

        // x
        AST::Identifier(token) => {
            let result = environment.lookup_binding(token);
            match result {
                Ok(object) => object,
                Err(e) => panic!("Cannot resolve identifier: {}", e),
            }
        },

        _ => panic!("Not implemented")
    }
}