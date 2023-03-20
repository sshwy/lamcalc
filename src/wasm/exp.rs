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
    /// alpha-equivalence setoid id
    pub alpha_id: usize,
}

#[derive(Serialize, Debug, Clone)]
/// abstraction data
pub struct Abs {
    /// identifier
    pub ident: String,
    /// setoid id
    pub alpha_id: usize,
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
                    alpha_id: 0,
                }),
            },
            crate::Exp::Abs(id, body) => Self {
                parentheses: false,
                inner: InnerExp::Abs(Abs {
                    ident: id.0.clone(),
                    alpha_id: 0,
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
    fn for_each_captured_by<'a, F>(&'a mut self, de: usize, f: F)
    where
        F: Fn(&mut Var) -> () + Clone,
    {
        match &mut self.inner {
            InnerExp::Var(var) => {
                if var.code == de {
                    f(var);
                }
            }
            InnerExp::Abs(abs) => abs.body.for_each_captured_by(de + 1, f),
            InnerExp::App(app) => {
                app.func.for_each_captured_by(de, f.clone());
                app.body.for_each_captured_by(de, f);
            }
        }
    }
    /// 初始化后添加括号，添加 alpha-equivalence setoid id
    fn decorate(
        &mut self,
        is_app_func: bool,
        is_app_body: bool,
        is_tail: bool,
        redex_counter: &mut usize,
        setoid_counter: &mut usize,
    ) {
        match &mut self.inner {
            InnerExp::Var(_) => {}
            InnerExp::Abs(abs) => {
                abs.in_beta_redex = is_app_func;
                if is_app_func || !is_tail {
                    self.parentheses = true;
                }
                abs.body.decorate(
                    false,
                    false,
                    self.parentheses || is_tail,
                    redex_counter,
                    setoid_counter,
                );
                *setoid_counter = *setoid_counter + 1;
                abs.alpha_id = *setoid_counter;
                abs.body.for_each_captured_by(1, |v| {
                    v.alpha_id = *setoid_counter;
                });
            }
            InnerExp::App(app) => {
                if is_app_body {
                    self.parentheses = true;
                }
                app.func
                    .decorate(true, false, false, redex_counter, setoid_counter);
                app.body.decorate(
                    false,
                    true,
                    self.parentheses || is_tail,
                    redex_counter,
                    setoid_counter,
                );
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
        let mut setoid_counter = 0;
        exp.decorate(false, false, true, &mut redex_counter, &mut setoid_counter);
        exp
    }
    #[doc(hidden)]
    pub fn into_var(&mut self) -> &mut Var {
        match &mut self.inner {
            InnerExp::Var(var) => var,
            _ => panic!("not var"),
        }
    }
    #[doc(hidden)]
    pub fn into_app(&mut self) -> &mut App {
        match &mut self.inner {
            InnerExp::App(app) => app,
            _ => panic!("not app"),
        }
    }
    #[doc(hidden)]
    pub fn into_abs(&mut self) -> &mut Abs {
        match &mut self.inner {
            InnerExp::Abs(abs) => abs,
            _ => panic!("not app"),
        }
    }
    fn into_abs_ref(&self) -> &Abs {
        match &self.inner {
            InnerExp::Abs(abs) => abs,
            _ => panic!("not app"),
        }
    }
}

impl Exp<String> {
    /// Resolve beta-redex based on it's `display_exp`
    ///
    /// this operation requires mutable reference of `display_exp`
    /// to mark the modified part.
    ///
    /// return the alpha_id of reduced part.
    pub(crate) fn reduce_beta_redex(
        &mut self,
        display_exp: &JsExp,
        id: usize,
    ) -> Result<usize, Error> {
        if let InnerExp::App(app) = &display_exp.inner {
            if let Some(beta_redex) = app.beta_redex {
                let alpha_id = app.func.into_abs_ref().alpha_id;
                if beta_redex == id {
                    if self.beta_reduce() {
                        // display_exp.marked = true;
                        return Ok(alpha_id);
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
                        Ok(alpha_id) => Ok(alpha_id),
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

#[cfg(test)]
mod tests {
    use crate::lambda;

    use super::JsExp;

    #[test]
    fn test_jsexp() {
        let exp = lambda!(x. x (x. x) (y. x (y. y) y));
        let mut jsexp = JsExp::from_exp(&exp);
        dbg!(&jsexp);
        {
            let id = jsexp.into_abs().alpha_id;
            assert_eq!(
                id,
                jsexp
                    .into_abs()
                    .body
                    .into_app()
                    .func
                    .into_app()
                    .func
                    .into_var()
                    .alpha_id
            );
            assert_eq!(
                id,
                jsexp
                    .into_abs()
                    .body
                    .into_app()
                    .body
                    .into_abs()
                    .body
                    .into_app()
                    .func
                    .into_app()
                    .func
                    .into_var()
                    .alpha_id
            );
        }
        {
            let abs = jsexp
                .into_abs()
                .body
                .into_app()
                .func
                .into_app()
                .body
                .into_abs();
            assert_eq!(abs.alpha_id, abs.body.into_var().alpha_id);
        }
    }
}
