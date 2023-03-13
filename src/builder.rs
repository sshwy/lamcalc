//! 快速创建 lambda 表达式的辅助函数和 macro.
use crate::exp::{Exp, Ident};
use std::hash::Hash;

/// 创建一个函数应用的表达式（左结合）
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
pub fn abs<T>(v: T, exp: Exp<T>) -> Exp<T>
where
    T: Clone + Eq + Hash + ToString,
{
    let mut exp = exp;
    exp.bind(&v, 1); // 下面有 abstraction 故初值为 1
    Exp::Abs(Ident(v, 0), Box::new(exp))
}

/// 创建一个变量表达式
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
/// 快速构建以 string 为标识符的 lambda 表达式.
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
