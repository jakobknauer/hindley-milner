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
pub struct Poly(pub HashSet<TypeVar>, pub Mono);

const ARROW: &str = "→";

impl Mono {
    pub fn arrow(tau1: Mono, tau2: Mono) -> Mono {
        Mono::App(String::from(ARROW), vec![tau1, tau2])
    }

    #[allow(nonstandard_style)]
    pub fn nullary(C: impl Into<String>) -> Mono {
        Mono::App(C.into(), Vec::new())
    }

    pub fn free(&self) -> HashSet<TypeVar> {
        match self {
            Mono::Var(alpha) => HashSet::from([alpha.clone()]),
            Mono::App(_, taus) => taus.iter().flat_map(|tau| tau.free()).collect(),
        }
    }

    #[allow(nonstandard_style)]
    pub fn generalize(&self, Gamma: &Ctxt) -> Poly {
        let alphas = &self.free() - &Gamma.free();
        Poly(alphas, self.clone())
    }

    #[allow(nonstandard_style)]
    pub fn replace(&self, alpha: &str, beta: &str) -> Mono {
        match self {
            Mono::Var(gamma) if gamma == alpha => Mono::Var(beta.into()),
            Mono::Var(_) => self.clone(),
            Mono::App(C, taus) => Mono::App(C.clone(), taus.iter().map(|tau| tau.replace(alpha, beta)).collect()),
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

    pub fn replace(&self, alpha: &str, beta: &str) -> Poly {
        let Poly(alphas, tau) = self;
        if alphas.contains(alpha) {
            Poly(alphas.clone(), tau.clone())
        } else {
            Poly(alphas.clone(), tau.replace(alpha, beta))
        }
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
            write!(f, ". {}", tau)
        }
    }
}
