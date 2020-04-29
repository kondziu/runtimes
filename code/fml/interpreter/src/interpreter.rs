use fml_ast::AST;
use fml_ast::Identifier;
use crate::world::World;
use crate::environment::EnvironmentStack;
use crate::heap::{Memory, Function, Reference, Instance, FunctionReference};

use std::collections::HashMap;

//macro_rules! extract_identifier_token {
//    ($ast:expr) => {
//        match &**($ast) {
//            AST::Identifier(token) => token.to_string(),
//            ast => panic!("Expected AST::Identifier, but found {:?}", ast),
//        }
//    }
//}

macro_rules! construct_function_definition {
    ($name:expr, $parameters:ident, $body:ident) => {
            Function::new($name.to_string(),
                          $parameters.iter()
                                .map (|parameter| parameter.to_string())
                                .collect(),
                          $body.clone())
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

macro_rules! find_actual_host_object {
    ($memory:expr, $object_reference:expr, $field_name:expr) => {
        {
            let mut cursor = $object_reference;
            loop {
                let object_instance: &Instance =
                    $memory.get_object(&cursor).expect("Could not find object instance");
                match object_instance {
                    Instance::Object { extends, fields, methods:_} => {
                        if fields.contains_key(&$field_name) {
                            break;
                        }
                        if let Some(parent_reference) = extends {
                            cursor = *parent_reference;
                            continue;
                        }
                        panic!("Cannot find field {} in object {:?}", $field_name, object_instance)
                    },
                    _ => panic!("Cannot find field {} in object {:?}", $field_name, object_instance)
                }
            }
            cursor
        }
    }
}

macro_rules! find_actual_host_object_for_method {
    ($memory:expr, $object_reference:expr, $method_name:expr) => {
        {
            let mut cursor = $object_reference;
            loop {
                let object_instance: &Instance =
                    $memory.get_object(&cursor).expect("Could not find object instance");
                match object_instance {
                    Instance::Object { extends, fields:_, methods} => {
                        if methods.contains_key(&$method_name) {
                            break;
                        }
                        if let Some(parent_reference) = extends {
                            cursor = *parent_reference;
                            continue;
                        }
                        panic!("Cannot find method {} in object {:?}", $method_name, object_instance)
                    },
                    _ => panic!("Cannot find method {} in object {:?}", $method_name, object_instance)
                }
            }
            cursor
        }
    }
}

pub fn soft_evaluate (stack: &mut EnvironmentStack, memory: &mut Memory, world: &mut impl World, expression: &AST) -> Reference {
    stack.add_soft_frame();
    let value = evaluate(stack, memory, world, expression);
    stack.remove_frame();
    value
}

pub fn hard_evaluate (stack: &mut EnvironmentStack, memory: &mut Memory, world: &mut impl World, bindings: Vec<(String, Reference)>, expression: &AST) -> Reference {
    stack.add_hard_frame();
    bindings.into_iter().for_each(|binding| {
        let (name, object) = binding;
        stack.register_binding(name, object).expect("Cannot register binding for argument");
    });
    let value = evaluate(stack, memory, world, expression);
    stack.remove_frame();
    value
}

pub fn evaluate (stack: &mut EnvironmentStack, memory: &mut Memory,
                 world: &mut impl World, expression: &AST) -> Reference {

    // Rules of Acquisition
    // 1. 'forever is at least as long as 'here
    // 2. Memory cannot exceed 'forever
    // 3. memory must live at least as long as 'here
    // 4. stack must live at least as long as 'here
    // 5. expression must live at least as long as 'forever

    match expression {

        AST::LocalDefinition {local: Identifier(local), value} => {
            let reference = soft_evaluate(stack, memory, world, &*value);
            stack.register_binding(local.to_string(), reference).expect("Cannot register binding");
            Reference::Unit
        },

        AST::LocalMutation {local: Identifier(local), value} => {
            let reference = soft_evaluate(stack, memory, world, &*value);
            stack.change_binding(local.to_string(), reference).expect("Cannot modify binding");
            Reference::Unit
        }

        AST::LocalAccess {local: Identifier(local)} => {
            *stack.lookup_binding(&local).expect("Cannot resolve identifier")
        },

        AST::Number(n) => Reference::Integer(*n),
        AST::Boolean(b) => Reference::Boolean(*b),
        AST::Unit => Reference::Unit,

        AST::Block(expressions) => {
            let mut reference = Reference::Unit;
            for expression in expressions {
                reference = evaluate(stack, memory, world, &*expression)
            }
            reference
        },

        AST::Conditional { condition, consequent, alternative} => {
            let condition_reference = soft_evaluate(stack, memory, world, &*condition);

            let next_expression =
                if evaluate_to_boolean(condition_reference) {
                    consequent
                } else {
                    alternative
                };

            soft_evaluate(stack, memory, world, &*next_expression)
        },

        AST::FunctionDefinition { function: Identifier(function), body, parameters } => {
            let function_definition = construct_function_definition!(function, parameters, body);
            let function_reference = memory.put_function(function_definition);
            stack.register_function(function.to_string(), function_reference).expect("Cannot bind function");
            Reference::Unit
        }

        AST::Loop { condition, body } => {
            let condition_reference = soft_evaluate(stack, memory, world, &*condition);

            while evaluate_to_boolean(condition_reference) {
                let _ = soft_evaluate(stack, memory, world, &*body);
            }

            Reference::Unit
        }

        AST::FunctionApplication {function: Identifier(function), arguments} => {
            let function_reference = stack.lookup_function(&function)
                .expect(&format!("Function {} not found on stack", function));

            let function_definition: Function = {
                let function_definition = memory.get_function(function_reference)
                    .expect(&format!("Function {:?} not found in memory", function_reference));
                function_definition.clone()
            };

            let bindings = {
                let mut bindings: Vec<(String, Reference)> = Vec::new();
                let iterator = function_definition.parameters.iter().zip(arguments.iter());
                for (parameter, expression) in iterator {
                    let reference = soft_evaluate(stack, memory, world, &(**expression).clone());
                    bindings.push((parameter.to_string(), reference))
                }
                bindings
            };

            hard_evaluate(stack, memory, world, bindings, &*function_definition.body)
        }

        AST::ArrayDefinition {size, value} => {
            let size_reference = soft_evaluate(stack, memory, world, &*size);
            let size_value = match size_reference {
                Reference::Integer(n) if n >= 0 => n as usize,
                Reference::Integer(n) if n <  0 => panic!("Array cannot have negative size"),
                _ => panic!("Cannot convert {:?} to integer", size_reference),
            };

            let mut elements: Vec<Reference> = Vec::new();
            stack.add_soft_frame();
            for _ in 0..size_value {
                elements.push(soft_evaluate(stack, memory, world, &*value))
            }
            stack.remove_frame();

            memory.put_object(Instance::array(elements))
        }

        AST::ArrayAccess {array, index} => {
            let index_reference = soft_evaluate(stack, memory, world, &*index);
            let index_value = match index_reference {
                Reference::Integer(n) => n,
                _ => panic!("Cannot convert {:?} to integer", index_reference),
            };

            let array_reference = soft_evaluate(stack, memory, world, &*array);
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

        AST::ArrayMutation {array, index, value} => {
            let (array, index) = match &**array {
                AST::ArrayAccess {array, index} => (array, index),
                _ => panic!("Cannot mutate non-array object"),
            };

            let index_reference = soft_evaluate(stack, memory, world, &*index);
            let index_value = match index_reference {
                Reference::Integer(n) => n,
                _ => panic!("Cannot convert {:?} to integer", index_reference),
            };

            let value_reference = soft_evaluate(stack, memory, world, &*value);

            let array_reference = soft_evaluate(stack, memory, world, &*array);
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

        AST::ObjectDefinition {extends, members} => {
            let super_object_reference: Option<Reference> = match extends {
                Some(e) => Some(soft_evaluate(stack, memory, world, &*e)),
                None => None,
            };

            let mut fields: HashMap<String, Reference> = HashMap::new();
            let mut methods: HashMap<String, FunctionReference> = HashMap::new();
            for member in members.iter() {
                match &**member {
                    AST::LocalDefinition {local: Identifier(local), value} => {
                        let definition_reference = soft_evaluate(stack, memory, world, &*value);
                        fields.insert(local.to_string(), definition_reference);
                    },
                    AST::FunctionDefinition {function: Identifier(function), parameters, body} => {
                        let function_definition = construct_function_definition!(function, parameters, body);
                        let function_reference = memory.put_function(function_definition);
                        methods.insert(function.to_string(), function_reference);
                    },
                    AST::OperatorDefinition {operator, parameters, body} => {
                        let definition_identifier = operator.to_str();
                        let function_definition = construct_function_definition!(definition_identifier, parameters, body);
                        let function_reference = memory.put_function(function_definition);
                        methods.insert(definition_identifier.to_string(), function_reference);
                    },
                    _ => panic!("Only local, function, and operator definitions can be object members. Not this: {:?}", *member)
                }
            }

            let object_instance = Instance::object(super_object_reference, fields, methods);
            memory.put_object(object_instance)
        },

        AST::FieldAccess {object, field: Identifier(field)} => {
            let object_reference = soft_evaluate(stack, memory, world, &*object);
            let actual_reference = find_actual_host_object!(memory, object_reference, *field);

            let actual_instance = memory.get_object(&actual_reference).expect("Could not find object instance");

            match actual_instance {
                Instance::Object { extends:_, fields, methods:_ } => *fields.get(&**field).unwrap(),
                _ => panic!("Fatal inconsistency in instance store.")
            }
        }

        AST::FieldMutation {field_path, value} => {
            let (object, field) = match &**field_path {
                AST::FieldAccess {object, field: Identifier(field)} => (object, field),
                _ => panic!("Cannot mutate non-array object"),
            };

            let value_reference = soft_evaluate(stack, memory, world, &**value);

            let object_reference = soft_evaluate(stack, memory, world, &*object);
            let actual_reference = find_actual_host_object!(memory, object_reference, *field);

            let actual_instance = memory.get_object_mut(&actual_reference).expect("Could not find object instance");

            match actual_instance {
                Instance::Object { extends:_, fields, methods: _ } => {
                    fields.insert(field.to_string(), value_reference);
                },
                _ => panic!("Fatal inconsistency in instance store.")
            }

            Reference::Unit
        },

        AST::MethodCall {method_path, arguments} => {
            let (object, method_name) = match &**method_path {
                AST::FieldAccess {object, field: Identifier(field)} => (object, field.to_string()),
                AST::OperatorAccess {object, operator} => (object, operator.to_string()),
                _ => panic!("Cannot call method on a non-object"),
            };

            let object_reference = soft_evaluate(stack, memory, world, &*object);
            let argument_references: Vec<Reference> = arguments.iter().map(|expression| {
                soft_evaluate(stack, memory, world, &(**expression).clone())
            }).collect();

            evaluate_method_call(stack, memory, world, object_reference, method_name, argument_references)
        }

        AST::Operation {operator, left, right} => {
            use fml_ast::Operator::*;

            let left_reference = soft_evaluate(stack, memory, world, &**left);
            let right_reference = soft_evaluate(stack, memory, world, &**right);

            let result = match (left_reference, operator, right_reference) {
                (left_reference, Equality, right_reference) => Reference::Boolean(left_reference == right_reference),
                (left_reference, Inequality, right_reference) => Reference::Boolean(left_reference != right_reference),

                (Reference::Integer(left_value), Multiplication, Reference::Integer(right_value)) => Reference::Integer(left_value * right_value),
                (Reference::Integer(left_value), Division, Reference::Integer(right_value)) => Reference::Integer(left_value / right_value),
                (Reference::Integer(left_value), Module, Reference::Integer(right_value)) => Reference::Integer(left_value % right_value),
                (Reference::Integer(left_value), Addition, Reference::Integer(right_value)) => Reference::Integer(left_value + right_value),
                (Reference::Integer(left_value), Subtraction, Reference::Integer(right_value)) => Reference::Integer(left_value - right_value),
                //(Reference::Integer(left_value), Inequality, Reference::Integer(right_value)) => Reference::Boolean(left_value != right_value),
                //(Reference::Integer(left_value), Equality, Reference::Integer(right_value)) => Reference::Boolean(left_value == right_value),
                (Reference::Integer(left_value), Less, Reference::Integer(right_value)) => Reference::Boolean(left_value < right_value),
                (Reference::Integer(left_value), LessEqual, Reference::Integer(right_value)) => Reference::Boolean(left_value <= right_value),
                (Reference::Integer(left_value), Greater, Reference::Integer(right_value)) => Reference::Boolean(left_value > right_value),
                (Reference::Integer(left_value), GreaterEqual, Reference::Integer(right_value)) => Reference::Boolean(left_value >= right_value),

                //(Reference::Boolean(left_value), Inequality, Reference::Boolean(right_value)) => Reference::Boolean(left_value != right_value),
                //(Reference::Boolean(left_value), Equality, Reference::Boolean(right_value)) => Reference::Boolean(left_value == right_value),
                (Reference::Boolean(left_value), Conjunction, Reference::Boolean(right_value)) => Reference::Boolean(left_value && right_value),
                (Reference::Boolean(left_value), Disjunction, Reference::Boolean(right_value)) => Reference::Boolean(left_value || right_value),

                //(Reference::Unit, Inequality, right_reference) => Reference::Boolean(Reference::Unit != right_reference),
                //(Reference::Unit, Equality, right_reference) => Reference::Boolean(Reference::Unit == right_reference),

                (Reference::Object(_), operator, right_reference) =>
                    evaluate_method_call(stack, memory, world, left_reference, operator.to_string(), vec!(right_reference)),

                _ => panic!("Operator {} is not implemented for operands {:?} and {:?}",
                            operator.to_string(), left_reference, right_reference)
            };

            result
        },

        AST::Print {format, arguments} => {
            let format_string = match &**format {
                AST::String(string) => string,
                _ => panic!("Format string must be a string, but it is this: {:?}", format),
            };

            if arguments.is_empty() {
                world.output(format_string.to_string());
            } else {
                let mut values: Vec<String> = arguments.iter().map(|argument| {
                    evaluate_to_string(soft_evaluate(stack, memory, world, &*argument))
                }).rev().collect();

                let mut escape = false;
                let mut result = String::new();

                for character in format_string.chars() {
                    match character {
                        '~' if !escape => {
                            match values.pop() {
                                Some(value) => result.push_str(&value),
                                None => panic!("Cannot fill placeholder in {}", format_string),
                            }
                        },
                        '\\' if !escape => { escape = true;                         },
                        '\\' if  escape => { escape = false; result.push('\\')      },
                        'n'  if  escape => { escape = false; result.push('\n')      },
                        't'  if  escape => { escape = false; result.push('\t')      },
                        'r'  if  escape => { escape = false; result.push('\r')      },
                        _ =>               { escape = false; result.push(character) },
                    }
                }

                if !values.is_empty() {
                    panic!("Too many arguments for format string {}", format_string)
                }

                world.output(result);
            }

            Reference::Unit
        },

        AST::String(_) => {
            panic!("String literals cannot occur outside of print.")
        },

        AST::OperatorAccess {object:_, operator:_} => {
            panic!("Operator access is not allowed, we don't have first class functions and junk")
        },

        AST::OperatorDefinition { operator:_, parameters:_, body:_} => {
            panic!("Operators can only be defined within bodies of objects")
        },
    }
}

fn evaluate_method_call(stack: &mut EnvironmentStack, memory: &mut Memory,
                        world: &mut impl World, object_reference: Reference,
                        method_name: String, arguments: Vec<Reference>) -> Reference {

    let actual_reference = find_actual_host_object_for_method!(memory, object_reference, method_name);
    let function_reference = match memory.get_object(&actual_reference) {
        Some(Instance::Object{extends:_, methods, fields:_}) => methods.get(&method_name).unwrap(),
        Some(instance) => panic!("Invalid instance type {:?}.", instance),
        None => panic!("Fatal inconsistency in instance store."),
    };

    let function_definition: Function = {
        let function_definition = memory.get_function(function_reference)
            .expect(&format!("Function {:?} not found in memory", function_reference));
        function_definition.clone()
    };

    let bindings = {
        let mut bindings: Vec<(String, Reference)> = Vec::new();
        let iterator = function_definition.parameters.iter().zip(arguments.iter());
        for (parameter, reference) in iterator {
            bindings.push((parameter.to_string(), *reference))
        }
        bindings.push(("this".to_string(), object_reference));
        bindings
    };

    hard_evaluate(stack, memory, world, bindings, &*function_definition.body)
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

fn evaluate_to_string(reference: Reference) -> String {
    match reference {
        Reference::Boolean(b) => format!("{}", b),
        Reference::Unit => "null".to_string(),
        Reference::Object(reference) => format!("<ref:{}>", reference),
        Reference::Integer(n) => format!("{}", n),
        Reference::Array {reference, size} => format!("<ref:{}, size:{}>", reference, size),
    }
}