use std::fmt::Debug;
use std::cmp::PartialEq;
use serde::{Serialize, Deserialize};

pub trait Portable {
    fn to_string(&self) -> String;
}

#[derive(PartialEq,Debug,Serialize,Deserialize,Clone)]
pub enum AST {
    Number(i32),
    Boolean(bool),
    Unit,

    VariableDefinition { name: Identifier, value: Box<AST> },
    ArrayDefinition { size: Box<AST>, value: Box<AST> },
    ObjectDefinition { extends: Option<Box<AST>>, members: Vec<Box<AST>> },

    VariableAccess { name: Identifier },
    FieldAccess { object: Box<AST>, field: Identifier },
    ArrayAccess { array: Box<AST>, index: Box<AST> },

    VariableMutation { name: Identifier, value: Box<AST> },
    FieldMutation { object: Box<AST>, field: Identifier, value: Box<AST> },
    ArrayMutation { array: Box<AST>, index: Box<AST>, value: Box<AST> },

    FunctionDefinition { function: Identifier, parameters: Vec<Identifier>, body: Box<AST> },
    OperatorDefinition { operator: Operator, parameters: Vec<Identifier>, body: Box<AST> },

    FunctionCall { function: Identifier, arguments: Vec<Box<AST>> },
    MethodCall { object: Box<AST>, method: Identifier, arguments: Vec<Box<AST>> },
    OperatorCall { object: Box<AST>, operator: Operator, arguments: Vec<Box<AST>> },
    Print { format: String, arguments: Vec<Box<AST>> },

    Top (Vec<Box<AST>>),
    Block (Vec<Box<AST>>),
    Operation { operator: Operator, left: Box<AST>, right: Box<AST> },
    Loop { condition: Box<AST>, body: Box<AST> },
    Conditional { condition: Box<AST>, consequent: Box<AST>, alternative: Box<AST> },
}

#[derive(PartialEq,Eq,Hash,Debug,Clone,Serialize,Deserialize)]
pub struct Identifier(pub String);

impl Identifier {
    pub fn from(str: &str) -> Identifier {
        Identifier(str.to_string())
    }
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
    pub fn to_str(&self) -> &str { &self.0 }
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

impl Operator {
    pub fn to_str(&self) -> &str {
        match self {
            Operator::Multiplication => "*",
            Operator::Division       => "/",
            Operator::Module         => "%",
            Operator::Addition       => "+",
            Operator::Subtraction    => "-",
            Operator::Inequality     => "!=",
            Operator::Equality       => "==",
            Operator::Less           => "<",
            Operator::LessEqual      => "<=",
            Operator::Greater        => ">",
            Operator::GreaterEqual   => ">=",
            Operator::Disjunction    => "&",
            Operator::Conjunction    => "|",
        }
    }

    pub fn to_string(&self) -> String {
        self.to_str().to_string()
    }
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