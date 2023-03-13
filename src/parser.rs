use pest::{iterators::Pair, Parser};
// use pest::Parser;
use pest_derive::Parser;

use crate::{builder, Error, Exp};

#[derive(Parser)]
#[grammar_inline = r#"
blank       = _{ " "+ }
ident       = @{ ('a'..'z')+ }
exp         =  { app | bounded_exp | abs }
bounded_exp =  { ident | "(" ~ abs ~ ")" | "(" ~ app ~ ")" }
tail_exp    =  { bounded_exp | abs }
abs         =  { "\\" ~ ident ~ blank? ~ "." ~ blank? ~ exp }
app         =  { (bounded_exp ~ blank)+ ~ tail_exp }
main        =  { SOI ~ blank? ~ exp ~ blank? ~ EOI }
"#]
pub struct LambdaParser;

fn build_lambda(tt: Pair<Rule>) -> Result<Exp<String>, Error> {
    match tt.as_rule() {
        Rule::EOI => Err(Error::EmptyExp),
        Rule::blank => Err(Error::EmptyExp),

        Rule::exp => build_lambda(tt.into_inner().next().unwrap()),
        Rule::bounded_exp => build_lambda(tt.into_inner().next().unwrap()),
        Rule::tail_exp => build_lambda(tt.into_inner().next().unwrap()),
        Rule::main => build_lambda(tt.into_inner().next().unwrap()),

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
    }
}

/**
解析一个 lambda 表达式.

使用方法见主页。

使用 PEG 描述的语法规则如下（`~`表示拼接，`+` 表示一次或更多，`?` 表示 0 次或一次，`|` 表示选择）：

```peg
blank       = { " "+ }
ident       = { ('a'..'z')+ }
exp         = { app | bounded_exp | abs }
bounded_exp = { ident | "(" ~ abs ~ ")" | "(" ~ app ~ ")" }
tail_exp    = { bounded_exp | abs }
abs         = { "\\" ~ ident ~ blank? ~ "." ~ blank? ~ exp }
app         = { (bounded_exp ~ blank)+ ~ tail_exp }
main        = { SOI ~ blank? ~ exp ~ blank? ~ EOI }
```
*/
pub fn parse(lambda: &str) -> Result<Exp<String>, Error> {
    match &mut LambdaParser::parse(Rule::main, lambda) {
        Ok(rules) => {
            let rule = rules.next().unwrap();
            // dbg!(&rule);
            build_lambda(rule)
        }
        Err(err) => Err(Error::ParseError(err.to_string())),
    }
}
