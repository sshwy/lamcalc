use crate::{parser, wasm::exp::JsExp, Error, Exp};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[derive(Clone)]
struct Step {
    raw_exp: Exp<String>,
    display_exp: JsExp,
    last_reduce: Option<usize>,
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
        })
    }
    fn to_triple(&self) -> (&JsExp, Option<usize>, String) {
        (&self.display_exp, self.last_reduce, self.raw_exp.to_string())
    }
}

/// Calculator handles the simplification of Lambda expression
#[wasm_bindgen]
pub struct Calculator {
    steps: Vec<Step>,
}

/// 实现 lambda 表达式的分布计算、化简
#[wasm_bindgen]
impl Calculator {
    /// New instance
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { steps: Vec::new() }
    }
    /// Set initial expression
    pub fn init(&mut self, expr: &str) -> Result<(), String> {
        let (exp, _) = parser::parse_exp(expr).map_err(|e| e.to_string())?;
        let wasm_exp = JsExp::from_exp(&exp);
        self.steps = vec![Step {
            raw_exp: exp,
            display_exp: wasm_exp,
            last_reduce: None,
        }];
        Ok(())
    }
    /// Resolve beta reduction for the `steps`-th expression
    pub fn beta_reduce(&mut self, step: usize, redex_id: usize) -> Result<(), String> {
        if step >= self.steps.len() {
            return Err(format!("invalid step {}", step));
        }
        let mut last = self.steps.swap_remove(step);
        while step < self.steps.len() { // remove succeeding steps
            self.steps.swap_remove(step);
        }
        let cur = last
            .reduce_beta_redex(redex_id)
            .map_err(|e| format!("化简错误：{}", e))?;
        self.steps.push(last);
        self.steps.push(cur);
        Ok(())
    }
    /// Get all steps
    ///
    /// return `(JsExp, Option<usize>, String)[]`
    pub fn history(&self) -> Result<JsValue, String> {
        let res: Vec<_> = self.steps.iter().map(Step::to_triple).collect();
        serde_wasm_bindgen::to_value(&res)
            .map_err(|e| e.to_string())
    }
}

impl Default for Calculator {
    fn default() -> Self {
        Self::new()
    }
}
