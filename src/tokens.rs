#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    RightParen,
    LeftParen,
    Sum,
    Sub,
    Div,
    Mul,
    Eq,
    EqEq,
    Greater,
    GreaterEq,
    Less,
    LessEq,
    Identifier(String),
    Str(String),
    Number(i32),
    If,
    Else,
    Let,
    For,
    While,
}
