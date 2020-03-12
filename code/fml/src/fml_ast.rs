use std::fmt::{Debug, Error, Formatter};
use std::cmp::PartialEq;
//use std::string::ToString;

#[derive(PartialEq)]
pub enum AST<'ast> {
    Unit,
    Number(i32),
    Identifier(&'ast str),
    StringLiteral(&'ast str),
    BooleanLiteral(bool),
    Assignment {identifier: Box<AST<'ast>>, value: Box<AST<'ast>>},
    Mutation {identifier: Box<AST<'ast>>, value: Box<AST<'ast>>},
    FunctionDefinition {identifier: Box<AST<'ast>>, parameters: Vec<Box<AST<'ast>>>, body: Box<AST<'ast>>},
    FunctionApplication {identifier: Box<AST<'ast>>, arguments: Vec<Box<AST<'ast>>>},
    Block (Vec<Box<AST<'ast>>>),
    Operation {operator: Operator, left: Box<AST<'ast>>, right: Box<AST<'ast>>},
    Loop {condition: Box<AST<'ast>>, body: Box<AST<'ast>>},
    Conditional {condition: Box<AST<'ast>>, consequent: Box<AST<'ast>>, alternative: Box<AST<'ast>>},
    ArrayDefinition {size: Box<AST<'ast>>, value: Box<AST<'ast>>},
    ArrayAccess {array: Box<AST<'ast>>, index: Box<AST<'ast>>},
    ArrayMutation {array: Box<AST<'ast>>, value: Box<AST<'ast>>},
    ObjectDefinition {parameters: Vec<Box<AST<'ast>>>, members: Vec<Box<AST<'ast>>>},
    FieldAccess {object: Box<AST<'ast>>, identifier: Box<AST<'ast>>},
    FieldMutation {field: Box<AST<'ast>>, value: Box<AST<'ast>>},
    MethodCall {field: Box<AST<'ast>>, arguments: Vec<Box<AST<'ast>>>},
    Print {format: Box<AST<'ast>>, arguments: Vec<Box<AST<'ast>>>},
}

#[derive(PartialEq,Debug)]
pub enum Operator {
    Times,
    Plus,
    Minus,
    Divide,
    Unequal,
    Equal,
    Less,
    Greater,
    GreaterEqual,
    LessEqual,
    Or,
    And,
}

impl Debug for AST<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::AST::*;
        match self {
            Unit => write!(fmt, "Unit()"),
            Number(n) => write!(fmt, "Number({:?})", n),
            Identifier(id) => write!(fmt, "Identifier({})", id),
            StringLiteral(s) => write!(fmt, "StringLiteral({:?})", s),
            BooleanLiteral(b) => write!(fmt, "Boolean({})", b),
            Assignment {identifier, value} =>
                write!(fmt, "Assignment(identifier={:?}, value={:?})", identifier, value),
            Mutation {identifier, value} =>
                write!(fmt, "Mutation(identifier={:?}, value={:?})", identifier, value),
            FunctionDefinition {identifier,parameters, body} =>
                write!(fmt, "FunctionDefinition(identifier={:?}, parameters={:?}, body={:?})", identifier, parameters, body),
            FunctionApplication {identifier,arguments} =>
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
            ObjectDefinition {parameters, members} =>
                write!(fmt, "ObjectDefinition(parameters={:?}, members={:?})", parameters, members),
            FieldAccess {object, identifier} =>
                write!(fmt, "FieldAccess(object={:?}, identifier={:?})", object, identifier),
            FieldMutation {field, value} =>
                write!(fmt, "FieldMutation(field={:?}, value={:?})", field, value),
            MethodCall {field, arguments} =>
                write!(fmt, "MethodCall(field={:?}, arguments={:?})", field, arguments),
            Print {format, arguments} =>
                write!(fmt, "Print(format={:?}, arguments={:?})", format, arguments),
            //Error => write!(fmt, "error"),
        }
    }
}
