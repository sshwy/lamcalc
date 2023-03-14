use crate::{Exp, Error};
use std::hash::Hash;

/// maximum number of reductions in a simplification
pub const SIMPLIFY_LIMIT: i32 = 1 << 7;

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
    pub(crate) fn eval_normal_order(&mut self) -> bool {
        match self {
            Exp::Var(_) => false,
            Exp::Abs(_, body) => body.eval_normal_order(),
            Exp::App(l, body) => {
                // let l2 = l.clone();
                if let Exp::Abs(_, abs_body) = &mut **l {
                    abs_body.subst_de(1, &body);
                    // eprintln!("将 \x1b[32m{}\x1b[0m 带入 \x1b[32m{}\x1b[0m", body, l2);
                    *self = *abs_body.clone();
                    true
                } else if l.eval_normal_order() {
                    true
                } else {
                    body.eval_normal_order()
                }
            }
        }
    }
}
