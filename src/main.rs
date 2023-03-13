mod curve;
mod arithmetic;

use num_bigint::BigInt;
use std::str::FromStr;

fn main() {
    mod_field!(MF => "97");

    let a_i: BigInt = BigInt::from_str("2").unwrap();
    let a: arithmetic::ModN<MF> = arithmetic::ModN::new(a_i);

    let b_i: BigInt = BigInt::from_str("3").unwrap();
    let b: arithmetic::ModN<MF>  = arithmetic::ModN::new(b_i);

    let curve = curve::Curve{a, b};

    let x_i: BigInt = BigInt::from_str("3").unwrap();
    let x: arithmetic::ModN<MF> = arithmetic::ModN::new(x_i);
    
    let y_i: BigInt = BigInt::from_str("6").unwrap();
    let y: arithmetic::ModN<MF> = arithmetic::ModN::new(y_i);

    let q = curve::Point::Pt{x, y};

    let q2: curve::Point<MF> = curve.add(q.clone(), q);

    dbg!("{}", q2);
}
