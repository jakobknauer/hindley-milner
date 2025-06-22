mod algorithm_j;
mod ctxt;
mod expr;
mod parse;
mod types;

use std::io::{self, Write};

use crate::{
    algorithm_j::{InferenceError, infer},
    ctxt::Ctxt,
    parse::{ParseError, parse},
};

fn main() {
    loop {
        print!(">>> ");
        let _ = io::stdout().flush();

        let mut text = String::new();
        io::stdin().read_line(&mut text).unwrap();

        if text.trim().is_empty() {
            break;
        }

        try_infer(&text);
    }
}

#[allow(nonstandard_style)]
fn try_infer(text: &str) {
    let Gamma = Ctxt::new();

    match parse(text) {
        Err(ParseError::UnexpectedToken { unexpected, expected }) => {
            println!("Unexpected token of type {unexpected:?}, expected {expected} instead.")
        }
        Err(ParseError::UnexpectedEOF) => println!("Parsing failed: Unexpectedly reached end of file."),
        Err(ParseError::TrailingTokens) => println!("Parsing failed: Extra tokens at end of input."),
        Err(ParseError::TokenizerError(msg)) => println!("Parsing failed: Tokenization failed: '{msg}'."),

        Ok(e) => match infer(&e, &Gamma) {
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
