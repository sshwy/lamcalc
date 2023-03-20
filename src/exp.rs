use std::fmt::Write;

/// Identifier of variables.
///
/// Adopt [De Bruijn index](https://en.wikipedia.org/wiki/De_Bruijn_index)
/// for the second field, where 0 is for free variables and others are for
/// bounded/captured variables.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ident<T: Clone + Eq>(pub T, pub usize);

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
    /// Substitute free variables with expression
    pub fn subst_unbounded(&mut self, name: &T, exp: &Exp<T>) -> &mut Self {
        match self {
            Exp::Var(Ident(ident, de)) => {
                if name == ident && *de == 0 {
                    *self = exp.clone();
                }
            }
            Exp::Abs(_, body) => {
                body.subst_unbounded(name, exp);
            }
            Exp::App(l, body) => {
                l.subst_unbounded(name, exp);
                body.subst_unbounded(name, exp);
            }
        };
        self
    }
    /// 进行标识符的替换
    /// 在不允许表达式中出现自由变量的情况下（遇到了就忽略），被替换的变量
    /// 的 de_bruijn_index 总是 >0，并且我们总是将某个 abstraction 的参数
    /// 进行替换。因此只用记 de_bruijn_index 即可。
    fn subst_de(&mut self, de_index: usize, exp: Exp<T>) -> &mut Self {
        match self {
            Exp::Var(Ident(_, de)) => {
                if de_index == *de {
                    *self = exp;
                }
            }
            Exp::Abs(_, body) => {
                let mut exp = exp;
                exp.push(0);
                body.subst_de(de_index + 1, exp);
            }
            Exp::App(l, body) => {
                l.subst_de(de_index, exp.clone());
                body.subst_de(de_index, exp);
            }
        };
        self
    }
    /// 将表达式中被**外部**捕获的变量的 code 都减少 1
    fn lift(&mut self, min_de: usize) {
        match self {
            Exp::Var(Ident(_, code)) => {
                if *code >= min_de {
                    *code = *code - 1;
                }
            }
            Exp::Abs(_, body) => {
                body.lift(min_de + 1);
            }
            Exp::App(func, body) => {
                func.lift(min_de);
                body.lift(min_de);
            }
        }
    }
    /// 将当前表达式进行抽象，或者放入某个抽象的子表达式，
    /// 这会导致被外部捕获的变量 的 de bruijn index + 1
    fn push(&mut self, cur_de: usize) {
        match self {
            Exp::Var(Ident(_, code)) => {
                if *code > cur_de {
                    *code = *code + 1;
                }
            }
            Exp::Abs(_, body) => body.push(cur_de + 1),
            Exp::App(func, body) => {
                func.push(cur_de);
                body.push(cur_de);
            }
        }
    }
    fn unwrap_abs(&mut self) {
        if let Exp::Abs(_, body) = self {
            *self = *body.to_owned();
        } else {
            panic!("not abs!")
        }
    }
    pub(crate) fn beta_reduce(&mut self) -> bool {
        if let Exp::App(func, body) = self {
            let mut is_redex = false;
            if let Exp::Abs(_, _) = &mut **func {
                is_redex = true;
            }
            if is_redex {
                let mut func = *func.to_owned();
                func.subst_de(0, *body.to_owned());
                func.lift(1);
                func.unwrap_abs();
                *self = func;
                return true;
            }
        }
        false
    }
}

// public methods
impl<T: Clone + Eq> Exp<T> {
    /// Remove name of indentifiers, keeping just de bruijn code.
    /// If there are free variables, they will become the same thing.
    pub fn purify(&self) -> Exp<()> {
        match self {
            Exp::Var(Ident(_, code)) => Exp::Var(Ident((), *code)),
            Exp::Abs(Ident(_, code), body) => Exp::Abs(Ident((), *code), Box::new(body.purify())),
            Exp::App(func, body) => Exp::App(Box::new(func.purify()), Box::new(body.purify())),
        }
    }
    /// return body for Abs or App, otherwise panic.
    pub fn into_body(&mut self) -> &mut Self {
        match self {
            Exp::Abs(_, body) | Exp::App(_, body) => &mut **body,
            _ => panic!("no body"),
        }
    }
    /// return identifer for Abs or Var, otherwise panic.
    pub fn into_ident(&mut self) -> &mut Ident<T> {
        match self {
            Exp::Var(ident) | Exp::Abs(ident, _) => ident,
            _ => panic!("no identifier"),
        }
    }
    /// return function sub expression for App, otherwise panic.
    pub fn into_func(&mut self) -> &mut Self {
        match self {
            Exp::App(func, _) => &mut **func,
            _ => panic!("no func"),
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
            Exp::Var(ident) => write!(f, "{}", ident.1),
            Exp::Abs(_, exp) => write!(f, "λ {}", exp),
            Exp::App(l, exp) => {
                // 如果 l 是 lambda 那么要加括号
                match **l {
                    Exp::Var(_) => write!(f, "{}", l),
                    _ => write!(f, "({})", l),
                }?;
                f.write_char(' ')?;
                match **exp {
                    Exp::App(_, _) => write!(f, "({})", exp),
                    _ => write!(f, "{}", exp),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lambda;

    #[test]
    fn subst() {
        let tt = lambda!(x. (y. x));
        let and = lambda!(x. y. x y x);

        let mut e = and.clone();
        e.subst_de(0, tt);
        assert_eq!(
            format!("{:#}", e),
            "λx. λy. ((λx. λy. x<2>) y<1>) λx. λy. x<2>"
        );
    }
}
