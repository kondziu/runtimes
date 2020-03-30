#[macro_use]
extern crate lalrpop_util;

#[macro_use]
extern crate fml_ast;

//extern crate unescape;

lalrpop_mod!(pub fml); // synthesized by LALRPOP

pub fn parse(input: &str) -> fml_ast::AST {
    fml::TopLevelParser::new().parse(input).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::fml::TopLevelParser;
    use fml_ast::{AST, Operator};

    #[allow(dead_code)]
    fn parse_ok(input: &str, correct: AST) {
        println!("{}", input);
        for i in 0..input.len() {
            if i%10 == 0 {
                print!(" ");
            } else {
                print!("{}", i % 10);
            }
        }
        println!();
        assert_eq!(TopLevelParser::new().parse(input), Ok(correct));
    }

    #[allow(dead_code)]
    fn parse_err(input: &str) {
        println!("{}", input);
        assert!(TopLevelParser::new().parse(input).is_err());
    }

    #[test] fn test_unit()         { parse_ok("null", AST::Unit);        }
    #[test] fn test_nothing()      { parse_ok("",     AST::Unit);        }

    #[test] fn test_0()            { parse_ok("0",    AST::Number(0));   }
    #[test] fn test_negative_0()   { parse_ok("-0",   AST::Number(0));   }
    #[test] fn test_2()            { parse_ok("2",    AST::Number(2));   }
    #[test] fn test_negative_2()   { parse_ok("-2",   AST::Number(-2));  }
    #[test] fn test_42()           { parse_ok("42",   AST::Number(42));  }
    #[test] fn test_042()          { parse_ok("042",  AST::Number(42));  }
    #[test] fn test_00()           { parse_ok("00",   AST::Number(0));   }
    #[test] fn test_negative_042() { parse_ok("-042", AST::Number(-42)); }
    #[test] fn test_negative_00()  { parse_ok("-00",  AST::Number(0));   }

    #[test] fn test_underscore()             { parse_ok("_",     AST::Identifier("_".to_string()));     }
    #[test] fn test_underscore_identifier()  { parse_ok("_x",    AST::Identifier("_x".to_string()));    }
    #[test] fn test_identifier()             { parse_ok("x",     AST::Identifier("x".to_string()));     }
    #[test] fn test_identifier_with_number() { parse_ok("x1",    AST::Identifier("x1".to_string()));    }
    #[test] fn test_multiple_underscores()   { parse_ok("___",   AST::Identifier("___".to_string()));   }
    #[test] fn test_long_identifier()        { parse_ok("stuff", AST::Identifier("stuff".to_string())); }

    #[test] fn test_true()  { parse_ok("true", AST::Boolean(true));  }
    #[test] fn test_false() { parse_ok("false", AST::Boolean(false)); }

    #[test] fn test_number_in_parens() { parse_ok("(1)", AST::Number(1)); }
    #[test] fn test_number_in_two_parens() { parse_ok("((1))", AST::Number(1)); }
    #[test] fn test_number_parens_with_whitespace() { parse_ok("( 1 )", AST::Number(1)); }

    #[test] fn test_local_definition() {
        parse_ok("let x = 1",
                 AST::LocalDefinition {
                     identifier: Box::new(AST::Identifier("x".to_string())),
                     value: Box::new(AST::Number(1))});
    }

    #[test] fn test_mutation()   {
        parse_ok("x <- 1", AST::LocalMutation {
            identifier: Box::new(AST::Identifier("x".to_string())),
            value: Box::new(AST::Number(1))});
    }

    #[test] fn test_function_no_args() {
        parse_ok("function f () <- 1",
                 AST::FunctionDefinition {
                     name: Box::new(AST::Identifier("f".to_string())),
                     parameters: vec!(),
                     body: Box::new(AST::Number(1))}); }

    #[test] fn test_function_one_arg() {
        parse_ok("function f (x) <- x",
                 AST::FunctionDefinition {
                     name: Box::new(AST::Identifier("f".to_string())),
                     parameters: vec!(Box::new(AST::Identifier("x".to_string()))),
                     body: Box::new(AST::Identifier("x".to_string()))});
    }

    #[test] fn test_function_many_args() {
        parse_ok("function f (x, y, z) <- x",
                 AST::FunctionDefinition {
                     name: Box::new(AST::Identifier("f".to_string())),
                     parameters: vec!(Box::new(AST::Identifier("x".to_string())),
                                      Box::new(AST::Identifier("y".to_string())),
                                      Box::new(AST::Identifier("z".to_string()))),
                     body: Box::new(AST::Identifier("x".to_string()))});
    }

    #[test] fn test_application_no_args() {
        parse_ok("f ()",
                 AST::FunctionApplication {
                     function: Box::new(AST::Identifier("f".to_string())),
                     arguments: vec!()});
    }

    #[test] fn test_application_one_arg() {
        parse_ok("f (0)",
                 AST::FunctionApplication {
                     function: Box::new(AST::Identifier("f".to_string())),
                     arguments: vec!(Box::new(AST::Number(0)))});
    }

    #[test] fn test_application_more_args() {
        parse_ok("f (1, x, true)",
                 AST::FunctionApplication {
                     function: Box::new(AST::Identifier("f".to_string())),
                     arguments: vec!(Box::new(AST::Number(1)),
                                     Box::new(AST::Identifier("x".to_string())),
                                     Box::new(AST::Boolean(true)))});
    }

    #[test] fn test_application_no_spaces() {
        parse_ok("f(0,-1)",
                 AST::FunctionApplication {
                     function: Box::new(AST::Identifier("f".to_string())),
                     arguments: vec!(Box::new(AST::Number(0)),
                                     Box::new(AST::Number(-1)))});
    }

    #[test] fn test_application_more_spaces() {
        parse_ok("f    (   0    , -1 )",
                 AST::FunctionApplication {
                     function: Box::new(AST::Identifier("f".to_string())),
                     arguments: vec!(Box::new(AST::Number(0)),
                                     Box::new(AST::Number(-1)))});
    }

    #[test] fn test_application_extra_comma() {
        parse_ok("f(0,-1,)",
                 AST::FunctionApplication {
                     function: Box::new(AST::Identifier("f".to_string())),
                     arguments: vec!(Box::new(AST::Number(0)),
                                     Box::new(AST::Number(-1)))});
    }

    #[test] fn test_application_just_a_comma()      { parse_err("f(,)");}
    #[test] fn test_application_many_extra_commas() { parse_err("f(x,,)");}

    #[test] fn test_empty_block_is_unit()  { parse_ok("begin end", AST::Unit);}
    #[test] fn test_block_one_expression() { parse_ok("begin 1 end",AST::Number(1));}

    #[test] fn test_block_one_expression_and_semicolon() {
        parse_ok("begin 1; end",AST::Number(1))
    }
    #[test] fn test_block_many_expressions() {
        parse_ok("begin 1; 2; 3 end",
                 AST::Block(
                     vec!(Box::new(AST::Number(1)),
                          Box::new(AST::Number(2)),
                          Box::new(AST::Number(3)))))
    }

    #[test] fn test_block_trailing_semicolon() {
        parse_ok("begin 1; 2; 3; end",
                 AST::Block(
                     vec!(Box::new(AST::Number(1)),
                          Box::new(AST::Number(2)),
                          Box::new(AST::Number(3)))))
    }

    #[test] fn test_loop() {
        parse_ok("while true do null",
                 AST::Loop {
                     condition: Box::new(AST::Boolean(true)),
                     body: Box::new(AST::Unit)})
    }

    #[test] fn test_conditional() {
        parse_ok("if true then false else true",
                 AST::Conditional{
                     condition: Box::new(AST::Boolean(true)),
                     consequent: Box::new(AST::Boolean(false)),
                     alternative: Box::new(AST::Boolean(true))})
    }

    #[test] fn test_conditional_no_alternative() {
        parse_ok("if true then false",
                 AST::Conditional{
                     condition: Box::new(AST::Boolean(true)),
                     consequent: Box::new(AST::Boolean(false)),
                     alternative: Box::new(AST::Unit)})
    }

    #[test] fn test_conditional_so_many() {
        parse_ok("if x then \
                        if y then 3 else 2 \
                    else \
                        if y then 1 else 0",
                 AST::Conditional{
                     condition: Box::new(AST::Identifier("x".to_string())),
                     consequent: Box::new(
                         AST::Conditional{
                             condition: Box::new(AST::Identifier("y".to_string())),
                             consequent: Box::new(AST::Number(3)),
                             alternative: Box::new(AST::Number(2))}),
                     alternative: Box::new(
                         AST::Conditional{
                             condition: Box::new(AST::Identifier("y".to_string())),
                             consequent: Box::new(AST::Number(1)),
                             alternative: Box::new(AST::Number(0))})})
    }

    #[test]
    fn test_array_definition() {
        parse_ok("array(10,0)",
                 AST::ArrayDefinition {
                     size: Box::new(AST::Number(10)),
                     value: Box::new(AST::Number(0))})
    }

    #[test]
    fn test_array_definition_spaces() {
        parse_ok("array ( 10, 0 )",
                 AST::ArrayDefinition {
                     size: Box::new(AST::Number(10)),
                     value: Box::new(AST::Number(0))})
    }

    #[test]
    fn test_empty_object() {
        parse_ok("object begin end",
                 AST::ObjectDefinition {
                     extends: None,
                     members: vec!()})
    }

    #[test]
    fn test_empty_object_with_superobject() {
        parse_ok("object extends y begin end",
                 AST::ObjectDefinition {
                     extends: Some(Box::new(AST::Identifier("y".to_string()))),
                     members: vec!()})
    }

    #[test]
    fn test_object_extending_expression() {
        parse_ok("object extends if y then 1 else true begin end",
                 AST::ObjectDefinition {
                     extends: Some(Box::new(AST::Conditional{
                         condition: Box::new(AST::Identifier("y".to_string())),
                         consequent: Box::new(AST::Number(1)),
                         alternative: Box::new(AST::Boolean(true))})),
                     members: vec!()})
    }

    #[test]
    fn test_object_extending_ad_hoc_object() {
        parse_ok("object extends object begin end begin end",
                 AST::ObjectDefinition {
                     extends: Some(Box::new(AST::ObjectDefinition {
                         extends: None,
                         members: vec!()})),
                     members: vec!()})
    }

    #[test]
    fn test_object_with_one_field() {
        parse_ok("object begin let y = x; end",
                 AST::ObjectDefinition {
                     extends: None,
                     members: vec!(Box::new(
                         AST::LocalDefinition {
                             identifier: Box::new(AST::Identifier("y".to_string())),
                             value: Box::new(AST::Identifier("x".to_string()))}))})
    }

    #[test]
    fn test_object_with_one_method() {
        parse_ok("object begin function m (x, y, z) <- y; end",
                 AST::ObjectDefinition {
                     extends: None,
                     members: vec!(Box::new(
                         AST::FunctionDefinition {
                             name: Box::new(AST::Identifier("m".to_string())),
                             parameters: vec!(Box::new(AST::Identifier("x".to_string())),
                                              Box::new(AST::Identifier("y".to_string())),
                                              Box::new(AST::Identifier("z".to_string()))),
                             body: Box::new(AST::Identifier("y".to_string()))}))})
    }

    #[test]
    fn test_object_with_an_operator() {
        parse_ok("object begin function + (y) <- y; end",
                 AST::ObjectDefinition {
                     extends: None,
                     members: vec!(Box::new(
                         AST::OperatorDefinition {
                             operator: Operator::Addition,
                             parameters: vec!(Box::new(AST::Identifier("y".to_string()))),
                             body: Box::new(AST::Identifier("y".to_string()))}))})
    }

    #[test]
    fn test_object_with_many_members() {
        parse_ok("object begin \
                    let a = x; \
                    let b = true; \
                    function m (x, y, z) <- y; \
                    function id (x) <- x; \
                    function me () <- this; \
                end",
                 AST::ObjectDefinition {
                     extends: None,
                     members: vec!(
                         Box::new(AST::LocalDefinition {
                             identifier: Box::new(AST::Identifier("a".to_string())),
                             value: Box::new(AST::Identifier("x".to_string()))}),
                         Box::new(AST::LocalDefinition {
                             identifier: Box::new(AST::Identifier("b".to_string())),
                             value: Box::new(AST::Boolean(true))}),
                         Box::new(AST::FunctionDefinition {
                             name: Box::new(AST::Identifier("m".to_string())),
                             parameters: vec!(Box::new(AST::Identifier("x".to_string())),
                                              Box::new(AST::Identifier("y".to_string())),
                                              Box::new(AST::Identifier("z".to_string()))),
                             body: Box::new(AST::Identifier("y".to_string()))}),
                         Box::new(AST::FunctionDefinition {
                             name: Box::new(AST::Identifier("id".to_string())),
                             parameters: vec!(Box::new(AST::Identifier("x".to_string()))),
                             body: Box::new(AST::Identifier("x".to_string()))}),
                         Box::new(AST::FunctionDefinition {
                             name: Box::new(AST::Identifier("me".to_string())),
                             parameters: vec!(),
                             body: Box::new(AST::Identifier("this".to_string()))}))})
    }

    #[test] fn test_field_access_from_identifier () {
        parse_ok("a.b",
                 AST::FieldAccess {
                     object: Box::new(AST::Identifier("a".to_string())),
                     field: Box::new(AST::Identifier("b".to_string()))});
    }

    #[test] fn test_field_access_from_number () {
        parse_ok("1.b",
                 AST::FieldAccess {
                     object: Box::new(AST::Number(1)),
                     field: Box::new(AST::Identifier("b".to_string()))});
    }

    #[test] fn test_field_access_from_boolean () {
        parse_ok("true.b",
                 AST::FieldAccess {
                     object: Box::new(AST::Boolean(true)),
                     field: Box::new(AST::Identifier("b".to_string()))});
    }

    #[test] fn test_field_access_from_parenthesized_expression () {
        parse_ok("(if x then 1 else 2).b",
                 AST::FieldAccess {
                     object: Box::new(
                         AST::Conditional{
                             condition: Box::new(AST::Identifier("x".to_string())),
                             consequent: Box::new(AST::Number(1)),
                             alternative: Box::new(AST::Number(2))}),
                     field: Box::new(AST::Identifier("b".to_string()))});
    }

    #[test] fn test_field_chain_access () {
        parse_ok("a.b.c.d",
                 AST::FieldAccess {
                     object: Box::new(
                         AST::FieldAccess {
                             object: Box::new(AST::FieldAccess {
                                 object: Box::new(AST::Identifier("a".to_string())),
                                 field: Box::new(AST::Identifier("b".to_string()))}),
                             field: Box::new(AST::Identifier("c".to_string()))}),
                     field: Box::new(AST::Identifier("d".to_string()))});
    }

    #[test] fn test_field_mutation_from_identifier () {
        parse_ok("a.b <- 1",
                 AST::FieldMutation {
                     field_path: Box::new(AST::FieldAccess {
                         object: Box::new(AST::Identifier("a".to_string())),
                         field: Box::new(AST::Identifier("b".to_string()))}),
                     value: Box::new(AST::Number(1))});
    }

    #[test] fn test_method_call_from_identifier () {
        parse_ok("a.b (1)",
                 AST::MethodCall {
                     method_path: Box::new(AST::FieldAccess {
                         object: Box::new(AST::Identifier("a".to_string())),
                         field: Box::new(AST::Identifier("b".to_string()))}),
                     arguments: vec!(Box::new(AST::Number(1)))});
    }

    #[test] fn test_method_call_to_operator () {
        parse_ok("a.+(1)",
                 AST::MethodCall {
                     method_path: Box::new(AST::OperatorAccess {
                         object: Box::new(AST::Identifier("a".to_string())),
                         operator: Operator::Addition}),
                     arguments: vec!(Box::new(AST::Number(1)))});
    }

    #[test] fn test_array_access () {
        parse_ok("a[1]",
                 AST::ArrayAccess {
                     array: Box::new(AST::Identifier("a".to_string())),
                     index: Box::new(AST::Number(1))});
    }

    #[test] fn test_array_access_from_object () {
        parse_ok("a.b[1]",
                 AST::ArrayAccess {
                     array: Box::new(
                         AST::FieldAccess {
                             object: Box::new(AST::Identifier("a".to_string())),
                             field: Box::new(AST::Identifier("b".to_string()))}),
                     index: Box::new(AST::Number(1))});
    }

    #[test] fn test_array_access_from_array () {
        parse_ok("a[b][1]",
                 AST::ArrayAccess {
                     array: Box::new(
                         AST::ArrayAccess {
                             array: Box::new(AST::Identifier("a".to_string())),
                             index: Box::new(AST::Identifier("b".to_string()))}),
                     index: Box::new(AST::Number(1))});
    }

    #[test] fn test_array_call_method_on_member () {
        parse_ok("a[b](1)",
                 {
                     AST::MethodCall {
                         method_path: Box::new(
                             AST::ArrayAccess {
                                 array: Box::new(AST::Identifier("a".to_string())),
                                 index: Box::new(AST::Identifier("b".to_string()))}),
                         arguments: vec!(Box::new(AST::Number(1)))}});
    }

    #[test] fn test_array_access_member_of_member () {
        parse_ok("a[b].a",
                 {
                     AST::FieldAccess {
                         object: Box::new(
                             AST::ArrayAccess {
                                 array: Box::new(AST::Identifier("a".to_string())),
                                 index: Box::new(AST::Identifier("b".to_string()))}),
                         field: Box::new(AST::Identifier("a".to_string()))}});
    }

    #[test] fn test_array_access_with_array_access_as_index () {
        parse_ok("a[b[c]]",
                 AST::ArrayAccess {
                     array: Box::new(AST::Identifier("a".to_string())),
                     index: Box::new(
                         AST::ArrayAccess {
                             array: Box::new(AST::Identifier("b".to_string())),
                             index: Box::new(AST::Identifier("c".to_string()))})});
    }

    #[test] fn test_array_access_from_function_call () {
        parse_ok("f(0,0)[x]",
                 AST::ArrayAccess {
                     array: Box::new(
                         AST::FunctionApplication {
                             function: Box::new(AST::Identifier("f".to_string())),
                             arguments: vec!(Box::new(AST::Number(0)),
                                             Box::new(AST::Number(0)))}),
                     index: Box::new(
                         AST::Identifier("x".to_string()))});
    }

    #[test] fn test_print_call_with_arguments() {
        parse_ok("print(\"~ ~ ~\", 1, true, x)",
                 AST::Print {
                     format: Box::new(AST::String("~ ~ ~".to_string())),
                     arguments: vec!(
                         Box::new(AST::Number(1)),
                         Box::new(AST::Boolean(true)),
                         Box::new(AST::Identifier("x".to_string())))});
    }

    #[test] fn test_print_call_without_arguments() {
        parse_ok("print(\"~ ~ ~\")",
                 AST::Print {
                     format: Box::new(AST::String("~ ~ ~".to_string())),
                     arguments: vec!()});
    }

    #[test] fn test_print_call_string() {
        parse_ok("print(\"hello world\")",
                 AST::Print {
                     format: Box::new(AST::String("hello world".to_string())),
                     arguments: vec!()});
    }

    #[test] fn test_print_call_empty_string() {
        parse_ok("print(\"\")",
                 AST::Print {
                     format: Box::new(AST::String("".to_string())),
                     arguments: vec!()});
    }

    #[test] fn test_print_call_escape_newline() {
        parse_ok("print(\"\\n\")",
                 AST::Print {
                     format: Box::new(AST::String("\\n".to_string())),
                     arguments: vec!()});
    }

    #[test] fn test_print_call_escape_tab() {
        parse_ok("print(\"\\t\")",
                 AST::Print {
                     format: Box::new(AST::String("\\t".to_string())),
                     arguments: vec!()});
    }

    #[test] fn test_print_call_escape_backspace() {
        parse_ok("print(\"\\b\")",
                 AST::Print {
                     format: Box::new(AST::String("\\b".to_string())),
                     arguments: vec!()});
    }

    #[test] fn test_print_call_escape_return() {
        parse_ok("print(\"\\r\")",
                 AST::Print {
                     format: Box::new(AST::String("\\r".to_string())),
                     arguments: vec!()});
    }

    #[test] fn test_print_call_escape_backslash() {
        parse_ok("print(\"\\\\\")",
                 AST::Print {
                     format: Box::new(AST::String("\\\\".to_string())),
                     arguments: vec!()});
    }

    #[test] fn test_print_call_botched_escape() { parse_err("print(\"\\\")");  }
    #[test] fn test_print_call_invalid_escape() { parse_err("print(\"\\a\")"); }


    #[test] fn test_simple_disjunction() {
        parse_ok("true | false",
                 AST::Operation {
                     operator: Operator::Disjunction,
                     left: Box::new(AST::Boolean(true)),
                     right: Box::new(AST::Boolean(false))});
    }

    #[test] fn test_double_disjunction() {
        parse_ok("true | false | true",
                 AST::Operation {
                     operator: Operator::Disjunction,
                     left: Box::new(
                         AST::Operation {
                             operator: Operator::Disjunction,
                             left: Box::new(AST::Boolean(true)),
                             right: Box::new(AST::Boolean(false))}),
                     right: Box::new(AST::Boolean(true))});
    }

    #[test] fn test_simple_conjunction() {
        parse_ok("true & false",
                 AST::Operation {
                     operator: Operator::Conjunction,
                     left: Box::new(AST::Boolean(true)),
                     right: Box::new(AST::Boolean(false))});
    }

    #[test] fn test_double_conjunction() {
        parse_ok("true & false & true",
                 AST::Operation {
                     operator: Operator::Conjunction,
                     left: Box::new(
                         AST::Operation {
                             operator: Operator::Conjunction,
                             left: Box::new(AST::Boolean(true)),
                             right: Box::new(AST::Boolean(false))}),
                     right: Box::new(AST::Boolean(true))});
    }

    #[test] fn test_simple_equality() {
        parse_ok("true == false",
                 AST::Operation {
                     operator: Operator::Equality,
                     left: Box::new(AST::Boolean(true)),
                     right: Box::new(AST::Boolean(false))});
    }


    #[test] fn test_simple_inequality() {
        parse_ok("true != false",
                 AST::Operation {
                     operator: Operator::Inequality,
                     left: Box::new(AST::Boolean(true)),
                     right: Box::new(AST::Boolean(false))});
    }

    #[test] fn test_disjunction_and_conjunction() {
        //or (true, (true & false & false)))
        parse_ok("true | true & false",
                 AST::Operation {
                     operator: Operator::Disjunction,
                     left: Box::new(AST::Boolean(true)),
                     right: Box::new(AST::Operation {
                         operator: Operator::Conjunction,
                         left: Box::new(AST::Boolean(true)),
                         right: Box::new(AST::Boolean(false))
                     })
                 });
    }

    #[test] fn test_disjunction_and_conjunctions() {
        //or (true, (true & false & false)))
        parse_ok("true & false | true & false",
                 AST::Operation {
                     operator: Operator::Disjunction,
                     left: Box::new(AST::Operation {
                         operator: Operator::Conjunction,
                         left: Box::new(AST::Boolean(true)),
                         right: Box::new(AST::Boolean(false))
                     }),
                     right: Box::new(AST::Operation {
                         operator: Operator::Conjunction,
                         left: Box::new(AST::Boolean(true)),
                         right: Box::new(AST::Boolean(false))
                     })
                 });
    }

    #[test] fn test_disjunctions_and_conjunctions() {
        //or (true, (true & false & false)))
        parse_ok("true & false | true & false | true & false",
                 AST::Operation {
                     operator: Operator::Disjunction,
                     left: Box::new(AST::Operation {
                         operator: Operator::Disjunction,
                         left: Box::new(AST::Operation {
                             operator: Operator::Conjunction,
                             left: Box::new(AST::Boolean(true)),
                             right: Box::new(AST::Boolean(false))
                         }),
                         right: Box::new(AST::Operation {
                             operator: Operator::Conjunction,
                             left: Box::new(AST::Boolean(true)),
                             right: Box::new(AST::Boolean(false))
                         })
                     }),
                     right: Box::new(AST::Operation {
                         operator: Operator::Conjunction,
                         left: Box::new(AST::Boolean(true)),
                         right: Box::new(AST::Boolean(false))
                     })});
    }

    #[test] fn test_more_disjunctions_and_more_conjunctions() {
        //or (true, (true & false & false)))
        parse_ok("true & false & true | true & true & false & true | true & false",
                 AST::Operation {
                     operator: Operator::Disjunction,
                     left: Box::new(AST::Operation {
                         operator: Operator::Disjunction,
                         left: Box::new(AST::Operation {
                             operator: Operator::Conjunction,
                             left: Box::new(AST::Operation {
                                 operator: Operator::Conjunction,
                                 left: Box::new(AST::Boolean(true)),
                                 right: Box::new(AST::Boolean(false))
                             }),
                             right: Box::new(AST::Boolean(true))
                         }),
                         right: Box::new(AST::Operation {
                             operator: Operator::Conjunction,
                             left: Box::new(AST::Operation {
                                 operator: Operator::Conjunction,
                                 left: Box::new(AST::Operation {
                                     operator: Operator::Conjunction,
                                     left: Box::new(AST::Boolean(true)),
                                     right: Box::new(AST::Boolean(true))
                                 }),
                                 right: Box::new(AST::Boolean(false))
                             }),
                             right: Box::new(AST::Boolean(true))
                         })
                     }),
                     right: Box::new(AST::Operation {
                         operator: Operator::Conjunction,
                         left: Box::new(AST::Boolean(true)),
                         right: Box::new(AST::Boolean(false))
                     })});
    }

    #[test] fn test_simple_addition() {
        parse_ok("1 + 2",
                 AST::Operation {
                     operator: Operator::Addition,
                     left: Box::new(AST::Number(1)),
                     right: Box::new(AST::Number(2))});
    }

    #[test] fn test_addition_to_field_object() {
        parse_ok("a.x + 2",
                 AST::Operation {
                     operator: Operator::Addition,
                     left: Box::new(AST::FieldAccess {
                         field: Box::new(AST::Identifier("x".to_string())),
                         object: Box::new(AST::Identifier("a".to_string()))}),
                     right: Box::new(AST::Number(2))});
    }

    #[test] fn test_simple_subtraction() {
        parse_ok("1 - 2",
                 AST::Operation {
                     operator: Operator::Subtraction,
                     left: Box::new(AST::Number(1)),
                     right: Box::new(AST::Number(2))});
    }

    #[test] fn test_simple_multiplication() {
        parse_ok("1 * 2",
                 AST::Operation {
                     operator: Operator::Multiplication,
                     left: Box::new(AST::Number(1)),
                     right: Box::new(AST::Number(2))});
    }

    #[test] fn test_simple_module() {
        parse_ok("1 % 2",
                 AST::Operation {
                     operator: Operator::Module,
                     left: Box::new(AST::Number(1)),
                     right: Box::new(AST::Number(2))});
    }

    #[test] fn test_simple_division() {
        parse_ok("1 / 2",
                 AST::Operation {
                     operator: Operator::Division,
                     left: Box::new(AST::Number(1)),
                     right: Box::new(AST::Number(2))});
    }

    #[test] fn test_simple_less_than() {
        parse_ok("1 < 2",
                 AST::Operation {
                     operator: Operator::Less,
                     left: Box::new(AST::Number(1)),
                     right: Box::new(AST::Number(2))});
    }

    #[test] fn test_simple_less_or_equal() {
        parse_ok("1 <= 2",
                 AST::Operation {
                     operator: Operator::LessEqual,
                     left: Box::new(AST::Number(1)),
                     right: Box::new(AST::Number(2))});
    }

    #[test] fn test_simple_greater_than() {
        parse_ok("1 > 2",
                 AST::Operation {
                     operator: Operator::Greater,
                     left: Box::new(AST::Number(1)),
                     right: Box::new(AST::Number(2))});
    }

    #[test] fn test_simple_greater_or_equal() {
        parse_ok("1 >= 2",
                 AST::Operation {
                     operator: Operator::GreaterEqual,
                     left: Box::new(AST::Number(1)),
                     right: Box::new(AST::Number(2))});
    }

    #[test] fn test_comment() {
        parse_ok("(* a *)", AST::Unit);
    }
    #[test] fn test_comment_in_expression() {
        parse_ok("1 + (* a *) 2",
                 AST::Operation {
                     operator: Operator::Addition,
                     left: Box::new(AST::Number(1)),
                     right: Box::new(AST::Number(2))});
    }

    #[test] fn test_multiline_comment() {
        parse_ok("(* \n\n\n *)", AST::Unit);
    }
}
