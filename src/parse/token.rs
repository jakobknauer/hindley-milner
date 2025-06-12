use std::{char, iter::Peekable};

#[derive(Debug)]
pub enum Token {
    Var(String),
    CapVar(String),
    Lambda,
    Dot,
    Let,
    In,
    Equals,
    LParen,
    RParen,
}

pub fn tokenize(text: &str) -> Vec<Token> {
    let tokens = Tokenizer::new(text.chars()).collect();
    dbg!(text, &tokens);
    tokens
}

struct Tokenizer<I: Iterator<Item = char>> {
    text: Peekable<I>,
}

impl<I: Iterator<Item = char>> Tokenizer<I> {
    fn new(text: I) -> Tokenizer<I> {
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

    /// TODO: Refactor this into a macro
    fn consume_and_return(&mut self, token: Token) -> Token {
        self.consume();
        token
    }

    fn consume_keyword_or_var(&mut self) -> String {
        use peeking_take_while::PeekableExt;
        self.text.peeking_take_while(|c| c.is_ascii_alphanumeric()).collect()
    }
}

impl<I: Iterator<Item = char>> Iterator for Tokenizer<I> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        use Token::*;

        while self.on_whitespace() {
            self.consume()
        }

        Some(match self.current()? {
            '.' => self.consume_and_return(Dot),
            'λ' => self.consume_and_return(Lambda),
            '=' => self.consume_and_return(Equals),
            '(' => self.consume_and_return(LParen),
            ')' => self.consume_and_return(RParen),

            c if c.is_ascii_alphabetic() => {
                let token = self.consume_keyword_or_var();
                match token.as_str() {
                    "lambda" => Lambda,
                    "let" => Let,
                    "in" => In,
                    _ if token.chars().nth(0).unwrap().is_ascii_uppercase() => CapVar(token),
                    _ => Var(token),
                }
            }

            c => panic!("unexpected character '{c}'"),
        })
    }
}
