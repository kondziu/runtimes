#[macro_use]
pub mod ast;
pub mod environment;
pub mod heap;
pub mod interpreter;

extern crate serde;
extern crate serde_lexpr;
extern crate serde_json;
extern crate serde_yaml;


#[test] fn memory_instance_test() {
    let mut memory = heap::Memory::new();
    let object = heap::Instance::empty();
    let reference = memory.put_object(object);
    assert!(memory.get_object(&reference).is_some())
}

#[test] fn memory_function_test() {
    let mut memory = heap::Memory::new();
    let function_body = ast::AST::Identifier("x".to_string());
    let object = heap::Function::new(
        "f".to_string(),
        vec!("x".to_string()),
        &function_body);
    let reference = memory.put_function(object);
    assert!(memory.get_function(reference).is_some())
}

#[test] fn environment_basic_test() {
    let mut gamma = environment::EnvironmentStack::new();
    assert!(gamma.register_binding("x".to_string(),
                                   heap::Reference::Object(0)).is_ok());
    assert_eq!(gamma.lookup_binding("x"),
               Ok(&heap::Reference::Object(0)));
}

#[test] fn environment_basic_function_test() {
    let mut gamma = environment::EnvironmentStack::new();
    assert!(gamma.register_function("f".to_string(),
                                   heap::FunctionReference::Function(0)).is_ok());
    assert_eq!(gamma.lookup_function("f"),
               Ok(&heap::FunctionReference::Function(0)));
}

#[test] fn environment_parent_test() {
    let mut gamma = environment::EnvironmentStack::new();
    assert!(gamma.register_binding("x".to_string(),
                                   heap::Reference::Object(0)).is_ok());

    gamma.add_new_level();
    assert_eq!(gamma.lookup_binding("x"),
               Ok(&heap::Reference::Object(0)));
}

#[test] fn environment_parent_function_test() {
    let mut gamma = environment::EnvironmentStack::new();
    assert!(gamma.register_function("f".to_string(),
                                    heap::FunctionReference::Function(0)).is_ok());

    gamma.add_new_level();
    assert_eq!(gamma.lookup_function("f"),
               Ok(&heap::FunctionReference::Function(0)));
}

#[test] fn environment_shadowing_test() {
    let mut gamma = environment::EnvironmentStack::new();
    assert!(gamma.register_binding("x".to_string(),
                                   heap::Reference::Object(0)).is_ok());

    gamma.add_new_level();
    assert!(gamma.register_binding("x".to_string(),
                                   heap::Reference::Object(1)).is_ok());
    assert_eq!(gamma.lookup_binding("x"),
               Ok(&heap::Reference::Object(1)));

    gamma.remove_newest_level();
    assert_eq!(gamma.lookup_binding("x"),
               Ok(&heap::Reference::Object(0)));
}

#[test] fn environment_shadowing_function_test() {
    let mut gamma = environment::EnvironmentStack::new();
    assert!(gamma.register_function("f".to_string(),
                                    heap::FunctionReference::Function(0)).is_ok());

    gamma.add_new_level();
    assert!(gamma.register_function("f".to_string(),
                                    heap::FunctionReference::Function(1)).is_ok());
    assert_eq!(gamma.lookup_function("f"),
               Ok(&heap::FunctionReference::Function(1)));

    gamma.remove_newest_level();
    assert_eq!(gamma.lookup_function("f"),
               Ok(&heap::FunctionReference::Function(0)));
}

#[test] fn environment_undefined_lookup_test() {
    let gamma = environment::EnvironmentStack::new();
    assert!(gamma.lookup_binding("x").is_err());
}

#[test] fn environment_undefined_function_test() {
    let gamma = environment::EnvironmentStack::new();
    assert!(gamma.lookup_binding("f").is_err());
}

#[test] fn environment_define_redefine_error_test() {
    let mut gamma = environment::EnvironmentStack::new();
    assert!(gamma.register_binding("x".to_string(),
                                   heap::Reference::Object(0)).is_ok());
    assert!(gamma.register_binding("x".to_string(),
                                   heap::Reference::Object(1)).is_err());
}

#[test] fn environment_function_redefine_error_test() {
    let mut gamma = environment::EnvironmentStack::new();
    assert!(gamma.register_function("f".to_string(),
                                    heap::FunctionReference::Function(0)).is_ok());
    assert!(gamma.register_function("f".to_string(),
                                    heap::FunctionReference::Function(1)).is_err());
}

#[test] fn environment_redefine_test() {
    let mut gamma = environment::EnvironmentStack::new();
    assert!(gamma.register_binding("x".to_string(),
                                   heap::Reference::Object(0)).is_ok());
    assert!(gamma.change_binding("x".to_string(),
                                 heap::Reference::Object(1)).is_ok());
    assert_eq!(gamma.lookup_binding("x"),
               Ok(&heap::Reference::Object(1)));
}

#[test] fn environment_redefine_undefined_error_test() {
    let mut gamma = environment::EnvironmentStack::new();
    assert!(gamma.change_binding("x".to_string(),
                                 heap::Reference::Object(1)).is_err());
}

// let x <- 1
#[test] fn interpreter_define_local_test() {
    let mut memory = heap::Memory::new();
    let mut gamma = environment::EnvironmentStack::new();

    let ast = ast::AST::LocalDefinition {
        identifier: Box::new(ast::AST::Identifier("x".to_string())),
        value: Box::new(ast::AST::Number(1))
    };

    assert_eq!(interpreter::evaluate(&mut gamma, &mut memory,&ast),
               heap::Reference::Unit);
    assert_eq!(gamma.lookup_binding("x"),
               Ok(&heap::Reference::Integer(1)))
}

// x = 1
#[test] fn interpreter_redefine_local_test() {
    let mut memory = heap::Memory::new();
    let mut gamma = environment::EnvironmentStack::new();
    assert!(gamma.register_binding("x".to_string(),
                                   heap::Reference::Integer(0)).is_ok());

    let ast = ast::AST::LocalMutation {
        identifier: Box::new(ast::AST::Identifier("x".to_string())),
        value: Box::new(ast::AST::Number(1))
    };

    assert_eq!(interpreter::evaluate(&mut gamma, &mut memory, &ast),
               heap::Reference::Unit);
    assert_eq!(gamma.lookup_binding("x"),
               Ok(&heap::Reference::Integer(1)))
}

// x
#[test] fn interpreter_identifier_local_lookup_test() {
    let mut memory = heap::Memory::new();
    let mut gamma = environment::EnvironmentStack::new();
    assert!(gamma.register_binding("x".to_string(),
                                   heap::Reference::Integer(1)).is_ok());

    let ast = ast::AST::Identifier("x".to_string());

    assert_eq!(interpreter::evaluate(&mut gamma, &mut memory, &ast),
               heap::Reference::Integer(1))
}

// 42
#[test] fn interpreter_number_test() {
    let mut memory = heap::Memory::new();
    let ast = ast::AST::Number(42);
    assert_eq!(interpreter::evaluate(&mut environment::EnvironmentStack::new(),
                                     &mut memory,
                                     &ast),
               heap::Reference::Integer(42))
}

// null
#[test] fn interpreter_unit_test() {
    let mut memory = heap::Memory::new();
    let ast = ast::AST::Unit;
    assert_eq!(interpreter::evaluate(&mut environment::EnvironmentStack::new(),
                                     &mut memory,
                                     &ast),
               heap::Reference::Unit)
}

// true
#[test] fn interpreter_boolean_test() {
    let mut memory = heap::Memory::new();
    let ast = ast::AST::Boolean(true);
    assert_eq!(interpreter::evaluate(&mut environment::EnvironmentStack::new(),
                                     &mut memory,
                                     &ast),
               heap::Reference::Boolean(true))
}

// begin 1; 2; 3; end
#[test] fn interpreter_block_test() {
    let mut memory = heap::Memory::new();
    let ast = ast::AST::Block(vec!(
        Box::new(ast::AST::Number(1)),
        Box::new(ast::AST::Number(2)),
        Box::new(ast::AST::Number(3)),
    ));
    assert_eq!(interpreter::evaluate(&mut environment::EnvironmentStack::new(),
                                     &mut memory,
                                     &ast),
               heap::Reference::Integer(3))
}

// if true then 1 else 2
#[test] fn interpreter_conditional_consequent() {
    let mut memory = heap::Memory::new();
    let ast = ast::AST::Conditional {
        condition: Box::new(ast::AST::Boolean(true)),
        consequent: Box::new(ast::AST::Number(1)),
        alternative: Box::new(ast::AST::Number(2)),
    };
    assert_eq!(interpreter::evaluate(&mut environment::EnvironmentStack::new(),
                                     &mut memory,
                                     &ast),
               heap::Reference::Integer(1))
}

// if false then 1 else 2
#[test] fn interpreter_conditional_alternative() {
    let mut memory = heap::Memory::new();
    let ast = ast::AST::Conditional {
        condition: Box::new(ast::AST::Unit),
        consequent: Box::new(ast::AST::Number(1)),
        alternative: Box::new(ast::AST::Number(2)),
    };
    assert_eq!(interpreter::evaluate(&mut environment::EnvironmentStack::new(),
                                     &mut memory,
                                     &ast),
               heap::Reference::Integer(2))
}

// function f(x) x
#[test] fn interpreter_function_definition() {
    let mut memory = heap::Memory::new();
    let mut stack = environment::EnvironmentStack::new();
    let expression = ast::AST::FunctionDefinition {
        name: Box::new(ast::AST::Identifier("f".to_string())),
        parameters: vec!(Box::new(ast::AST::Identifier("x".to_string()))),
        body: Box::new(ast::AST::Identifier("x".to_string())),
    };
    assert_eq!(interpreter::evaluate(&mut stack, &mut memory, &expression),
               heap::Reference::Unit);

    let reference = *stack.lookup_function("f").unwrap();
    assert!(memory.contains_function(reference));
    let function = memory.get_function(reference).unwrap();

    let name = "f".to_string();
    let parameters = vec!("x".to_string());
    let body = ast::AST::Identifier("x".to_string());
    let expected = heap::Function::new(name, parameters, &body);
    assert_eq!(function, &expected);
}

fn main() {
    println!("Hello, world!");
}
