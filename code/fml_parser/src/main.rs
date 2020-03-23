#[macro_use]
pub mod ast;
pub mod environment;
pub mod interpreter;

extern crate serde;
extern crate serde_lexpr;
extern crate serde_json;
extern crate serde_yaml;

#[test] fn environment_basic_test() {
    let mut gamma = environment::Environment::new();
    assert!(gamma.define_binding("x", environment::Object::Reference(0)).is_ok());
    assert_eq!(gamma.lookup_binding("x"),
               Ok(environment::Object::Reference(0)));
}

#[test] fn environment_parent_test() {
    let mut gamma_parent = environment::Environment::new();
    assert!(gamma_parent.define_binding("x", environment::Object::Reference(0)).is_ok());

    let gamma_child = gamma_parent.child();
    assert_eq!(gamma_child.lookup_binding("x"),
               Ok(environment::Object::Reference(0)));
}

#[test] fn environment_shadowing_test() {
    let mut gamma_parent = environment::Environment::new();
    assert!(gamma_parent.define_binding("x", environment::Object::Reference(0)).is_ok());

    let mut gamma_child = gamma_parent.child();
    assert!(gamma_child.define_binding("x", environment::Object::Reference(1)).is_ok());
    assert_eq!(gamma_child.lookup_binding("x"),
               Ok(environment::Object::Reference(1)));
}

#[test] fn environment_undefined_lookup_test() {
    let gamma = environment::Environment::new();
    assert!(gamma.lookup_binding("x").is_err());
}

#[test] fn environment_define_redefine_error_test() {
    let mut gamma = environment::Environment::new();
    assert!(gamma.define_binding("x", environment::Object::Reference(0)).is_ok());
    assert!(gamma.define_binding("x", environment::Object::Reference(1)).is_err());
}

#[test] fn environment_redefine_test() {
    let mut gamma = environment::Environment::new();
    assert!(gamma.define_binding("x", environment::Object::Reference(0)).is_ok());
    assert!(gamma.redefine_binding("x", environment::Object::Reference(1)).is_ok());
    assert_eq!(gamma.lookup_binding("x"),
               Ok(environment::Object::Reference(1)));
}

#[test] fn environment_redefine_undefined_error_test() {
    let mut gamma = environment::Environment::new();
    assert!(gamma.redefine_binding("x", environment::Object::Reference(1)).is_err());
}

// let x <- 1
#[test] fn interpreter_define_local_test() {
    let mut gamma = environment::Environment::new();

    let ast = ast::AST::LocalDefinition {
        identifier: Box::new(ast::AST::Identifier("x")),
        value: Box::new(ast::AST::Number(1))
    };

    assert_eq!(interpreter::evaluate(&mut gamma, &ast),
               environment::Object::Unit);
    assert_eq!(gamma.lookup_binding("x"),
               Ok(environment::Object::Integer(1)))
}

// x = 1
#[test] fn interpreter_redefine_local_test() {
    let mut gamma = environment::Environment::new();
    assert!(gamma.define_binding("x", environment::Object::Integer(0)).is_ok());

    let ast = ast::AST::LocalMutation {
        identifier: Box::new(ast::AST::Identifier("x")),
        value: Box::new(ast::AST::Number(1))
    };

    assert_eq!(interpreter::evaluate(&mut gamma, &ast),
               environment::Object::Unit);
    assert_eq!(gamma.lookup_binding("x"),
               Ok(environment::Object::Integer(1)))
}

// x
#[test] fn interpreter_identifier_local_lookup_test() {
    let mut gamma = environment::Environment::new();
    assert!(gamma.define_binding("x", environment::Object::Integer(1)).is_ok());

    let ast = ast::AST::Identifier("x");

    assert_eq!(interpreter::evaluate(&mut gamma, &ast),
               environment::Object::Integer(1))
}

// 42
#[test] fn interpreter_number_test() {
    let ast = ast::AST::Number(42);
    assert_eq!(interpreter::evaluate(&mut environment::Environment::new(), &ast),
               environment::Object::Integer(42))
}

// null
#[test] fn interpreter_unit_test() {
    let ast = ast::AST::Unit;
    assert_eq!(interpreter::evaluate(&mut environment::Environment::new(), &ast),
               environment::Object::Unit)
}

// true
#[test] fn interpreter_boolean_test() {
    let ast = ast::AST::Boolean(true);
    assert_eq!(interpreter::evaluate(&mut environment::Environment::new(), &ast),
               environment::Object::Boolean(true))
}

// begin 1; 2; 3; end
#[test] fn interpreter_block_test() {
    let ast = ast::AST::Block(vec!(
        Box::new(ast::AST::Number(1)),
        Box::new(ast::AST::Number(2)),
        Box::new(ast::AST::Number(3)),
    ));
    assert_eq!(interpreter::evaluate(&mut environment::Environment::new(), &ast),
               environment::Object::Integer(3))
}

#[test] fn interpreter_conditional_consequent() {
    let ast = ast::AST::Conditional {
        condition: Box::new(ast::AST::Boolean(true)),
        consequent: Box::new(ast::AST::Number(1)),
        alternative: Box::new(ast::AST::Number(2)),
    };
    assert_eq!(interpreter::evaluate(&mut environment::Environment::new(), &ast),
               environment::Object::Integer(1))
}

#[test] fn interpreter_conditional_alternative() {
    let ast = ast::AST::Conditional {
        condition: Box::new(ast::AST::Unit),
        consequent: Box::new(ast::AST::Number(1)),
        alternative: Box::new(ast::AST::Number(2)),
    };
    assert_eq!(interpreter::evaluate(&mut environment::Environment::new(), &ast),
               environment::Object::Integer(2))
}

fn main() {
    println!("Hello, world!");
}
