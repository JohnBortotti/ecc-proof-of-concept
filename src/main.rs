mod curve;
mod arithmetic;

use num_bigint::BigInt;
use std::str::FromStr;

fn main() {

    // Elliptic-curve Diffie–Hellman simulation
    mod_field!(MF => "17");

    let a_i: BigInt = BigInt::from_str("0").unwrap();
    let a: arithmetic::ModN<MF> = arithmetic::ModN::new(a_i);

    let b_i: BigInt = BigInt::from_str("7").unwrap();
    let b: arithmetic::ModN<MF>  = arithmetic::ModN::new(b_i);

    let curve = curve::Curve{a, b};

    // (G) generator point -> subgroup order 18
    let x: arithmetic::ModN<MF> = arithmetic::ModN::new(BigInt::from_str("6").unwrap());
    let y: arithmetic::ModN<MF> = arithmetic::ModN::new(BigInt::from_str("6").unwrap());

    let g = curve::Point::Pt{x,y};

    // private_key (d) is a rondom integer in the interval [1, n-1]
    // public_key  (Q) is a point Q on the curve, where Q = private_key * G 

    // Alice credentials (da, Qa)
    let alice_private_key = 6;
    let alice_pub_key = curve.mul(g.clone(), alice_private_key); 

    // Bob credentials (db, Qb)
    let bob_private_key = 13;
    let bob_pub_key = curve.mul(g.clone(), bob_private_key); 

    // Alice computation
    let alice_computed_point = curve.mul(bob_pub_key.clone(), alice_private_key);

    // Bob computation
    let bob_computed_point = curve.mul(alice_pub_key.clone(), bob_private_key);

    // the shared secret is the x coord of computed point
    // da * Qb = da * db * G = db * da * G = db * Qa
    match (alice_computed_point, bob_computed_point) {
        (_, curve::Point::Inf) | (curve::Point::Inf, _) => {},
        (curve::Point::Pt {x: ax, y: _}, curve::Point::Pt {x: bx, y: _}) => {
            assert_eq!(ax, bx);

        }
    }
}
