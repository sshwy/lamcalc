//! <span class="feat-badge" style="color: chocolate; font-weight: bold; background: blanchedalmond; padding: 0 5px; border-radius: 5px; display: inline-block;">feature: wasm</span> WASM bindings for this library.

use wasm_bindgen::prelude::wasm_bindgen;

use crate::parser;

/// 解析一个 lambda 表达式，返回结果 json str 或者错误 json
#[wasm_bindgen]
pub fn parse_exp(lambda: &str) -> Result<String, String> {
    match parser::parse_exp(lambda) {
        Ok(exp) => match serde_json::to_string(&exp.0) {
            Ok(s) => Ok(s),
            Err(e) => Err(e.to_string()),
        },
        Err(e) => Err(e.to_string()),
    }
}
