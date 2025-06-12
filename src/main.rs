mod algorithm_j;
mod ctxt;
mod expr;
mod parse;
mod types;

use ctxt::{Binding, Ctxt};
use expr::Expr;
use types::{Mono, Poly};

#[allow(nonstandard_style)]
fn main() {
    let plus = Expr::var("plus");
    let double = Expr::abs("x", Expr::app(Expr::app(plus.clone(), Expr::var("x")), Expr::var("x")));

    let quadruple = Expr::r#let(
        "double",
        double.clone(),
        Expr::abs(
            "n",
            Expr::app(Expr::var("double"), Expr::app(Expr::var("double"), Expr::var("n"))),
        ),
    );

    let int = Mono::nullary("Int");

    let Gamma = &Ctxt::new()
        | Binding(
            "plus".to_string(),
            Poly::mono(Mono::arrow(int.clone(), Mono::arrow(int.clone(), int))),
        );

    let sigma = algorithm_j::infer(&quadruple, &Gamma).unwrap();

    println!("{Gamma} ⊢ {quadruple} : {sigma}");

    parse::token::tokenize("let double = lambda x . plus x x in lambda n . double (double n)  ");
}
