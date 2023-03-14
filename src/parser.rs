#![allow(missing_docs)]
//! 解析 lambda 表达式.
//!
//! 使用 PEG 描述的语法规则如下（`~`表示拼接，`+` 表示一次或更多，`?` 表示 0 次或一次，`|` 表示选择）：
//!
//! ```pest
#![doc = include_str!("./grammar.pest")]
//! ```
//!
use std::collections::HashMap;

use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

use crate::{builder, Error, Exp};

/// 使用 [pest](https://pest.rs/) 构建的 lambda 表达式解析器
#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct LambdaParser;

fn build_lambda(tt: Pair<Rule>) -> Result<Exp<String>, Error> {
    match tt.as_rule() {
        Rule::ident => Ok(builder::unbounded_var(tt.as_span().as_str().to_string())),
        Rule::abs => {
            let mut it = tt.into_inner();
            let id = it.next().unwrap();
            let body = it.next().unwrap();
            Ok(builder::abs(
                id.as_span().as_str().to_string(),
                build_lambda(body)?,
            ))
        }
        Rule::app => {
            let items: Result<Vec<Exp<String>>, Error> =
                tt.into_inner().map(|exp| build_lambda(exp)).collect();
            Ok(builder::app(items?))
        }

        Rule::exp => build_lambda(tt.into_inner().next().unwrap()),
        Rule::bounded_exp => build_lambda(tt.into_inner().next().unwrap()),
        Rule::tail_exp => build_lambda(tt.into_inner().next().unwrap()),

        _ => Err(Error::InvalidRule(tt.to_string())),
    }
}

/// 只接受 def 规则的表达式
fn build_def(tt: Pair<Rule>) -> Result<(String, Exp<String>), Error> {
    if let Rule::def = tt.as_rule() {
        let mut def = tt.into_inner().to_owned();
        let ident = def.next().unwrap().as_span().as_str().to_string();
        let exp = def.next().unwrap();

        let lam = build_lambda(exp)?;
        return Ok((ident, lam));
    }
    Err(Error::InvalidRule(tt.to_string()))
}

/// 解析一个 lambda 表达式
pub fn parse_exp(lambda: &str) -> Result<Exp<String>, Error> {
    let exp = LambdaParser::parse(Rule::exp, lambda)
        .map_err(|e| Error::InvalidRule(e.to_string()))?
        .next()
        .unwrap();
    build_lambda(exp)
}

/// 解析单行 lambda 表达式的定义
pub fn parse_def(lambda: &str) -> Result<(String, Exp<String>), Error> {
    let def = LambdaParser::parse(Rule::def, lambda)
        .map_err(|e| Error::InvalidRule(e.to_string()))?
        .next()
        .unwrap();
    build_def(def)
}

/// 解析多行定义
pub fn parse_multiline(lambda: &str) -> Result<HashMap<String, Exp<String>>, Error> {
    let lines = LambdaParser::parse(Rule::file, lambda)
        .map_err(|e| Error::InvalidRule(e.to_string()))?
        .next()
        .unwrap()
        .into_inner();

    let mut map = HashMap::new();

    for rule in lines.into_iter() {
        if let Rule::EOI = rule.as_rule() {
            break;
        } else if let Rule::line = rule.as_rule() {
            let optline = rule.into_inner().to_owned().next();
            if let None = optline {
                continue;
            }
            let line = optline.unwrap();
            if let Rule::com = line.as_rule() {
                eprintln!("comment: {}", line.as_span().as_str().to_string())
            } else {
                let (ident, lam) = build_def(line)?;
                map.insert(ident, lam);
            }
        } else {
            return Err(Error::InvalidRule(rule.to_string()));
        }
    }

    Ok(map)
}
