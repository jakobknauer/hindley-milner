use std::collections::HashMap;

use crate::ctxt::{Binding, Ctxt};
use crate::expr::Expr;
use crate::types::{Mono, Poly};

#[allow(nonstandard_style)]
pub fn infer(e: &Expr, Gamma: &Ctxt) -> Option<Poly> {
    let mut algorithm = Algorithm::new();
    let tau = algorithm.infer(e, &Gamma)?;
    let tau = algorithm.canonicalize(&tau);
    let tau = tau.generalize(&Gamma);
    Some(tau)
}

struct Algorithm {
    counter: u32,
    union_find: HashMap<Mono, Mono>,
}

impl Algorithm {
    pub fn new() -> Algorithm {
        Algorithm {
            counter: 0,
            union_find: HashMap::new(),
        }
    }

    #[allow(nonstandard_style)]
    pub fn infer(&mut self, e: &Expr, Gamma: &Ctxt) -> Option<Mono> {
        match e {
            Expr::Var(x) => Gamma.get(x).map(|sigma| self.inst(sigma)),
            Expr::App(e0, e1) => {
                let tau0 = self.infer(e0, Gamma)?;
                let tau1 = self.infer(e1, Gamma)?;
                let tau_prime = self.new_var();
                self.unify(&tau0, &Mono::arrow(tau1, tau_prime.clone()));
                Some(tau_prime)
            }
            Expr::Abs(x, e) => {
                let tau = self.new_var();
                let Gamma_prime = Gamma | Binding(x.clone(), Poly::mono(tau.clone()));
                let tau_prime = self.infer(e, &Gamma_prime)?;
                Some(Mono::arrow(tau, tau_prime))
            }
            Expr::Let(x, e0, e1) => {
                let tau = self.infer(e0, Gamma)?;
                let tau = self.canonicalize(&tau);
                let tau = tau.generalize(&Gamma);
                self.infer(e1, &(Gamma | Binding(x.clone(), tau)))
            }
        }
    }

    fn inst(&mut self, Poly(alphas, tau): &Poly) -> Mono {
        alphas
            .iter()
            .fold(tau.clone(), |tau, alpha| tau.replace(alpha, &self.new_var()))
    }

    // TODO: create actual fresh variables
    fn new_var(&mut self) -> Mono {
        self.counter += 1;
        let alpha = format!("x{}", self.counter);
        Mono::Var(alpha)
    }

    #[allow(nonstandard_style)]
    fn unify(&mut self, tau1: &Mono, tau2: &Mono) {
        let tau1 = self.canonicalize(tau1);
        let tau2 = self.canonicalize(tau2);

        if tau1 == tau2 {
            return;
        }

        match (&tau1, &tau2) {
            (Mono::App(C1, taus1), Mono::App(C2, taus2)) if C1 == C2 && taus1.len() == taus2.len() => {
                for (tau1, tau2) in taus1.iter().zip(taus2) {
                    self.unify(tau1, &tau2);
                }
            }
            (Mono::Var(_), _) => {
                self.union_find.insert(tau1, tau2);
            }
            (_, Mono::Var(_)) => {
                self.union_find.insert(tau2, tau1);
            }
            _ => panic!("Cannot unify types"),
        }
    }

    #[allow(nonstandard_style)]
    fn canonicalize(&mut self, tau: &Mono) -> Mono {
        match tau {
            Mono::Var(_) => match self.union_find.get(tau) {
                Some(tau) => self.canonicalize(&tau.clone()),
                None => tau.clone(),
            },
            Mono::App(C, taus) => Mono::App(C.clone(), taus.iter().map(|tau| self.canonicalize(tau)).collect()),
        }
    }
}
