pub enum AST<'ast> {
    Number(i32),
    Identifier(&'ast str),
}
