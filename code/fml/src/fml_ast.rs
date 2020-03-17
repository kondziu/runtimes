use std::fmt::{Debug, Error, Formatter};
use std::cmp::PartialEq;

#[derive(PartialEq,Debug)]
pub enum AST<'ast> {
    Unit,

    Number(i32),
    Identifier(&'ast str),
    StringLiteral(&'ast str),
    Boolean(bool),

    LocalDefinition { identifier: Box<AST<'ast>>, value: Box<AST<'ast>>},
    ArrayDefinition { size: Box<AST<'ast>>, value: Box<AST<'ast>>},
    ObjectDefinition { extends: Option<Box<AST<'ast>>>, parameters: Vec<Box<AST<'ast>>>, members: Vec<Box<AST<'ast>>>},

    LocalMutation { identifier: Box<AST<'ast>>, value: Box<AST<'ast>> },
    FieldMutation { field_path: Box<AST<'ast>>, value: Box<AST<'ast>> },
    ArrayMutation { array: Box<AST<'ast>>, value: Box<AST<'ast>> },

    FunctionDefinition { name: Box<AST<'ast>>, parameters: Vec<Box<AST<'ast>>>, body: Box<AST<'ast>> },
    OperatorDefinition { operator: Operator, parameters: Vec<Box<AST<'ast>>>, body: Box<AST<'ast>> },

    FunctionApplication { function: Box<AST<'ast>>, arguments: Vec<Box<AST<'ast>>> },
    MethodCall { method_path: Box<AST<'ast>>, arguments: Vec<Box<AST<'ast>>> },
    Print { format: Box<AST<'ast>>, arguments: Vec<Box<AST<'ast>>> },

    FieldAccess { object: Box<AST<'ast>>, field: Box<AST<'ast>> },
    OperatorAccess { object: Box<AST<'ast>>, operator: Operator },
    ArrayAccess { array: Box<AST<'ast>>, index: Box<AST<'ast>> },

    Block (Vec<Box<AST<'ast>>>),
    Operation { operator: Operator, left: Box<AST<'ast>>, right: Box<AST<'ast>> },
    Loop { condition: Box<AST<'ast>>, body: Box<AST<'ast>> },
    Conditional { condition: Box<AST<'ast>>, consequent: Box<AST<'ast>>, alternative: Box<AST<'ast>> },
}

#[derive(PartialEq,Debug,Copy,Clone)]
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
    Disjunction, // |
    Conjunction, // &
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