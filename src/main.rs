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
    let Gamma = &Ctxt::new()
        | Binding(
            "plus".to_string(),
            Poly::mono(Mono::arrow(int.clone(), Mono::arrow(int.clone(), int))),
        );

    let text = "let double = lambda x . plus x x in lambda n . double (double n)";

    // let text = "lambda f . lambda x. x f";
    // let Gamma = Ctxt::new();

    match parse::parse(text) {
        Err(ParseError::UnexpectedToken { unexpected, expected }) => {
            println!("Unexpected token of type {unexpected:?}, expected {expected} instead.")
        }
        Err(ParseError::UnexpectedEOF) => println!("Unexpectedly reached end of file."),
        Err(ParseError::TrailingTokens) => println!("Tokens remained after parsing finished."),
        Err(ParseError::TokenizerError(msg)) => println!("Error during tokenization: {msg}."),

        Ok(e) => match algorithm_j::infer(&e, &Gamma) {
            Err(InferenceError::UnknownVar(x)) => {
                println!("Type inference failed: encountered unknown variable during inference: '{x}'.")
            }
            Err(InferenceError::ImpossibleUnification(tau1, tau2)) => {
                println!("Type inference failed: cannot unify types '{tau1}' and '{tau2}'.")
            }

            Ok(sigma) => println!("{Gamma} ⊢ {e} : {sigma}"),
        },
    }
}
