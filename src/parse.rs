mod token;

use std::iter::Peekable;

use crate::{expr::Expr, parse::token::Token};

pub enum ParseError {
    UnexpectedToken { unexpected: Token, expected: String },
    UnexpectedEOF,
    TrailingTokens,
}

type ParseResult = Result<Expr, ParseError>;

pub fn parse(text: &str) -> ParseResult {
    let tokens = token::tokenize(text);
    Parser::new(tokens.into_iter()).parse()
}

struct Parser<I: Iterator<Item = Token>> {
    tokens: Peekable<I>,
}

impl<I: Iterator<Item = Token>> Parser<I> {
    fn new(tokens: I) -> Self {
        Parser {
            tokens: tokens.peekable(),
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
                self.consume()?;

                let token = self.consume()?;
                let Var(x) = token else {
                    return Err(ParseError::UnexpectedToken {
                        expected: "variable".to_string(),
                        unexpected: token,
                    });
                };

                let token = self.consume()?;
                let Dot = token else {
                    return Err(ParseError::UnexpectedToken {
                        expected: "'.'".to_string(),
                        unexpected: token,
                    });
                };

                let e = self.parse_expr()?;

                Ok(Expr::abs(x, e))
            }
            Let => {
                self.consume()?;

                let token = self.consume()?;
                let Var(x) = token else {
                    return Err(ParseError::UnexpectedToken {
                        expected: "a variable".to_string(),
                        unexpected: token,
                    });
                };

                let token = self.consume()?;
                let Equals = token else {
                    return Err(ParseError::UnexpectedToken {
                        expected: "'='".to_string(),
                        unexpected: token,
                    });
                };

                let e1 = self.parse_expr()?;

                let token = self.consume()?;
                let In = token else {
                    return Err(ParseError::UnexpectedToken {
                        expected: "'in'".to_string(),
                        unexpected: token,
                    });
                };

                let e2 = self.parse_expr()?;

                Ok(Expr::r#let(x, e1, e2))
            }
            LParen | Var(..) => self.parse_app(),
            token => Err(ParseError::UnexpectedToken {
                expected: "'lambda', 'λ', 'let', '(', or a variable".to_string(),
                unexpected: token.clone(),
            }),
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
                self.consume()?;

                let e = self.parse_expr()?;

                let token = self.consume()?;
                let RParen = token else {
                    return Err(ParseError::UnexpectedToken {
                        expected: "')'".to_string(),
                        unexpected: token,
                    });
                };

                Ok(e)
            }
            Var(x) => {
                let e = Expr::var(x);
                self.consume()?;
                Ok(e)
            }
            token => Err(ParseError::UnexpectedToken {
                expected: "''(', or a variable".to_string(),
                unexpected: token.clone(),
            }),
        }
    }
}
