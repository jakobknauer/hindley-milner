use std::collections::{HashMap, HashSet};

use crate::ctxt::Ctxt;

pub type TypeVar = String;
pub type TypeFunc = String;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Mono {
    Var(TypeVar),
    App(TypeFunc, Vec<Mono>),
}

#[derive(Clone, Debug)]
pub struct Poly(pub HashSet<TypeVar>, pub Mono);

const ARROW: &str = "→";

impl Mono {
    pub fn arrow(tau1: Mono, tau2: Mono) -> Mono {
        Mono::App(String::from(ARROW), vec![tau1, tau2])
    }

    #[allow(nonstandard_style)]
    pub fn nullary(C: impl Into<TypeFunc>) -> Mono {
        Mono::App(C.into(), Vec::new())
    }

    pub fn free(&self) -> HashSet<TypeVar> {
        match self {
            Mono::Var(alpha) => HashSet::from([alpha.clone()]),
            Mono::App(_, taus) => taus.iter().flat_map(|tau| tau.free()).collect(),
        }
    }

    #[allow(nonstandard_style)]
    pub fn generalize(self, Gamma: &Ctxt) -> Poly {
        let alphas = &self.free() - &Gamma.free();
        Poly(alphas, self)
    }

    #[allow(nonstandard_style)]
    pub fn canonicalize(self, aliases: &HashMap<TypeVar, Mono>) -> Mono {
        match self {
            Mono::Var(ref alpha) => match aliases.get(alpha) {
                Some(tau) => tau.clone().canonicalize(aliases),
                None => self,
            },
            Mono::App(C, taus) => Mono::App(C, taus.into_iter().map(|tau| tau.canonicalize(aliases)).collect()),
        }
    }

    #[allow(nonstandard_style)]
    pub fn replace(self, alpha: &str, beta: &Mono) -> Mono {
        match self {
            Mono::Var(gamma) if gamma == alpha => beta.clone(),
            Mono::Var(_) => self,
            Mono::App(C, taus) => Mono::App(C, taus.into_iter().map(|tau| tau.replace(alpha, beta)).collect()),
        }
    }

    pub fn occurs(&self, alpha: &str) -> bool {
        match self {
            Mono::Var(beta) => alpha == beta,
            Mono::App(_, taus) => taus.iter().any(|tau| tau.occurs(alpha)),
        }
    }
}

impl Poly {
    pub fn mono(tau: Mono) -> Poly {
        Poly(HashSet::new(), tau)
    }

    pub fn free(&self) -> HashSet<TypeVar> {
        let Poly(alphas, tau) = self;
        &tau.free() - &alphas
    }

    pub fn inst(self, new_vars: impl IntoIterator<Item = Mono>) -> Mono {
        let Poly(alphas, tau) = self;
        alphas
            .into_iter()
            .zip(new_vars)
            .fold(tau, |tau, (alpha, beta)| tau.replace(&alpha, &beta))
    }
}

impl std::fmt::Display for Mono {
    #[allow(nonstandard_style)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mono::Var(alpha) => write!(f, "{}", alpha),
            Mono::App(C, taus) if C == ARROW => {
                assert!(taus.len() == 2);
                write!(f, "({} {} {})", taus[0], ARROW, taus[1])
            }
            Mono::App(C, taus) if taus.len() == 0 => {
                write!(f, "{}", C)
            }
            Mono::App(C, taus) => {
                write!(f, "({}", C)?;
                for tau in taus {
                    write!(f, " {}", tau)?;
                }
                write!(f, ")")
            }
        }
    }
}

impl std::fmt::Display for Poly {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Poly(alphas, tau) = self;
        if alphas.is_empty() {
            tau.fmt(f)
        } else {
            write!(f, "∀")?;
            for alpha in alphas {
                write!(f, " {}", alpha)?;
            }
            write!(f, " . {}", tau)
        }
    }
}
