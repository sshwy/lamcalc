//! Expression data for browser
use serde::Serialize;

use crate::{Error, Exp};

#[derive(Serialize, Debug, Clone)]
/// variable data
pub struct Var {
    /// identifier
    pub ident: String,
    /// de bruijn code
    pub code: usize,
}

#[derive(Serialize, Debug, Clone)]
/// abstraction data
pub struct Abs {
    /// identifier
    pub ident: String,
    /// sub expression
    pub body: Box<JsExp>,
    /// whether it's in a beta-redex
    pub in_beta_redex: bool,
}
#[derive(Serialize, Debug, Clone)]
/// application data
pub struct App {
    /// the former sub expression
    pub func: Box<JsExp>,
    /// the latter sub expression
    pub body: Box<JsExp>,
    /// id of its beta_redex (greater than 0)
    pub beta_redex: Option<usize>,
}

#[derive(Serialize, Debug, Clone)]
/// typed data of Lambda expression
pub enum InnerExp {
    #[allow(missing_docs)]
    Var(Var),
    #[allow(missing_docs)]
    Abs(Abs),
    #[allow(missing_docs)]
    App(App),
}

/// Expression data in a fronend-friendly format
#[derive(Serialize, Debug, Clone)]
pub struct JsExp {
    /// whether wrapping parentheses
    pub parentheses: bool,
    /// inner type of this expression
    pub inner: InnerExp,
}

impl JsExp {
    fn init_exp(exp: &crate::Exp<String>) -> Self {
        match exp {
            crate::Exp::Var(v) => Self {
                parentheses: false,
                inner: InnerExp::Var(Var {
                    ident: v.0.clone(),
                    code: v.1,
                }),
            },
            crate::Exp::Abs(id, body) => Self {
                parentheses: false,
                inner: InnerExp::Abs(Abs {
                    ident: id.0.clone(),
                    body: Box::new(Self::init_exp(body)),
                    in_beta_redex: false,
                }),
            },
            crate::Exp::App(func, body) => Self {
                parentheses: false,
                inner: InnerExp::App(App {
                    func: Box::new(Self::init_exp(func)),
                    body: Box::new(Self::init_exp(body)),
                    beta_redex: None,
                }),
            },
        }
    }
    /// 初始化后添加括号
    fn decorate(
        &mut self,
        is_app_func: bool,
        is_app_body: bool,
        is_tail: bool,
        redex_counter: &mut usize,
    ) {
        match &mut self.inner {
            InnerExp::Var(_) => {}
            InnerExp::Abs(abs) => {
                abs.in_beta_redex = is_app_func;
                if is_app_func || !is_tail {
                    self.parentheses = true;
                }
                abs.body
                    .decorate(false, false, self.parentheses || is_tail, redex_counter);
            }
            InnerExp::App(app) => {
                if is_app_body {
                    self.parentheses = true;
                }
                app.func.decorate(true, false, false, redex_counter);
                app.body
                    .decorate(false, true, self.parentheses || is_tail, redex_counter);
                if let InnerExp::Abs(_) = app.func.inner {
                    *redex_counter = *redex_counter + 1;
                    app.beta_redex = Some(*redex_counter)
                }
            }
        }
    }
    pub(crate) fn from_exp(expr: &crate::Exp<String>) -> Self {
        let mut exp = Self::init_exp(expr);
        let mut redex_counter = 0;
        exp.decorate(false, false, true, &mut redex_counter);
        exp
    }
}

impl Exp<String> {
    /// Resolve beta-redex based on it's `display_exp`
    ///
    /// this operation requires mutable reference of `display_exp`
    /// to mark the modified part
    pub(crate) fn reduce_beta_redex(
        &mut self,
        display_exp: &JsExp,
        id: usize,
    ) -> Result<(), Error> {
        if let InnerExp::App(app) = &display_exp.inner {
            if let Some(beta_redex) = app.beta_redex {
                if beta_redex == id {
                    if self.beta_reduce() {
                        // display_exp.marked = true;
                        return Ok(());
                    }
                    return Err(Error::InvalidRedex);
                }
            }
        }
        match self {
            Exp::Var(_) => Err(Error::RedexNotFound),
            Exp::Abs(_, body) => {
                if let InnerExp::Abs(abs) = &display_exp.inner {
                    return body.reduce_beta_redex(&abs.body, id);
                }
                Err(Error::InvalidDisplayExp)
            }
            Exp::App(func, body) => {
                if let InnerExp::App(app) = &display_exp.inner {
                    match func.reduce_beta_redex(&app.func, id) {
                        Ok(_) => Ok(()),
                        Err(Error::RedexNotFound) => body.reduce_beta_redex(&app.body, id),
                        Err(e) => Err(e),
                    }
                } else {
                    Err(Error::InvalidDisplayExp)
                }
            }
        }
    }
}
