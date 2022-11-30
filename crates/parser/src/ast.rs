use crate::span::Spanned;

#[derive(Clone, Debug, PartialEq)]
pub enum AstNode {
    Bool(bool),
    Integer(i64),
    String(String),

    List(Vec<Spanned<AstNode>>),
    Symbol(String),
    Error,
}
