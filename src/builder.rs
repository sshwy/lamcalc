//! Conveniently build Lambda expressions.
use crate::exp::{Exp, Ident};
use std::hash::Hash;

impl<T: Clone + Eq + Hash + ToString> Exp<T> {
    /// 标识符与 var 相同的 unbounded 变量绑定为 var
    ///
    /// de_bruijn_index 的初值为 0
    fn bind(&mut self, id: &T, de_bruijn_index: usize) -> &mut Self {
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
/// 创建一个函数应用的表达式（左结合）
#[doc(hidden)]
pub fn app<T>(exps: Vec<Exp<T>>) -> Exp<T>
where
    T: Clone + Eq + Hash + ToString,
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

/// 创建一个函数表达式，自动捕获 `exp` 中的同名变量（除了同名参数的 abs 内部）
#[doc(hidden)]
pub fn abs<T>(v: T, exp: Exp<T>) -> Exp<T>
where
    T: Clone + Eq + Hash + ToString,
{
    let mut exp = exp;
    exp.bind(&v, 1); // 下面有 abstraction 故初值为 1
    Exp::Abs(Ident(v, 0), Box::new(exp))
}

/// 创建一个变量表达式
#[doc(hidden)]
pub fn unbounded_var<T>(v: T) -> Exp<T>
where
    T: Clone + Eq + Hash + ToString,
{
    Exp::Var(Ident(v, 0))
}

#[doc(hidden)]
pub fn exp_var<T>(exp: &Exp<T>) -> Exp<T>
where
    T: Clone + Eq + Hash + ToString,
{
    exp.clone()
}

// The keyword metavariable $crate can be used to refer to the current crate;
/// Build lambda expression with [`String`] identifier conveniently.
/// Generally:
/// 
/// 1. Dot `.` can be used to define abstraction.
/// 2. Parentheses can be used to denote subexpression.
/// 3. Application is left associated by default.
/// 
/// Checkout examples in home page.
#[macro_export]
macro_rules! lambda {
    // 消掉外层括号
    [($( $t:tt )+)] => {
        lambda![$( $t )+]
    };
    // variable expression
    [{$v:expr}] => {
        $crate::builder::exp_var(&$v)
    };
    // variable
    [$v:ident] => {
        $crate::builder::unbounded_var::<&str>(stringify!($v))
    };
    // abstraction
    [$v:ident.$( $t:tt )+] => {
        $crate::builder::abs(stringify!($v), lambda![$( $t )+])
    };
    // application
    [$l:tt $( $t:tt )+] => {
        $crate::builder::app(vec![lambda![$l], $( lambda![$t] ),+])
    };
}
