use crate::{parser, wasm::exp::JsExp, Exp};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

struct Step {
    raw_exp: Exp<String>,
    display_exp: JsExp,
    last_reduce: Option<usize>,
}

/// Calculator 可以处理 lambda 表达式每一步的化简过程
#[wasm_bindgen]
pub struct Calculator {
    steps: Vec<Step>,
}

/// 实现 lambda 表达式的分布计算、化简
#[wasm_bindgen]
impl Calculator {
    /// 新建一个 lambda 表达式计算器
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { steps: Vec::new() }
    }
    /// 从一个 lambda 表达式开始演算
    ///
    /// return initial JsExp
    pub fn init(&mut self, expr: &str) -> Result<JsValue, String> {
        let (exp, _) = parser::parse_exp(expr).map_err(|e| e.to_string())?;
        let wasm_exp = JsExp::from_exp(&exp);
        self.steps = vec![Step {
            raw_exp: exp,
            display_exp: wasm_exp,
            last_reduce: None,
        }];
        serde_wasm_bindgen::to_value(&self.steps[0].display_exp).map_err(|e| e.to_string())
    }
    /// 对某一步的表达式进行 beta reduce
    ///
    /// return [JsExp, String]
    pub fn beta_reduce(&mut self, step: usize, redex_id: usize) -> Result<JsValue, String> {
        if step >= self.steps.len() {
            return Err(format!("invalid step {}", step));
        }
        let cur = &self.steps[step];
        let mut exp = cur.raw_exp.clone();
        exp.reduce_beta_redex(&cur.display_exp, redex_id)
            .map_err(|e| format!("化简错误：{}", e))?;
        let display_exp = JsExp::from_exp(&exp);
        let item = Step {
            raw_exp: exp,
            display_exp,
            last_reduce: Some(redex_id),
        };
        let res = serde_wasm_bindgen::to_value(&(&item.display_exp, item.raw_exp.to_string()))
            .map_err(|e| e.to_string());
        println!("last_reduce: {}", &item.last_reduce.unwrap());
        self.steps.push(item);
        res
    }
    // return (Exp, Vec<Step>)
    // pub fn history() -> Result<JsValue, String> {
    //     todo!()
    // }
    // pub fn test() -> Exp {
    //     todo!()
    // }
}

impl Default for Calculator {
    fn default() -> Self {
        Self::new()
    }
}
