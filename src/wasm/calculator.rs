use std::collections::HashMap;

use crate::{parser, wasm::exp::JsExp, Error, Exp};
use serde::Serialize;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
// use web_sys::console;

#[derive(Clone, Serialize)]
enum Mutation {
    /// redex_id and alpha_id
    BetaReduce {
        redex: u32,
        alpha: u32,
    },
    EtaReduce {
        redex: u32,
        alpha: u32,
    },
    SubstAlpha {
        alpha: u32,
        name: String,
    },
}

#[derive(Clone, Serialize)]
struct Step {
    #[serde(skip)]
    raw_exp: Exp<String>,
    display_exp: JsExp,
    last_action: Option<Mutation>,
    /// id for vue
    id: String,
}

impl Step {
    /// Resolve beta-redex with `id` to get the following step
    ///
    /// Require mutable reference to mark the modified part of `display_exp`
    fn beta_reduce_by_id(&mut self, id: u32) -> Result<Step, Error> {
        let mut raw_exp = self.raw_exp.clone();
        let alpha_id = raw_exp.beta_reduce_by_id(&self.display_exp, id)?;
        self.last_action = Some(Mutation::BetaReduce {
            redex: id,
            alpha: alpha_id,
        });
        let display_exp = JsExp::from_exp(&raw_exp);
        let id = raw_exp.to_string();
        Ok(Self {
            raw_exp,
            display_exp,
            id,
            last_action: None,
        })
    }
    fn eta_reduce_by_id(&mut self, id: u32) -> Result<Step, Error> {
        let mut raw_exp = self.raw_exp.clone();
        let alpha_id = raw_exp.eta_reduce_by_id(&self.display_exp, id)?;
        self.last_action = Some(Mutation::EtaReduce {
            redex: id,
            alpha: alpha_id,
        });
        let display_exp = JsExp::from_exp(&raw_exp);
        let id = raw_exp.to_string();
        Ok(Self {
            raw_exp,
            display_exp,
            id,
            last_action: None,
        })
    }
    /// replace free variables with expression by alpha_id
    fn replace_by_alpha_id(
        &mut self,
        id: u32,
        name: String,
        exp: &Exp<String>,
    ) -> Result<Step, Error> {
        // console::debug_1(
        //     &format!("replace id={} name={} exp={}, self={:?}", id, name, exp, self.raw_exp)
        //         .as_str()
        //         .into(),
        // );
        let mut raw_exp = self.raw_exp.clone();
        let var = match raw_exp.find_var_by_alpha_id(&self.display_exp, id) {
            None => return Err(Error::VarNotFound(name, id)),
            Some(r) => r,
        };
        assert!(var.into_ident().unwrap().0 == name);
        *var = exp.to_owned();
        self.last_action = Some(Mutation::SubstAlpha { alpha: id, name });
        let display_exp = JsExp::from_exp(&raw_exp);
        let id = raw_exp.to_string();
        Ok(Self {
            raw_exp,
            display_exp,
            id,
            last_action: None,
        })
    }
}

/// Calculator handles the simplification of Lambda expression
#[wasm_bindgen]
pub struct Calculator {
    steps: Vec<Step>,
    defs: HashMap<String, Exp<String>>,
}

/// A calculator for lambda expression, supporting evaluation and simplification step by step.
#[wasm_bindgen]
impl Calculator {
    /// New instance
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            steps: Vec::new(),
            defs: HashMap::new(),
        }
    }
    /// Set initial expression
    pub fn init(&mut self, expr: &str) -> Result<(), String> {
        let (raw_exp, _) = parser::parse_exp(expr).map_err(|e| e.to_string())?;
        let wasm_exp = JsExp::from_exp(&raw_exp);
        let id = raw_exp.to_string();
        self.steps = vec![Step {
            raw_exp,
            id,
            display_exp: wasm_exp,
            last_action: None,
        }];
        Ok(())
    }
    fn trim_steps(&mut self, step_num: usize) -> Result<Step, String> {
        if step_num >= self.steps.len() {
            return Err(format!("invalid step {}", step_num));
        }
        let last = self.steps.swap_remove(step_num);
        while step_num < self.steps.len() {
            // remove succeeding steps
            self.steps.swap_remove(step_num);
        }
        Ok(last)
    }

    /// Resolve beta reduction for the `steps`-th expression
    pub fn beta_reduce(&mut self, step: usize, redex_id: u32) -> Result<(), String> {
        let mut last = self.trim_steps(step)?;
        let cur = last
            .beta_reduce_by_id(redex_id)
            .map_err(|e| format!("化简错误：{}", e))?;
        self.steps.push(last);
        self.steps.push(cur);
        Ok(())
    }
    /// Resolve beta reduction for the `steps`-th expression
    pub fn eta_reduce(&mut self, step: usize, id: u32) -> Result<(), String> {
        let mut last = self.trim_steps(step)?;
        let cur = last
            .eta_reduce_by_id(id)
            .map_err(|e| format!("化简错误：{}", e))?;
        self.steps.push(last);
        self.steps.push(cur);
        Ok(())
    }
    /// Replace free variable with alpha_id with corresponding expresion in defs
    pub fn replace_def_alpha(
        &mut self,
        step: usize,
        name: &str,
        alpha_id: u32,
    ) -> Result<(), String> {
        let mut last = self.trim_steps(step)?;
        let exp = self
            .defs
            .get(name)
            .ok_or(format!("expression not found name = {}", name))?;
        let cur = last
            .replace_by_alpha_id(alpha_id, name.to_string(), exp)
            .map_err(|e| format!("化简错误：{}", e))?;
        self.steps.push(last);
        self.steps.push(cur);
        Ok(())
    }
    /// add definitions from multiline content
    pub fn add_defs(&mut self, content: &str) -> Result<(), String> {
        let (map, _) = parser::parse_file(content).map_err(|e| e.to_string())?;
        for (name, exp) in map {
            self.defs.insert(name, exp);
        }
        Ok(())
    }
    /// Get all steps
    ///
    /// return `Step`
    pub fn history(&self) -> Result<JsValue, String> {
        serde_wasm_bindgen::to_value(&self.steps).map_err(|e| e.to_string())
    }
    /// Get all named definitions
    ///
    /// return `HashMap<String, JsExp>`
    pub fn get_defs(&self) -> Result<JsValue, String> {
        let mut jsmap = HashMap::new();
        for (name, exp) in &self.defs {
            jsmap.insert(name, JsExp::from_exp(exp));
        }
        serde_wasm_bindgen::to_value(&jsmap).map_err(|e| e.to_string())
    }
}

impl Default for Calculator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::Calculator;

    #[test]
    fn test_calculator() -> Result<(), String> {
        let mut calc = Calculator::new();
        calc.init("I y")?;
        calc.add_defs(
            r"
            I = \x. x
            K = \x. \y. x
        ",
        )?;

        Ok(())
    }
}
