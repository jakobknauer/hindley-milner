use std::collections::HashMap;

use crate::{
    ctxt::{Binding, Ctxt},
    expr::Expr,
    types::{Mono, Poly, TypeVar},
};

#[allow(nonstandard_style)]
pub fn infer(e: &Expr, Gamma: &Ctxt) -> Option<Poly> {
    let mut algorithm = AlgorithmJ::new();
    let tau = algorithm
        .infer(e, Gamma)?
        .canonicalize(&algorithm.aliases)
        .generalize(Gamma);
    Some(tau)
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
    pub fn infer(&mut self, e: &Expr, Gamma: &Ctxt) -> Option<Mono> {
        match e {
            Expr::Var(x) => {
                let sigma = Gamma.get(x)?;
                let tau = sigma.clone().inst(self.new_vars());
                Some(tau)
            }
            Expr::App(e0, e1) => {
                let tau0 = self.infer(e0, Gamma)?;
                let tau1 = self.infer(e1, Gamma)?;
                let tau_prime = self.new_var();
                self.unify(tau0, Mono::arrow(tau1, tau_prime.clone()));
                Some(tau_prime)
            }
            Expr::Abs(x, e) => {
                let tau = self.new_var();
                let Gamma_prime = Gamma | Binding(x.clone(), Poly::mono(tau.clone()));
                let tau_prime = self.infer(e, &Gamma_prime)?;
                Some(Mono::arrow(tau, tau_prime))
            }
            Expr::Let(x, e0, e1) => {
                let tau = self.infer(e0, Gamma)?.canonicalize(&self.aliases).generalize(Gamma);
                let Gamma_prime = Gamma | Binding(x.clone(), tau);
                let tau_prime = self.infer(e1, &Gamma_prime);
                tau_prime
            }
        }
    }

    // TODO: create actual fresh variables
    fn new_var(&mut self) -> Mono {
        self.counter += 1;
        let alpha = format!("x{}", self.counter);
        Mono::Var(alpha)
    }

    fn new_vars(&mut self) -> impl IntoIterator<Item = Mono> {
        std::iter::from_fn(|| Some(self.new_var()))
    }

    #[allow(nonstandard_style)]
    fn unify(&mut self, tau1: Mono, tau2: Mono) {
        let tau1 = tau1.canonicalize(&self.aliases);
        let tau2 = tau2.canonicalize(&self.aliases);

        match (tau1, tau2) {
            (tau1, tau2) if tau1 == tau2 => (),
            (Mono::App(C1, taus1), Mono::App(C2, taus2)) if C1 == C2 && taus1.len() == taus2.len() => {
                for (tau1, tau2) in taus1.into_iter().zip(taus2) {
                    self.unify(tau1, tau2);
                }
            }
            (Mono::Var(alpha), tau) | (tau, Mono::Var(alpha)) => {
                self.aliases.insert(alpha, tau);
            }
            (tau1, tau2) => panic!("cannot unify canonicalized types {tau1} and {tau2}"),
        }
    }
}
