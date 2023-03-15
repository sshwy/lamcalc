//! <span class="feat-badge" style="color: chocolate; font-weight: bold; background: blanchedalmond; padding: 0 5px; border-radius: 5px; display: inline-block;">feature: wasm</span> WASM bindings for this library.

use serde::Serialize;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::{
    parser::{self, Token},
    Exp,
};

#[derive(Serialize)]
struct ParseExpResult {
    exp: Exp<String>,
    tokens: Vec<Token>,
}

/// 解析一个 lambda 表达式，返回结果 json str 或者错误 json
#[wasm_bindgen]
pub fn parse_exp(lambda: &str) -> Result<JsValue, String> {
    let (exp, tokens) = parser::parse_exp(lambda).map_err(|e| e.to_string())?;
    serde_wasm_bindgen::to_value(&ParseExpResult { exp, tokens }).map_err(|e| e.to_string())
}
