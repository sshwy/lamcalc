//! Y Combinator

use lamcalc::{lambda, Error};

fn main() -> Result<(), Error> {
    // prepare some nats
    let zero = lambda!(f. (x. x));
    let suc = lambda!(n. f. x. f (n f x));
    let prev = lambda!(n. f. x. n (g. h. h (g f)) (u. x) (u. u));
    let mut nats = vec![zero.clone()];
    for i in 1..30 {
        let sx = lambda!({suc} {nats[i - 1]}).simplify()?.to_owned();
        nats.push(sx);
        assert_eq!(
            lambda!({prev} {nats[i]}).simplify()?.to_string(),
            { &nats[i - 1] }.to_string()
        );
    }
    let mul = lambda!(n. m. f. x. n (m f) x);

    let mut y = lambda!(f. (x. f (x x)) (x. f (x x)));

    let if_n_is_zero = lambda!(n.n(w.x.y.y)(x.y.x));

    assert_eq!(
        lambda!({if_n_is_zero} {nats[0]} {nats[2]} {nats[1]} )
            .simplify()?
            .to_string(),
        nats[2].to_string()
    );

    let fact = lambda!(y. n. {if_n_is_zero} n (f. x. f x) ({mul} n (y ({prev} n))));
    let y_fact = lambda!({y} {fact});

    let res = lambda!({y_fact} {nats[3]}).simplify()?.to_owned();
    assert_eq!(res.to_string(), nats[6].to_string());

    eprintln!("simplify y: {}", y.simplify().unwrap_err()); // lamcalc::Error::SimplifyLimitExceeded

    Ok(())
}
