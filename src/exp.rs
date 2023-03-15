use std::{fmt::Write, hash::Hash};

use serde_derive::Serialize;

/// 变量的标识符，可以在输出 lamda 表达式的时候更 human-readable.
///
/// 第一个参数是标识符本身，第二个是变量的 ID，这里采用
/// [De Bruijn index](https://en.wikipedia.org/wiki/De_Bruijn_index)
/// 实现，在做 beta 规约的时候会方便很多。
/// 如果为 0 那么就是自由变量/unbounded（lambda 函数的参数）。
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct Ident<T: Clone + Eq + Hash + ToString>(pub T, pub usize);

/// lambda 演算中的表达式.
///
/// T 是变量的标识符类型
///
/// 使用 `{}` 则输出简洁的 lambda 表达式，
/// 使用 `{:#}` 可以输出 De Bruijn index，
/// 使用 `{:-}` 输出 De Bruijn encoding 格式
///
/// 使用 [`lambda`](crate::lambda) macro 可以以快速创建 lambda 表达式
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum Exp<T: Clone + Eq + Hash + ToString> {
    /// 一个变量
    Var(Ident<T>),
    /// 一个函数的定义 (abstraction)
    Abs(Ident<T>, Box<Exp<T>>),
    /// 函数的应用 (application)
    App(Box<Exp<T>>, Box<Exp<T>>),
}

impl<T: Clone + Eq + Hash + ToString> Exp<T> {
    /// 标识符与 var 相同的 unbounded 变量绑定为 var
    ///
    /// de_bruijn_index 的初值为 0
    pub(crate) fn bind(&mut self, id: &T, de_bruijn_index: usize) -> &mut Self {
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
    /// 进行标识符的替换
    /// 在不允许表达式中出现自由变量的情况下（遇到了就忽略），被替换的变量
    /// 的 de_bruijn_index 总是 >0，并且我们总是将某个 abstraction 的参数
    /// 进行替换。因此只用记 de_bruijn_index 即可。
    pub(crate) fn subst_de(&mut self, de_index: usize, exp: &Exp<T>) -> &mut Self {
        // dbg!(de_index);
        // eprintln!("{:#}", self);
        match self {
            Exp::Var(Ident(_, de)) => {
                if de_index == *de {
                    *self = exp.clone();
                }
            }
            Exp::Abs(_, body) => {
                body.subst_de(de_index + 1, exp);
            }
            Exp::App(l, body) => {
                l.subst_de(de_index, exp);
                body.subst_de(de_index, exp);
            }
        };
        self
    }
}

impl<T: Clone + Eq + Hash + ToString> std::fmt::Display for Exp<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Exp::Var(ident) => {
                if f.alternate() {
                    write!(f, "{}<{}>", ident.0.to_string(), ident.1)
                } else if f.sign_minus() {
                    write!(f, "{}", ident.1)
                } else {
                    f.write_str(&ident.0.to_string())
                }
            }
            Exp::Abs(ident, exp) => {
                if f.sign_minus() {
                    f.write_char('λ')?;
                } else {
                    write!(f, "λ{}. ", ident.0.to_string())?;
                }

                if f.alternate() {
                    write!(f, "{:#}", exp)
                } else if f.sign_minus() {
                    write!(f, "{:-}", exp)
                } else {
                    write!(f, "{}", exp)
                }
            }
            Exp::App(l, exp) => {
                // 如果 l 是 lambda 那么要加括号
                let l_str = if f.alternate() {
                    format!("{:#}", l)
                } else if f.sign_minus() {
                    format!("{:-}", l)
                } else {
                    format!("{}", l)
                };
                let exp_str = if f.alternate() {
                    format!("{:#}", exp)
                } else if f.sign_minus() {
                    format!("{:-}", exp)
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
