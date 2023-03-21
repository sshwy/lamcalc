use std::collections::HashMap;

use crate::{parser, wasm::exp::JsExp, Error, Exp};
use serde::Serialize;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[derive(Clone, Serialize)]
enum Mutation {
    /// redex_id and alpha_id
    BetaReduce {
        redex: usize,
        alpha: usize,
    },
    EtaReduce {
        redex: usize,
        alpha: usize,
    },
    SubstUnbounded(String),
}

#[derive(Clone, Serialize)]
struct Step {
    #[serde(skip)]
    raw_exp: Exp<String>,
    display_exp: JsExp,
    last_action: Option<Mutation>,
    // last_reduce: Option<(usize, usize)>,
    // replaced_name: Option<String>,
    /// id for vue
    id: String,
}

impl Step {
    /// Resolve beta-redex with `id` to get the following step
    ///
    /// Require mutable reference to mark the modified part of `display_exp`
    fn beta_reduce_by_id(&mut self, id: usize) -> Result<Step, Error> {
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
    fn eta_reduce_by_id(&mut self, id: usize) -> Result<Step, Error> {
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
    /// replace free variables with expression by name
    fn replace_free_variable(&mut self, name: &str, exp: &Exp<String>) -> Result<Step, Error> {
        let mut raw_exp = self.raw_exp.clone();
        let name = name.to_string();
        raw_exp.subst_unbounded(&name, exp);
        self.last_action = Some(Mutation::SubstUnbounded(name));
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

/// 实现 lambda 表达式的分布计算、化简
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
    pub fn beta_reduce(&mut self, step: usize, redex_id: usize) -> Result<(), String> {
        let mut last = self.trim_steps(step)?;
        let cur = last
            .beta_reduce_by_id(redex_id)
            .map_err(|e| format!("化简错误：{}", e))?;
        self.steps.push(last);
        self.steps.push(cur);
        Ok(())
    }
    /// Resolve beta reduction for the `steps`-th expression
    pub fn eta_reduce(&mut self, step: usize, id: usize) -> Result<(), String> {
        let mut last = self.trim_steps(step)?;
        let cur = last
            .eta_reduce_by_id(id)
            .map_err(|e| format!("化简错误：{}", e))?;
        self.steps.push(last);
        self.steps.push(cur);
        Ok(())
    }
    /// Replace free variable with `name` with corresponding expresion in defs
    pub fn replace_def_occurrance(&mut self, step: usize, name: &str) -> Result<(), String> {
        let mut last = self.trim_steps(step)?;
        let exp = self
            .defs
            .get(name)
            .ok_or(format!("expression not found name = {}", name))?;
        let cur = last
            .replace_free_variable(name, exp)
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
