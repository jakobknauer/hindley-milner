mod token;
#[macro_use]
mod macros;

use std::{collections::HashSet, iter::Peekable};

use crate::{
    expr::Expr,
    parse::token::Token,
    types::{Mono, Poly, TypeVar},
};

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    UnexpectedToken { unexpected: Token, expected: String },
    UnexpectedEOF,
    TrailingTokens,
    TokenizerError(String),
}

pub type ParseResult<T> = Result<T, ParseError>;

pub fn parse(text: &str) -> ParseResult<Expr> {
    let tokens = token::tokenize(text)?;
    Parser::new(tokens).parse_expr()
}

pub fn parse_poly(text: &str) -> ParseResult<Poly> {
    let tokens = token::tokenize(text)?;
    Parser::new(tokens).parse_poly()
}

fn unexpected_token_error<T>(unexpected: &Token, expected: &str) -> ParseResult<T> {
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

    fn parse_expr(mut self) -> ParseResult<Expr> {
        let result = self.parse_expr_internal()?;
        if self.current().is_err() {
            Ok(result)
        } else {
            Err(ParseError::TrailingTokens)
        }
    }

    fn parse_poly(mut self) -> ParseResult<Poly> {
        let result = self.parse_poly_internal()?;
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

    fn parse_expr_internal(&mut self) -> ParseResult<Expr> {
        use Token::*;

        match self.current()? {
            Lambda => {
                expect_token!(self, Lambda, "'lambda', or 'λ'");
                let x = expect_variable!(self, "a variable");
                expect_token!(self, Dot, "'.'");
                let e = self.parse_expr_internal()?;

                Ok(Expr::abs(x, e))
            }
            Let => {
                expect_token!(self, Let, "'let'");
                let x = expect_variable!(self, "a variable");
                expect_token!(self, Equals, "'='");
                let e1 = self.parse_expr_internal()?;
                expect_token!(self, In, "'in'");
                let e2 = self.parse_expr_internal()?;

                Ok(Expr::r#let(x, e1, e2))
            }

            LParen | Var(..) => self.parse_app(),

            token => unexpected_token_error(token, "'lambda', 'λ', 'let', '(', or a variable"),
        }
    }

    fn parse_app(&mut self) -> ParseResult<Expr> {
        use Token::*;

        let mut e = self.parse_atomic_expr()?;

        while let Ok(Var(..)) | Ok(LParen) = self.current() {
            let arg = self.parse_atomic_expr()?;
            e = Expr::app(e, arg);
        }

        Ok(e)
    }

    fn parse_atomic_expr(&mut self) -> ParseResult<Expr> {
        use Token::*;

        match self.current()? {
            LParen => {
                expect_token!(self, LParen, "'('");
                let e = self.parse_expr_internal()?;
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

    fn parse_poly_internal(&mut self) -> ParseResult<Poly> {
        use Token::*;

        match self.current()? {
            ForAll => {
                expect_token!(self, ForAll, "'forall', or '∀' ");

                let mut vars: HashSet<TypeVar> = [expect_variable!(self, "a type variable")].into();
                while let Ok(Var(_)) = self.current() {
                    let alpha = expect_variable!(self, "a type variable");
                    vars.insert(alpha);
                }

                expect_token!(self, Dot, "a variable, or '.'");
                let tau = self.parse_mono()?;

                Ok(Poly(vars, tau))
            }
            Var(..) | VarCap(..) => {
                let sigma = Poly::mono(self.parse_mono()?);
                Ok(sigma)
            }
            token => unexpected_token_error(token, "'(', or a type variable or type function"),
        }
    }

    fn parse_mono(&mut self) -> ParseResult<Mono> {
        use Token::*;

        let tau1 = self.parse_mono_arrow_arg()?;

        if let Ok(Arrow) = self.current() {
            expect_token!(self, Arrow, "'→'");
            let tau2 = self.parse_mono()?;
            Ok(Mono::arrow(tau1, tau2))
        } else {
            Ok(tau1)
        }
    }

    #[allow(nonstandard_style)]
    fn parse_mono_arrow_arg(&mut self) -> ParseResult<Mono> {
        use Token::*;

        match self.current()? {
            VarCap(..) => {
                let C = expect_varcap!(self, "a type function");

                let mut taus = Vec::new();
                while let Ok(Var(..)) | Ok(VarCap(..)) | Ok(LParen) = self.current() {
                    taus.push(self.parse_atomic_mono()?);
                }

                Ok(Mono::App(C, taus))
            }
            Var(..) | LParen => Ok(self.parse_atomic_mono()?),
            token => unexpected_token_error(token, "'(', or a type variable or type function"),
        }
    }

    fn parse_atomic_mono(&mut self) -> ParseResult<Mono> {
        use Token::*;

        match self.current()? {
            LParen => {
                expect_token!(self, LParen, "'('");
                let tau = self.parse_mono()?;
                expect_token!(self, RParen, "')'");

                Ok(tau)
            }
            Var(..) => {
                let alpha = expect_variable!(self, "a type variable");
                Ok(Mono::Var(alpha))
            }
            VarCap(..) => {
                let alpha = expect_varcap!(self, "a type function");
                Ok(Mono::nullary(alpha))
            }
            token => unexpected_token_error(token, "'(', or a type variable or type function"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_basic() {
        assert_eq!(parse("x"), Ok(Expr::var("x")));
        assert_eq!(parse("lambda x . y"), Ok(Expr::abs("x", Expr::var("y"))));
        assert_eq!(parse("λ x . y"), Ok(Expr::abs("x", Expr::var("y"))));
        assert_eq!(parse("x y"), Ok(Expr::app(Expr::var("x"), Expr::var("y"))));
        assert_eq!(
            parse("let x = y in z"),
            Ok(Expr::r#let("x", Expr::var("y"), Expr::var("z")))
        );
        assert_eq!(parse("(x)"), Ok(Expr::var("x")));

        // parens and applications
        assert_eq!(
            parse("x y z"),
            Ok(Expr::app(Expr::app(Expr::var("x"), Expr::var("y")), Expr::var("z")))
        );
        assert_eq!(
            parse("(x y) z"),
            Ok(Expr::app(Expr::app(Expr::var("x"), Expr::var("y")), Expr::var("z")))
        );
        assert_eq!(
            parse("x (y z)"),
            Ok(Expr::app(Expr::var("x"), Expr::app(Expr::var("y"), Expr::var("z"))))
        );
    }

    #[test]
    fn test_parse_malformed() {
        assert!(matches!(parse("lambda x y"), Err(ParseError::UnexpectedToken { .. })));
        assert!(matches!(parse("lambda x ."), Err(ParseError::UnexpectedEOF)));
        assert!(matches!(parse("lambda x ? y"), Err(ParseError::TokenizerError(..))));
        assert!(matches!(parse("lambda x . y )"), Err(ParseError::TrailingTokens)));
        assert!(matches!(parse("let x in y"), Err(ParseError::UnexpectedToken { .. })));
        assert!(matches!(parse("let x = y in"), Err(ParseError::UnexpectedEOF)));
        assert!(matches!(
            parse("let lambda = y in z"),
            Err(ParseError::UnexpectedToken { .. })
        ));
    }

    #[test]
    fn test_parse_atomic_and_lambda() {
        assert_eq!(
            parse("lambda x . y z"),
            Ok(Expr::abs("x", Expr::app(Expr::var("y"), Expr::var("z"))))
        );
        assert!(matches!(parse("z lambda x . y"), Err(ParseError::TrailingTokens)));
        assert_eq!(
            parse("(lambda x . y) z"),
            Ok(Expr::app(Expr::abs("x", Expr::var("y")), Expr::var("z")))
        );
        assert_eq!(
            parse("z (lambda x . y)"),
            Ok(Expr::app(Expr::var("z"), Expr::abs("x", Expr::var("y"))))
        );
    }

    #[test]
    fn test_parse_atomic_and_let() {
        assert_eq!(
            parse("let x = y in z a"),
            Ok(Expr::r#let(
                "x",
                Expr::var("y"),
                Expr::app(Expr::var("z"), Expr::var("a"))
            ))
        );
        assert!(matches!(parse("a let x = y in y"), Err(ParseError::TrailingTokens)));
        assert_eq!(
            parse("(let x = y in z) a"),
            Ok(Expr::app(
                Expr::r#let("x", Expr::var("y"), Expr::var("z")),
                Expr::var("a")
            ))
        );
        assert_eq!(
            parse("a (let x = y in z)"),
            Ok(Expr::app(
                Expr::var("a"),
                Expr::r#let("x", Expr::var("y"), Expr::var("z"))
            ))
        );
    }
}
