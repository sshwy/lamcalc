#![warn(missing_docs)]
#![doc = include_str!("../README.md")]
//!
//! ## Example: Church Encoding
//!
//! ```rust
#![doc = include_str!("../examples/church_encoding.rs")]
//! ```
//!
//! ## Example: Parser
//!
//! ```rust
#![doc = include_str!("../examples/parser.rs")]
//! ```
//!
//! ## Example: Y Combinator
//!
//! ```rust
#![doc = include_str!("../examples/y_combinator.rs")]
//! ```

#[doc(hidden)]
pub mod builder;
mod error;
mod eval;
mod exp;
pub mod parser;
#[cfg(feature = "wasm")]
pub mod wasm;

pub use error::Error;
pub use eval::SIMPLIFY_LIMIT;
pub use exp::Exp;
pub use exp::Ident;

#[cfg(test)]
mod tests {
    use crate::{lambda, Error};
    use super::{Exp, Ident};

    #[test]
    fn test_display() {
        println!(
            "{}",
            Exp::Abs(
                Ident(String::from("x"), 0),
                Box::new(Exp::App(
                    Box::new(Exp::Var(Ident(String::from("y"), 0))),
                    Box::new(Exp::Var(Ident(String::from("x"), 2)))
                ))
            )
        )
    }
    #[test]
    fn test_clone() {
        let mut tt = lambda!(x. y. x).purify();
        let mut ff = tt.clone();
        ff.into_body().into_body().into_ident().1 = 1;
        assert_eq!(tt.into_body().into_body().into_ident().1, 2);
        println!("tt = {}, ff = {}", tt, ff);
    }
    #[test]
    fn test_macros() {
        let l_true = lambda!(x.y.x);
        let l_false = lambda!(x.y.y);
        let y_comb = lambda!(f.(x. f (x x)) (x. f (x x)));
        assert_eq!(format!("{}", y_comb), "λf. (λx. f (x x)) λx. f (x x)");
        assert_eq!(
            format!("{:#}", y_comb),
            "λf. (λx. f<2> (x<1> x<1>)) λx. f<2> (x<1> x<1>)"
        );

        println!("{}\n{}\n{}\n", l_true, l_false, y_comb);
        println!("{:#}\n{:#}\n{:#}", l_true, l_false, y_comb);

        let test_app = lambda!(x. y. z. x y z);
        let test_app2 = lambda!(x. y. z. (x (y z)) (x z));

        println!("{}\n{}", test_app, test_app2);
    }
    #[test]
    fn test_eval() {
        let tt = lambda!(x. (y. x));
        let and = lambda!(x. y. x y x);
        // let or = lambda!(x. y. x y [tt]);
        // let neg = lambda!(x. x [ff] [tt]);

        let mut res = lambda!({and} {tt} {tt});

        println!("res = {}", res);
        while res.eval_normal_order(false) {
            println!("res = {}", res);
        }
        assert_eq!(res.to_string(), "λx. λy. x");
    }
    #[test]
    fn test_nat() -> Result<(), Error> {
        let zero = lambda!(s. (z. z));
        let suc = lambda!(n. s. z. s (n s z));
        let mut plus = lambda!(n. m. n {suc} m);
        plus.simplify()?;

        let mut nats = vec![zero.clone()];
        for i in 1..10 {
            let x = nats.last().unwrap();
            let mut sx = lambda!({suc} {x});
            sx.simplify()?;
            eprintln!("{} = {}", i, sx.purify());
            nats.push(sx);
        }
        let mut test = lambda!({plus} {nats[4]} {nats[3]});
        test.simplify()?;
        println!("test = {:#}", test);

        assert_eq!(test.to_string(), nats[7].to_string());
        Ok(())
    }
}
