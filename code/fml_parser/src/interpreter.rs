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

pub fn evaluate<'a>(environment: &'a mut Environment, expression: &'a AST) -> Object {
    match expression {

        AST::LocalDefinition {identifier, value} => {
            let value = evaluate(&mut environment.child(), &*value);
            let token = extract_identifier_token!(&**identifier);
            environment.define_binding(token, value).unwrap();
            Object::Unit
        },

        AST::LocalMutation {identifier, value} => {
            let value = evaluate(&mut environment.child(), &*value);
            let token = extract_identifier_token!(&**identifier);
            environment.redefine_binding(token, value).unwrap(); //FIXME
            Object::Unit
        }

        AST::Identifier(token) => {
            let result = environment.lookup_binding(token);
            match result {
                Ok(object) => object,
                Err(e) => panic!("Cannot resolve identifier: {}", e),
            }
        },

        AST::Number(n) => Object::Integer(*n),
        //AST::String(s) => Object::String(s),
        AST::Boolean(b) => Object::Boolean(*b),
        AST::Unit => Object::Unit,

        AST::Block(expressions) => {
            let mut object = Object::Unit;
            for expression in expressions {
                object = evaluate(environment, &*expression)
            }
            object
        },

        AST::Conditional { condition, consequent, alternative} => {
            let value = evaluate(&mut environment.child(),&*condition);
            let next_expression = if evaluate_to_boolean(value) { &*consequent }
                                       else { &*alternative };
            evaluate(&mut environment.child(), next_expression)
        },

        AST::Loop { condition, body } => {
            while evaluate_to_boolean(evaluate(&mut environment.child(),
                                                     &*condition)) {
                evaluate(&mut environment.child(), &*body);
            }
            Object::Unit
        }

        AST::FunctionDefinition { name, body, parameters } => {
            let name_token = extract_identifier_token!(&**name);
            fn to_token<'p> (parameter: &Box<AST<'p>>) -> &'p str {
                extract_identifier_token!(&**parameter)
            }
            let parameter_tokens: Vec<&str> = parameters
                .iter()
                .map (|parameter: &Box<AST>| to_token(parameter) )
                .collect();

            // TODO put body on a heap
            // get reference to body from heap
            let body_reference = 0;
            environment.define_function(name_token,
                                        parameter_tokens,
                                        body_reference).unwrap();
            Object::Unit
        }

        _ => panic!("Not implemented")
    }
}

fn evaluate_to_boolean(object: Object) -> bool {
    match object {
        Object::Boolean(b) => b,
        Object::Unit => false,
        Object::Reference(_) => true,
        Object::Integer(n) => n == 0,
    }
}