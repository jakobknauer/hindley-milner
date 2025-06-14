mod token;
#[macro_use]
mod macros;

use std::iter::Peekable;

use crate::{expr::Expr, parse::token::Token};

pub enum ParseError {
    UnexpectedToken { unexpected: Token, expected: String },
    UnexpectedEOF,
    TrailingTokens,
    TokenizerError(String),
}

pub type ParseResult = Result<Expr, ParseError>;

pub fn parse(text: &str) -> ParseResult {
    let tokens = token::tokenize(text)?;
    Parser::new(tokens).parse()
}

fn unexpected_token_error(unexpected: &Token, expected: &str) -> ParseResult {
    Err(ParseError::UnexpectedToken {
        expected: expected.to_string(),
        unexpected: unexpected.clone(),
    })
}

struct Parser<I: Iterator<Item = Token>> {
    tokens: Peekable<I>,
}

impl<I: Iterator<Item = Token>> Parser<I> {
    fn new(tokens: impl IntoIterator<IntoIter = I>) -> Self {
        Parser {
            tokens: tokens.into_iter().peekable(),
        }
    }

    fn parse(mut self) -> ParseResult {
        let result = self.parse_expr()?;
        if self.current().is_err() {
            Ok(result)
        } else {
            Err(ParseError::TrailingTokens)
        }
    }

    fn current(&mut self) -> Result<&Token, ParseError> {
        self.tokens.peek().ok_or(ParseError::UnexpectedEOF)
    }

    fn consume(&mut self) -> Result<Token, ParseError> {
        self.tokens.next().ok_or(ParseError::UnexpectedEOF)
    }

    fn parse_expr(&mut self) -> ParseResult {
        use Token::*;

        match self.current()? {
            Lambda => {
                expect_token!(self, Lambda, "'lambda', or 'λ'");
                let x = expect_variable!(self, "a variable");
                expect_token!(self, Dot, "'.'");
                let e = self.parse_expr()?;

                Ok(Expr::abs(x, e))
            }
            Let => {
                expect_token!(self, Let, "'let'");
                let x = expect_variable!(self, "a variable");
                expect_token!(self, Equals, "'='");
                let e1 = self.parse_expr()?;
                expect_token!(self, In, "'in'");
                let e2 = self.parse_expr()?;

                Ok(Expr::r#let(x, e1, e2))
            }

            LParen | Var(..) => self.parse_app(),

            token => unexpected_token_error(token, "'lambda', 'λ', 'let', '(', or a variable"),
        }
    }

    fn parse_app(&mut self) -> ParseResult {
        use Token::*;

        let mut e = self.parse_atomic()?;

        while matches!(self.current(), Ok(Var(..)) | Ok(LParen)) {
            let arg = self.parse_atomic()?;
            e = Expr::app(e, arg);
        }

        Ok(e)
    }

    fn parse_atomic(&mut self) -> ParseResult {
        use Token::*;

        match self.current()? {
            LParen => {
                expect_token!(self, LParen, "'('");
                let e = self.parse_expr()?;
                expect_token!(self, RParen, "')'");

                Ok(e)
            }
            Var(x) => {
                let e = Expr::var(x);
                self.consume()?;

                Ok(e)
            }

            token => unexpected_token_error(token, "'(', or a variable"),
        }
    }
}
