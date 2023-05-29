# LamCalc: An implementation for Lambda Calculus 

[![docs.rs](https://img.shields.io/docsrs/lamcalc/latest)](https://docs.rs/lamcalc/latest/lamcalc/)
[![Crates.io](https://img.shields.io/crates/v/lamcalc)](https://crates.io/crates/lamcalc)
[![tutorial](https://img.shields.io/badge/tutorial-Github_Pages-green)](https://sshwy.github.io/lamcalc/)

LamCalc implements untyped Lambda Calculus, Inspired by [Lambda Calculus: Basic Interpreter in Rust (Part 2)](https://tejqunair.com/posts/lambda-part-2/).

Current status: stabalized v1.

## Features

- `lambda!` macro for convenient definition.
- Implemented using De Bruijn index.
- Parser for expressions/definitions/files.
- WASM package for web application.

## Quick View

```rust
use lamcalc::{lambda, Error, parser::parse_exp};

fn main () -> Result<(), Error> {
    // define using macro
    let tt = lambda!(x. y. x); // use macro to define lambda
    let ff = lambda!(x. (y. y)); // add parentheses for clarity
    let and = lambda!(x.y.x y x); // space between dots are not necessary

    // multiple printing format
    println!("and = {}", and);   // print lambda
    println!("and = {:#}", and); // lambda with De Bruijn index
    println!("and = {}", and.purify()); // De Bruijn encoding

    // use braces to refer to previously defined lambda
    let mut and_f_t = lambda!({and} {ff} {tt}); 
    and_f_t.simplify()?; // get simplified result
    assert_eq!(and_f_t, ff);

    // parse lambda expression string
    let y_combinator = lambda!(f.(x. f (x x)) (x. f (x x)));
    let y_str = r#"\f.(\x. f (x x)) (\x. f (x x))"#;
    let (y2, _) = parse_exp(y_str)?;
    
    assert_eq!(y2, y_combinator);

    Ok(())
}
```

See `examples/` for more.