use std::collections::HashSet;

pub type TypeVar = String;
pub type TypeFunc = String;

pub enum Mono {
    Var(TypeVar),
    App(TypeFunc, Vec<Mono>),
}

pub enum Poly {
    Mono(Mono),
    Quant(TypeVar, Box<Poly>),
}

impl Mono {
    pub fn free(&self) -> HashSet<TypeVar> {
        match self {
            Mono::Var(alpha) => HashSet::from([alpha.clone()]),
            Mono::App(_, taus) => taus.iter().flat_map(|tau| tau.free()).collect(),
        }
    }
}

impl Poly {
    pub fn free(&self) -> HashSet<TypeVar> {
        match self {
            Poly::Mono(sigma) => sigma.free(),
            Poly::Quant(alpha, sigma) => &sigma.free() - &HashSet::from([alpha.clone()]),
        }
    }
}
