//! Y Combinator

use lamcalc::{lambda, Error};

fn main() -> Result<(), Error> {
    // prepare some nats
    let zero = lambda!(f. (x. x));
    let suc = lambda!(n. f. x. f (n f x));
    let prev = lambda!(n. f. x. n (g. h. h (g f)) (u. x) (u. u));
    let mut nats = vec![zero];
    for i in 1..10 {
        let sx = lambda!({suc} {nats[i - 1]}).simplify()?.to_owned();
        nats.push(sx);
        assert_eq!(
            lambda!({prev} {nats[i]}).simplify()?.to_string(),
            nats[i - 1].to_string()
        );
    }

    // utilities
    let mul = lambda!(n. m. f. x. n (m f) x);
    let if_n_is_zero = lambda!(n. n (w. x. y. y) (x. y. x));

    assert_eq!(
        lambda!({if_n_is_zero} {nats[0]} {nats[2]} {nats[1]} )
            .simplify()?
            .purify(),
        nats[2].purify()
    );

    // Y combinator
    let mut y = lambda!(f. (x. f (x x)) (x. f (x x)));

    // factorial
    let mut fact = lambda!(y. n. {if_n_is_zero} n (f. x. f x) ({mul} n (y ({prev} n))));

    eprintln!("simplify fact");
    while fact.eval_normal_order(true) { 
        eprintln!("fact = {}", fact);
    }

    let y_fact = lambda!({y} {fact});

    let res = lambda!({y_fact} {nats[3]}).purify().simplify()?.to_owned();
    eprintln!("{}", res);
    assert_eq!(res, nats[6].purify());

    // if you try to simplify Y combinator ...
    eprintln!("simplify y: {}", y.simplify().unwrap_err()); // lamcalc::Error::SimplifyLimitExceeded

    Ok(())
}
