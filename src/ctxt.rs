use std::collections::HashSet;

use crate::{
    expr::Var,
    types::{Poly, TypeVar},
};

#[derive(Clone)]
struct Binding(Var, Poly);

#[derive(Clone)]
pub struct Ctxt(Vec<Binding>);

impl Ctxt {
    pub const fn new() -> Ctxt {
        Ctxt(Vec::new())
    }

    pub fn free(&self) -> HashSet<TypeVar> {
        let Ctxt(bindings) = self;
        bindings.iter().flat_map(|Binding(_, sigma)| sigma.free()).collect()
    }

    pub fn get(&self, x: &Var) -> Option<&Poly> {
        let Ctxt(bindings) = self;
        bindings
            .iter()
            .rev()
            .filter_map(|Binding(y, sigma)| (x == y).then_some(sigma))
            .next()
    }

    pub fn bind(self, x: impl Into<Var>, sigma: Poly) -> Self {
        let Ctxt(mut bindings) = self;
        bindings.push(Binding(x.into(), sigma));
        Ctxt(bindings)
    }
}

impl std::fmt::Display for Ctxt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Ctxt(bindings) = self;

        if bindings.len() == 0 {
            write!(f, "")
        } else {
            write!(f, "{}", bindings[0])?;
            for binding in bindings.iter().skip(1) {
                write!(f, ", {}", binding)?;
            }
            Ok(())
        }
    }
}

impl std::fmt::Display for Binding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Binding(x, sigma) = self;
        write!(f, "{} : {}", x, sigma)
    }
}
