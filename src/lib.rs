use lalrpop_util::lalrpop_mod;
pub mod ast;
pub mod lexer;
pub mod tokens;

pub mod grammar {
    pub use self::grammar::*;
    use super::lalrpop_mod;

    lalrpop_mod!(pub grammar);
}
