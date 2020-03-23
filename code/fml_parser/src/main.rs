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
    assert_eq!(gamma.lookup_binding("x"), Ok(environment::Object::Reference(0)));
}

#[test] fn environment_parent_test() {
    let mut gamma_parent = environment::Environment::new();
    assert!(gamma_parent.define_binding("x", environment::Object::Reference(0)).is_ok());

    let gamma_child = gamma_parent.child();
    assert_eq!(gamma_child.lookup_binding("x"), Ok(environment::Object::Reference(0)));
}

#[test] fn environment_shadowing_test() {
    let mut gamma_parent = environment::Environment::new();
    assert!(gamma_parent.define_binding("x", environment::Object::Reference(0)).is_ok());

    let mut gamma_child = gamma_parent.child();
    assert!(gamma_child.define_binding("x", environment::Object::Reference(1)).is_ok());
    assert_eq!(gamma_child.lookup_binding("x"), Ok(environment::Object::Reference(1)));
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
    assert_eq!(gamma.lookup_binding("x"), Ok(environment::Object::Reference(1)));
}

#[test] fn environment_redefine_undefined_error_test() {
    let mut gamma = environment::Environment::new();
    assert!(gamma.redefine_binding("x", environment::Object::Reference(1)).is_err());
}

#[test] fn interpreter_define_test() {
    let mut gamma = environment::Environment::new();

    let ast = ast::AST::LocalDefinition {
        identifier: Box::new(ast::AST::Identifier("x")),
        value: Box::new(ast::AST::Number(1))
    };

    assert_eq!(interpreter::evaluate(&mut gamma, ast), environment::Object::Unit)
}

fn main() {
    println!("Hello, world!");
}
