mod algorithm_j;
mod ctxt;
mod expr;
mod parse;
mod types;

use crate::{
    algorithm_j::InferenceError,
    ctxt::{Binding, Ctxt},
    parse::ParseError,
    types::{Mono, Poly},
};

#[allow(nonstandard_style)]
fn main() {
    let int = Mono::nullary("Int");
    let Gamma = Ctxt::new()
        | Binding(
            "plus".to_string(),
            Poly::mono(Mono::arrow(int.clone(), Mono::arrow(int.clone(), int))),
        );

    let text = "let double = lambda x . plus x x in lambda n . double (double n)";

    match parse::parse(text) {
        Err(ParseError::UnexpectedToken { unexpected, expected }) => {
            println!("Unexpected token of type {unexpected:?}, expected {expected} instead.")
        }
        Err(ParseError::UnexpectedEOF) => println!("Parsing failed: Unexpectedly reached end of file."),
        Err(ParseError::TrailingTokens) => println!("Parsing failed: Extra tokens at end of input."),
        Err(ParseError::TokenizerError(msg)) => println!("Parsing failed: Tokenization failed: '{msg}'."),

        Ok(e) => match algorithm_j::infer(&e, &Gamma) {
            Err(InferenceError::UnknownVar(x)) => {
                println!("Type inference failed: Encountered unknown variable during inference: '{x}'.")
            }
            Err(InferenceError::ImpossibleUnification(tau1, tau2)) => {
                println!("Type inference failed: Cannot unify types '{tau1}' and '{tau2}'.")
            }
            Err(InferenceError::RecursiveType(tau, alpha)) => {
                println!("Type inference failed: Unifying '{tau}' and '{alpha}' would create recursive type.")
            }

            Ok(sigma) => println!("{Gamma} ⊢ {e} : {sigma}"),
        },
    }
}
