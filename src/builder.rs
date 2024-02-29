//! Conveniently build Lambda expressions.
use crate::exp::{Exp, Ident};

impl<T: Clone + Eq> Exp<T> {
    /// 标识符与 var 相同的 unbounded 变量绑定为 var
    ///
    /// de_bruijn_index 的初值为 0
    fn bind(&mut self, id: &T, de_bruijn_index: u32) -> &mut Self {
        match self {
            Exp::Var(var) => {
                if var.0 == *id {
                    var.1 = de_bruijn_index
                }
            }
            Exp::Abs(var, exp) => {
                if var.0 != *id {
                    exp.bind(id, de_bruijn_index + 1);
                }
            }
            Exp::App(l, exp) => {
                l.bind(id, de_bruijn_index);
                exp.bind(id, de_bruijn_index);
            }
        };
        self
    }
}

#[doc(hidden)]
pub fn app<T>(exps: Vec<Exp<T>>) -> Exp<T>
where
    T: Clone + Eq,
{
    let mut res = None;
    for exp in exps {
        res = match res {
            Some(l) => Some(Exp::App(Box::new(l), Box::new(exp))),
            None => Some(exp),
        }
    }
    res.unwrap()
}

#[doc(hidden)]
pub fn abs<T>(v: T, exp: Exp<T>) -> Exp<T>
where
    T: Clone + Eq,
{
    let mut exp = exp;
    exp.bind(&v, 1); // 下面有 abstraction 故初值为 1
    Exp::Abs(Ident(v, 0), Box::new(exp))
}

#[doc(hidden)]
pub fn unbounded_var<T>(v: T) -> Exp<T>
where
    T: Clone + Eq,
{
    Exp::Var(Ident(v, 0))
}

/// Build lambda expression with [`String`] identifier conveniently.
/// Generally:
///
/// 1. Dot `.` can be used to define abstraction.
/// 2. Parentheses can be used to denote subexpression.
/// 3. Application is left associated by default.
///
/// If you find an expression not parsed, try adding parentheses to subexpressions.
///
/// For examples please checkout the home page.
#[macro_export]
macro_rules! lambda {
    // 消掉外层括号
    [($( $t:tt )+)] => {
        lambda![$( $t )+]
    };
    // variable expression
    [{$v:expr}] => {
        $v.clone()
    };
    // variable
    // The keyword metavariable $crate can be used to refer to the current crate;
    [$v:ident] => {
        $crate::builder::unbounded_var::<String>(String::from(stringify!($v)))
    };
    // abstraction
    [$v:ident.$( $t:tt )+] => {
        $crate::builder::abs(String::from(stringify!($v)), lambda![$( $t )+])
    };
    // application
    [$l:tt $( $t:tt )+] => {
        $crate::builder::app(vec![lambda![$l], $( lambda![$t] ),+])
    };
}

#[cfg(test)]
mod tests {
    use crate::builder::{abs, app, unbounded_var};

    #[test]
    fn test_builder() {
        let and = lambda!(x. (y. x y x));
        assert_eq!(
            and,
            abs(
                String::from("x"),
                abs(
                    String::from("y"),
                    app(vec![
                        unbounded_var(String::from("x")),
                        unbounded_var(String::from("y")),
                        unbounded_var(String::from("x")),
                    ])
                )
            )
        );
    }
}
