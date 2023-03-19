#![warn(missing_docs)]
//! <span class="feat-badge" style="color: chocolate; font-weight: bold; background: blanchedalmond; padding: 0 5px; border-radius: 5px; display: inline-block;">feature: wasm</span> interprete lambda expressions in browser
use crate::parser;
use exp::JsExp;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

mod calculator;
pub mod exp;

pub use calculator::Calculator;

/// Parse Lambda expression
///
/// return ```(JsExp, Vec<Token>)```
#[wasm_bindgen]
pub fn parse_exp(lambda: &str) -> Result<JsValue, String> {
    let (exp, tokens) = parser::parse_exp(lambda).map_err(|e| e.to_string())?;
    let jsexp = JsExp::from_exp(&exp);
    serde_wasm_bindgen::to_value(&(jsexp, tokens)).map_err(|e| e.to_string())
}

/// Parse Lambda definition
///
/// return ```(String, JsExp, Vec<Token>)```
#[wasm_bindgen]
pub fn parse_def(lambda: &str) -> Result<JsValue, String> {
    let (name, exp, tokens) = parser::parse_def(lambda).map_err(|e| e.to_string())?;
    let jsexp = JsExp::from_exp(&exp);
    serde_wasm_bindgen::to_value(&(name, jsexp, tokens)).map_err(|e| e.to_string())
}
