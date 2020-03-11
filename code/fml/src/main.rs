#[macro_use]
extern crate lalrpop_util;
extern crate unescape;

pub mod fml_ast;
//pub mod tools;

lalrpop_mod!(pub fml); // syntesized by LALRPOP

use crate::fml::ExpressionParser;
use crate::fml_ast::AST;
use crate::fml_ast::AST::{Number, FunctionDefinition, FunctionApplication};
use crate::fml_ast::AST::Identifier;
use crate::fml_ast::AST::StringLiteral;
use crate::fml_ast::AST::BooleanLiteral;
use crate::fml_ast::AST::Assignment;
use crate::fml_ast::AST::Mutation;

fn parse_ok(input: &str, correct: AST) {
    assert_eq!(ExpressionParser::new().parse(input), Ok(correct));
}

fn parse_err(input: &str) {
    assert!(ExpressionParser::new().parse(input).is_err());
}

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

#[test] fn test_function_no_args() { parse_ok("function f () <- 1",
                                              FunctionDefinition {
                                                  identifier: Box::new(Identifier("f")),
                                                  parameters: vec!(),
                                                  body: Box::new(Number(1))}); }

#[test] fn test_function_one_arg() { parse_ok("function f (x) <- x",
                                              FunctionDefinition {
                                                  identifier: Box::new(Identifier("f")),
                                                  parameters: vec!(Box::new(Identifier("x"))),
                                                  body: Box::new(Identifier("x"))}); }

#[test] fn test_function_many_args() { parse_ok("function f (x, y, z) <- x",
                                                FunctionDefinition {
                                                    identifier: Box::new(Identifier("f")),
                                                    parameters: vec!(Box::new(Identifier("x")),
                                                                     Box::new(Identifier("y")),
                                                                     Box::new(Identifier("z"))),
                                                    body: Box::new(Identifier("x"))}); }

#[test] fn test_application_no_args() { parse_ok("f ()",
                                                 FunctionApplication {
                                                     identifier: Box::new(Identifier("f")),
                                                     arguments: vec!()}); }

#[test] fn test_application_one_arg() { parse_ok("f (0)",
                                                 FunctionApplication {
                                                     identifier: Box::new(Identifier("f")),
                                                     arguments: vec!(Box::new(Number(0)))}); }

#[test] fn test_application_more_args() { parse_ok("f (1, x, true)",
                                                   FunctionApplication {
                                                       identifier: Box::new(Identifier("f")),
                                                       arguments: vec!(Box::new(Number(1)),
                                                                       Box::new(Identifier("x")),
                                                                       Box::new(BooleanLiteral(true)))}); }

#[test] fn test_application_no_spaces() { parse_ok("f(0,-1)",
                                                   FunctionApplication {
                                                       identifier: Box::new(Identifier("f")),
                                                       arguments: vec!(Box::new(Number(0)),
                                                                       Box::new(Number(-1)))}); }

#[test] fn test_application_more_spaces() { parse_ok("f    (   0    , -1 )",
                                                     FunctionApplication {
                                                         identifier: Box::new(Identifier("f")),
                                                         arguments: vec!(Box::new(Number(0)),
                                                                         Box::new(Number(-1)))}); }

#[cfg(not(test))]
fn main() {
    println!("cargo test");
}
