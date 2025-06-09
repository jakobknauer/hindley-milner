use std::collections::HashSet;

use crate::expr;
use crate::types;

pub struct Binding(pub expr::Var, pub types::Poly);

pub struct Ctxt(pub Vec<Binding>);

impl Ctxt {
    pub fn free(self) -> HashSet<types::TypeVar> {
        let Ctxt(bindings) = self;
        bindings.iter().flat_map(|Binding(_, sigma)| sigma.free()).collect()
    }
}
