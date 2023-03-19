use crate::{Exp, Error};
use std::hash::Hash;

/// maximum number of reductions in a simplification
pub const SIMPLIFY_LIMIT: i32 = 1 << 10;

impl<T> Exp<T>
where
    T: Clone + Eq + Hash + ToString,
{
    /// Simplify repeatedly using beta-reduction in normal order
    /// for at most [`SIMPLIFY_LIMIT`] times.
    pub fn simplify(&mut self) -> Result<&mut Self, Error> {
        for _ in 0..SIMPLIFY_LIMIT {
            if !self.eval_normal_order() {
                return Ok(self)
            }
        }
        Err(Error::SimplifyLimitExceeded)
    }

    /// The leftmost, outermost redex is always reduced first.
    /// That is, whenever possible the arguments are substituted into
    /// the body of an abstraction before the arguments are reduced.
    ///
    /// return `false` if nothing changes, otherwise `true`.
    pub fn eval_normal_order(&mut self) -> bool {
        if self.beta_reduce() {
            return true;
        }
        match self {
            Exp::Var(_) => false,
            Exp::Abs(_, body) => body.eval_normal_order(),
            Exp::App(l, body) => {
                if l.eval_normal_order() {
                    true
                } else {
                    body.eval_normal_order()
                }
            }
        }
    }
}
