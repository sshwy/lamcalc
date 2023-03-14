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

pub mod builder;
mod error;
mod eval;
mod exp;
pub mod parser;

pub use error::Error;
pub use exp::Exp;
pub use exp::Ident;
pub use eval::SIMPLIFY_LIMIT;

#[cfg(test)]
mod tests {
    use crate::{lambda, parser::{parse_multiline, parse_def}, Error};

    #[test]
    fn display() {
        use super::{Exp, Ident};
        println!(
            "{}",
            Exp::Abs(
                Ident("x", 0),
                Box::new(Exp::App(
                    Box::new(Exp::Var(Ident("y", 0))),
                    Box::new(Exp::Var(Ident("x", 2)))
                ))
            )
        )
    }
    #[test]
    fn macros() {
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
    fn subst() {
        let tt = lambda!(x. (y. x));
        let and = lambda!(x. y. x y x);

        let mut e = and.clone();
        e.subst_de(0, &tt);
        assert_eq!(
            format!("{:#}", e),
            "λx. λy. ((λx. λy. x<2>) y<1>) λx. λy. x<2>"
        );
    }
    #[test]
    fn eval() {
        let tt = lambda!(x. (y. x));
        let and = lambda!(x. y. x y x);
        // let or = lambda!(x. y. x y [tt]);
        // let neg = lambda!(x. x [ff] [tt]);

        let mut res = lambda!({and} {tt} {tt});

        println!("res = {}", res);
        while res.eval_normal_order() {
            println!("res = {}", res);
        }
        assert_eq!(res.to_string(), "λx. λy. x");
    }
    #[test]
    fn nat() -> Result<(), Error> {
        let zero = lambda!(s. (z. z));
        let suc = lambda!(n. s. z. s (n s z));
        let mut plus = lambda!(n. m. n {suc} m);
        plus.simplify()?;

        // n + m = ?
        // n 的本质是什么？也就是说 Nat 类型是什么？
        // Nat: S -> Z -> R 是一个高阶函数。
        // R 是什么的类型？
        // zero: s. z. z。故 zero：S -> Z -> Z。
        // suc: Nat -> Nat = Nat -> S -> Z -> R
        // 则带入 zero 可以发现 Nat = S -> Z -> Z。
        // S: Z -> Z

        let mut nats = vec![zero.clone()];
        for i in 1..10 {
            let x = nats.last().unwrap();
            let mut sx = lambda!({suc} {x});
            sx.simplify()?;
            eprintln!("{} = {:-}", i, sx);
            nats.push(sx);
        }
        let mut test = lambda!({plus} {nats[4]} {nats[3]});
        test.simplify()?;
        println!("test = {:-}", test);

        assert_eq!(test.to_string(), nats[7].to_string());
        Ok(())
        // println!("one = {:-}", nats[1]);
    }
    #[test]
    fn y_comb() -> Result<(), Error> {
        let mut y_comb = lambda!(f.(x. f (x x)) (x. f (x x)));
        y_comb.simplify().unwrap_err(); // 无限递归
        eprintln!("y_comb = {:-}", y_comb);
        Ok(())
    }
    #[test]
    fn parser() -> Result<(), Error> {
        let y_comb = lambda!(f.(x. f (x x)) (x. f (x x)));
        let lambda = r#"
            // test parse_desf

            // Y combinator
            Y = \f.(\x. f (x x)) (\x. f (x x))
            // true
            tt = \x. \y. x
            // false
            ff = \x. \y. y 
        "#;
        let res = parse_multiline(lambda)?;

        eprintln!("Y = {}", res["Y"]);
        eprintln!("tt = {:-}", res["tt"]);

        assert_eq!(res["Y"].to_string(), y_comb.to_string());

        let (_, tt) = parse_def(r"tt = \x. \y. x")?;

        assert_eq!(res["tt"].to_string(), tt.to_string());

        Ok(())
    }
}
