# LamCalc: A lambda calculus implementation in Rust

lamcalc 实现了无类型 lambda 演算.

Inspired by [Lambda Calculus: Basic Interpreter in Rust (Part 2)](https://tejqunair.com/posts/lambda-part-2/).

## Quick View

```rust
use lamcalc::{lambda, Error, parser::parse_exp};

fn main () -> Result<(), Error> {
    // define using macro
    let tt = lambda!(x. y. x); // use macro to define lambda
    let ff = lambda!(x. (y. y)); // add parentheses for clarity
    let and = lambda!(x.y.x y x); // space between dot are not necessary

    // multiple printing format
    println!("and = {}", and);   // print lambda
    println!("and = {:#}", and); // lambda with De Bruijn index
    println!("and = {:-}", and); // De Bruijn encoding

    // use braces to refer to previously defined lambda
    let mut res = lambda!({and} {ff} {tt}); 
    res.simplify()?; // get simplified result
    assert_eq!(res.to_string(), ff.to_string());

    // parse lambda expression string
    let y_combinator = lambda!(f.(x. f (x x)) (x. f (x x)));
    let y_str = r#"\f.(\x. f (x x)) (\x. f (x x))"#;
    let y2 = parse_exp(y_str)?;
    assert_eq!(y2.to_string(), y_combinator.to_string());

    Ok(())
}
```
