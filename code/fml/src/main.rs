#[macro_use]
extern crate lalrpop_util;
extern crate unescape;

pub mod fml_ast;
//pub mod tools;

lalrpop_mod!(pub fml); // syntesized by LALRPOP

use crate::fml::TopLevelParser;
use crate::fml_ast::AST;
use crate::fml_ast::AST::*;

fn parse_ok(input: &str, correct: AST) {
    assert_eq!(TopLevelParser::new().parse(input), Ok(correct));
}

fn parse_err(input: &str) {
    assert!(TopLevelParser::new().parse(input).is_err());
}

#[test] fn test_unit()         { parse_ok("null", Unit);        }

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

#[test] fn test_string()           { parse_ok("\"hello world\"", StringLiteral("hello world")); }
#[test] fn test_empty_string()     { parse_ok("\"\"",            StringLiteral(""));     }
#[test] fn test_escape_newline()   { parse_ok("\"\\n\"",         StringLiteral("\\n"));  }
#[test] fn test_escape_tab()       { parse_ok("\"\\t\"",         StringLiteral("\\t"));  }
#[test] fn test_escape_backspace() { parse_ok("\"\\b\"",         StringLiteral("\\b"));  }
#[test] fn test_escape_return()    { parse_ok("\"\\r\"",         StringLiteral("\\r"));  }
#[test] fn test_escape_backslash() { parse_ok("\"\\\\\"",        StringLiteral("\\\\")); }
#[test] fn test_botched_escape()   { parse_err("\"\\\"");  }
#[test] fn test_invalid_escape()   { parse_err("\"\\a\""); }

#[test] fn test_true()  { parse_ok("true",  BooleanLiteral(true));  }
#[test] fn test_false() { parse_ok("false", BooleanLiteral(false)); }

#[test] fn test_number_in_parens() { parse_ok("(1)", Number(1)); }
#[test] fn test_number_in_two_parens() { parse_ok("((1))", Number(1)); }
#[test] fn test_number_parens_with_whitespace() { parse_ok("( 1 )", Number(1)); }

#[test] fn test_assignment() { parse_ok("let x = 1",
                                        Assignment { identifier: Box::new(Identifier("x")),
                                                            value: Box::new(Number(1))}); }
#[test] fn test_mutation()   { parse_ok("x <- 1",
                                        Mutation   { identifier: Box::new(Identifier("x")),
                                                            value: Box::new(Number(1))}); }

#[test] fn test_function_no_args() {
    parse_ok("function f () <- 1",
             FunctionDefinition {
                 identifier: Box::new(Identifier("f")),
                 parameters: vec!(),
                 body: Box::new(Number(1))}); }

#[test] fn test_function_one_arg() {
    parse_ok("function f (x) <- x",
             FunctionDefinition {
                 identifier: Box::new(Identifier("f")),
                 parameters: vec!(Box::new(Identifier("x"))),
                 body: Box::new(Identifier("x"))});
}

#[test] fn test_function_many_args() {
    parse_ok("function f (x, y, z) <- x",
             FunctionDefinition {
                 identifier: Box::new(Identifier("f")),
                 parameters: vec!(Box::new(Identifier("x")),
                                  Box::new(Identifier("y")),
                                  Box::new(Identifier("z"))),
                 body: Box::new(Identifier("x"))});
}

#[test] fn test_application_no_args() {
    parse_ok("f ()",
             FunctionApplication {
                 identifier: Box::new(Identifier("f")),
                 arguments: vec!()});
}

#[test] fn test_application_one_arg() {
    parse_ok("f (0)",
             FunctionApplication {
                 identifier: Box::new(Identifier("f")),
                 arguments: vec!(Box::new(Number(0)))});
}

#[test] fn test_application_more_args() {
    parse_ok("f (1, x, true)",
             FunctionApplication {
                 identifier: Box::new(Identifier("f")),
                 arguments: vec!(Box::new(Number(1)),
                                 Box::new(Identifier("x")),
                                 Box::new(BooleanLiteral(true)))});
}

#[test] fn test_application_no_spaces() {
    parse_ok("f(0,-1)",
             FunctionApplication {
                 identifier: Box::new(Identifier("f")),
                 arguments: vec!(Box::new(Number(0)),
                                 Box::new(Number(-1)))});
}

#[test] fn test_application_more_spaces() {
    parse_ok("f    (   0    , -1 )",
             FunctionApplication {
                 identifier: Box::new(Identifier("f")),
                 arguments: vec!(Box::new(Number(0)),
                                 Box::new(Number(-1)))});
}

#[test] fn test_application_extra_comma() {
    parse_ok("f(0,-1,)",
             FunctionApplication {
                 identifier: Box::new(Identifier("f")),
                 arguments: vec!(Box::new(Number(0)),
                                 Box::new(Number(-1)))});
}

#[test] fn test_application_just_a_comma()      { parse_err("f(,)");}
#[test] fn test_application_many_extra_commas() { parse_err("f(x,,)");}

#[test] fn test_empty_block_is_unit() { parse_ok("begin end", Unit) }
#[test] fn test_block_one_expression() { parse_ok("begin 1 end",
                                                  Block(vec!(Box::new(Number(1))))) }
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
                 condition: Box::new(BooleanLiteral(true)),
                 body: Box::new(Unit)})
}

#[test] fn test_conditional() {
    parse_ok("if true then false else true",
             Conditional{
                 condition: Box::new(BooleanLiteral(true)),
                 consequent: Box::new(BooleanLiteral(false)),
                 alternative: Box::new(BooleanLiteral(true))})
}

#[test] fn test_conditional_no_alternative() {
    parse_ok("if true then false",
             Conditional{
                 condition: Box::new(BooleanLiteral(true)),
                 consequent: Box::new(BooleanLiteral(false)),
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
                 parameters: vec!(),
                 members: vec!()})
}

#[test]
fn test_empty_object_with_one_parameter() {
    parse_ok("object (x) begin end",
             ObjectDefinition {
                 parameters: vec!(Box::new(Identifier("x"))),
                 members: vec!()})
}

#[test]
fn test_empty_object_with_many_parameters() {
    parse_ok("object (x, y, z) begin end",
             ObjectDefinition {
                 parameters: vec!(Box::new(Identifier("x")),
                                  Box::new(Identifier("y")),
                                  Box::new(Identifier("z"))),
                 members: vec!()})
}

#[test]
fn test_object_with_one_field() {
    parse_ok("object (x) begin let y = x; end",
             ObjectDefinition {
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
                 parameters: vec!(Box::new(Identifier("x"))),
                 members: vec!(Box::new(
                     FunctionDefinition {
                        identifier: Box::new(Identifier("m")),
                        parameters: vec!(Box::new(Identifier("x")),
                                        Box::new(Identifier("y")),
                                          Box::new(Identifier("z"))),
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
                 parameters: vec!(Box::new(Identifier("x"))),
                 members: vec!(
                     Box::new(Assignment {
                        identifier: Box::new(Identifier("a")),
                        value: Box::new(Identifier("x"))}),
                     Box::new(Assignment {
                         identifier: Box::new(Identifier("b")),
                         value: Box::new(BooleanLiteral(true))}),
                     Box::new(FunctionDefinition {
                        identifier: Box::new(Identifier("m")),
                        parameters: vec!(Box::new(Identifier("x")),
                                      Box::new(Identifier("y")),
                                      Box::new(Identifier("z"))),
                        body: Box::new(Identifier("y"))}),
                     Box::new(FunctionDefinition {
                         identifier: Box::new(Identifier("id")),
                         parameters: vec!(Box::new(Identifier("x"))),
                         body: Box::new(Identifier("x"))}),
                     Box::new(FunctionDefinition {
                         identifier: Box::new(Identifier("me")),
                         parameters: vec!(),
                         body: Box::new(Identifier("this"))}))})
}

#[test] fn test_field_access_from_identifier () {
    parse_ok("a.b",
             FieldAccess {
                 object: Box::new(Identifier("a")),
                 identifier: Box::new(Identifier("b"))});
}

#[test] fn test_field_access_from_number () {
    parse_ok("1.b",
             FieldAccess {
                 object: Box::new(Number(1)),
                 identifier: Box::new(Identifier("b"))});
}

#[test] fn test_field_access_from_boolean () {
    parse_ok("true.b",
             FieldAccess {
                 object: Box::new(BooleanLiteral(true)),
                 identifier: Box::new(Identifier("b"))});
}

#[test] fn test_field_access_from_parenthesized_expression () {
    parse_ok("(if x then 1 else 2).b",
             FieldAccess {
                 object: Box::new(
                     Conditional{
                        condition: Box::new(Identifier("x")),
                        consequent: Box::new(Number(1)),
                        alternative: Box::new(Number(2))}),
                 identifier: Box::new(Identifier("b"))});
}

#[test] fn test_field_chain_access () {
    parse_ok("a.b.c.d",
             FieldAccess {
                 object: Box::new(
                     FieldAccess {
                        object: Box::new(FieldAccess {
                            object: Box::new(Identifier("a")),
                            identifier: Box::new(Identifier("b"))}),
                        identifier: Box::new(Identifier("c"))}),
                 identifier: Box::new(Identifier("d"))});
}

#[cfg(not(test))]
fn main() {
    println!("cargo test");
}
