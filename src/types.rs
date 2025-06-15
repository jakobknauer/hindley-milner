use std::collections::{HashMap, HashSet};

use crate::ctxt::Ctxt;

pub type TypeVar = String;
pub type TypeFunc = String;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Mono {
    Var(TypeVar),
    App(TypeFunc, Vec<Mono>),
}

#[derive(Clone, Debug, Eq)]
pub struct Poly(HashSet<TypeVar>, Mono);

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

mod poly_eq {
    use super::*;

    impl PartialEq for Poly {
        fn eq(&self, other: &Self) -> bool {
            poly_eq::PolyEq {
                sigma1: self,
                sigma2: other,
                left_to_right: HashMap::new(),
                right_to_left: HashMap::new(),
            }
            .eq()
        }
    }

    struct PolyEq<'a> {
        sigma1: &'a Poly,
        sigma2: &'a Poly,
        left_to_right: HashMap<&'a String, &'a String>,
        right_to_left: HashMap<&'a String, &'a String>,
    }

    impl<'a> PolyEq<'a> {
        fn eq(&mut self) -> bool {
            let Poly(_, tau1) = self.sigma1;
            let Poly(_, tau2) = self.sigma2;
            self.structurally_equal(tau1, tau2)
        }

        #[allow(nonstandard_style)]
        fn structurally_equal(&mut self, tau1: &'a Mono, tau2: &'a Mono) -> bool {
            let Poly(alphas1, ..) = self.sigma1;
            let Poly(alphas2, ..) = self.sigma2;

            match (tau1, tau2) {
                // alpha1 and alpha2 are bound variables of sigma1 and sigma2, respectively
                (Mono::Var(alpha1), Mono::Var(alpha2)) if alphas1.contains(alpha1) && alphas2.contains(alpha2) => {
                    match (self.left_to_right.get(alpha1), self.right_to_left.get(alpha2)) {
                        (None, None) => {
                            self.left_to_right.insert(alpha1, alpha2);
                            self.right_to_left.insert(alpha2, alpha1);
                            true
                        }
                        (Some(beta2), Some(beta1)) if beta1 == &alpha1 && beta2 == &alpha2 => true,
                        _ => false,
                    }
                }
                // alpha1 and alpha2 are not bound in sigma1 and sigma2, respectively
                (Mono::Var(alpha1), Mono::Var(alpha2)) if !alphas1.contains(alpha1) && !alphas2.contains(alpha2) => {
                    alpha1 == alpha2
                }
                // both sides are an application of equal structure
                (Mono::App(C1, taus1), Mono::App(C2, taus2)) if C1 == C2 && taus1.len() == taus2.len() => taus1
                    .into_iter()
                    .zip(taus2.into_iter())
                    .all(|(tau1, tau2)| self.structurally_equal(tau1, tau2)),
                _ => false,
            }
        }
    }
}
