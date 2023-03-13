use crate::exp::{Exp, Ident};
use std::hash::Hash;

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

pub fn abs<T>(v: T, exp: Exp<T>) -> Exp<T>
where
    T: Clone + Eq + Hash + ToString,
{
    let mut exp = exp;
    exp.bind(&v, 1); // 下面有 abstraction 故初值为 1
    Exp::Abs(Ident(v, 0), Box::new(exp))
}

pub fn unbounded_var<T>(v: T) -> Exp<T>
where
    T: Clone + Eq + Hash + ToString,
{
    Exp::Var(Ident(v, 0))
}

pub fn exp_var<T>(exp: &Exp<T>) -> Exp<T>
where
    T: Clone + Eq + Hash + ToString,
{
    exp.clone()
}

/// 快速构建以 string 为标识符的 lambda 表达式
// The keyword metavariable $crate can be used to refer to the current crate;
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