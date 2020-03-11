use std::fmt::{Debug, Error, Formatter};
use std::cmp::PartialEq;
//use std::string::ToString;

#[derive(PartialEq)]
pub enum AST<'ast> {
    Number(i32),
    Identifier(&'ast str),
    StringLiteral(&'ast str),
    BooleanLiteral(bool),
    Assignment {identifier: Box<AST<'ast>>, value: Box<AST<'ast>>},
    Mutation {identifier: Box<AST<'ast>>, value: Box<AST<'ast>>},
    FunctionDefinition {identifier: Box<AST<'ast>>, parameters: Vec<Box<AST<'ast>>>, body: Box<AST<'ast>>},
    FunctionApplication {identifier: Box<AST<'ast>>, arguments: Vec<Box<AST<'ast>>>},
    //ArgumentList (Vec<Box<AST<'ast>>>),
}

impl Debug for AST<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::AST::*;
        match self {
            Number(n) => write!(fmt, "Number({:?})", n),
            Identifier(id) => write!(fmt, "Identifier({})", id),
            StringLiteral(s) => write!(fmt, "StringLiteral({:?})", s),
            BooleanLiteral(b) => write!(fmt, "Boolean({})", b),
            Assignment {identifier, value} =>
                write!(fmt, "Assignment({:?}, {:?})", identifier, value),
            Mutation {identifier, value} =>
                write!(fmt, "Assignment({:?}, {:?})", identifier, value),
            FunctionDefinition {identifier,parameters, body} =>
                write!(fmt, "FunctionDefinition({:?} ({:?}) {:?})", identifier, parameters, body),
            FunctionApplication {identifier,arguments} =>
                write!(fmt, "FunctionApplication({:?} ({:?}))", identifier, arguments),
            //ArgumentList (elements) => write!(fmt, "[{:?}]", elements),
            //Error => write!(fmt, "error"),
        }
    }
}
