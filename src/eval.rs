#[cfg(feature = "experimental")]
use std::sync::Mutex;

use crate::{Error, Exp};

/// Profile data (experimental)
#[cfg(feature = "experimental")]
pub struct ProfileData {
    pub beta_reduce_counter: u64,
    pub eta_reduce_counter: u64,
    pub eval_fn_counter: u64,
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
    pub fn simplify(&mut self, optimize: bool) -> Result<&mut Self, Error> {
        #[cfg(feature = "experimental")]
        GLOBAL_PROFILE.lock().unwrap().reset_counter();
        for _ in 0..SIMPLIFY_LIMIT {
            if !self.eval_normal_order(false, optimize) {
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
    pub fn eval_normal_order(&mut self, eta_reduce: bool, optimize: bool) -> bool {
        #[cfg(feature = "experimental")]
        GLOBAL_PROFILE.lock().unwrap().inc_eval_fn_counter();

        if optimize {
            if self.try_add_opt_1() {
                return true;
            }
            if self.try_add_opt_2() {
                return true;
            }
            if self.try_mul_opt() {
                return true;
            }
        }

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
            Exp::Abs(_, body) => body.eval_normal_order(eta_reduce, optimize),
            Exp::App(l, body) => {
                if l.eval_normal_order(eta_reduce, optimize) {
                    true
                } else {
                    body.eval_normal_order(eta_reduce, optimize)
                }
            }
        }
    }
}

// Church encoding optimization
mod optimize {
    use crate::{lambda, Exp};

    const PURE_MUL: std::cell::OnceCell<Exp<()>> = std::cell::OnceCell::new();

    impl<T> Exp<T>
    where
        T: Clone + Eq,
    {
        fn try_into_church_num(&self) -> Option<(u64, T, T)> {
            let (f, body) = self.into_abs()?;
            let (x, mut body) = body.into_abs()?;
            let mut val = 0;
            while let Some((func, app_body)) = body.into_app() {
                let f = func.into_ident()?;
                if f.1 != 2 {
                    return None;
                }
                val += 1;
                body = app_body;
            }
            if body.into_ident()?.1 != 1 {
                return None;
            }
            Some((val, f.0.clone(), x.0.clone()))
        }
        fn from_church_num(num: u64, f: T, x: T) -> Self {
            let mut cur = Exp::Var(crate::Ident(x.clone(), 1));
            for _ in 0..num {
                cur = Exp::App(
                    Box::new(Exp::Var(crate::Ident(f.clone(), 2))),
                    Box::new(cur),
                );
            }
            cur = Exp::Abs(crate::Ident(x, 0), Box::new(cur));
            cur = Exp::Abs(crate::Ident(f, 0), Box::new(cur));
            cur
        }
        fn is_add(&self) -> bool {
            let add = lambda!(n. m. f. x. n f (m f x)).purify();
            self.purify() == add
        }
        /// Check if the function is `add k`
        fn is_add_k(&self) -> Option<u64> {
            let (_m, body) = self.into_abs()?;
            let (_f, body) = body.into_abs()?;
            let (_x, mut body) = body.into_abs()?;
            let mut val = 0;
            while let Some((func, app_body)) = body.into_app() {
                if let Some(f) = func.into_ident() {
                    if f.1 != 2 {
                        return None;
                    }
                    val += 1;
                    body = app_body;
                } else if let Some((m1, f1)) = func.into_app() {
                    let m1 = m1.into_ident()?;
                    let f1 = f1.into_ident()?;
                    if m1.1 != 3 || f1.1 != 2 {
                        return None;
                    } else {
                        return Some(val);
                    }
                } else {
                    return None;
                }
            }
            None
        }
        fn is_mul(&self) -> bool {
            let binding = PURE_MUL;
            let value = binding.get_or_init(|| lambda!(n. m. f. x. n (m f) x).purify());
            &self.purify() == value
        }
    }
    impl<T> Exp<T>
    where
        T: Clone + Eq,
    {
        /// Try to apply add. optimization, return false if nothing changed
        ///
        /// The target form is `(add a) b`
        pub fn try_add_opt_1(&mut self) -> bool {
            let mut inner = || {
                let (add, a) = self.into_app_mut()?;
                if !add.is_add() {
                    return None;
                }

                let (va, f, x) = a.try_into_church_num()?;
                let m = add.into_abs()?.1.into_abs()?.0 .0.clone();

                eprintln!("add opt 1: va = {va}");
                // construct add a
                let mut cur = Exp::App(
                    Box::new(Exp::Var(crate::Ident(m.clone(), 3))),
                    Box::new(Exp::Var(crate::Ident(f.clone(), 2))),
                );
                cur = Exp::App(
                    Box::new(cur),
                    Box::new(Exp::Var(crate::Ident(x.clone(), 1))),
                );
                for _ in 0..va {
                    cur = Exp::App(
                        Box::new(Exp::Var(crate::Ident(f.clone(), 2))),
                        Box::new(cur),
                    );
                }
                cur = Exp::Abs(crate::Ident(x, 0), Box::new(cur));
                cur = Exp::Abs(crate::Ident(f, 0), Box::new(cur));
                cur = Exp::Abs(crate::Ident(m, 0), Box::new(cur));

                *self = cur;
                Some(())
            };
            inner().is_some()
        }

        /// Try to apply add. optimization, return false if nothing changed
        ///
        /// The target form is `(add a) b`
        pub fn try_add_opt_2(&mut self) -> bool {
            let mut inner = || {
                let (add_a, b) = self.into_app_mut()?;
                let vb = b.try_into_church_num()?;
                // eprintln!("try add opt: match b = {}", vb.0);
                // eprintln!("try add opt: add_a = {}", add_a.purify());
                let va = add_a.is_add_k()?;
                eprintln!("add opt: va = {va}, vb = {}", vb.0);
                let s = Self::from_church_num(va + vb.0, vb.1, vb.2);
                *self = s;
                Some(())
            };
            inner().is_some()
        }
        /// Try to apply add. optimization, return false if nothing changed
        ///
        /// The target form is `(add a) b`
        pub fn try_mul_opt(&mut self) -> bool {
            let mut inner = || {
                let (mul_a, b) = self.into_app_mut()?;
                let vb = b.try_into_church_num()?;
                let (mul, a) = mul_a.into_app_mut()?;
                if !mul.is_mul() {
                    return None;
                }
                let va = a.try_into_church_num()?;
                let s = Self::from_church_num(va.0 * vb.0, va.1, va.2);
                *self = s;
                Some(())
            };
            inner().is_some()
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::{lambda, Exp};

        #[test]
        fn test_church() {
            assert!(
                lambda!(f. x. f (f (f x)))
                    .purify()
                    .try_into_church_num()
                    .unwrap()
                    .0
                    == 3
            );
            assert!(lambda!(f. x. f (f (x x)))
                .purify()
                .try_into_church_num()
                .is_none());

            assert!(
                Exp::from_church_num(10, (), ())
                    .try_into_church_num()
                    .unwrap()
                    .0
                    == 10
            );

            let add = lambda!(n. m. f. x. n f (m f x));
            for i in 0..5 {
                let mut e = lambda!({add} {Exp::from_church_num(i, "f".into(), "x".into())});
                e.simplify(true).unwrap();
                eprintln!("{}", e);
            }
        }
        #[test]
        fn test_add_opt() {
            let a = Exp::from_church_num(10, "f", "x").to_string_exp();
            let b = Exp::from_church_num(15, "f", "x").to_string_exp();
            let add = lambda!(n. m. f. x. n f (m f x));
            // let add_a = lambda!({add} {a}).simplify(false).unwrap().to_owned();
            let mut e = lambda!({add} {a} {b});
            eprintln!("{}", e);
            while e.eval_normal_order(false, true) {
                eprintln!("{}", e);
            }
            assert_eq!(e.try_into_church_num().unwrap().0, 25)
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[cfg(feature = "experimental")]
    fn bench_pred() -> Result<(), crate::Error> {
        use crate::eval::GLOBAL_PROFILE;

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
