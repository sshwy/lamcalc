#![warn(missing_docs)]
//! <span class="feat-badge" style="color: chocolate; font-weight: bold; background: blanchedalmond; padding: 0 5px; border-radius: 5px; display: inline-block;">feature: wasm</span> WASM bindings for this library.
use crate::parser::{self, Token};
use exp::JsExp;
use serde::Serialize;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

mod calculator;
mod exp;

pub use calculator::Calculator;

#[derive(Serialize)]
struct ParseExpResult(JsExp, Vec<Token>);

/// 解析一个 lambda 表达式
/// 
/// return ```(JsExp, Vec<Token>)```
#[wasm_bindgen]
pub fn parse_exp(lambda: &str) -> Result<JsValue, String> {
    let (exp, tokens) = parser::parse_exp(lambda).map_err(|e| e.to_string())?;
    let jsexp = JsExp::from_exp(&exp);
    serde_wasm_bindgen::to_value(&ParseExpResult(jsexp, tokens)).map_err(|e| e.to_string())
}
