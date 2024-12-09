use std::num::ParseIntError;

use logos::Logos;

#[derive(Default, Debug, Clone, PartialEq)]
pub enum LexicalError {
    InvalidInteger(ParseIntError),
    #[default]
    InvalidToken,
}

impl From<ParseIntError> for LexicalError {
    fn from(err: ParseIntError) -> Self {
        LexicalError::InvalidInteger(err)
    }
}

#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(skip r"[\n\t\f]", skip r"#.*\n?", error = LexicalError)]
pub enum Token {
    #[token("(")]
    RightParen,
    #[token(")")]
    LeftParen,
    #[token("=")]
    Eq,
    #[token("==")]
    EqEq,
    #[token(">")]
    Greater,
    #[token(">=")]
    GreaterEq,
    #[token("<")]
    Less,
    #[token("<=")]
    LessEq,
    #[token("+")]
    Add,
    #[token("-")]
    Sub,
    #[token("*")]
    Mul,
    #[token("/")]
    Div,
    #[token("^")]
    Pow,
    Neg,
    #[regex("[1-9][0-9]*", |l| l.slice().parse::<f64>().unwrap())]
    Num(f64),
    // #[token("let")]
    // Let,
    // #[token("if")]
    // If,
    // #[token("else")]
    // Else,
    // #[token("while")]
    // While,
    // #[token("for")]
    // For,
}
