use crate::ast::AST;
use crate::environment::EnvironmentStack;
use crate::heap::{Memory, Function, Reference};

macro_rules! extract_identifier_token {
    ($ast:expr) => {
        match &**($ast) {
            AST::Identifier(token) => token.to_string(),
            ast => panic!("Expected AST::Identifier, but found {:?}", ast),
        }
    }
}

pub fn soft_evaluate (stack: &mut EnvironmentStack, memory: &mut Memory, expression: &AST) -> Reference {
    stack.add_soft_frame();
    let value = evaluate(stack, memory, expression);
    stack.remove_frame();
    value
}

pub fn hard_evaluate (stack: &mut EnvironmentStack, memory: &mut Memory, bindings: Vec<(String, Reference)>, expression: &AST) -> Reference {
    stack.add_hard_frame();
    bindings.into_iter().for_each(|binding| {
        let (name, object) = binding;
        stack.register_binding(name, object).expect("Cannot register binding for argument");
    });
    let value = evaluate(stack, memory, expression);
    stack.remove_frame();
    value
}

pub fn evaluate (stack: &mut EnvironmentStack, memory: &mut Memory, expression: &AST) -> Reference {
    // Rules of Acquisition
    // 1. 'forever is at least as long as 'here
    // 2. Memory cannot exceed 'forever
    // 3. memory must live at least as long as 'here
    // 4. stack must live at least as long as 'here
    // 5. expression must live at least as long as 'forever

    match expression {

        AST::LocalDefinition {identifier, value} => {
            let reference = soft_evaluate(stack, memory, &*value);
            let name = extract_identifier_token!(identifier);
            stack.register_binding(name, reference).expect("Cannot register binding");
            Reference::Unit
        },

        AST::LocalMutation {identifier, value} => {
            let reference = soft_evaluate(stack, memory, &*value);
            let name = extract_identifier_token!(identifier);
            stack.change_binding(name, reference).expect("Cannot modify binding");
            Reference::Unit
        }

        AST::Identifier(token) => {
            *stack.lookup_binding(token).expect("Cannot resolve identifier")
        },

        AST::Number(n) => Reference::Integer(*n),
        //AST::String(s) => Object::String(s),
        AST::Boolean(b) => Reference::Boolean(*b),
        AST::Unit => Reference::Unit,

        AST::Block(expressions) => {
            let mut reference = Reference::Unit;
            for expression in expressions {
                reference = evaluate(stack, memory, &*expression)
            }
            reference
        },

        AST::Conditional { condition, consequent, alternative} => {
            let condition_reference = soft_evaluate(stack, memory, &*condition);

            let next_expression =
                if evaluate_to_boolean(condition_reference) {
                    consequent
                } else {
                    alternative
                };

            soft_evaluate(stack, memory, &*next_expression)
        },

        AST::Loop { condition, body } => {
            let condition_reference = soft_evaluate(stack, memory, &*condition);

            while evaluate_to_boolean(condition_reference) {
                let _ = soft_evaluate(stack, memory, &*body);
            }

            Reference::Unit
        }

        AST::FunctionDefinition { name, body, parameters } => {
            let name = extract_identifier_token!(name);
            let params: Vec<String> = parameters.iter()
                .map (|parameter: &Box<AST>| extract_identifier_token!(parameter) ).collect();

            let function_definition = Function::new(name.to_string(),
                                                    params,
                                                    body.clone());

            let function_reference = memory.put_function(function_definition);

            stack.register_function(name, function_reference).expect("Cannot bind function");
            Reference::Unit
        }

        AST::FunctionApplication {function, arguments} => {
            let name = extract_identifier_token!(function);

            let function_reference = stack.lookup_function(&name)
                .expect(&format!("Function {} not found on stack", name));

            let function_definition: Function = {
                let function_definition = memory.get_function(function_reference)
                    .expect(&format!("Function {:?} not found in memory", function_reference));
                function_definition.clone()
            };

            let bindings = {
                let mut bindings: Vec<(String, Reference)> = Vec::new();
                let iterator = function_definition.parameters.iter().zip(arguments.iter());
                for (parameter, expression) in iterator {
                    let reference = soft_evaluate(stack, memory, &(**expression).clone());
                    bindings.push((parameter.to_string(), reference))
                }
                bindings
            };

            hard_evaluate(stack, memory, bindings, &*function_definition.body)
        }

        //AST::ObjectDefinition {}

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