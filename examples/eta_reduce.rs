use lamcalc::lambda;

fn main() {
    let mut exp = lambda!(a. b. c. f a b c);

    eprintln!("exp = {}", exp);
    assert!(exp
        .into_abs_mut()
        .unwrap()
        .1
        .into_abs_mut()
        .unwrap()
        .1
        .is_eta_redex());

    assert!(exp
        .into_abs_mut()
        .unwrap()
        .1
        .into_abs_mut()
        .unwrap()
        .1
        .eta_reduce());
    eprintln!("exp = {}", exp);

    assert!(exp.into_abs_mut().unwrap().1.eta_reduce());
    eprintln!("exp = {}", exp);

    assert!(exp.eta_reduce());
    eprintln!("exp = {}", exp);
}
