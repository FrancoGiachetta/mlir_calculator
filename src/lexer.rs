use core::panic;
use std::{iter::Peekable, str::Chars};

use crate::tokens::Token;

pub struct Lexer<'a> {
    source: Peekable<Chars<'a>>,
    tokens: Vec<Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            source: input.chars().peekable(),
            tokens: Vec::new(),
        }
    }

    pub fn tokens(&self) -> &Vec<Token> {
        &self.tokens
    }

    pub fn scan_tokens(&mut self) {
        while let Some(c) = self.source.next() {
            match c {
                ')' => self.tokens.push(Token::RightParen),
                '(' => self.tokens.push(Token::LeftParen),
                '+' => self.tokens.push(Token::Sum),
                '-' => self.tokens.push(Token::Sub),
                '/' => self.tokens.push(Token::Div),
                '*' => self.tokens.push(Token::Mul),
                '=' => {
                    if self.match_char('=').is_some() {
                        self.tokens.push(Token::EqEq);
                    } else {
                        self.tokens.push(Token::Eq);
                    }
                }
                '<' => {
                    if self.match_char('=').is_some() {
                        self.tokens.push(Token::GreaterEq);
                    } else {
                        self.tokens.push(Token::Greater);
                    }
                }
                '>' => {
                    if self.match_char('=').is_some() {
                        self.tokens.push(Token::LessEq);
                    } else {
                        self.tokens.push(Token::Less);
                    }
                }
                '"' => self.string(),
                c if c.is_numeric() => self.number(c),
                c if c.is_alphanumeric() => self.identifier(c),
                c if c.is_whitespace() => {}
                _ => todo!(),
            }
        }
    }

    fn peek(&mut self) -> Option<char> {
        self.source.next()
    }

    fn check_follow(&mut self) -> Option<&char> {
        self.source.peek()
    }

    fn match_char(&mut self, expected: char) -> Option<char> {
        self.source.next_if_eq(&expected)
    }

    fn string(&mut self) {
        let mut string = String::new();

        while let Some(c) = self.peek() {
            match self.check_follow() {
                None => panic!("Error: EOF when parsing number"),
                Some(next_c) if *next_c == '"' => break,
                Some(_) => string.push(c),
            }
        }

        self.tokens.push(Token::Str(string));
    }

    fn number(&mut self, initial_num: char) {
        let mut number = String::from(initial_num);

        while let Some(c) = self.peek() {
            match self.check_follow() {
                None => panic!("Error: EOF when parsing number"),
                Some(c) if !c.is_numeric() => break,
                _ => number.push(c),
            }
        }
        let number = number.parse::<i32>().unwrap();

        self.tokens.push(Token::Number(number));
    }

    fn identifier(&mut self, initial_char: char) {
        let mut identifier = String::from(initial_char);

        while let Some(c) = self.peek() {
            match self.check_follow() {
                None => panic!("Error: EOF when parsing identifier"),
                Some(c) if !c.is_alphanumeric() && *c != '_' => break,
                _ => identifier.push(c),
            }
        }

        self.tokens.push(Token::Identifier(identifier));
    }
}
