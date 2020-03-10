#[macro_use]
extern crate lalrpop_util;

pub mod fml_ast;
lalrpop_mod!(pub fml); // syntesized by LALRPOP

use crate::fml::ExpressionParser;
use crate::fml_ast::AST::Number;
use crate::fml_ast::AST::Identifier;

#[test]
fn numbers() {
    assert_eq!(ExpressionParser::new().parse("0"),
               Ok(Number(0)));
    assert_eq!(ExpressionParser::new().parse("-0"),
               Ok(Number(0)));
    assert_eq!(ExpressionParser::new().parse("2"),
               Ok(Number(2)));
    assert_eq!(ExpressionParser::new().parse("-2"),
               Ok(Number(-2)));
    assert_eq!(ExpressionParser::new().parse("42"),
               Ok(Number(42)));
    assert_eq!(ExpressionParser::new().parse("042"),
               Ok(Number(42)));
    assert_eq!(ExpressionParser::new().parse("00"),
               Ok(Number(0)));
    assert_eq!(ExpressionParser::new().parse("-042"),
               Ok(Number(-42)));
    assert_eq!(ExpressionParser::new().parse("-00"),
               Ok(Number(0)));
}

#[test]
fn identifiers() {
    assert_eq!(ExpressionParser::new().parse("_"),
               Ok(Identifier("_")));
    assert_eq!(ExpressionParser::new().parse("_x"),
               Ok(Identifier("_x")));
    assert_eq!(ExpressionParser::new().parse("x"),
               Ok(Identifier("x")));
    assert_eq!(ExpressionParser::new().parse("x1"),
               Ok(Identifier("x1")));
    assert_eq!(ExpressionParser::new().parse("spaceship"),
               Ok(Identifier("spaceship")));
    assert_eq!(ExpressionParser::new().parse("___"),
               Ok(Identifier("___")));
}

#[cfg(not(test))]
fn main() {
    println!("cargo test");
}
