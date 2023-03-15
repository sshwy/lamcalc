#![allow(missing_docs)]
//! 解析 lambda 表达式.
//!
//! 使用 PEG 描述的语法规则如下（`~`表示拼接，`+` 表示一次或更多，`?` 表示 0 次或一次，`|` 表示选择）：
//!
//! ```pest
#![doc = include_str!("./grammar.pest")]
//! ```
//!
use crate::{builder, Error, Exp};
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;
use std::collections::HashMap;

/// 使用 [pest](https://pest.rs/) 构建的 lambda 表达式解析器
#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct LambdaParser;

/// 原始代码的 token
#[derive(Debug)]
pub enum Token {
    /// 一个 . 字符
    DotSym,
    /// '('
    LPar,
    /// ')'
    RPar,
    /// '='
    Eq,
    /// 换行
    LineFeed,
    /// 一个 lambda 符号
    LamSym(String),
    /// 空白字符串，不含换行
    Blank(String),
    /// identifier
    Ident(String),
    /// 注释
    Comment(String),
}

fn build_lambda(tt: Pair<Rule>) -> Result<(Exp<String>, Vec<Token>), Error> {
    match tt.as_rule() {
        Rule::ident => {
            let ident = tt.as_span().as_str().to_string();
            Ok((
                builder::unbounded_var(ident.clone()),
                vec![Token::Ident(ident)],
            ))
        }
        Rule::abs => {
            let mut it = tt.into_inner();
            let mut tks = vec![];

            let mut cur = it.next().unwrap();
            tks.push(Token::LamSym(cur.as_span().as_str().to_string()));
            cur = it.next().unwrap();

            let ident = cur.as_span().as_str().to_string();
            tks.push(Token::Ident(ident.clone()));
            cur = it.next().unwrap();

            if let Rule::blank = cur.as_rule() {
                tks.push(Token::Blank(cur.as_span().as_str().to_string()));
                cur = it.next().unwrap()
            }
            if let Rule::dot = cur.as_rule() {
                tks.push(Token::DotSym);
                cur = it.next().unwrap()
            } else {
                return Err(Error::InvalidRule(cur.to_string()));
            }
            if let Rule::blank = cur.as_rule() {
                tks.push(Token::Blank(cur.as_span().as_str().to_string()));
                cur = it.next().unwrap()
            }
            if let Rule::exp = cur.as_rule() {
                let (body_exp, mut body_tks) = build_lambda(cur)?;
                tks.append(&mut body_tks);
                Ok((builder::abs(ident, body_exp), tks))
            } else {
                Err(Error::InvalidRule(cur.to_string()))
            }
        }
        Rule::app => {
            let mut it = tt.into_inner();
            let mut cur = it.next().unwrap();
            let mut exprs = vec![];
            let mut tks = vec![];
            loop {
                if let Rule::tail_exp | Rule::bounded_exp = cur.as_rule() {
                    let (cur_exp, mut cur_tks) = build_lambda(cur)?;
                    exprs.push(cur_exp);
                    tks.append(&mut cur_tks);
                } else if let Rule::blank = cur.as_rule() {
                    tks.push(Token::Blank(cur.as_span().as_str().to_string()));
                } else {
                    return Err(Error::InvalidRule(cur.to_string()));
                }
                cur = match it.next() {
                    Some(nex) => nex,
                    None => {
                        break;
                    }
                };
            }
            Ok((builder::app(exprs), tks))
        }

        Rule::exp | Rule::tail_exp => build_lambda(tt.into_inner().next().unwrap()),

        Rule::bounded_exp => {
            let mut tks = vec![];
            let mut it = tt.into_inner();
            let mut cur = it.next().unwrap();

            if let Rule::ident = cur.as_rule() {
                return build_lambda(cur);
            }

            tks.push(Token::LPar);
            if let Rule::blank = cur.as_rule() {
                tks.push(Token::Blank(cur.as_span().as_str().to_string()));
                cur = it.next().unwrap();
            }
            let (exp, mut exp_tks) = build_lambda(cur)?;
            tks.append(&mut exp_tks);
            if let Some(nex) = it.next() {
                tks.push(Token::Blank(nex.as_span().as_str().to_string()));
            }
            tks.push(Token::RPar);

            Ok((exp, tks))
        }

        _ => Err(Error::InvalidRule(tt.to_string())),
    }
}

/// 只接受 def 规则的表达式
fn build_def(tt: Pair<Rule>) -> Result<(String, Exp<String>, Vec<Token>), Error> {
    if let Rule::def = tt.as_rule() {
        let mut it = tt.into_inner();
        let ident = it.next().unwrap().as_span().as_str().to_string();

        let mut tks = vec![Token::Ident(ident.clone())];

        let mut cur = it.next().unwrap();
        if let Rule::blank = cur.as_rule() {
            tks.push(Token::Blank(cur.as_span().as_str().to_string()));
            cur = it.next().unwrap();
        }
        if let Rule::eq = cur.as_rule() {
            tks.push(Token::Eq);
            cur = it.next().unwrap();
        } else {
            return Err(Error::InvalidRule(cur.to_string()));
        }
        if let Rule::blank = cur.as_rule() {
            tks.push(Token::Blank(cur.as_span().as_str().to_string()));
            cur = it.next().unwrap();
        }
        if let Rule::exp = cur.as_rule() {
            let (exp, mut exp_tks) = build_lambda(cur)?;
            tks.append(&mut exp_tks);
            return Ok((ident, exp, tks));
        }
        return Err(Error::InvalidRule(cur.to_string()));
    }
    Err(Error::InvalidRule(tt.to_string()))
}

/// 解析一个 lambda 表达式，返回其词组
pub fn parse_exp(lambda: &str) -> Result<(Exp<String>, Vec<Token>), Error> {
    let exp = LambdaParser::parse(Rule::exp, lambda)
        .map_err(|e| Error::ParseError(e.to_string()))?
        .next()
        .unwrap();
    build_lambda(exp)
}

/// 解析单行 lambda 表达式的定义，返回其标识符（变量名）和词组
pub fn parse_def(lambda: &str) -> Result<(String, Exp<String>, Vec<Token>), Error> {
    let def = LambdaParser::parse(Rule::def, lambda)
        .map_err(|e| Error::ParseError(e.to_string()))?
        .next()
        .unwrap();
    build_def(def)
}

/// 解析多行定义，返回一个 map 表示标识符与表达式的对应关系（后面的会覆盖前面的），以及全文的词组
pub fn parse_file(lambda: &str) -> Result<(HashMap<String, Exp<String>>, Vec<Token>), Error> {
    let lines = LambdaParser::parse(Rule::file, lambda)
        .map_err(|e| Error::ParseError(e.to_string()))?
        .next()
        .unwrap()
        .into_inner();

    let mut map = HashMap::new();
    let mut tks = vec![];

    for rule in lines.into_iter() {
        if let Rule::EOI = rule.as_rule() {
            break;
        } else if let Rule::line = rule.as_rule() {
            let mut it = rule.into_inner();
            let mut cur = match it.next() {
                Some(nex) => nex,
                None => continue,
            };
            if let Rule::blank = cur.as_rule() {
                tks.push(Token::Blank(cur.as_span().as_str().to_string()));
                cur = match it.next() {
                    Some(nex) => nex,
                    None => continue,
                }
            }

            if let Rule::def = cur.as_rule() {
                let (ident, lam, mut def_tks) = build_def(cur)?;
                tks.append(&mut def_tks);
                map.insert(ident, lam);
                cur = match it.next() {
                    Some(nex) => nex,
                    None => continue,
                };
                if let Rule::blank = cur.as_rule() {
                    tks.push(Token::Blank(cur.as_span().as_str().to_string()));
                    cur = match it.next() {
                        Some(nex) => nex,
                        None => continue,
                    }
                }
            }

            if let Rule::com = cur.as_rule() {
                tks.push(Token::Comment(cur.as_span().as_str().to_string()));
                continue;
            }
            return Err(Error::InvalidRule(cur.to_string()));
        } else if let Rule::newline = rule.as_rule() {
            tks.push(Token::LineFeed);
        } else {
            return Err(Error::InvalidRule(rule.to_string()));
        }
    }

    Ok((map, tks))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{lambda, Error};

    fn tks_str(tks: &[Token]) -> String {
        let mut res = String::new();
        for tk in tks {
            res.push_str(match tk {
                Token::DotSym => ".",
                Token::LPar => "(",
                Token::RPar => ")",
                Token::Eq => "=",
                Token::LineFeed => "\n",

                Token::LamSym(s) | Token::Ident(s) | Token::Comment(s) | Token::Blank(s) => s,
            });
        }
        res
    }

    #[test]
    fn test_parse_exp() -> Result<(), Error> {
        let s = r"\x .  (  x  x  )  (  x  x  )";

        let (exp, tks) = parse_exp(s)?;
        assert_eq!(tks_str(&tks), s);
        assert_eq!(exp.to_string(), lambda!(x. (x x) (x x)).to_string());

        Ok(())
    }
    #[test]
    fn test_parse_def() -> Result<(), Error> {
        let s = r"tt    =   \x. \y. x";
        let (name, tt, tks) = parse_def(s)?;

        assert_eq!(name, "tt");
        assert_eq!(tks_str(&tks), s);
        assert_eq!(tt.to_string(), lambda!(x.y.x).to_string());

        Ok(())
    }
    #[test]
    fn test_parse_file() -> Result<(), Error> {
        let y_comb = lambda!(f.(x. f (x x)) (x. f (x x)));
        let lambda = r#"
            // test parse_desf

            // Y combinator
            Y = \f. (\x. f (x x) ) (\x. f (x x) )    
            tt = \x. \y. x   // true
            // false
            ff = \x. \y. y// false
        "#;

        let (res, tks) = parse_file(lambda)?;

        assert_eq!(res["Y"].to_string(), y_comb.to_string());
        assert_eq!(tks_str(&tks), lambda);

        dbg!(tks);

        Ok(())
    }
}
