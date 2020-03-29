use std::fmt::Debug;
use std::cmp::PartialEq;
use serde::{Serialize, Deserialize};

pub trait Portable {
  fn to_string(&self) -> String;
}

#[derive(PartialEq,Debug,Serialize,Deserialize,Clone)]
pub enum AST {
    String(String),
    Number(i32),
    Boolean(bool),
    Unit,
    Identifier(String),

    LocalDefinition { identifier: Box<AST>, value: Box<AST> },
    ArrayDefinition { size: Box<AST>, value: Box<AST> },
    ObjectDefinition { extends: Option<Box<AST>>, parameters: Vec<Box<AST>>, members: Vec<Box<AST>> },

    LocalMutation { identifier: Box<AST>, value: Box<AST> },
    FieldMutation { field_path: Box<AST>, value: Box<AST> },
    ArrayMutation { array: Box<AST>, value: Box<AST> },

    FunctionDefinition { name: Box<AST>, parameters: Vec<Box<AST>>, body: Box<AST> },
    OperatorDefinition { operator: Operator, parameters: Vec<Box<AST>>, body: Box<AST> },

    FunctionApplication { function: Box<AST>, arguments: Vec<Box<AST>> },
    MethodCall { method_path: Box<AST>, arguments: Vec<Box<AST>> },
    Print { format: Box<AST>, arguments: Vec<Box<AST>> },

    FieldAccess { object: Box<AST>, field: Box<AST> },
    OperatorAccess { object: Box<AST>, operator: Operator },
    ArrayAccess { array: Box<AST>, index: Box<AST> },

    Block (Vec<Box<AST>>),
    Operation { operator: Operator, left: Box<AST>, right: Box<AST> },
    Loop { condition: Box<AST>, body: Box<AST> },
    Conditional { condition: Box<AST>, consequent: Box<AST>, alternative: Box<AST> },
}

#[derive(PartialEq,Debug,Copy,Clone,Serialize,Deserialize)]
pub enum Operator {
    Multiplication,
    Division,
    Module,
    Addition,
    Subtraction,
    Inequality,
    Equality,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Disjunction,
    Conjunction,
}

#[macro_export]
macro_rules! make_operator_ast {
    ( $head:expr, $tail:expr ) => {
        ($tail).into_iter().fold($head, |left, right| {
            let (operator, value) = right;
            AST::Operation {
                operator: operator,
                left: Box::new(left),
                right: Box::new(value)}
        })
    }
}

#[macro_export]
macro_rules! put_into_boxes {
    ( $collection:expr ) => {
        ($collection).into_iter().map(|e| Box::new(e)).collect();
    }
}

#[macro_export]
macro_rules! option_into_box {
    ( $option:expr ) => {
        match $option {
            Some(value) => Some(Box::new(value)),
            None => None
        }
    }
}