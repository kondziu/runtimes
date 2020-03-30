pub mod environment;
pub mod heap;
pub mod interpreter;

#[macro_use]
extern crate fml_ast;
extern crate fml_parser;

#[cfg(test)]
mod memory_tests {
    use crate::heap::Memory;
    use crate::heap::Instance;
    use crate::heap::Function;

    #[test] fn instance () {
        let mut memory = Memory::new();
        let object = Instance::empty();
        let reference = memory.put_object(object);
        assert!(memory.get_object(&reference).is_some())
    }

    #[test] fn function () {
        let mut memory = Memory::new();
        let function_body = fml_ast::AST::Identifier("x".to_string());
        let object = Function::new(
            "f".to_string(),
            vec!("x".to_string()),
            Box::new(function_body));
        let reference = memory.put_function(object);
        assert!(memory.get_function(&reference).is_some())
    }
}

#[cfg(test)]
mod environment_tests {
    use crate::environment::EnvironmentStack;
    use crate::heap::Reference;
    use crate::heap::FunctionReference;

    #[test]
    fn basic () {
        let mut gamma = EnvironmentStack::new();
        assert!(gamma.register_binding("x".to_string(),
                                       Reference::Object(0)).is_ok());
        assert_eq!(gamma.lookup_binding("x"),
                   Ok(&Reference::Object(0)));
    }

    #[test]
    fn basic_function () {
        let mut gamma = EnvironmentStack::new();
        assert!(gamma.register_function("f".to_string(),
                                        FunctionReference::Function(0)).is_ok());
        assert_eq!(gamma.lookup_function("f"),
                   Ok(&FunctionReference::Function(0)));
    }

    #[test]
    fn soft_frame () {
        let mut gamma = EnvironmentStack::new();
        assert!(gamma.register_binding("x".to_string(),
                                       Reference::Object(0)).is_ok());

        gamma.add_soft_frame();
        assert!(gamma.register_binding("x".to_string(),
                                       Reference::Object(1)).is_ok());

        gamma.add_soft_frame();
        assert_eq!(gamma.lookup_binding("x"),
                   Ok(&Reference::Object(1)));
    }

    #[test]
    fn hard_frame () {
        let mut gamma = EnvironmentStack::new();
        assert!(gamma.register_binding("x".to_string(),
                                       Reference::Object(0)).is_ok());

        gamma.add_soft_frame();
        assert!(gamma.register_binding("x".to_string(),
                                       Reference::Object(1)).is_ok());

        gamma.add_hard_frame();
        assert_eq!(gamma.lookup_binding("x"),
                   Ok(&Reference::Object(0)));
    }

    #[test]
    fn hard_frame_exclusion () {
        let mut gamma = EnvironmentStack::new();

        gamma.add_soft_frame();
        assert!(gamma.register_binding("x".to_string(),
                                       Reference::Object(0)).is_ok());

        gamma.add_hard_frame();
        assert!(gamma.lookup_binding("x").is_err());
    }

    #[test]
    fn soft_frame_function () {
        let mut gamma = EnvironmentStack::new();
        assert!(gamma.register_function("f".to_string(),
                                        FunctionReference::Function(0)).is_ok());

        gamma.add_soft_frame();
        assert!(gamma.register_function("f".to_string(),
                                        FunctionReference::Function(1)).is_ok());

        gamma.add_soft_frame();
        assert_eq!(gamma.lookup_function("f"),
                   Ok(&FunctionReference::Function(1)));
    }

    #[test]
    fn hard_frame_function () {
        let mut gamma = EnvironmentStack::new();
        assert!(gamma.register_function("f".to_string(),
                                        FunctionReference::Function(0)).is_ok());

        gamma.add_soft_frame();
        assert!(gamma.register_function("f".to_string(),
                                        FunctionReference::Function(1)).is_ok());

        gamma.add_hard_frame();
        assert_eq!(gamma.lookup_function("f"),
                   Ok(&FunctionReference::Function(0)));
    }

    #[test]
    fn hard_frame_function_exclusion () {
        let mut gamma = EnvironmentStack::new();

        gamma.add_soft_frame();
        assert!(gamma.register_function("f".to_string(),
                                        FunctionReference::Function(0)).is_ok());

        gamma.add_hard_frame();
        assert!(gamma.lookup_function("f").is_err());
    }

    #[test]
    fn shadowing () {
        let mut gamma = EnvironmentStack::new();
        assert!(gamma.register_binding("x".to_string(),
                                       Reference::Object(0)).is_ok());

        gamma.add_soft_frame();
        assert!(gamma.register_binding("x".to_string(),
                                       Reference::Object(1)).is_ok());
        assert_eq!(gamma.lookup_binding("x"),
                   Ok(&Reference::Object(1)));

        gamma.remove_frame();
        assert_eq!(gamma.lookup_binding("x"),
                   Ok(&Reference::Object(0)));
    }

    #[test]
    fn shadowing_function () {
        let mut gamma = EnvironmentStack::new();
        assert!(gamma.register_function("f".to_string(),
                                        FunctionReference::Function(0)).is_ok());

        gamma.add_soft_frame();
        assert!(gamma.register_function("f".to_string(),
                                        FunctionReference::Function(1)).is_ok());
        assert_eq!(gamma.lookup_function("f"),
                   Ok(&FunctionReference::Function(1)));

        gamma.remove_frame();
        assert_eq!(gamma.lookup_function("f"),
                   Ok(&FunctionReference::Function(0)));
    }

    #[test]
    fn undefined_lookup () {
        let gamma = EnvironmentStack::new();
        assert!(gamma.lookup_binding("x").is_err());
    }

    #[test]
    fn undefined_function () {
        let gamma = EnvironmentStack::new();
        assert!(gamma.lookup_binding("f").is_err());
    }

    #[test]
    fn define_redefine_error () {
        let mut gamma = EnvironmentStack::new();
        assert!(gamma.register_binding("x".to_string(),
                                       Reference::Object(0)).is_ok());
        assert!(gamma.register_binding("x".to_string(),
                                       Reference::Object(1)).is_err());
    }

    #[test]
    fn function_redefine_error () {
        let mut gamma = EnvironmentStack::new();
        assert!(gamma.register_function("f".to_string(),
                                        FunctionReference::Function(0)).is_ok());
        assert!(gamma.register_function("f".to_string(),
                                        FunctionReference::Function(1)).is_err());
    }

    #[test]
    fn redefine () {
        let mut gamma = EnvironmentStack::new();
        assert!(gamma.register_binding("x".to_string(),
                                       Reference::Object(0)).is_ok());
        assert!(gamma.change_binding("x".to_string(),
                                     Reference::Object(1)).is_ok());
        assert_eq!(gamma.lookup_binding("x"),
                   Ok(&Reference::Object(1)));
    }

    #[test]
    fn redefine_undefined_error () {
        let mut gamma = EnvironmentStack::new();
        assert!(gamma.change_binding("x".to_string(),
                                     Reference::Object(1)).is_err());
    }
}

#[cfg(test)]
mod interpreter_tests {
    use crate::heap::Memory;
    use crate::heap::Instance;
    use crate::heap::Function;
    use crate::heap::Reference;
    use crate::heap::FunctionReference;
    use crate::environment::EnvironmentStack;
    use crate::interpreter::evaluate;

    // let x <- 1
    #[test]
    fn define_local () {
        let mut memory = Memory::new();
        let mut gamma = EnvironmentStack::new();

        let ast = fml_ast::AST::LocalDefinition {
            identifier: Box::new(fml_ast::AST::Identifier("x".to_string())),
            value: Box::new(fml_ast::AST::Number(1))
        };

        assert_eq!(evaluate(&mut gamma, &mut memory, &ast), Reference::Unit);
        assert_eq!(gamma.lookup_binding("x"), Ok(&Reference::Integer(1)))
    }

    // x = 1
    #[test]
    fn redefine_local() {
        let mut memory = Memory::new();
        let mut gamma = EnvironmentStack::new();
        assert!(gamma.register_binding("x".to_string(), Reference::Integer(0)).is_ok());

        let ast = fml_ast::AST::LocalMutation {
            identifier: Box::new(fml_ast::AST::Identifier("x".to_string())),
            value: Box::new(fml_ast::AST::Number(1))
        };

        assert_eq!(evaluate(&mut gamma, &mut memory, &ast), Reference::Unit);
        assert_eq!(gamma.lookup_binding("x"), Ok(&Reference::Integer(1)))
    }

    // x
    #[test]
    fn identifier_local_lookup() {
        let mut memory = Memory::new();
        let mut gamma = EnvironmentStack::new();
        assert!(gamma.register_binding("x".to_string(),
                                       Reference::Integer(1)).is_ok());

        let ast = fml_ast::AST::Identifier("x".to_string());

        assert_eq!(evaluate(&mut gamma, &mut memory, &ast), Reference::Integer(1))
    }

    // 42
    #[test]
    fn number() {
        let mut memory = Memory::new();
        let mut gamma = EnvironmentStack::new();

        let ast = fml_ast::AST::Number(42);

        assert_eq!(evaluate(&mut gamma, &mut memory, &ast), Reference::Integer(42))
    }

    // null
    #[test]
    fn unit() {
        let mut memory = Memory::new();
        let mut gamma = EnvironmentStack::new();

        let ast = fml_ast::AST::Unit;

        assert_eq!(evaluate(&mut gamma, &mut memory, &ast), Reference::Unit)
    }

    // true
    #[test]
    fn boolean() {
        let mut memory = Memory::new();
        let mut gamma = EnvironmentStack::new();

        let ast = fml_ast::AST::Boolean(true);

        assert_eq!(evaluate(&mut gamma, &mut memory, &ast),
                   Reference::Boolean(true))
    }

    // begin 1; 2; 3; end
    #[test]
    fn block() {
        let mut memory = Memory::new();
        let mut gamma = EnvironmentStack::new();

        let ast = fml_ast::AST::Block(vec!(
            Box::new(fml_ast::AST::Number(1)),
            Box::new(fml_ast::AST::Number(2)),
            Box::new(fml_ast::AST::Number(3)),
        ));

        assert_eq!(evaluate(&mut gamma, &mut memory, &ast), Reference::Integer(3))
    }

    // if true then 1 else 2
    #[test]
    fn conditional_consequent() {
        let mut memory = Memory::new();
        let mut gamma = EnvironmentStack::new();

        let ast = fml_ast::AST::Conditional {
            condition: Box::new(fml_ast::AST::Boolean(true)),
            consequent: Box::new(fml_ast::AST::Number(1)),
            alternative: Box::new(fml_ast::AST::Number(2)),
        };

        assert_eq!(evaluate(&mut gamma, &mut memory, &ast), Reference::Integer(1))
    }

    // if false then 1 else 2
    #[test]
    fn conditional_alternative() {
        let mut memory = Memory::new();
        let mut gamma = EnvironmentStack::new();

        let ast = fml_ast::AST::Conditional {
            condition: Box::new(fml_ast::AST::Unit),
            consequent: Box::new(fml_ast::AST::Number(1)),
            alternative: Box::new(fml_ast::AST::Number(2)),
        };

        assert_eq!(evaluate(&mut gamma, &mut memory, &ast), Reference::Integer(2))
    }

    // function f(x) x
    #[test]
    fn function_definition() {
        let mut memory = Memory::new();
        let mut gamma = EnvironmentStack::new();

        let ast = fml_ast::AST::FunctionDefinition {
            name: Box::new(fml_ast::AST::Identifier("f".to_string())),
            parameters: vec!(Box::new(fml_ast::AST::Identifier("x".to_string()))),
            body: Box::new(fml_ast::AST::Identifier("x".to_string())),
        };

        assert_eq!(evaluate(&mut gamma, &mut memory, &ast), Reference::Unit);

        let reference = gamma.lookup_function("f").unwrap();
        assert!(memory.contains_function(reference));
        let function = memory.get_function(reference).unwrap();

        let name = "f".to_string();
        let parameters = vec!("x".to_string());
        let body = fml_ast::AST::Identifier("x".to_string());
        let expected = Function::new(name, parameters, Box::new(body));
        assert_eq!(function, &expected);
    }

    // function f(x) x; f(1)
    #[test]
    fn function_call() {
        let mut memory = Memory::new();
        let mut gamma = EnvironmentStack::new();
        let expression =
            fml_ast::AST::Block(vec!(
                Box::new(fml_ast::AST::FunctionDefinition {
                    name: Box::new(fml_ast::AST::Identifier("f".to_string())),
                    parameters: vec!(Box::new(fml_ast::AST::Identifier("x".to_string()))),
                    body: Box::new(fml_ast::AST::Identifier("x".to_string()))
                }),
                Box::new(fml_ast::AST::FunctionApplication {
                    function: Box::new(fml_ast::AST::Identifier("f".to_string())),
                    arguments: vec!(Box::new(fml_ast::AST::Number(1))),
                }),
            ));
        assert_eq!(evaluate(&mut gamma, &mut memory, &expression),
                   Reference::Integer(1));
    }
}


fn main() {
    println!("Hello, world!");
}
