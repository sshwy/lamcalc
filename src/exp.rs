use std::fmt::Write;

/// Identifier of variables.
///
/// Adopt [De Bruijn index](https://en.wikipedia.org/wiki/De_Bruijn_index)
/// for the second field, where 0 is for free variables and others are for
/// bounded/captured variables.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ident<T: Clone + Eq>(pub T, pub u32);

/// Expression in Lambda Calculus.
///
/// T represents the type of indentifiers.
///
/// Formatting:
///
/// - use `{}` for simple format
/// - use `{:#}` for extra De Bruijn index information
///
/// use [`lambda`](crate::lambda) macro to create lambda expression efficiently.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Exp<T: Clone + Eq> {
    /// 一个变量
    Var(Ident<T>),
    /// 一个函数的定义 (abstraction)
    Abs(Ident<T>, Box<Exp<T>>),
    /// 函数的应用 (application)
    App(Box<Exp<T>>, Box<Exp<T>>),
}

impl<T: Clone + Eq> Exp<T> {
    // iterate over each variable
    fn reduce_by_var_with_depth<F, D>(&mut self, f: F, depth: u32, sum: D) -> D
    where
        F: Fn(
                // variable expression
                &mut Exp<T>,
                // number of abstractions containing this variable
                u32,
                // result sum
                D,
            ) -> D
            + Clone,
    {
        match self {
            Exp::Var(_) => f(self, depth, sum),
            Exp::Abs(_, body) => body.reduce_by_var_with_depth(f, depth + 1, sum),
            Exp::App(func, body) => {
                let sum = func.reduce_by_var_with_depth(f.clone(), depth, sum);
                body.reduce_by_var_with_depth(f, depth, sum)
            }
        }
    }
    /// iterate over each variable
    /// the second parameter for f is `depth`: number of abstractions
    /// containing this variable.
    pub fn for_each_var<F>(&mut self, f: F)
    where
        F: Fn(&mut Exp<T>, u32) -> () + Clone,
    {
        self.reduce_by_var_with_depth(|v, dep, _| Some(f(v, dep)), 0, None);
    }
    /// Substitute free variables (de bruijn index = 0) with expression
    pub fn subst_unbounded(&mut self, name: &T, exp: &Exp<T>) -> &mut Self {
        self.for_each_var(|e, _| {
            if let Exp::Var(ident) = &e {
                if ident.0 == *name && ident.1 == 0 {
                    *e = exp.clone();
                }
            }
        });
        self
    }
    /// 进行标识符的替换
    /// 在不允许表达式中出现自由变量的情况下（遇到了就忽略），被替换的变量
    /// 的 de_bruijn_index 总是 >0，并且我们总是将某个 abstraction 的参数
    /// 进行替换。因此只用记 de_bruijn_index 即可。
    fn subst_de(&mut self, de_index: u32, exp: &Exp<T>) -> &mut Self {
        self.for_each_var(|v, dep| {
            if let Exp::Var(ident) = &v {
                if ident.1 == de_index + dep {
                    let mut exp = exp.clone();
                    exp.shift_outer_captured_var(dep as isize);
                    *v = exp;
                }
            }
        });
        self
    }
    /// alter the de bruijn index of outer captured variable
    /// by a uniform shift.
    ///
    /// 将当前表达式进行抽象，或者放入某个抽象的子表达式，
    /// 这会导致被外部捕获的变量 的 de bruijn index + 1
    fn shift_outer_captured_var(&mut self, shift: isize) {
        self.for_each_var(|v, dep| {
            let ident = v.into_ident().unwrap();
            if ident.1 > dep {
                if shift > 0 {
                    ident.1 = ident.1 + shift as u32
                } else {
                    ident.1 = ident.1 - (-shift) as u32
                }
            }
        });
    }
    /// Check whether current expression is a beta reduction.
    pub fn is_beta_redex(&mut self) -> bool {
        if let Exp::App(func, _) = self {
            if let Exp::Abs(_, _) = **func {
                return true;
            }
        }
        false
    }
    /// Try making beta reduction, return false if nothing changed.
    pub fn beta_reduce(&mut self) -> bool {
        if !self.is_beta_redex() {
            return false;
        }
        let (func, body) = self.into_app().unwrap();
        let mut func = func.to_owned();
        func.subst_de(0, body);
        func.shift_outer_captured_var(-1);
        *self = func.into_abs().unwrap().1.to_owned();
        true
    }
    /// Check whether current expression is a eta reduction.
    pub fn is_eta_redex(&mut self) -> bool {
        self.into_abs()
            .and_then(|body| {
                body.1.into_app().and_then(|(func, app_body)| {
                    Some(
                        app_body
                            .into_ident()
                            .and_then(|ident| Some(ident.1 == 1))
                            .unwrap_or(false)
                            && func.reduce_by_var_with_depth(
                                |v, dep, prev| prev && v.into_ident().unwrap().1 != 1 + dep,
                                0,
                                true,
                            ),
                    )
                })
            })
            .unwrap_or(false)
    }
    /// Eta reduce requires the function's extensionality axiom,
    /// thus is not enabled by default.
    pub fn eta_reduce(&mut self) -> bool {
        if !self.is_eta_redex() {
            return false;
        }
        let (_, body) = self.into_abs().unwrap();
        let (func, _) = body.into_app().unwrap();
        func.shift_outer_captured_var(-1); // func.lift(1);
        *self = func.to_owned();
        true
    }
    /// Remove name of indentifiers, keeping just de bruijn code.
    /// If there are free variables, they will become the same thing.
    pub fn purify(&self) -> Exp<()> {
        match self {
            Exp::Var(Ident(_, code)) => Exp::Var(Ident((), *code)),
            Exp::Abs(Ident(_, code), body) => Exp::Abs(Ident((), *code), Box::new(body.purify())),
            Exp::App(func, body) => Exp::App(Box::new(func.purify()), Box::new(body.purify())),
        }
    }
    /// return func and body for App.
    pub fn into_app(&mut self) -> Option<(&mut Self, &mut Self)> {
        match self {
            Exp::App(func, body) => Some((&mut **func, &mut **body)),
            _ => None,
        }
    }
    /// return body for Abs.
    pub fn into_abs(&mut self) -> Option<(&mut Ident<T>, &mut Self)> {
        match self {
            Exp::Abs(ident, body) => Some((ident, &mut **body)),
            _ => None,
        }
    }
    /// return identifer for Var.
    pub fn into_ident(&mut self) -> Option<&mut Ident<T>> {
        match self {
            Exp::Var(ident) => Some(ident),
            _ => None,
        }
    }
}

impl std::fmt::Display for Exp<String> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Exp::Var(ident) => {
                if f.alternate() {
                    write!(f, "{}<{}>", ident.0.to_string(), ident.1)
                } else {
                    f.write_str(&ident.0.to_string())
                }
            }
            Exp::Abs(ident, exp) => {
                write!(f, "λ{}. ", ident.0.to_string())?;

                if f.alternate() {
                    write!(f, "{:#}", exp)
                } else {
                    write!(f, "{}", exp)
                }
            }
            Exp::App(l, exp) => {
                // 如果 l 是 lambda 那么要加括号
                let l_str = if f.alternate() {
                    format!("{:#}", l)
                } else {
                    format!("{}", l)
                };
                let exp_str = if f.alternate() {
                    format!("{:#}", exp)
                } else {
                    format!("{}", exp)
                };
                match **l {
                    Exp::Var(_) => f.write_str(&l_str),
                    _ => {
                        f.write_char('(')?;
                        f.write_str(&l_str)?;
                        f.write_char(')')
                    }
                }?;
                f.write_char(' ')?;
                match **exp {
                    Exp::App(_, _) => {
                        f.write_char('(')?;
                        f.write_str(&exp_str)?;
                        f.write_char(')')
                    }
                    _ => f.write_str(&exp_str),
                }
            }
        }
    }
}

impl std::fmt::Display for Exp<()> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Exp::Var(ident) => ident.1.fmt(f),
            Exp::Abs(_, exp) => write!(f, "λ{}", exp),
            Exp::App(l, exp) => write!(f, "[{}]({})", l, exp),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{lambda, Error, Exp};

    #[test]
    fn subst() {
        let tt = lambda!(x. (y. x));
        let and = lambda!(x. y. x y x);

        let mut e = and.clone();
        e.subst_de(0, &tt);
        assert_eq!(
            format!("{:#}", e),
            "λx. λy. ((λx. λy. x<2>) y<1>) λx. λy. x<2>"
        );

        let mut exp = lambda!(z. x. (y. x y) z);
        {
            let x = exp.into_abs().unwrap().1.into_abs().unwrap().1;
            if let Exp::App(func, body) = x {
                func.subst_de(0, &body);
                func.shift_outer_captured_var(-1);
                // func.lift(1);
                *x = func.into_abs().unwrap().1.to_owned();
            }
        }
        assert_eq!(exp.purify().to_string(), "λλ[1](2)");
    }
    #[test]
    fn test_eta_reduce() {
        let mut exp = lambda!(x. y. f x y);
        assert!(exp.into_abs().unwrap().1.eta_reduce());
        assert!(exp.eta_reduce());
        assert_eq!(exp, lambda!(f));
        let mut exp2 = lambda!(x. x x);
        assert!(!exp2.eta_reduce());
    }
    #[test]
    fn test_subst_unbounded() -> Result<(), Error> {
        let mut exp = lambda!(x. y. f x y);
        exp.subst_unbounded(&String::from("f"), &lambda!(x. (y. z)));
        exp.simplify()?;
        assert_eq!(exp, lambda!(x. (y. z)));
        Ok(())
    }
    #[test]
    fn test_de_bruijn() {
        let pair = lambda!(x. y. f. f x y);
        assert_eq!(pair.purify().to_string(), "λλλ[[1](3)](2)");
    }
    #[test]
    fn test_beta_reduce() {
        let mut exp = lambda!((x. y. x) z);
        assert!(exp.is_beta_redex());
        assert!(exp.beta_reduce());
        assert_eq!(exp, lambda!(y.z));
    }
}
