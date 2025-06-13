mod token;

use std::iter::Peekable;

use crate::{expr::Expr, parse::token::Token};

pub fn parse(text: &str) -> Option<Expr> {
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

    fn parse(mut self) -> Option<Expr> {
        self.parse_expr().take_if(|_| self.current().is_none())
    }

    fn current(&mut self) -> Option<&Token> {
        self.tokens.peek()
    }

    fn consume(&mut self) -> Option<Token> {
        self.tokens.next()
    }

    fn parse_expr(&mut self) -> Option<Expr> {
        use Token::*;

        match self.current()? {
            Lambda => {
                self.consume();

                let Var(x) = self.consume()? else {
                    return None;
                };
                let Dot = self.consume()? else {
                    return None;
                };
                let e = self.parse_expr()?;

                Some(Expr::abs(x, e))
            }
            Let => {
                self.consume();

                let Var(x) = self.consume()? else {
                    return None;
                };

                let Equals = self.consume()? else {
                    return None;
                };

                let e1 = self.parse_expr()?;

                let In = self.consume()? else {
                    return None;
                };

                let e2 = self.parse_expr()?;

                Some(Expr::r#let(x, e1, e2))
            }
            LParen | Var(..) => self.parse_app(),
            _ => None,
        }
    }

    fn parse_app(&mut self) -> Option<Expr> {
        use Token::*;

        let mut e = self.parse_atomic()?;

        while matches!(self.current(), Some(Var(..)) | Some(LParen)) {
            let arg = self.parse_atomic()?;
            e = Expr::app(e, arg);
        }

        Some(e)
    }

    fn parse_atomic(&mut self) -> Option<Expr> {
        use Token::*;

        match self.current()? {
            LParen => {
                self.consume();
                let e = self.parse_expr()?;
                if let RParen = self.consume()? { Some(e) } else { None }
            }
            Var(x) => {
                let e = Expr::var(x);
                self.consume();
                Some(e)
            }
            _ => None,
        }
    }
}
