mod algorithm_j;
mod ctxt;
mod expr;
mod parse;
mod types;

use crate::{
    ctxt::{Binding, Ctxt},
    parse::ParseError,
    types::{Mono, Poly},
};

#[allow(nonstandard_style)]
fn main() {
    let int = Mono::nullary("Int");

    let Gamma = &Ctxt::new()
        | Binding(
            "plus".to_string(),
            Poly::mono(Mono::arrow(int.clone(), Mono::arrow(int.clone(), int))),
        );

    let parse_result = parse::parse("let double = lambda x . plus x x in lambda n . double (double n)");

    match parse_result {
        Ok(e) => {
            let sigma = algorithm_j::infer(&e, &Gamma).unwrap();
            println!("{Gamma} ⊢ {e} : {sigma}");
        }
        Err(ParseError::UnexpectedToken { unexpected, expected }) => {
            println!("Unexpected token of type {unexpected:?}, expected {expected} instead.")
        }
        Err(ParseError::UnexpectedEOF) => {
            println!("Unexpectedly reached end of file.")
        }
        Err(ParseError::TrailingTokens) => {
            println!("Tokens remained after parsing finished.")
        }
        Err(ParseError::TokenizerError(msg)) => {
            println!("Error during tokenization: {msg}.");
        }
    }
}
