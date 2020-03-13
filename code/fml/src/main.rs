#[macro_use]
extern crate lalrpop_util;
extern crate unescape;

#[macro_use]
pub mod fml_ast;

lalrpop_mod!(pub fml); // synthesized by LALRPOP

use crate::fml::TopLevelParser;
use crate::fml_ast::AST;
use crate::fml_ast::AST::*;
use crate::fml_ast::Operator::*;

fn parse_ok(input: &str, correct: AST) {
    assert_eq!(TopLevelParser::new().parse(input), Ok(correct));
}

fn parse_err(input: &str) {
    assert!(TopLevelParser::new().parse(input).is_err());
}

#[test] fn test_unit()         { parse_ok("null", Unit);        }
#[test] fn test_nothing()      { parse_ok("", Unit);            }

#[test] fn test_0()            { parse_ok("0", Number(0));      }
#[test] fn test_negative_0()   { parse_ok("-0", Number(0));     }
#[test] fn test_2()            { parse_ok("2", Number(2));      }
#[test] fn test_negative_2()   { parse_ok("-2", Number(-2));    }
#[test] fn test_42()           { parse_ok("42",   Number(42));  }
#[test] fn test_042()          { parse_ok("042",  Number(42));  }
#[test] fn test_00()           { parse_ok("00",   Number(0));   }
#[test] fn test_negative_042() { parse_ok("-042", Number(-42)); }
#[test] fn test_negative_00()  { parse_ok("-00",  Number(0));   }


#[test] fn test_underscore()             { parse_ok("_",     Identifier("_"));     }
#[test] fn test_underscore_identifier()  { parse_ok("_x",    Identifier("_x"));    }
#[test] fn test_identifier()             { parse_ok("x",     Identifier("x"));     }
#[test] fn test_identifier_with_number() { parse_ok("x1",    Identifier("x1"));    }
#[test] fn test_multiple_underscores()   { parse_ok("___",   Identifier("___"));   }
#[test] fn test_long_identifier()        { parse_ok("stuff", Identifier("stuff")); }

#[test] fn test_true()  { parse_ok("true", Boolean(true));  }
#[test] fn test_false() { parse_ok("false", Boolean(false)); }

#[test] fn test_number_in_parens() { parse_ok("(1)", Number(1)); }
#[test] fn test_number_in_two_parens() { parse_ok("((1))", Number(1)); }
#[test] fn test_number_parens_with_whitespace() { parse_ok("( 1 )", Number(1)); }

#[test] fn test_assignment() {
    parse_ok("let x = 1",
             Assignment {
                 identifier: Box::new(Identifier("x")),
                 value: Box::new(Number(1))});
}

#[test] fn test_mutation()   {
    parse_ok("x <- 1", Mutation {
        identifier: Box::new(Identifier("x")),
        value: Box::new(Number(1))});
}

#[test] fn test_function_no_args() {
    parse_ok("function f () <- 1",
             FunctionDefinition {
                 name: Box::new(Identifier("f")),
                 parameters: vec!(),
                 body: Box::new(Number(1))}); }

#[test] fn test_function_one_arg() {
    parse_ok("function f (x) <- x",
             FunctionDefinition {
                 name: Box::new(Identifier("f")),
                 parameters: vec!(Box::new(Identifier("x"))),
                 body: Box::new(Identifier("x"))});
}

#[test] fn test_function_many_args() {
    parse_ok("function f (x, y, z) <- x",
             FunctionDefinition {
                 name: Box::new(Identifier("f")),
                 parameters: vec!(Box::new(Identifier("x")),
                                  Box::new(Identifier("y")),
                                  Box::new(Identifier("z"))),
                 body: Box::new(Identifier("x"))});
}

#[test] fn test_application_no_args() {
    parse_ok("f ()",
             FunctionApplication {
                 function: Box::new(Identifier("f")),
                 arguments: vec!()});
}

#[test] fn test_application_one_arg() {
    parse_ok("f (0)",
             FunctionApplication {
                 function: Box::new(Identifier("f")),
                 arguments: vec!(Box::new(Number(0)))});
}

#[test] fn test_application_more_args() {
    parse_ok("f (1, x, true)",
             FunctionApplication {
                 function: Box::new(Identifier("f")),
                 arguments: vec!(Box::new(Number(1)),
                                 Box::new(Identifier("x")),
                                 Box::new(Boolean(true)))});
}

#[test] fn test_application_no_spaces() {
    parse_ok("f(0,-1)",
             FunctionApplication {
                 function: Box::new(Identifier("f")),
                 arguments: vec!(Box::new(Number(0)),
                                 Box::new(Number(-1)))});
}

#[test] fn test_application_more_spaces() {
    parse_ok("f    (   0    , -1 )",
             FunctionApplication {
                 function: Box::new(Identifier("f")),
                 arguments: vec!(Box::new(Number(0)),
                                 Box::new(Number(-1)))});
}

#[test] fn test_application_extra_comma() {
    parse_ok("f(0,-1,)",
             FunctionApplication {
                 function: Box::new(Identifier("f")),
                 arguments: vec!(Box::new(Number(0)),
                                 Box::new(Number(-1)))});
}

#[test] fn test_application_just_a_comma()      { parse_err("f(,)");}
#[test] fn test_application_many_extra_commas() { parse_err("f(x,,)");}

#[test] fn test_empty_block_is_unit()  { parse_ok("begin end", Unit);}
#[test] fn test_block_one_expression() { parse_ok("begin 1 end",Number(1));}

#[test] fn test_block_one_expression_and_semicolon() {
    parse_ok("begin 1; end",Number(1))
}
#[test] fn test_block_many_expressions() {
    parse_ok("begin 1; 2; 3 end",
             Block(
                 vec!(Box::new(Number(1)),
                      Box::new(Number(2)),
                      Box::new(Number(3)))))
}

#[test] fn test_block_trailing_semicolon() {
    parse_ok("begin 1; 2; 3; end",
             Block(
                 vec!(Box::new(Number(1)),
                      Box::new(Number(2)),
                      Box::new(Number(3)))))
}

#[test] fn test_loop() {
    parse_ok("while true do null",
             Loop {
                 condition: Box::new(Boolean(true)),
                 body: Box::new(Unit)})
}

#[test] fn test_conditional() {
    parse_ok("if true then false else true",
             Conditional{
                 condition: Box::new(Boolean(true)),
                 consequent: Box::new(Boolean(false)),
                 alternative: Box::new(Boolean(true))})
}

#[test] fn test_conditional_no_alternative() {
    parse_ok("if true then false",
             Conditional{
                 condition: Box::new(Boolean(true)),
                 consequent: Box::new(Boolean(false)),
                 alternative: Box::new(Unit)})
}

#[test] fn test_conditional_so_many() {
    parse_ok("if x then \
                        if y then 3 else 2 \
                    else \
                        if y then 1 else 0",
             Conditional{
                 condition: Box::new(Identifier("x")),
                 consequent: Box::new(
                     Conditional{
                         condition: Box::new(Identifier("y")),
                         consequent: Box::new(Number(3)),
                         alternative: Box::new(Number(2))}),
                 alternative: Box::new(
                     Conditional{
                         condition: Box::new(Identifier("y")),
                         consequent: Box::new(Number(1)),
                         alternative: Box::new(Number(0))})})
}

#[test]
fn test_array_definition() {
    parse_ok("array(10,0)",
             ArrayDefinition {
                 size: Box::new(Number(10)),
                 value: Box::new(Number(0))})
}

#[test]
fn test_array_definition_spaces() {
    parse_ok("array ( 10, 0 )",
             ArrayDefinition {
                 size: Box::new(Number(10)),
                 value: Box::new(Number(0))})
}

#[test]
fn test_empty_object() {
    parse_ok("object () begin end",
             ObjectDefinition {
                 extends: None,
                 parameters: vec!(),
                 members: vec!()})
}

#[test]
fn test_empty_object_with_one_parameter() {
    parse_ok("object (x) begin end",
             ObjectDefinition {
                 extends: None,
                 parameters: vec!(Box::new(Identifier("x"))),
                 members: vec!()})
}

#[test]
fn test_empty_object_with_superobject() {
    parse_ok("object (x) extends y begin end",
             ObjectDefinition {
                 extends: Some(Box::new(Identifier("y"))),
                 parameters: vec!(Box::new(Identifier("x"))),
                 members: vec!()})
}

#[test]
fn test_object_extending_expression() {
    parse_ok("object (x) extends if y then 1 else true begin end",
             ObjectDefinition {
                 extends: Some(Box::new(Conditional{
                     condition: Box::new(Identifier("y")),
                     consequent: Box::new(Number(1)),
                     alternative: Box::new(Boolean(true))})),
                 parameters: vec!(Box::new(Identifier("x"))),
                 members: vec!()})
}

#[test]
fn test_object_extending_ad_hoc_object() {
    parse_ok("object (x) extends object () begin end begin end",
             ObjectDefinition {
                 extends: Some(Box::new(ObjectDefinition {
                     extends: None,
                     parameters: vec!(),
                     members: vec!()})),
                 parameters: vec!(Box::new(Identifier("x"))),
                 members: vec!()})
}

#[test]
fn test_empty_object_with_many_parameters() {
    parse_ok("object (x, y, z) begin end",
             ObjectDefinition {
                 extends: None,
                 parameters: vec!(Box::new(Identifier("x")),
                                  Box::new(Identifier("y")),
                                  Box::new(Identifier("z"))),
                 members: vec!()})
}

#[test]
fn test_object_with_one_field() {
    parse_ok("object (x) begin let y = x; end",
             ObjectDefinition {
                 extends: None,
                 parameters: vec!(Box::new(Identifier("x"))),
                 members: vec!(Box::new(
                     Assignment {
                        identifier: Box::new(Identifier("y")),
                        value: Box::new(Identifier("x"))}))})
}

#[test]
fn test_object_with_one_method() {
    parse_ok("object (x) begin function m (x, y, z) <- y; end",
             ObjectDefinition {
                 extends: None,
                 parameters: vec!(Box::new(Identifier("x"))),
                 members: vec!(Box::new(
                     FunctionDefinition {
                        name: Box::new(Identifier("m")),
                        parameters: vec!(Box::new(Identifier("x")),
                                        Box::new(Identifier("y")),
                                          Box::new(Identifier("z"))),
                        body: Box::new(Identifier("y"))}))})
}

#[test]
fn test_object_with_an_operator() {
    parse_ok("object () begin function + (y) <- y; end",
             ObjectDefinition {
                 extends: None,
                 parameters: vec!(),
                 members: vec!(Box::new(
                     OperatorDefinition {
                         operator: Addition,
                         parameters: vec!(Box::new(Identifier("y"))),
                         body: Box::new(Identifier("y"))}))})
}

#[test]
fn test_object_with_many_members() {
    parse_ok("object (x) begin \
                    let a = x; \
                    let b = true; \
                    function m (x, y, z) <- y; \
                    function id (x) <- x; \
                    function me () <- this; \
                end",
             ObjectDefinition {
                 extends: None,
                 parameters: vec!(Box::new(Identifier("x"))),
                 members: vec!(
                     Box::new(Assignment {
                        identifier: Box::new(Identifier("a")),
                        value: Box::new(Identifier("x"))}),
                     Box::new(Assignment {
                         identifier: Box::new(Identifier("b")),
                         value: Box::new(Boolean(true))}),
                     Box::new(FunctionDefinition {
                        name: Box::new(Identifier("m")),
                        parameters: vec!(Box::new(Identifier("x")),
                                      Box::new(Identifier("y")),
                                      Box::new(Identifier("z"))),
                        body: Box::new(Identifier("y"))}),
                     Box::new(FunctionDefinition {
                         name: Box::new(Identifier("id")),
                         parameters: vec!(Box::new(Identifier("x"))),
                         body: Box::new(Identifier("x"))}),
                     Box::new(FunctionDefinition {
                         name: Box::new(Identifier("me")),
                         parameters: vec!(),
                         body: Box::new(Identifier("this"))}))})
}

#[test] fn test_field_access_from_identifier () {
    parse_ok("a.b",
             FieldAccess {
                 object: Box::new(Identifier("a")),
                 field: Box::new(Identifier("b"))});
}

#[test] fn test_field_access_from_number () {
    parse_ok("1.b",
             FieldAccess {
                 object: Box::new(Number(1)),
                 field: Box::new(Identifier("b"))});
}

#[test] fn test_field_access_from_boolean () {
    parse_ok("true.b",
             FieldAccess {
                 object: Box::new(Boolean(true)),
                 field: Box::new(Identifier("b"))});
}

#[test] fn test_field_access_from_parenthesized_expression () {
    parse_ok("(if x then 1 else 2).b",
             FieldAccess {
                 object: Box::new(
                     Conditional{
                        condition: Box::new(Identifier("x")),
                        consequent: Box::new(Number(1)),
                        alternative: Box::new(Number(2))}),
                 field: Box::new(Identifier("b"))});
}

#[test] fn test_field_chain_access () {
    parse_ok("a.b.c.d",
             FieldAccess {
                 object: Box::new(
                     FieldAccess {
                        object: Box::new(FieldAccess {
                            object: Box::new(Identifier("a")),
                            field: Box::new(Identifier("b"))}),
                        field: Box::new(Identifier("c"))}),
                 field: Box::new(Identifier("d"))});
}

#[test] fn test_field_mutation_from_identifier () {
    parse_ok("a.b <- 1",
             FieldMutation {
                 field_path: Box::new(FieldAccess {
                    object: Box::new(Identifier("a")),
                    field: Box::new(Identifier("b"))}),
                 value: Box::new(Number(1))});
}

#[test] fn test_method_call_from_identifier () {
    parse_ok("a.b (1)",
             MethodCall {
                 method_path: Box::new(FieldAccess {
                     object: Box::new(Identifier("a")),
                     field: Box::new(Identifier("b"))}),
                 arguments: vec!(Box::new(Number(1)))});
}

#[test] fn test_array_access () {
    parse_ok("a[1]",
             ArrayAccess {
                 array: Box::new(Identifier("a")),
                 index: Box::new(Number(1))});
}

#[test] fn test_array_access_from_object () {
    parse_ok("a.b[1]",
             ArrayAccess {
                 array: Box::new(
                     FieldAccess {
                         object: Box::new(Identifier("a")),
                         field: Box::new(Identifier("b"))}),
                 index: Box::new(Number(1))});
}

#[test] fn test_array_access_from_array () {
    parse_ok("a[b][1]",
             ArrayAccess {
                 array: Box::new(
                     ArrayAccess {
                         array: Box::new(Identifier("a")),
                         index: Box::new(Identifier("b"))}),
                 index: Box::new(Number(1))});
}

#[test] fn test_array_call_method_on_member () {
    parse_ok("a[b](1)",
              {
                  MethodCall {
                     method_path: Box::new(
                         ArrayAccess {
                             array: Box::new(Identifier("a")),
                             index: Box::new(Identifier("b"))}),
                     arguments: vec!(Box::new(Number(1)))}});
}

#[test] fn test_array_access_member_of_member () {
    parse_ok("a[b].a",
             {
                 FieldAccess {
                     object: Box::new(
                         ArrayAccess {
                             array: Box::new(Identifier("a")),
                             index: Box::new(Identifier("b"))}),
                     field: Box::new(Identifier("a"))}});
}

#[test] fn test_array_access_with_array_access_as_index () {
    parse_ok("a[b[c]]",
             ArrayAccess {
                 array: Box::new(Identifier("a")),
                 index: Box::new(
                     ArrayAccess {
                         array: Box::new(Identifier("b")),
                         index: Box::new(Identifier("c"))})});
}

#[test] fn test_array_access_from_function_call () {
    parse_ok("f(0,0)[x]",
             ArrayAccess {
                 array: Box::new(
                     FunctionApplication {
                        function: Box::new(Identifier("f")),
                        arguments: vec!(Box::new(Number(0)),
                                         Box::new(Number(0)))}),
                 index: Box::new(
                     Identifier("x"))});
}

#[test] fn test_print_call_with_arguments() {
    parse_ok("print(\"~ ~ ~\", 1, true, x)",
             Print {
                 format: Box::new(StringLiteral("~ ~ ~")),
                 arguments: vec!(
                     Box::new(Number(1)),
                     Box::new(Boolean(true)),
                     Box::new(Identifier("x")))});
}

#[test] fn test_print_call_without_arguments() {
    parse_ok("print(\"~ ~ ~\")",
             Print {
                 format: Box::new(StringLiteral("~ ~ ~")),
                 arguments: vec!()});
}

#[test] fn test_print_call_string() {
        parse_ok("print(\"hello world\")",
                 Print {
                     format: Box::new(StringLiteral("hello world")),
                     arguments: vec!()});
}

#[test] fn test_print_call_empty_string() {
        parse_ok("print(\"\")",
                 Print {
                     format: Box::new(StringLiteral("")),
                     arguments: vec!()});
}

#[test] fn test_print_call_escape_newline() {
        parse_ok("print(\"\\n\")",
                 Print {
                     format: Box::new(StringLiteral("\\n")),
                     arguments: vec!()});
}

#[test] fn test_print_call_escape_tab() {
        parse_ok("print(\"\\t\")",
                 Print {
                     format: Box::new(StringLiteral("\\t")),
                     arguments: vec!()});
}

#[test] fn test_print_call_escape_backspace() {
        parse_ok("print(\"\\b\")",
                 Print {
                     format: Box::new(StringLiteral("\\b")),
                     arguments: vec!()});
}

#[test] fn test_print_call_escape_return() {
    parse_ok("print(\"\\r\")",
             Print {
                 format: Box::new(StringLiteral("\\r")),
                 arguments: vec!()});
}

#[test] fn test_print_call_escape_backslash() {
    parse_ok("print(\"\\\\\")",
             Print {
                 format: Box::new(StringLiteral("\\\\")),
                 arguments: vec!()});
}

#[test] fn test_print_call_botched_escape() { parse_err("print(\"\\\")");  }
#[test] fn test_print_call_invalid_escape() { parse_err("print(\"\\a\")"); }


#[test] fn test_simple_disjunction() {
    parse_ok("true | false",
             Operation {
                 operator: Disjunction,
                 left: Box::new(Boolean(true)),
                 right: Box::new(Boolean(false))});
}

#[test] fn test_double_disjunction() {
    parse_ok("true | false | true",
             Operation {
                 operator: Disjunction,
                 left: Box::new(
                     Operation {
                         operator: Disjunction,
                         left: Box::new(Boolean(true)),
                         right: Box::new(Boolean(false))}),
                 right: Box::new(Boolean(true))});
}

#[test] fn test_simple_conjunction() {
    parse_ok("true & false",
             Operation {
                 operator: Conjunction,
                 left: Box::new(Boolean(true)),
                 right: Box::new(Boolean(false))});
}

#[test] fn test_double_conjunction() {
    parse_ok("true & false & true",
             Operation {
                 operator: Conjunction,
                 left: Box::new(
                     Operation {
                         operator: Conjunction,
                         left: Box::new(Boolean(true)),
                         right: Box::new(Boolean(false))}),
                 right: Box::new(Boolean(true))});
}

#[test] fn test_simple_equality() {
    parse_ok("true == false",
             Operation {
                 operator: Equality,
                 left: Box::new(Boolean(true)),
                 right: Box::new(Boolean(false))});
}


#[test] fn test_simple_inequality() {
    parse_ok("true != false",
             Operation {
                 operator: Inequality,
                 left: Box::new(Boolean(true)),
                 right: Box::new(Boolean(false))});
}

#[test] fn test_disjunction_and_conjunction() {
    //or (true, (true & false & false)))
    parse_ok("true | true & false",
             Operation {
                 operator: Disjunction,
                 left: Box::new(Boolean(true)),
                 right: Box::new(Operation {
                     operator: Conjunction,
                     left: Box::new(Boolean(true)),
                     right: Box::new(Boolean(false))
                 })
             });
}

#[test] fn test_disjunction_and_conjunctions() {
    //or (true, (true & false & false)))
    parse_ok("true & false | true & false",
             Operation {
                 operator: Disjunction,
                 left: Box::new(Operation {
                     operator: Conjunction,
                     left: Box::new(Boolean(true)),
                     right: Box::new(Boolean(false))
                 }),
                 right: Box::new(Operation {
                     operator: Conjunction,
                     left: Box::new(Boolean(true)),
                     right: Box::new(Boolean(false))
                 })
             });
}

#[test] fn test_disjunctions_and_conjunctions() {
    //or (true, (true & false & false)))
    parse_ok("true & false | true & false | true & false",
             Operation {
                 operator: Disjunction,
                 left: Box::new(Operation {
                     operator: Disjunction,
                     left: Box::new(Operation {
                         operator: Conjunction,
                         left: Box::new(Boolean(true)),
                         right: Box::new(Boolean(false))
                     }),
                     right: Box::new(Operation {
                         operator: Conjunction,
                         left: Box::new(Boolean(true)),
                         right: Box::new(Boolean(false))
                     })
                 }),
                 right: Box::new(Operation {
                     operator: Conjunction,
                     left: Box::new(Boolean(true)),
                     right: Box::new(Boolean(false))
                 })});
}

#[test] fn test_more_disjunctions_and_more_conjunctions() {
    //or (true, (true & false & false)))
    parse_ok("true & false & true | true & true & false & true | true & false",
             Operation {
                 operator: Disjunction,
                 left: Box::new(Operation {
                     operator: Disjunction,
                     left: Box::new(Operation {
                         operator: Conjunction,
                         left: Box::new(Operation {
                             operator: Conjunction,
                             left: Box::new(Boolean(true)),
                             right: Box::new(Boolean(false))
                         }),
                         right: Box::new(Boolean(true))
                     }),
                     right: Box::new(Operation {
                         operator: Conjunction,
                         left: Box::new(Operation {
                             operator: Conjunction,
                             left: Box::new(Operation {
                                 operator: Conjunction,
                                 left: Box::new(Boolean(true)),
                                 right: Box::new(Boolean(true))
                             }),
                             right: Box::new(Boolean(false))
                         }),
                         right: Box::new(Boolean(true))
                     })
                 }),
                 right: Box::new(Operation {
                     operator: Conjunction,
                     left: Box::new(Boolean(true)),
                     right: Box::new(Boolean(false))
                 })});
}

#[test] fn test_simple_addition() {
    parse_ok("1 + 2",
             Operation {
                 operator: Addition,
                 left: Box::new(Number(1)),
                 right: Box::new(Number(2))});
}

#[test] fn test_simple_subtraction() {
    parse_ok("1 - 2",
             Operation {
                 operator: Subtraction,
                 left: Box::new(Number(1)),
                 right: Box::new(Number(2))});
}

#[test] fn test_simple_multiplication() {
    parse_ok("1 * 2",
             Operation {
                 operator: Multiplication,
                 left: Box::new(Number(1)),
                 right: Box::new(Number(2))});
}

#[test] fn test_simple_division() {
    parse_ok("1 / 2",
             Operation {
                 operator: Division,
                 left: Box::new(Number(1)),
                 right: Box::new(Number(2))});
}

#[test] fn test_simple_less_than() {
    parse_ok("1 < 2",
             Operation {
                 operator: Less,
                 left: Box::new(Number(1)),
                 right: Box::new(Number(2))});
}

#[test] fn test_simple_less_or_equal() {
    parse_ok("1 <= 2",
             Operation {
                 operator: LessEqual,
                 left: Box::new(Number(1)),
                 right: Box::new(Number(2))});
}

#[test] fn test_simple_greater_than() {
    parse_ok("1 > 2",
             Operation {
                 operator: Greater,
                 left: Box::new(Number(1)),
                 right: Box::new(Number(2))});
}

#[test] fn test_simple_greater_or_equal() {
    parse_ok("1 >= 2",
             Operation {
                 operator: GreaterEqual,
                 left: Box::new(Number(1)),
                 right: Box::new(Number(2))});
}

#[cfg(not(test))]
fn main() {
    println!("cargo test");
}
