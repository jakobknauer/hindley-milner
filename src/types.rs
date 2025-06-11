use std::collections::HashSet;

use crate::ctxt::Ctxt;

pub type TypeVar = String;
pub type TypeFunc = String;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Mono {
    Var(TypeVar),
    App(TypeFunc, Vec<Mono>),
}

#[derive(Clone, Debug)]
pub enum Poly {
    Mono(Mono),
    Quant(TypeVar, Box<Poly>),
}

const ARROW: &str = "→";

impl Mono {
    pub fn new_arrow(tau1: Mono, tau2: Mono) -> Mono {
        Mono::App(String::from(ARROW), vec![tau1, tau2])
    }

    pub fn free(&self) -> HashSet<TypeVar> {
        match self {
            Mono::Var(alpha) => HashSet::from([alpha.clone()]),
            Mono::App(_, taus) => taus.iter().flat_map(|tau| tau.free()).collect(),
        }
    }

    pub fn generalize(&self, Gamma: &Ctxt) -> Poly {
        let alphas = &self.free() - &Gamma.free();
        let mut poly = Poly::Mono(self.clone());
        for alpha in alphas {
            poly = Poly::new_quant(alpha, poly);
        }
        poly
    }

    fn replace(&self, alpha: &str, beta: &str) -> Mono {
        match self {
            Mono::Var(gamma) if gamma == alpha => Mono::Var(beta.into()),
            Mono::Var(_) => self.clone(),
            Mono::App(C, taus) => Mono::App(C.clone(), taus.iter().map(|tau| tau.replace(alpha, beta)).collect()),
        }
    }
}

impl Poly {
    pub fn new_quant(alpha: TypeVar, sigma: Poly) -> Poly {
        Poly::Quant(alpha, Box::new(sigma))
    }

    pub fn free(&self) -> HashSet<TypeVar> {
        match self {
            Poly::Mono(sigma) => sigma.free(),
            Poly::Quant(alpha, sigma) => &sigma.free() - &HashSet::from([alpha.clone()]),
        }
    }

    pub fn replace(self, alpha: &String, beta: &String) -> Poly {
        match self {
            Poly::Mono(tau) => Poly::Mono(tau.replace(alpha, beta)),
            Poly::Quant(ref gamma, _) if gamma == alpha => self,
            Poly::Quant(gamma, sigma) => Poly::new_quant(gamma, sigma.replace(alpha, beta)),
        }
    }
}
