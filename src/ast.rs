use crate::grammar::ExprParser;

#[derive(Debug)]
pub enum Statement {
    DeclVar { name: String, value: Box<Expr> },
}

#[derive(Debug)]
pub enum Expr {
    Num(f64),
    BinaryOp {
        lhs: Box<Expr>,
        op: Op,
        rhs: Box<Expr>,
    },
    UnaryOp {
        op: Op,
        shs: Box<Expr>,
    },
}

#[derive(Debug)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

pub fn parse(input: &str) -> Box<Expr> {
    ExprParser::new().parse(input).unwrap()
}
