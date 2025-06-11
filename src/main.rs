use crate::expr::Expr::{Abs, App, Var};

mod algorithm_j;
mod ctxt;
mod expr;
mod types;
mod typing;

fn main() {
    let e = Abs(
        "x".to_string(),
        Box::new(Abs(
            "f".to_string(),
            Box::new(App(Box::new(Var("f".to_string())), Box::new(Var("x".to_string())))),
        )),
    );
    let sigma = algorithm_j::infer(&e);
    println!("{sigma:#?}");
}
