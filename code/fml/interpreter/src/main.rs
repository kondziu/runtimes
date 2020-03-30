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
    use fml_parser::parse;
    use std::collections::HashMap;

    macro_rules! make_function {
        ($name:expr, $body:expr, $( $parameter:expr ),*) => {
            Function::new($name.to_string(), {
                let mut parameters: Vec<String> = Vec::new();
                $(
                    parameters.push($parameter.to_string());
                )*
                parameters
            }, Box::new($body));
        };
        ($name:expr, $body:expr) => {
            Function::new($name.to_string(), Vec::new(), Box::new($body));
        }
    }

    macro_rules! make_array {
        ($size:expr, $value:expr) => {
            Instance::Array {
                size: $size,
                values: {
                    let mut values = Vec::new();
                    for _ in 0..$size {
                        values.push($value.clone())
                    }
                    values
                }
            }
        }
    }

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
        assert!(gamma.register_function("f".to_string(), reference).is_ok());

        let ast = parse("f(1)");

        assert_eq!(evaluate(&mut gamma, &mut memory, &ast), Reference::Integer(1));
    }

    // array(10, 1)
    #[test]
    fn array_definition() {
        let mut memory = Memory::new();
        let mut gamma = EnvironmentStack::new();

        let ast = parse("array(10, 1)");

        let expected_reference = Reference::Array {reference: 0, size: 10};
        assert_eq!(evaluate(&mut gamma, &mut memory, &ast), expected_reference);

        assert!(memory.contains_object(&expected_reference));
        assert_eq!(memory.get_object(&expected_reference),
                   Some(&make_array!(10, Reference::Integer(1))));
    }

    // a[1]
    #[test]
    fn array_access() {
        let mut memory = Memory::new();
        let mut gamma = EnvironmentStack::new();

        let expected_reference = memory.put_object(make_array!(10, Reference::Integer(1)));
        assert!(gamma.register_binding("a".to_string(), expected_reference).is_ok());

        let ast = parse("a[1]");
        assert_eq!(evaluate(&mut gamma, &mut memory, &ast), Reference::Integer(1));
    }

    // object begin end
    #[test]
    fn empty_object_definition() {
        let mut memory = Memory::new();
        let mut gamma = EnvironmentStack::new();

        let ast = parse("object begin end");

        let expected_reference = Reference::Object(0);
        assert_eq!(evaluate(&mut gamma, &mut memory, &ast), expected_reference);

        assert!(memory.contains_object(&expected_reference));
        assert_eq!(memory.get_object(&expected_reference),
                   Some(&Instance::empty()));
    }

    // object begin let x = 1; function add(x) <- (this.x) + x; end
    #[test]
    fn object_definition() {
        let mut memory = Memory::new();
        let mut gamma = EnvironmentStack::new();

        let ast = parse("object begin let x = 1; function add(x) <- (this.x) + x; end");

        let expected_reference = Reference::Object(1);
        assert_eq!(evaluate(&mut gamma, &mut memory, &ast), expected_reference);

        let method = make_function!("add", parse("(this.x) + x"), "x");

        assert!(memory.contains_function(&FunctionReference::Function(0)));
        assert_eq!(memory.get_function(&FunctionReference::Function(0)), Some(&method));

        let mut fields = HashMap::new();
        fields.insert("x".to_string(), Reference::Integer(1));

        let mut methods = HashMap::new();
        methods.insert("add".to_string(), FunctionReference::Function(0));

        assert!(memory.contains_object(&expected_reference));
        assert_eq!(memory.get_object(&expected_reference),
                Some(&Instance::Object{extends: None, fields, methods}));
    }

    // obj.x
    #[test]
    fn field_access() {
        let mut memory = Memory::new();
        let mut gamma = EnvironmentStack::new();

        let mut fields = HashMap::new();
        fields.insert("x".to_string(), Reference::Integer(42));

        let object_instance = Instance::Object{extends: None, fields, methods: HashMap::new()};
        let object_reference = memory.put_object(object_instance);
        assert!(gamma.register_binding("obj".to_string(), object_reference).is_ok());

        let ast = parse("obj.x");

        assert_eq!(evaluate(&mut gamma, &mut memory, &ast), Reference::Integer(42));
    }

    // obj.get()
    #[test]
    fn method_getter_call() {
        let mut memory = Memory::new();
        let mut gamma = EnvironmentStack::new();

        let mut fields = HashMap::new();
        fields.insert("x".to_string(), Reference::Integer(42));

        let mut methods = HashMap::new();
        let method_instance = make_function!("get", parse("(this.x)"));
        let method_reference = memory.put_function(method_instance);
        methods.insert("get".to_string(), method_reference);

        let object_instance = Instance::Object{extends: None, fields, methods};
        let object_reference = memory.put_object(object_instance);
        assert!(gamma.register_binding("obj".to_string(), object_reference).is_ok());

        let ast = parse("obj.get()");

        assert_eq!(evaluate(&mut gamma, &mut memory, &ast), Reference::Integer(42));
    }

    // fortytwo + 1
    #[test]
    fn operator_call() {
        let mut memory = Memory::new();
        let mut gamma = EnvironmentStack::new();

        let mut fields = HashMap::new();
        fields.insert("value".to_string(), Reference::Integer(42));

        let mut methods = HashMap::new();
        let method_instance = make_function!("+", parse("(this.value) + x"), "x");
        let method_reference = memory.put_function(method_instance);
        methods.insert("+".to_string(), method_reference);

        let object_instance = Instance::Object{extends: None, fields, methods};
        let object_reference = memory.put_object(object_instance);
        assert!(gamma.register_binding("fortytwo".to_string(), object_reference).is_ok());

        let ast = parse("fortytwo + 1");

        assert_eq!(evaluate(&mut gamma, &mut memory, &ast), Reference::Integer(43));
    }

    #[test]
    fn object_equality() {
        let mut memory = Memory::new();
        let mut gamma = EnvironmentStack::new();

        let mut fields = HashMap::new();
        fields.insert("value".to_string(), Reference::Integer(42));

        let mut methods = HashMap::new();
        let method_instance = make_function!("+", parse("(this.value) + x"), "x");
        let method_reference = memory.put_function(method_instance);
        methods.insert("+".to_string(), method_reference);

        let object_instance = Instance::Object{extends: None, fields, methods};
        let object_reference = memory.put_object(object_instance);
        assert!(gamma.register_binding("fortytwo".to_string(), object_reference).is_ok());

        let ast = parse("fortytwo == fortytwo");

        assert_eq!(evaluate(&mut gamma, &mut memory, &ast), Reference::Boolean(true));
    }

    #[test]
    fn object_inequality() {
        let mut memory = Memory::new();
        let mut gamma = EnvironmentStack::new();

        let mut fields = HashMap::new();
        fields.insert("value".to_string(), Reference::Integer(42));

        let mut methods = HashMap::new();
        let method_instance = make_function!("+", parse("(this.value) + x"), "x");
        let method_reference = memory.put_function(method_instance);
        methods.insert("+".to_string(), method_reference);

        let object_instance = Instance::Object{extends: None, fields, methods};
        let object_reference = memory.put_object(object_instance);
        assert!(gamma.register_binding("fortytwo".to_string(), object_reference).is_ok());

        let ast = parse("fortytwo != fortytwo");

        assert_eq!(evaluate(&mut gamma, &mut memory, &ast), Reference::Boolean(false));
    }
}

fn main() {
    println!("Hello, world!");
}
