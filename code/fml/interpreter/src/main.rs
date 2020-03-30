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

    macro_rules! make_function {
        ($name:expr, $body:expr, $( $parameter:expr ),*) => {
            Function::new($name.to_string(), {
                let mut parameters: Vec<String> = Vec::new();
                $(
                    parameters.push($parameter.to_string());
                )*
                parameters
            }, Box::new($body));
        }
    }

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
    use fml_parser::parse;

    // let x = 1
    #[test]
    fn define_local () {
        let mut memory = Memory::new();
        let mut gamma = EnvironmentStack::new();

        let ast = parse("let x = 1");

        assert_eq!(evaluate(&mut gamma, &mut memory, &ast), Reference::Unit);
        assert_eq!(gamma.lookup_binding("x"), Ok(&Reference::Integer(1)))
    }

    // x = 1
    #[test]
    fn redefine_local() {
        let mut memory = Memory::new();
        let mut gamma = EnvironmentStack::new();
        assert!(gamma.register_binding("x".to_string(), Reference::Integer(0)).is_ok());

        let ast = parse("x <- 1");

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

        let ast = parse("x");

        assert_eq!(evaluate(&mut gamma, &mut memory, &ast), Reference::Integer(1))
    }

    // 42
    #[test]
    fn number() {
        let mut memory = Memory::new();
        let mut gamma = EnvironmentStack::new();

        let ast = parse("42");

        assert_eq!(evaluate(&mut gamma, &mut memory, &ast), Reference::Integer(42))
    }

    // null
    #[test]
    fn unit() {
        let mut memory = Memory::new();
        let mut gamma = EnvironmentStack::new();

        let ast = parse("null");

        assert_eq!(evaluate(&mut gamma, &mut memory, &ast), Reference::Unit)
    }

    // true
    #[test]
    fn boolean() {
        let mut memory = Memory::new();
        let mut gamma = EnvironmentStack::new();

        let ast = parse("true");

        assert_eq!(evaluate(&mut gamma, &mut memory, &ast),
                   Reference::Boolean(true))
    }

    // begin 1; 2; 3; end
    #[test]
    fn block() {
        let mut memory = Memory::new();
        let mut gamma = EnvironmentStack::new();

        let ast = parse("begin 1; 2; 3; end");

        assert_eq!(evaluate(&mut gamma, &mut memory, &ast), Reference::Integer(3))
    }

    // if true then 1 else 2
    #[test]
    fn conditional_consequent() {
        let mut memory = Memory::new();
        let mut gamma = EnvironmentStack::new();

        let ast = parse("if true then 1 else 2");

        assert_eq!(evaluate(&mut gamma, &mut memory, &ast), Reference::Integer(1))
    }

    // if false then 1 else 2
    #[test]
    fn conditional_alternative() {
        let mut memory = Memory::new();
        let mut gamma = EnvironmentStack::new();

        let ast = parse("if false then 1 else 2");

        assert_eq!(evaluate(&mut gamma, &mut memory, &ast), Reference::Integer(2))
    }

    // function f(x) x
    #[test]
    fn function_definition() {
        let mut memory = Memory::new();
        let mut gamma = EnvironmentStack::new();

        let ast = parse("function f(x) <- x");

        println!("{:?}", ast);

        assert_eq!(evaluate(&mut gamma, &mut memory, &ast), Reference::Unit);

        let reference = gamma.lookup_function("f").unwrap();
        assert!(memory.contains_function(reference));
        let function = memory.get_function(reference).unwrap();

        let expected = make_function!("f", parse("x"), "x");
        assert_eq!(function, &expected);
    }

    // f(1)
    #[test]
    fn function_call() {
        let mut memory = Memory::new();
        let mut gamma = EnvironmentStack::new();

        let function = make_function!("f", parse("x"), "x");
        let reference = memory.put_function(function);
        gamma.register_function("f".to_string(), reference)
            .expect("Could not register function in test header");

        let ast = parse("f(1)");

        assert_eq!(evaluate(&mut gamma, &mut memory, &ast), Reference::Integer(1));
    }


}

fn main() {
    println!("Hello, world!");
}
