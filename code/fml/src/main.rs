#[macro_use] extern crate lalrpop_util;

pub mod ast;

lalrpop_mod!(pub fml); // syntesized by LALRPOP

#[test]
fn numbers() {
    assert!(fml::ExpressionParser::new().parse("0").is_ok());
    assert!(fml::ExpressionParser::new().parse("-0").is_ok());
    assert!(fml::ExpressionParser::new().parse("2").is_ok());
    assert!(fml::ExpressionParser::new().parse("-2").is_ok());
    assert!(fml::ExpressionParser::new().parse("42").is_ok());
    assert!(fml::ExpressionParser::new().parse("0100").is_ok());
    assert!(fml::ExpressionParser::new().parse("0000").is_ok());
}

#[cfg(not(test))]
fn main() {
    println!("cargo test");
}
