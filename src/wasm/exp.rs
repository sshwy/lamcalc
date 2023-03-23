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
    /// whether eta reduceable
    pub eta_redex: Option<usize>,
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
                    eta_redex: None,
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
    fn for_each_captured_by<'a, F, D>(&'a mut self, de: usize, f: F, sum: Option<D>) -> Option<D>
    where
        F: Fn(&mut Var, Option<D>) -> Option<D> + Clone,
    {
        match &mut self.inner {
            InnerExp::Var(var) => {
                if var.code == de {
                    return f(var, sum);
                }
                return sum;
            }
            InnerExp::Abs(abs) => abs.body.for_each_captured_by(de + 1, f, sum),
            InnerExp::App(app) => {
                let sum = app.func.for_each_captured_by(de, f.clone(), sum);
                app.body.for_each_captured_by(de, f, sum)
            }
        }
    }
    /// 初始化后添加括号，添加 alpha-equivalence setoid id
    fn decorate(
        &mut self,
        is_app_func: bool,
        is_app_body: bool,
        is_tail: bool,
        counter: &mut usize,
        // setoid_counter: &mut usize,
    ) {
        match &mut self.inner {
            InnerExp::Var(var) => {
                // free variable
                if var.alpha_id == 0 {
                    *counter += 1;
                    var.alpha_id = *counter
                }
            }
            InnerExp::Abs(abs) => {
                abs.in_beta_redex = is_app_func;
                if is_app_func || !is_tail {
                    self.parentheses = true;
                }
                abs.body
                    .decorate(false, false, self.parentheses || is_tail, counter);
                *counter += 1;
                abs.alpha_id = *counter;
                abs.body.for_each_captured_by(
                    1,
                    |v, _| {
                        v.alpha_id = *counter;
                        None
                    },
                    Some(()),
                );
                if let InnerExp::App(app) = &mut abs.body.inner {
                    if app
                        .func
                        .for_each_captured_by(1, |_, _| Some(()), None)
                        .is_none()
                    {
                        if let InnerExp::Var(var) = &app.body.inner {
                            if var.code == 1 {
                                *counter = *counter + 1;
                                abs.eta_redex = Some(*counter);
                            }
                        }
                    }
                }
            }
            InnerExp::App(app) => {
                if is_app_body {
                    self.parentheses = true;
                }
                app.func.decorate(true, false, false, counter);
                app.body
                    .decorate(false, true, self.parentheses || is_tail, counter);
                if let InnerExp::Abs(_) = app.func.inner {
                    *counter += 1;
                    // *redex_counter = *redex_counter + 1;
                    app.beta_redex = Some(*counter)
                }
            }
        }
    }
    pub(crate) fn from_exp(expr: &crate::Exp<String>) -> Self {
        let mut exp = Self::init_exp(expr);
        let mut counter = 0;
        // let mut setoid_counter = 0;
        exp.decorate(false, false, true, &mut counter);
        exp
    }
    fn into_app_ref(&self) -> Result<&App, Error> {
        match &self.inner {
            InnerExp::App(app) => Ok(app),
            _ => Err(Error::InvalidInnerType),
        }
    }
    fn into_abs_ref(&self) -> Result<&Abs, Error> {
        match &self.inner {
            InnerExp::Abs(abs) => Ok(abs),
            _ => Err(Error::InvalidInnerType),
        }
    }
    fn into_var(&self) -> Result<&Var, Error> {
        match &self.inner {
            InnerExp::Var(v) => Ok(v),
            _ => Err(Error::InvalidInnerType),
        }
    }
}

impl Exp<String> {
    // fn walk_reduce<F, D>(&mut self, jsexp: &JsExp, f: F, init: D) -> D
    // where
    //     F: Fn(&mut Self, &JsExp, D) -> D + Clone,
    // {
    //     let data = f(self, jsexp, init);
    //     match self {
    //         Exp::Var(_) => data,
    //         Exp::Abs(_, body) => {
    //             let abs = jsexp.into_abs_ref().unwrap();
    //             body.walk_reduce(&abs.body, f, data)
    //         }
    //         Exp::App(func, body) => {
    //             let app = jsexp.into_app_ref().unwrap();
    //             let data = func.walk_reduce(&app.func, f.clone(), data);
    //             body.walk_reduce(&app.body, f, data)
    //         }
    //     }
    // }

    /// Resolve beta-redex based on it's `display_exp`
    ///
    /// this operation requires mutable reference of `display_exp`
    /// to mark the modified part.
    ///
    /// return the alpha_id of reduced part.
    pub(crate) fn beta_reduce_by_id(
        &mut self,
        display_exp: &JsExp,
        id: usize,
    ) -> Result<usize, Error> {
        if let InnerExp::App(app) = &display_exp.inner {
            if let Some(beta_redex) = app.beta_redex {
                let alpha_id = app.func.into_abs_ref()?.alpha_id;
                if beta_redex == id {
                    if self.beta_reduce() {
                        // display_exp.marked = true;
                        return Ok(alpha_id);
                    }
                    return Err(Error::InvalidRedex(id, self.to_string()));
                }
            }
        }
        match self {
            Exp::Var(_) => Err(Error::RedexNotFound),
            Exp::Abs(_, body) => body.beta_reduce_by_id(&display_exp.into_abs_ref()?.body, id),
            Exp::App(func, body) => {
                let app = display_exp.into_app_ref()?;
                match func.beta_reduce_by_id(&app.func, id) {
                    Err(Error::RedexNotFound) => body.beta_reduce_by_id(&app.body, id),
                    r => r,
                }
            }
        }
    }
    /// return the alpha_id of reduced part.
    pub(crate) fn eta_reduce_by_id(
        &mut self,
        display_exp: &JsExp,
        id: usize,
    ) -> Result<usize, Error> {
        if let Exp::Abs(_, _) = self {
            let abs = display_exp.into_abs_ref()?;
            if abs.eta_redex.is_some() && abs.eta_redex.unwrap() == id {
                if self.eta_reduce() {
                    return Ok(abs.alpha_id);
                }
                return Err(Error::InvalidRedex(id, format!("{:?}", self)));
            }
        }
        match self {
            Exp::Var(_) => Err(Error::RedexNotFound),
            Exp::Abs(_, body) => body.eta_reduce_by_id(&display_exp.into_abs_ref()?.body, id),
            Exp::App(func, body) => {
                let app = display_exp.into_app_ref()?;
                match func.eta_reduce_by_id(&app.func, id) {
                    Err(Error::RedexNotFound) => body.eta_reduce_by_id(&app.body, id),
                    r => r,
                }
            }
        }
    }
    pub(crate) fn find_var_by_alpha_id(
        &mut self,
        display_exp: &JsExp,
        id: usize,
    ) -> Option<&mut Self> {
        match self {
            Exp::Var(_) => {
                if let Ok(var) = display_exp.into_var() {
                    if var.alpha_id == id {
                        return Some(self);
                    }
                }
                None
            }
            Exp::Abs(_, body) => {
                let abs = display_exp.into_abs_ref().unwrap();
                body.find_var_by_alpha_id(&abs.body, id)
            }
            Exp::App(func, body) => {
                let app = display_exp.into_app_ref().unwrap();
                match func.find_var_by_alpha_id(&app.func, id) {
                    None => body.find_var_by_alpha_id(&app.body, id),
                    r => r,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lambda;

    use super::{Abs, App, InnerExp, JsExp, Var};
    impl JsExp {
        fn into_var_mut(&mut self) -> &mut Var {
            match &mut self.inner {
                InnerExp::Var(var) => var,
                _ => panic!("not var"),
            }
        }
        fn into_app_mut(&mut self) -> &mut App {
            match &mut self.inner {
                InnerExp::App(app) => app,
                _ => panic!("not app"),
            }
        }
        fn into_abs_mut(&mut self) -> &mut Abs {
            match &mut self.inner {
                InnerExp::Abs(abs) => abs,
                _ => panic!("not app"),
            }
        }
    }

    #[test]
    fn test_jsexp() {
        let exp = lambda!(x. x (x. x) (y. x (y. y) y));
        let mut jsexp = JsExp::from_exp(&exp);
        dbg!(&jsexp);
        {
            let id = jsexp.into_abs_mut().alpha_id;
            assert_eq!(
                id,
                jsexp
                    .into_abs_mut()
                    .body
                    .into_app_mut()
                    .func
                    .into_app_mut()
                    .func
                    .into_var_mut()
                    .alpha_id
            );
            assert_eq!(
                id,
                jsexp
                    .into_abs_mut()
                    .body
                    .into_app_mut()
                    .body
                    .into_abs_mut()
                    .body
                    .into_app_mut()
                    .func
                    .into_app_mut()
                    .func
                    .into_var_mut()
                    .alpha_id
            );
        }
        {
            let abs = jsexp
                .into_abs_mut()
                .body
                .into_app_mut()
                .func
                .into_app_mut()
                .body
                .into_abs_mut();
            assert_eq!(abs.alpha_id, abs.body.into_var_mut().alpha_id);
        }
    }
}
