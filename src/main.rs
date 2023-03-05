// Modular arithmetic
// division over finite fields is not well defined, so we turn up doing inverse_mod_mult in order to 
// calc linear algebra on finite fields, for this reason, when calculating over finite fields, use
// modular arithmetic (inverse_mod in place of division).
//
// considering the field {0, 1, 2, 3}, what happens with 2/3? the field is not closed for this operation
//
// If gcd(k,N)=1 and ka ≡ kb(mod N), then a ≡ b (modN).
// If a and N are integers such that gcd(a,N)=1, then there exists an integer x such that ax ≡ 1 (modN).
//
// the inverse_mod of 'a' is some 'b' where 'a * b ≡ 1 (mod m)', e.g.:
// the inverse_mod of 3 (mod 5) -> 3 * b ≡ 1 (mod 5) -> 3 * 2 ≡ 6 ≡ 1 (mod 5)
//
// --------------------------
// Eliptic curves
// Weierstrass form -> y² = x³ + ax + b (short form, and easy arithmetic)
//
// the curve is non-singular if: 4a³ + 27b² != 0
// 
// inversion operation a.k.a. negation law (-)
// −O = O
//
// Let P1 = (u1, v1) ∈ EW. Then 1 = (u1, −v1 − a1u1 −a3)
// P1 + P2 = P2 + P1 (Commutativity)
// (P0 + P1) + P2 = P0 + (P1 + P2) (Associativity)
//
// --------------------------
// Enforcing modular arithmetic on Rust
// An recurrent aproach, is defining PrimeGroups with Macros (macro_rules!), so you get compile-checking
// on the fields, prohibiting diferent groups operating together

// Todo 
//  - use ModN for curve calculations
//  - optimize point representation (x, 0/1)

mod curve;
mod arithmetic;

use num_bigint::BigInt;
use std::str::FromStr;

fn main() {
    let a: BigInt = BigInt::from_str("0").unwrap();
    let b: BigInt = BigInt::from_str("7").unwrap();
    let p: BigInt = BigInt::from_str("97").unwrap();

    let curve = curve::Curve{a, b, p};

    let x: BigInt = BigInt::from_str("3").unwrap();
    let y: BigInt = BigInt::from_str("6").unwrap();

    let q = curve::Point::Pt{x, y};

    let q2 = curve.mul(q, 2);

    dbg!("{}", q2);

}
