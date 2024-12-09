use logos::{Logos, SpannedIter};

use crate::tokens::{LexicalError, Token};

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

pub struct Lexer<'input> {
    tokens: SpannedIter<'input, Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let tokens = Token::lexer(input).spanned();

        Self { tokens }
    }
}

impl Iterator for Lexer<'_> {
    type Item = Spanned<Token, usize, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.tokens
            .next()
            .map(|(token, span)| Ok((span.start, token?, span.end)))
    }
}
