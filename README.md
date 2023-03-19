# LamCalc: An implementation for Lambda Calculus 

[![docs.rs](https://img.shields.io/docsrs/lamcalc/latest)](https://docs.rs/lamcalc/latest/lamcalc/)

lamcalc implements untyped Lambda Calculus.

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
    let mut and_f_t = lambda!({and} {ff} {tt}); 
    and_f_t.simplify()?; // get simplified result
    assert_eq!(and_f_t, ff);

    // parse lambda expression string
    let y_combinator = lambda!(f.(x. f (x x)) (x. f (x x)));
    let y_str = r#"\f.(\x. f (x x)) (\x. f (x x))"#;
    let (y2, _) = parse_exp(y_str)?;
    // note that y2 has type Exp<String> but y_combinator has type Exp<&str>
    assert_eq!(y2, y_combinator);

    Ok(())
}
```
