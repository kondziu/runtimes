use crate::ast::AST;

use crate::environment::Environment;
use crate::objects::{self, Object};

fn extract_identifier_token<'a>(ast: &'a AST) -> Result<&'a str, String> {
    match ast {
        AST::Identifier(token) => Ok(token),
        _ => Err(format!("Cannot extract identifier token from non-Identifier AST: {:?}", ast))
    }
}

fn evaluate<'a>(environment: &'a mut Environment, expression: &AST) -> Result<Object, String> {
    match expression {
        AST::LocalDefinition {identifier, value} => define_local(environment, identifier, value),
        AST::LocalMutation {identifier, value}   => mutate_local(environment, identifier, value),
        AST::Identifier(_)                                           => lookup_local(environment, expression),

        _ => Err("Not implemented".to_string())
    }
}

fn define_local<'a>(environment: &'a mut Environment, identifier: &AST, value: &AST) -> Result<Object, String> {
    let identifier_token = extract_identifier_token(identifier)?;
    let mut child_environment = Environment::new(environment);
    let value = evaluate(&mut child_environment, value)?; // FIXME clone
    environment.define_local(identifier_token, *value)?;
    Ok(Object::Unit)
}

fn mutate_local<'a>(environment: &'a mut Environment, identifier: &AST, value: &AST) -> Result<Object, String> {
    let identifier_token = extract_identifier_token(identifier)?;
    let mut child_environment = Environment::new(environment);
    let value = evaluate(&mut child_environment,value)?; // FIXME clone
    environment.redefine_local(identifier_token, *value)?;
    Ok(Object::Unit)
}

fn lookup_local<'a>(environment: &'a mut Environment, identifier: &AST) -> Result<Object, String> {
    let identifier_token = extract_identifier_token(identifier)?;
    environment.lookup_local(identifier_token)
}

