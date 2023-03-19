use std::collections::HashMap;

use crate::{parser, wasm::exp::JsExp, Error, Exp};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[derive(Clone)]
struct Step {
    raw_exp: Exp<String>,
    display_exp: JsExp,
    last_reduce: Option<usize>,
    replaced_name: Option<String>,
}

impl Step {
    /// Resolve beta-redex with `id` to get the following step
    ///
    /// Require mutable reference to mark the modified part of `display_exp`
    fn reduce_beta_redex(&mut self, id: usize) -> Result<Step, Error> {
        let mut raw_exp = self.raw_exp.clone();
        raw_exp.reduce_beta_redex(&self.display_exp, id)?;
        self.last_reduce = Some(id);
        let display_exp = JsExp::from_exp(&raw_exp);
        Ok(Self {
            raw_exp,
            display_exp,
            last_reduce: None,
            replaced_name: None,
        })
    }
    /// replace free variables with expression by name
    fn replace_free_variable(&mut self, name: &str, exp: &Exp<String>) -> Result<Step, Error> {
        let mut raw_exp = self.raw_exp.clone();
        let name = name.to_string();
        raw_exp.subst_unbounded(&name, exp);
        self.replaced_name = Some(name);
        let display_exp = JsExp::from_exp(&raw_exp);
        Ok(Self {
            raw_exp,
            display_exp,
            last_reduce: None,
            replaced_name: None,
        })
    }
    fn to_quadruple(&self) -> (&JsExp, Option<usize>, Option<String>, String) {
        (
            &self.display_exp,
            self.last_reduce,
            self.replaced_name.clone(),
            self.raw_exp.to_string(),
        )
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
        let (exp, _) = parser::parse_exp(expr).map_err(|e| e.to_string())?;
        let wasm_exp = JsExp::from_exp(&exp);
        self.steps = vec![Step {
            raw_exp: exp,
            display_exp: wasm_exp,
            last_reduce: None,
            replaced_name: None,
        }];
        Ok(())
    }
    /// Resolve beta reduction for the `steps`-th expression
    pub fn beta_reduce(&mut self, step: usize, redex_id: usize) -> Result<(), String> {
        if step >= self.steps.len() {
            return Err(format!("invalid step {}", step));
        }
        let mut last = self.steps.swap_remove(step);
        while step < self.steps.len() {
            // remove succeeding steps
            self.steps.swap_remove(step);
        }
        let cur = last
            .reduce_beta_redex(redex_id)
            .map_err(|e| format!("化简错误：{}", e))?;
        self.steps.push(last);
        self.steps.push(cur);
        Ok(())
    }
    /// Replace free variable with `name` with corresponding expresion in defs
    pub fn replace_def_occurrance(&mut self, step: usize, name: &str) -> Result<(), String> {
        if step >= self.steps.len() {
            return Err(format!("invalid step {}", step));
        }
        let exp = self
            .defs
            .get(name)
            .ok_or(format!("expression not found name = {}", name))?;
        let mut last = self.steps.swap_remove(step);
        while step < self.steps.len() {
            // remove succeeding steps
            self.steps.swap_remove(step);
        }
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
    /// return `(JsExp, Option<usize>, Option<String>, String)[]`
    pub fn history(&self) -> Result<JsValue, String> {
        let res: Vec<_> = self.steps.iter().map(Step::to_quadruple).collect();
        serde_wasm_bindgen::to_value(&res).map_err(|e| e.to_string())
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
        calc.add_defs(r"
            I = \x. x
            K = \x. \y. x
        ")?;

        Ok(())
    }
}