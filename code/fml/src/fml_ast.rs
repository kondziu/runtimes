use std::fmt::{Debug, Error, Formatter};
use std::cmp::PartialEq;

#[derive(PartialEq)]
pub enum AST<'ast> {
    Unit,
    Number(i32),
    Identifier(&'ast str),
    StringLiteral(&'ast str),
    Boolean(bool),
    Assignment {identifier: Box<AST<'ast>>, value: Box<AST<'ast>>},
    Mutation {identifier: Box<AST<'ast>>, value: Box<AST<'ast>>},
    FunctionDefinition {name: Box<AST<'ast>>, parameters: Vec<Box<AST<'ast>>>, body: Box<AST<'ast>>},
    OperatorDefinition {operator: Operator, parameters: Vec<Box<AST<'ast>>>, body: Box<AST<'ast>>},
    FunctionApplication {function: Box<AST<'ast>>, arguments: Vec<Box<AST<'ast>>>},
    Block (Vec<Box<AST<'ast>>>),
    Operation {operator: Operator, left: Box<AST<'ast>>, right: Box<AST<'ast>>},
    Loop {condition: Box<AST<'ast>>, body: Box<AST<'ast>>},
    Conditional {condition: Box<AST<'ast>>, consequent: Box<AST<'ast>>, alternative: Box<AST<'ast>>},
    ArrayDefinition {size: Box<AST<'ast>>, value: Box<AST<'ast>>},
    ArrayAccess {array: Box<AST<'ast>>, index: Box<AST<'ast>>},
    ArrayMutation {array: Box<AST<'ast>>, value: Box<AST<'ast>>},
    ObjectDefinition {extends: Option<Box<AST<'ast>>>, parameters: Vec<Box<AST<'ast>>>, members: Vec<Box<AST<'ast>>>},
    FieldAccess {object: Box<AST<'ast>>, field: Box<AST<'ast>>},
    OperatorAccess {object: Box<AST<'ast>>, operator: Operator},
    FieldMutation {field_path: Box<AST<'ast>>, value: Box<AST<'ast>>},
    MethodCall {method_path: Box<AST<'ast>>, arguments: Vec<Box<AST<'ast>>>},
    Print {format: Box<AST<'ast>>, arguments: Vec<Box<AST<'ast>>>},
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

impl Debug for AST<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::AST::*;
        match self {
            Unit => write!(fmt, "Unit()"),
            Number(n) => write!(fmt, "Number({:?})", n),
            Identifier(id) => write!(fmt, "Identifier({})", id),
            StringLiteral(s) => write!(fmt, "StringLiteral({:?})", s),
            Boolean(b) => write!(fmt, "Boolean({})", b),
            Assignment {identifier, value} =>
                write!(fmt, "Assignment(identifier={:?}, value={:?})", identifier, value),
            Mutation {identifier, value} =>
                write!(fmt, "Mutation(identifier={:?}, value={:?})", identifier, value),
            FunctionDefinition { name: identifier,parameters, body} =>
                write!(fmt, "FunctionDefinition(identifier={:?}, parameters={:?}, body={:?})", identifier, parameters, body),
            OperatorDefinition {operator,parameters, body} =>
                write!(fmt, "OperatorDefinition(operator={:?}, parameters={:?}, body={:?})", operator, parameters, body),
            FunctionApplication { function: identifier,arguments} =>
                write!(fmt, "FunctionApplication(identifier={:?}, arguments={:?})", identifier, arguments),
            Block(expressions) =>
                write!(fmt, "Block({:?})", expressions),
            Operation {operator, left, right} =>
                write!(fmt, "Operation(operator={:?}, left={:?}, right={:?})", operator, left, right),
            Loop {condition, body} =>
                write!(fmt, "Loop(condition={:?}, body={:?})", condition, body),
            Conditional {condition, consequent, alternative} =>
                write!(fmt, "Conditional(condition={:?}, consequent={:?}, alternative={:?})", condition, consequent, alternative),
            ArrayDefinition {size, value} =>
                write!(fmt, "ArrayDefinition(size={:?}, value={:?})", size, value),
            ArrayAccess {array, index} =>
                write!(fmt, "ArrayAccess(array={:?}, index={:?})", array, index),
            ArrayMutation {array, value} =>
                write!(fmt, "ArrayMutation(array={:?}, value={:?})", array, value),
            ObjectDefinition {extends, parameters, members} =>
                write!(fmt, "ObjectDefinition(extends={:?}, parameters={:?}, members={:?})", extends, parameters, members),
            FieldAccess {object, field: identifier } =>
                write!(fmt, "FieldAccess(object={:?}, identifier={:?})", object, identifier),
            OperatorAccess {object, operator } =>
                write!(fmt, "OperatorAccess(object={:?}, operator={:?})", object, operator),
            FieldMutation { field_path: field, value} =>
                write!(fmt, "FieldMutation(field={:?}, value={:?})", field, value),
            MethodCall { method_path: field, arguments} =>
                write!(fmt, "MethodCall(field={:?}, arguments={:?})", field, arguments),
            Print {format, arguments} =>
                write!(fmt, "Print(format={:?}, arguments={:?})", format, arguments),
            //Error => write!(fmt, "error"),
        }
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