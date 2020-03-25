use crate::ast::AST;
use crate::environment::EnvironmentStack;
use crate::heap::{Memory, Function, Reference, FunctionReference};

macro_rules! extract_identifier_token {
    ($ast:expr) => {
        match &**($ast) {
            AST::Identifier(token) => token.to_string(),
            ast => panic!("Expected AST::Identifier, but found {:?}", ast),
        }
    }
}

//macro_rules! evaluate_in_new_frame {
//    ($stack:expr, $memory:expr, $expression:expr) => {
//        $stack.add_new_level();
//        let object = evaluate($stack, $memory, $expression);
//        $stack.remove_newest_level();
//        object
//    }
//}

pub fn evaluate<'forever> (stack: &'forever mut EnvironmentStack, memory: &'forever mut Memory<'forever>, expression: &'forever AST) -> Reference {
    match expression {

        AST::LocalDefinition {identifier, value} => {
            stack.add_new_level();
            let value = evaluate(stack, memory, &*value);
            stack.remove_newest_level();

            let name = extract_identifier_token!(identifier);
            stack.register_binding(name, value);

            Reference::Unit
        },

        AST::LocalMutation {identifier, value} => {
            stack.add_new_level();
            let value = evaluate(stack, memory, &*value);
            stack.remove_newest_level();

            let name = extract_identifier_token!(identifier);
            stack.change_binding(name, value).unwrap();

            Reference::Unit
        }

        AST::Identifier(token) => {
            let result = stack.lookup_binding(token);
            match result {
                Ok(object) => *object,
                Err(e) => panic!("Cannot resolve identifier: {}", e),
            }
        },

        AST::Number(n) => Reference::Integer(*n),
        //AST::String(s) => Object::String(s),
        AST::Boolean(b) => Reference::Boolean(*b),
        AST::Unit => Reference::Unit,

        AST::Block(expressions) => {
            let mut object = Reference::Unit;
            for expression in expressions {
                object = evaluate(stack, memory, &*expression)
            }
            object
        },

        AST::Conditional { condition, consequent, alternative} => {
            stack.add_new_level();
            let object = evaluate(stack, memory, &*condition);
            assert!(stack.remove_newest_level().is_ok());

            let next_expression =
                if evaluate_to_boolean(object) { &*consequent } else { &*alternative };

            stack.add_new_level();
            let result = evaluate(stack, memory, next_expression);
            assert!(stack.remove_newest_level().is_ok());

            result
        },

        AST::Loop { condition, body } => {
            stack.add_new_level();
            let object = evaluate(stack, memory,&*condition);
            assert!(stack.remove_newest_level().is_ok());

            while evaluate_to_boolean(object) {
                stack.add_new_level();
                evaluate(stack, memory, &*body);
                assert!(stack.remove_newest_level().is_ok());
            }

            Reference::Unit
        }

        AST::FunctionDefinition { name, body, parameters } => {
            let name = extract_identifier_token!(name);
            let params: Vec<String> = parameters
                .iter()
                .map (|parameter: &Box<AST>| extract_identifier_token!(parameter) )
                .collect();

            // TODO put body on a heap
            // get reference to body from heap
            let body_reference = 0;
            let function = Function::new(name.to_string(),params, &**body);
            let function_reference = memory.put_function(function);

            stack.register_function(name, function_reference);

            Reference::Unit
        }

        _ => panic!("Not implemented")
    }
}

fn evaluate_to_boolean(object: Reference) -> bool {
    match object {
        Reference::Boolean(b) => b,
        Reference::Unit => false,
        Reference::Object(_) => true,
        Reference::Integer(n) => n == 0,
    }
}