#[macro_use]
extern crate lalrpop_util;
extern crate unescape;

pub mod fml_ast;
//pub mod tools;

lalrpop_mod!(pub fml); // syntesized by LALRPOP

use crate::fml::ExpressionParser;
use crate::fml_ast::AST::Number;
use crate::fml_ast::AST::Identifier;
use crate::fml_ast::AST::StringLiteral;
use crate::fml_ast::AST::BooleanLiteral;

#[test]
fn numbers() {
    assert_eq!(ExpressionParser::new().parse("0"), Ok(Number(0)));
    assert_eq!(ExpressionParser::new().parse("-0"), Ok(Number(0)));
    assert_eq!(ExpressionParser::new().parse("2"), Ok(Number(2)));
    assert_eq!(ExpressionParser::new().parse("-2"), Ok(Number(-2)));
    assert_eq!(ExpressionParser::new().parse("42"), Ok(Number(42)));
    assert_eq!(ExpressionParser::new().parse("042"), Ok(Number(42)));
    assert_eq!(ExpressionParser::new().parse("00"), Ok(Number(0)));
    assert_eq!(ExpressionParser::new().parse("-042"), Ok(Number(-42)));
    assert_eq!(ExpressionParser::new().parse("-00"), Ok(Number(0)));
}

#[test]
fn identifiers() {
    assert_eq!(ExpressionParser::new().parse("_"), Ok(Identifier("_")));
    assert_eq!(ExpressionParser::new().parse("_x"), Ok(Identifier("_x")));
    assert_eq!(ExpressionParser::new().parse("x"), Ok(Identifier("x")));
    assert_eq!(ExpressionParser::new().parse("x1"), Ok(Identifier("x1")));
    assert_eq!(ExpressionParser::new().parse("spaceship"), Ok(Identifier("spaceship")));
    assert_eq!(ExpressionParser::new().parse("___"), Ok(Identifier("___")));
}

#[test]
fn string_literals() {
    assert_eq!(ExpressionParser::new().parse("'hello world'"), Ok(StringLiteral("hello world")));
    assert_eq!(ExpressionParser::new().parse("''"), Ok(StringLiteral("")));
    assert_eq!(ExpressionParser::new().parse("'\\n'"), Ok(StringLiteral("\\n")));
    assert_eq!(ExpressionParser::new().parse("'\\t'"), Ok(StringLiteral("\\t")));
    assert_eq!(ExpressionParser::new().parse("'\\b'"), Ok(StringLiteral("\\b")));
    assert_eq!(ExpressionParser::new().parse("'\\r'"), Ok(StringLiteral("\\r")));
    assert_eq!(ExpressionParser::new().parse("'\\\\'"), Ok(StringLiteral("\\\\")));
    assert!(ExpressionParser::new().parse("'\\'").is_err());
    assert!(ExpressionParser::new().parse("'\\a'").is_err());
}

#[test]
fn boolean_literals() {
    assert_eq!(ExpressionParser::new().parse("true"), Ok(BooleanLiteral(true)));
    assert_eq!(ExpressionParser::new().parse("false"), Ok(BooleanLiteral(false)));
}

#[cfg(not(test))]
fn main() {
    println!("cargo test");
}
