#[macro_use]
extern crate lalrpop_util;

pub mod fml_ast;
lalrpop_mod!(pub fml); // syntesized by LALRPOP

use crate::fml::ExpressionParser;
use crate::fml_ast::AST::Number;

#[test]
 fn number_0() {
    assert_eq!(ExpressionParser::new().parse("0"),
               Ok(Box::new(Number(0))));
}

#[test]
fn numer_() {
    assert_eq!(ExpressionParser::new().parse("0"),
               Ok(Box::new(Number(0))));
    assert_eq!(ExpressionParser::new().parse("-0"),
               Ok(Box::new(Number(0))));
    assert_eq!(ExpressionParser::new().parse("2"),
               Ok(Box::new(Number(2))));
    assert_eq!(ExpressionParser::new().parse("-2"),
               Ok(Box::new(Number(-2))));
    assert_eq!(ExpressionParser::new().parse("42"),
               Ok(Box::new(Number(42))));
    assert_eq!(ExpressionParser::new().parse("042"),
               Ok(Box::new(Number(42))));
    assert_eq!(ExpressionParser::new().parse("00"),
               Ok(Box::new(Number(0))));
    assert_eq!(ExpressionParser::new().parse("-042"),
               Ok(Box::new(Number(-42))));
    assert_eq!(ExpressionParser::new().parse("-00"),
               Ok(Box::new(Number(0))));
}

#[test]
fn identifiers() {
    assert!(ExpressionParser::new().parse("_").is_ok());
    assert!(ExpressionParser::new().parse("_0").is_ok());
    assert!(ExpressionParser::new().parse("_x").is_ok());
    assert!(ExpressionParser::new().parse("x").is_ok());
    assert!(ExpressionParser::new().parse("x1").is_ok());
    assert!(ExpressionParser::new().parse("spaceship").is_ok());
    assert!(ExpressionParser::new().parse("___").is_ok());
}

#[cfg(not(test))]
fn main() {
    println!("cargo test");
}
