use crate::ast::AST;
use crate::environment::EnvironmentStack;
use crate::heap::{Memory, Function, Reference, Instance};

macro_rules! extract_identifier_token {
    ($ast:expr) => {
        match &**($ast) {
            AST::Identifier(token) => token.to_string(),
            ast => panic!("Expected AST::Identifier, but found {:?}", ast),
        }
    }
}

macro_rules! extract_array_offset {
    ($array_instance:expr, $index_value:expr) => {
        match $array_instance {
            Instance::Array {size, values:_} if $index_value >= 0 && ($index_value as usize) < *size =>
                $index_value as usize,
            Instance::Array {size, values} if $index_value <  0 && ($index_value.abs() as usize) < *size =>
                values.len() - 1 + $index_value.abs() as usize,
            Instance::Array {size:_, values:_} =>
                panic!("Array index out of bounds: {}", $index_value),
            _ => panic!("Attempt to index a non-array object"),
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

        AST::ArrayDefinition {size, value} => {
            let size_reference = soft_evaluate(stack, memory, &*size);
            let size_value = match size_reference {
                Reference::Integer(n) if n >= 0 => n as usize,
                Reference::Integer(n) if n <  0 => panic!("Array cannot have negative size"),
                _ => panic!("Cannot convert {:?} to integer", size_reference),
            };

            let mut elements: Vec<Reference> = Vec::new();
            stack.add_soft_frame();
            for _ in 0..size_value {
                elements.push(soft_evaluate(stack, memory, &*value))
            }
            stack.remove_frame();

            memory.put_object(Instance::array(elements))
        }

        AST::ArrayAccess {array, index} => {
            let index_reference = soft_evaluate(stack, memory, &*index);
            let index_value = match index_reference {
                Reference::Integer(n) => n,
                _ => panic!("Cannot convert {:?} to integer", index_reference),
            };

            let array_reference = soft_evaluate(stack, memory, &*array);
            let array_instance: &Instance =
                memory.get_object(&array_reference).expect("Could not find array instance");

            let reference = match array_instance {
                Instance::Array {size:_, values} =>
                    values.get(extract_array_offset!(array_instance, index_value)),
                _ => panic!("Attempt to index a non-array object")
            };

            match reference {
                Some(reference) => *reference,
                None => panic!("Could not reference array element at index {}", index_value),
            }
        }

        AST::ArrayMutation {array, value} => {
            let (array, index) = match &**array {
                AST::ArrayAccess {array, index} => (array, index),
                _ => panic!("Cannot mutate non-array object"),
            };

            let index_reference = soft_evaluate(stack, memory, &*index);
            let index_value = match index_reference {
                Reference::Integer(n) => n,
                _ => panic!("Cannot convert {:?} to integer", index_reference),
            };

            let value_reference = soft_evaluate(stack, memory, &*value);

            let array_reference = soft_evaluate(stack, memory, &*array);
            let array_instance: &Instance =
                memory.get_object(&array_reference).expect("Could not find array instance");

            let offset = extract_array_offset!(array_instance, index_value);

            let array_instance_mut: &mut Instance =
                memory.get_object_mut(&array_reference).expect("Could not find array instance");

            match array_instance_mut {
                Instance::Array {size:_, values} => {
                    values.insert(offset, value_reference)
                }
                _ => panic!("Attempt to index a non-array object")
            };

            Reference::Unit
        }

//        AST::ObjectDefinition {extends, members} => {
//
//        }

        _ => panic!("Not implemented")
    }
}



fn evaluate_to_boolean(reference: Reference) -> bool {
    match reference {
        Reference::Boolean(b) => b,
        Reference::Unit => false,
        Reference::Object(_) => true,
        Reference::Integer(n) => n == 0,
        Reference::Array {reference: _, size: _} => true,
    }
}