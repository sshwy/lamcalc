#[cfg(feature = "experimental")]
use std::sync::Mutex;

use crate::{Error, Exp};

/// Profile data (experimental)
#[cfg(feature = "experimental")]
pub struct ProfileData {
    pub beta_reduce_counter: usize,
    pub eta_reduce_counter: usize,
    pub eval_fn_counter: usize,
}
#[cfg(feature = "experimental")]
impl ProfileData {
    fn reset_counter(&mut self) {
        self.beta_reduce_counter = 0;
        self.eta_reduce_counter = 0;
        self.eval_fn_counter = 0;
    }
    fn inc_beta_counter(&mut self) {
        self.beta_reduce_counter += 1;
    }
    fn inc_eta_counter(&mut self) {
        self.eta_reduce_counter += 1;
    }
    fn inc_eval_fn_counter(&mut self) {
        self.eval_fn_counter += 1;
    }
}

#[cfg(feature = "experimental")]
pub static GLOBAL_PROFILE: Mutex<ProfileData> = Mutex::new(ProfileData {
    beta_reduce_counter: 0,
    eta_reduce_counter: 0,
    eval_fn_counter: 0,
});

/// maximum number of reductions in a simplification
pub const SIMPLIFY_LIMIT: i32 = 1 << 10;

impl<T> Exp<T>
where
    T: Clone + Eq,
{
    /// Simplify repeatedly using beta-reduction in normal order
    /// for at most [`SIMPLIFY_LIMIT`] times.
    pub fn simplify(&mut self) -> Result<&mut Self, Error> {
        #[cfg(feature = "experimental")]
        GLOBAL_PROFILE.lock().unwrap().reset_counter();
        for _ in 0..SIMPLIFY_LIMIT {
            if !self.eval_normal_order(false) {
                return Ok(self);
            }
        }
        Err(Error::SimplifyLimitExceeded)
    }

    /// The leftmost, outermost redex is always reduced first.
    /// That is, whenever possible the arguments are substituted into
    /// the body of an abstraction before the arguments are reduced.
    ///
    /// return `false` if nothing changes, otherwise `true`.
    pub fn eval_normal_order(&mut self, eta_reduce: bool) -> bool {
        #[cfg(feature = "experimental")]
        GLOBAL_PROFILE.lock().unwrap().inc_eval_fn_counter();
        if self.beta_reduce() {
            #[cfg(feature = "experimental")]
            GLOBAL_PROFILE.lock().unwrap().inc_beta_counter();
            return true;
        }
        if eta_reduce && self.eta_reduce() {
            #[cfg(feature = "experimental")]
            GLOBAL_PROFILE.lock().unwrap().inc_eta_counter();
            return true;
        }
        match self {
            Exp::Var(_) => false,
            Exp::Abs(_, body) => body.eval_normal_order(eta_reduce),
            Exp::App(l, body) => {
                if l.eval_normal_order(eta_reduce) {
                    true
                } else {
                    body.eval_normal_order(eta_reduce)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[cfg(feature = "experimental")]
    fn bench_pred() -> Result<(), crate::Error> {
        use crate::eval::GLOBAL_PROFILE;
        use crate::lambda;

        let suc = lambda!(n. f. x. f (n f x));
        let prev = lambda!(n. f. x. n (g. h. h (g f)) (u. x) (u. u));

        let mut nats = vec![lambda!(f. (x. x))];
        for i in 1..101 {
            let sx = lambda!({suc} {nats[i - 1]}).simplify()?.to_owned();
            nats.push(sx);
            assert_eq!(
                lambda!({prev} {nats[i]}).simplify()?.to_string(),
                nats[i - 1].to_string()
            );
        }

        GLOBAL_PROFILE.lock().unwrap().reset_counter();
        let mut exp = lambda!({prev} {nats[100]});
        while exp.eval_normal_order(true) {}
        {
            let data = GLOBAL_PROFILE.lock().unwrap();
            eprintln!(
                "beta_reduce = {}, eta_reduce = {}, eval = {} exp = {}",
                data.beta_reduce_counter,
                data.eta_reduce_counter,
                data.eval_fn_counter,
                exp.purify()
            );
        }

        Ok(())
    }
}
