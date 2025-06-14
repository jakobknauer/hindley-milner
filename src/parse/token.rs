use std::{char, iter::Peekable};

use crate::{consume_and_return, parse::ParseError};

#[derive(Debug, Clone)]
pub enum Token {
    Var(String),
    Lambda,
    Dot,
    Let,
    In,
    Equals,
    LParen,
    RParen,
}

pub fn tokenize(text: &str) -> Result<Vec<Token>, ParseError> {
    Tokenizer::new(text.chars()).collect()
}

struct Tokenizer<I: Iterator<Item = char>> {
    text: Peekable<I>,
}

impl<I: Iterator<Item = char>> Tokenizer<I> {
    fn new(text: I) -> Self {
        Tokenizer { text: text.peekable() }
    }

    fn current(&mut self) -> Option<&char> {
        self.text.peek()
    }

    fn on_whitespace(&mut self) -> bool {
        self.current().is_some_and(|c| c.is_whitespace())
    }

    fn consume(&mut self) {
        self.text.next();
    }

    fn consume_keyword_or_var(&mut self) -> String {
        use peeking_take_while::PeekableExt;
        self.text.peeking_take_while(|c| c.is_ascii_alphanumeric()).collect()
    }
}

impl<I: Iterator<Item = char>> Iterator for Tokenizer<I> {
    type Item = Result<Token, ParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        use Token::*;

        while self.on_whitespace() {
            self.consume()
        }

        let token = match self.current()? {
            '.' => consume_and_return!(self, Dot),
            'λ' => consume_and_return!(self, Lambda),
            '=' => consume_and_return!(self, Equals),
            '(' => consume_and_return!(self, LParen),
            ')' => consume_and_return!(self, RParen),

            c if c.is_ascii_alphabetic() => {
                let token = self.consume_keyword_or_var();
                match token.as_str() {
                    "lambda" => Lambda,
                    "let" => Let,
                    "in" => In,
                    _ => Var(token),
                }
            }

            c => return Some(Err(ParseError::TokenizerError(format!("unexpected character '{c}'")))),
        };

        Some(Ok(token))
    }
}
