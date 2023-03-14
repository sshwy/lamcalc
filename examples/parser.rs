use lamcalc::{parser, Error, lambda};

fn main() -> Result<(), Error> {
    // parse single expression
    let tt = parser::parse_exp(r"\x. \y. x")?;

    // parse defination statement
    let (ident, ff) = parser::parse_def(r"ff = \x. \y. y")?;
    assert_eq!(ident, "ff");

    println!("ff = {}", ff);

    // parse multiple definations
    let map = parser::parse_multiline(r##"
        // and
        and = \x. \y. x y x

        // or
        or = \x. \y. x x y
    "##)?;

    let and_t_f = lambda!({map["and"]} {tt} {ff}).simplify()?.to_owned();
    assert_eq!(and_t_f, ff);

    let or_t_f = lambda!({map["or"]} {tt} {ff}).simplify()?.to_owned();
    assert_eq!(or_t_f, tt);

    Ok(())
}
