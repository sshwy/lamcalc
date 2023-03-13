use crate::{Exp, Error};
use std::hash::Hash;

impl<T> Exp<T>
where
    T: Clone + Eq + Hash + ToString,
{
    /// 化简自身，最多化简 100 次
    pub fn simplify(&mut self) -> Result<(), Error> {
        for _ in 0..100 {
            if !self.eval_normal_order() {
                return Ok(())
            }
        }
        Err(Error::SimplifyLimitExceeded)
    }

    /// The leftmost, outermost redex is always reduced first.
    /// That is, whenever possible the arguments are substituted into
    /// the body of an abstraction before the arguments are reduced.
    ///
    /// 如果没有发生变化返回 false，否则返回 true
    pub fn eval_normal_order(&mut self) -> bool {
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
