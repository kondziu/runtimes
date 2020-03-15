#[macro_use]
//
//use std::fmt::{Debug};
//use std::cmp::PartialEq;

extern crate lalrpop_util;
//
pub mod ast;

lalrpop_mod!(pub macro_solution); // syntesized by LALRPOP

use crate::macro_solution::TopLevelParser;

use crate::ast::AST;
use crate::ast::AST::*;

fn leaf__(s: &str) -> Box<AST> {
    Box::new(AST::Leaf(s))
}

fn parent__<'x>(s: &'x str, v: Vec<Box<AST<'x>>>) -> Box<AST<'x>> {
    Box::new(AST::Parent(s, v))
}

#[test]
fn test1 () {
    println!("{:?}", TopLevelParser::new().parse("1"));
    assert!(TopLevelParser::new().parse("1").is_ok());
}

#[test]
fn test2 () {
    println!("{:?}", TopLevelParser::new().parse("a"));
    assert!(TopLevelParser::new().parse("a").is_ok());
}

#[test]
fn test3 () {
    println!("{:?}", TopLevelParser::new().parse("true"));
    assert!(TopLevelParser::new().parse("true").is_ok());
}

#[test]
fn test4 () {
    println!("{:?}", TopLevelParser::new().parse("if true then 1 else 2"));
    assert!(TopLevelParser::new().parse("if true then 1 else 2").is_ok());
}

#[test]
fn test5 () {
    println!("{:?}", TopLevelParser::new().parse("if true then if false then 3 else 4 else 2"));
    assert!(TopLevelParser::new().parse("if true then if false then 3 else 4 else 2").is_ok());
}

fn main() {
    println!("{:?}", TopLevelParser::new().parse("1"));
}
