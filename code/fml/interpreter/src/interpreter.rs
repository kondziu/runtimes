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

pub fn evaluate (stack: &mut EnvironmentStack, memory: &mut Memory, expression: &AST) -> Reference {
    // 1. 'forever is at least as long as 'here
    // 2. Memory cannot exceed 'forever
    // 3. memory must live at least as long as 'here
    // 4. stack must live at least as long as 'here
    // 5. expression must live at least as long as 'forever
//pub fn evaluate(stack: &mut EnvironmentStack, memory: &mut Memory, expression: &AST) -> Reference {
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

            let function = Function::new(name.to_string(),params, body.clone());
            let function_reference = memory.put_function(function);

            stack.register_function(name, function_reference);

            Reference::Unit
        }

        AST::FunctionApplication {function, arguments} => {
            let name = extract_identifier_token!(function);
            let function_reference = stack.lookup_function(&name).unwrap();         // TODO error handling
            let mut function: Function = (*memory.get_function(function_reference).unwrap()).clone();               // TODO error handling

            let bindings = {
                let mut bindings: Vec<(String, Reference)> = Vec::new();
                for (parameter, expression) in function.parameters.iter().zip(arguments.iter()) {
                    stack.add_new_level();
                    let object = evaluate(stack, memory, &(**expression).clone());
                    assert!(stack.remove_newest_level().is_ok());
                    bindings.push((parameter.to_string(), object))
                }
                bindings
            };

            stack.add_new_level();
            bindings.into_iter().for_each(|binding| {
               let (name, object) = binding;
               stack.register_binding(name, object);
            });
            let object = evaluate(stack, memory, &*function.body);
            stack.remove_newest_level();
            object
        }

            //let function = &Function::new("f".to_string(), vec!("x".to_string()), &AST::Unit);

//            let bindings: Vec<(String, Reference)> =
//                function.parameters.iter()
//                    .zip(arguments.iter())
//                    .map(|e| {
//                        let parameter = e.0.to_string();
//                        let expression = &**(e.1);
//                        stack.add_new_level();
//                        let object = evaluate(stack, memory, expression);
//                        assert!(stack.remove_newest_level().is_ok());
//                        (parameter, object)
//                    }).collect();
//
//            stack.add_new_level();
//            bindings.into_iter().for_each(|binding| {
//                let (name, object) = binding;
//               stack.register_binding(name, object);
//            });
//            assert!(stack.remove_newest_level().is_ok());

            //let m: &'forever mut Memory<'forever> = memory;


            //Reference::Unit


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