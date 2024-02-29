use lamcalc::lambda;

fn main() {
    let and = lambda!(x. y. (x y x));
    let tt = lambda!(x. (y. x));
    let ff = lambda!(x. (y. y));
    let mut exp = lambda!({and} {tt} {ff});

    eprintln!("exp = {}", exp); // ((λx. λy. (x y) x) λx. λy. x) λx. λy. y

    assert!(exp.into_app_mut().unwrap().0.is_beta_redex());
    assert!(exp.into_app_mut().unwrap().0.beta_reduce());
    eprintln!("exp = {}", exp); // (λy. ((λx. λy. x) y) λx. λy. x) λx. λy. y

    assert!(exp
        .into_app_mut()
        .unwrap()
        .0 // λy. ((λx. λy. x) y) λx. λy. x
        .into_abs_mut()
        .unwrap()
        .1 // ((λx. λy. x) y) λx. λy. x
        .into_app_mut()
        .unwrap()
        .0 // (λx. λy. x) y
        .beta_reduce());
    eprintln!("exp = {}", exp); // (λy. (λy. y) λx. λy. x) λx. λy. y

    assert!(exp
        .into_app_mut()
        .unwrap()
        .0 // λy. (λy. y) λx. λy. x
        .into_abs_mut()
        .unwrap()
        .1 // (λy. y) λx. λy. x
        .beta_reduce());
    eprintln!("exp = {}", exp); // (λy. y) λx. λy. y

    assert!(exp.beta_reduce());
    eprintln!("exp = {}", exp); // λx. λy. y
}
