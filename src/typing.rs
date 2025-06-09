use std::collections::HashSet;

use crate::ctxt;
use crate::expr;
use crate::types;

pub struct Typing(pub ctxt::Ctxt, pub expr::Expr, pub types::Poly);

impl Typing {
    pub fn free(self) -> HashSet<String> {
        let Typing(Gamma, _, sigma) = self;
        &sigma.free() - &Gamma.free()
    }
}
