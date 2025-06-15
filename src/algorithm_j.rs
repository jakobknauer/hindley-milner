use std::collections::HashMap;

use crate::{
    ctxt::Ctxt,
    expr::Expr,
    types::{Mono, Poly, TypeVar},
};

#[derive(Debug, PartialEq, Eq)]
pub enum InferenceError {
    UnknownVar(String),
    ImpossibleUnification(Mono, Mono),
    RecursiveType(Mono, String),
}

pub type InferenceResult<T> = Result<T, InferenceError>;

#[allow(nonstandard_style)]
pub fn infer(e: &Expr, Gamma: &Ctxt) -> InferenceResult<Poly> {
    let mut algorithm = AlgorithmJ::new();
    let tau = algorithm
        .infer(e, Gamma)?
        .canonicalize(&algorithm.aliases)
        .generalize(Gamma);
    Ok(tau)
}

struct AlgorithmJ {
    counter: u32,
    aliases: HashMap<TypeVar, Mono>,
}

impl AlgorithmJ {
    pub fn new() -> AlgorithmJ {
        AlgorithmJ {
            counter: 0,
            aliases: HashMap::new(),
        }
    }

    #[allow(nonstandard_style)]
    pub fn infer(&mut self, e: &Expr, Gamma: &Ctxt) -> InferenceResult<Mono> {
        match e {
            Expr::Var(x) => {
                let sigma = Gamma.get(x).ok_or_else(|| InferenceError::UnknownVar(x.into()))?;
                let tau = sigma.clone().inst(self.new_vars());
                Ok(tau)
            }
            Expr::App(e0, e1) => {
                let tau0 = self.infer(e0, Gamma)?;
                let tau1 = self.infer(e1, Gamma)?;
                let tau_prime = self.new_var();
                self.unify(tau0, Mono::arrow(tau1, tau_prime.clone()))?;
                Ok(tau_prime)
            }
            Expr::Abs(x, e) => {
                let tau = self.new_var();
                let Gamma_prime = Gamma.clone().bind(x, Poly::mono(tau.clone()));
                let tau_prime = self.infer(e, &Gamma_prime)?;
                Ok(Mono::arrow(tau, tau_prime))
            }
            Expr::Let(x, e0, e1) => {
                let tau = self.infer(e0, Gamma)?.canonicalize(&self.aliases).generalize(Gamma);
                let Gamma_prime = Gamma.clone().bind(x, tau);
                let tau_prime = self.infer(e1, &Gamma_prime)?;
                Ok(tau_prime)
            }
        }
    }

    fn new_var(&mut self) -> Mono {
        self.counter += 1;
        let alpha = format!("_{}", self.counter);
        Mono::Var(alpha)
    }

    fn new_vars(&mut self) -> impl IntoIterator<Item = Mono> {
        std::iter::from_fn(|| Some(self.new_var()))
    }

    #[allow(nonstandard_style)]
    fn unify(&mut self, tau1: Mono, tau2: Mono) -> InferenceResult<()> {
        let tau1 = tau1.canonicalize(&self.aliases);
        let tau2 = tau2.canonicalize(&self.aliases);

        match (tau1, tau2) {
            (tau1, tau2) if tau1 == tau2 => Ok(()),
            (Mono::App(C1, taus1), Mono::App(C2, taus2)) if C1 == C2 && taus1.len() == taus2.len() => {
                for (tau1, tau2) in taus1.into_iter().zip(taus2) {
                    self.unify(tau1, tau2)?
                }
                Ok(())
            }
            (Mono::Var(alpha), tau) | (tau, Mono::Var(alpha)) => {
                if tau.occurs(&alpha) {
                    Err(InferenceError::RecursiveType(tau, alpha))
                } else {
                    self.aliases.insert(alpha, tau);
                    Ok(())
                }
            }
            (tau1, tau2) => Err(InferenceError::ImpossibleUnification(tau1, tau2)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parse::{parse, parse_poly};

    use super::*;

    const EMPTY: Ctxt = Ctxt::new();

    #[test]
    #[allow(nonstandard_style)]
    fn test_var_from_context() {
        let C = parse_poly("C").unwrap();
        let Gamma = Ctxt::new().bind("x", C.clone());

        assert_eq!(infer(&parse("x").unwrap(), &Gamma), Ok(C));
    }

    #[test]
    fn test_var_not_in_context() {
        assert_eq!(
            infer(&parse("x").unwrap(), &EMPTY),
            Err(InferenceError::UnknownVar("x".into()))
        );
    }

    #[test]
    fn test_identity() {
        assert_eq!(
            infer(&parse("λ x . x").unwrap(), &EMPTY),
            Ok(parse_poly("∀ a . a → a").unwrap())
        );
    }

    #[test]
    #[allow(nonstandard_style)]
    fn test_unify_vars() {
        let Unifier = parse_poly("∀ x . x → x → x").unwrap();

        let Gamma = Ctxt::new().bind("unify", Unifier);

        assert_eq!(
            infer(&parse("λ x . λ y . unify x y").unwrap(), &Gamma),
            Ok(parse_poly("∀ a . a → a → a ").unwrap())
        );
    }

    #[test]
    #[allow(nonstandard_style)]
    fn test_unify_type_and_var() {
        let Unifier = parse_poly("∀ x . x → x → x").unwrap();
        let Int = parse_poly("Int").unwrap();
        let a = parse_poly("a").unwrap();

        let Gamma = Ctxt::new().bind("unify", Unifier).bind("n", Int.clone()).bind("x", a);

        assert_eq!(infer(&parse("unify n x").unwrap(), &Gamma), Ok(Int));
    }

    #[test]
    #[allow(nonstandard_style)]
    fn test_unify_identical_types() {
        let Unifier = parse_poly("∀ x . x → x → x").unwrap();
        let Int = parse_poly("Int").unwrap();

        let Gamma = Ctxt::new()
            .bind("unify", Unifier)
            .bind("n", Int.clone())
            .bind("m", Int.clone());

        assert_eq!(infer(&parse("unify n m").unwrap(), &Gamma), Ok(Int));
    }

    #[test]
    #[allow(nonstandard_style)]
    fn test_unify_distinct_types() {
        let Unifier = parse_poly("∀ x . x → x → x").unwrap();
        let Int = parse_poly("Int").unwrap();
        let String = parse_poly("String").unwrap();

        let Gamma = Ctxt::new().bind("unify", Unifier).bind("n", Int).bind("s", String);

        assert!(matches!(
            infer(&parse("unify n s").unwrap(), &Gamma),
            Err(InferenceError::ImpossibleUnification(..))
        ));
    }

    #[test]
    fn test_apply() {
        assert_eq!(
            infer(&parse("λ f . λ x . f x").unwrap(), &EMPTY),
            Ok(parse_poly("∀ a b . (a → b) → a → b").unwrap())
        )
    }

    #[test]
    fn test_apply_2() {
        assert_eq!(
            infer(&parse("λ f . λ x . x f").unwrap(), &EMPTY),
            Ok(parse_poly("∀ a b . a → (a → b) → b").unwrap())
        )
    }

    #[test]
    fn test_concat() {
        assert_eq!(
            infer(&parse("λ f . λ g . λ x . f (g x)").unwrap(), &EMPTY),
            Ok(parse_poly("∀ a b c . (b → c) → (a → b) → (a → c)").unwrap())
        )
    }

    #[test]
    #[allow(nonstandard_style)]
    fn test_specialize_let() {
        let Int = parse_poly("Int").unwrap();

        let Gamma = Ctxt::new().bind("n", Int.clone());

        assert_eq!(infer(&parse("let id = λ x . x in id n").unwrap(), &Gamma), Ok(Int));
    }

    #[test]
    fn test_recursive_unification() {
        assert!(matches!(
            infer(&parse("λ x . x x").unwrap(), &EMPTY),
            Err(InferenceError::RecursiveType(..))
        ));
    }
}
