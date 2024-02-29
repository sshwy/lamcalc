//! Church encoding

use lamcalc::{lambda, Error};

fn main() -> Result<(), Error> {
    let zero = lambda!(s. (z. z));
    let suc = lambda!(n. s. z. s (n s z));
    let plus = lambda!(n. m. n {suc} m).simplify()?.to_owned();

    let mut nats = vec![zero];
    for i in 1..10 {
        let sx = lambda!({suc} {nats[i - 1]}).simplify()?.to_owned();
        nats.push(sx);
    }

    let sum = lambda!({plus} {nats[4]} {nats[3]}).simplify()?.to_owned();
    assert_eq!(sum, nats[7]);

    Ok(())
}
