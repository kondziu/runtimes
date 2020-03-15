use std::fmt::{Debug};
use std::cmp::PartialEq;

#[derive(PartialEq, Debug)]
pub enum AST<'ast> {
    Leaf(&'ast str),
    Parent(&'ast str, Vec<Box<AST<'ast>>>)
}