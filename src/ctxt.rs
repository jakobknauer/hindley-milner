use std::collections::HashSet;

use crate::expr::Var;
use crate::types::{Poly, TypeVar};

#[derive(Clone)]
pub struct Binding(pub Var, pub Poly);

pub struct Ctxt(pub Vec<Binding>);

impl Ctxt {
    pub fn new() -> Ctxt {
        Ctxt(Vec::new())
    }

    pub fn free(&self) -> HashSet<TypeVar> {
        let Ctxt(bindings) = self;
        bindings.iter().flat_map(|Binding(_, sigma)| sigma.free()).collect()
    }

    pub fn get(&self, x: &Var) -> Option<Poly> {
        let Ctxt(bindings) = self;
        bindings
            .iter()
            .filter_map(|Binding(y, sigma)| (x == y).then_some(sigma.clone()))
            .next()
    }
}

impl core::ops::BitOr<Binding> for &Ctxt {
    type Output = Ctxt;

    fn bitor(self, rhs: Binding) -> Self::Output {
        let Ctxt(bindings) = self;
        let mut bindings = bindings.clone();
        bindings.push(rhs);
        Ctxt(bindings)
    }
}
