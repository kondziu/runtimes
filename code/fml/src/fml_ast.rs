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
            //ArgumentList (elements) => write!(fmt, "[{:?}]", elements),
            //Error => write!(fmt, "error"),
        }
    }
}
