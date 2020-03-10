use std::fmt::{Debug, Error, Formatter};
use std::cmp::PartialEq;

#[derive(PartialEq)]
pub enum AST<'ast> {
    Number(i32),
    Identifier(&'ast str),
    StringLiteral(&'ast str),
    BooleanLiteral(bool),
}

impl Debug for AST<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::AST::*;
        match *self {
            Number(n) => write!(fmt, "Number({:?})", n),
            Identifier(id) => write!(fmt, "Identifier({})", id),
            StringLiteral(s) => write!(fmt, "StringLiteral({:?})", s),
            BooleanLiteral(b) => write!(fmt, "Boolean({})", b),
            //Op(ref l, op, ref r) => write!(fmt, "({:?} {:?} {:?})", l, op, r),
            //Error => write!(fmt, "error"),
        }
    }
}
