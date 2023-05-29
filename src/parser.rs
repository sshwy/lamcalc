#![allow(missing_docs)]
//! Parse Lambda expressions in text. Support CJK characters.
//!
//! The parsing expression grammar (PEG) of Lambda expression
//! definitions is defined as (`~` for concatenation, `*` for zero or more,
//! `+` for one or more,
//! `?` for one or none, and `|` for selection):
//!
//! ```pest
#![doc = include_str!("./grammar.pest")]
//! ```
//!
use crate::{builder, Error, Exp};
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;
use serde::Serialize;
use std::collections::HashMap;

/// 使用 [pest](https://pest.rs/) 构建的 lambda 表达式解析器
#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct LambdaParser;

/// Token of lambda expression
#[derive(Debug, Serialize)]
#[serde(tag = "kind", content = "data")]
pub enum Token {
    /// Dot symbol `.`
    DotSym,
    /// Left parenthesis `(`
    LPar,
    /// Right parenthesis `)`
    RPar,
    /// Equal symbol `=`
    Eq,
    /// Line feed `\n`
    LineFeed,
    /// Lambda symbol `\`
    LamSym(String),
    /// String consists of blank characters
    Blank(String),
    /// Variable identifier
    Ident(String),
    /// Comment
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
                panic!("invalid parsing rule: {}", cur)
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
                panic!("invalid parsing rule: {}", cur)
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
                    panic!("invalid parsing rule: {}", cur)
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

        _ => panic!("invalid parsing rule: {}", tt),
    }
}

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
            panic!("invalid parsing rule: {}", cur)
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
        panic!("invalid parsing rule: {}", cur)
    }
    panic!("invalid parsing rule: {}", tt)
}

/// Parse a lambda expression. e. g. `\f. (\x. f (x x)) \x. f (x x)`.
///
/// Return its expression object and token list.
pub fn parse_exp(lambda: &str) -> Result<(Exp<String>, Vec<Token>), Error> {
    let exp = LambdaParser::parse(Rule::exp, lambda)
        .map_err(|e| Error::ParseError(e.to_string()))?
        .next()
        .unwrap();
    build_lambda(exp)
}

/// Parse a single line of definition of lambda exp. e. g. `Y = \f. (\x. f (x x)) \x. f (x x)`.
///
/// Return its idetifier, expression object and token list.
pub fn parse_def(lambda: &str) -> Result<(String, Exp<String>, Vec<Token>), Error> {
    let def = LambdaParser::parse(Rule::def, lambda)
        .map_err(|e| Error::ParseError(e.to_string()))?
        .next()
        .unwrap();
    build_def(def)
}

/// Parse multiple definitions of lambda expression one by a line.
///
/// e. g.
///
/// ```plain
/// Y = \f. (\x. f (x x)) \x. f (x x)
/// I = \z. z
/// T = \x. \y. x
/// F = \x. \y. y
/// O = \f. \x. x
/// S = \n. \f. \x. f (n f x)
/// One = \f. \x. f x
/// Two = \f. \x. f (f x)
/// Plus = \n. \m. \f. \x. n f (m f x)
/// Mul = \n. \m. \f. \x. n (m f) x
/// Exp = \n. \m. n (Mul m) One
/// ```
///
/// Return a map from idetifier to expression object, and the whole content's token list.
///
/// For multiple definitions of the same variable, the last one will be adopted.
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
            panic!("invalid parsing rule: {}", cur)
        } else if let Rule::newline = rule.as_rule() {
            tks.push(Token::LineFeed);
        } else {
            panic!("invalid parsing rule: {}", rule)
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

        assert!(parse_exp(r"( x \x.").is_err());

        Ok(())
    }
    #[test]
    fn test_parse_def() -> Result<(), Error> {
        let s = r"tt    =   \x. \y. x";
        let (name, tt, tks) = parse_def(s)?;

        assert_eq!(name, "tt");
        assert_eq!(tks_str(&tks), s);
        assert_eq!(tt.to_string(), lambda!(x.y.x).to_string());

        assert!(parse_def(" = x.x").is_err());

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

        assert!(parse_file(" = x.x").is_err());

        dbg!(tks);

        Ok(())
    }
    #[test]
    fn test_cjk() -> Result<(), Error> {
        let s = r"(\x. \y. x 即 是 y y 即 是 x) 色 空";

        let (mut exp, tks) = parse_exp(s)?;
        assert_eq!(tks_str(&tks), s);
        assert_eq!(
            exp.to_string(),
            "((λx. λy. ((((((x 即) 是) y) y) 即) 是) x) 色) 空"
        );
        eprintln!("{:#}", exp);
        exp.eval_normal_order(false);
        eprintln!("{:#}", exp);
        exp.simplify()?;
        assert_eq!(exp.to_string(), "((((((色 即) 是) 空) 空) 即) 是) 色");

        Ok(())
    }
}
