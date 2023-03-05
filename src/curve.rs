use num_bigint::{BigUint, ToBigUint};
use num_traits::{Zero, One};

#[derive(Debug, Clone)]
pub enum Point {
    Pt { x: BigUint, y: BigUint },
    Inf
}

// weierstrass form
pub struct Curve {
    pub a: BigUint,
    pub b: BigUint,
    pub p: BigUint
}

impl Curve {
    fn egcd(a: &BigUint, b: &BigUint) -> (BigUint, BigUint, BigUint) {
        if a.is_zero() {
            return (b.to_biguint().unwrap(), Zero::zero(), One::one());
        }

        let (gcd, x1, y1) = Curve::egcd(&(b % a), a);
        let x = y1 - (b / a) * &x1;
        let y = x1;

        (gcd, x, y)
    }

    fn inverse_mod(a: &BigUint, m: &BigUint) -> Option<BigUint> {
        let (g, x, _) = Curve::egcd(&a, &m);

        if g == One::one() {
            Some((x % m + m) % m)
        }
        else {
            None
        }

    }

    fn modular_division(modulo: &BigUint, num: &BigUint, den: &BigUint) -> BigUint {
        let inverse_denominator = Curve::inverse_mod(&(den % modulo), modulo).unwrap();
        ((num % modulo) * inverse_denominator) % modulo
    }

    // point doubling
    // for point doubling, calc the tangent slope in P, 
    // take the intersection point for slope and curve, 
    // invert the y coord of found point
    pub fn double(&self, p: Point) -> Point {
        match p {
            // matching identity cases
            Point::Inf => Point::Inf,
            Point::Pt {x, y} => {
                if y == Zero::zero() { 
                    Point::Inf
                } else {
                    let p: &BigUint = &self.p.to_biguint().unwrap();
                    let a: BigUint = self.a.to_biguint().unwrap();
                    let x: &BigUint = &x.to_biguint().unwrap();
                    let y: BigUint = y.to_biguint().unwrap();

                    // tangent slope (Weierstrass curve derivation with respect to y)
                    // division isn't well defined on modular arithmetic, so we use inverse_mod
                    let s: BigUint = Curve::modular_division(&(p), &(3.to_biguint().unwrap() * (x.pow(2)) + a), &(2.to_biguint().unwrap() * &y));

                    // y coord of intersection point 
                    let i: BigUint = (&y + p - (&s * x) % p) % p;

                    let rx: BigUint = (s.pow(2) - 2.to_biguint().unwrap() * x) % p;
                    let ry: BigUint = (p - (s * &rx) % p + p - i) % p;

                    Point::Pt{x: rx, y: ry}
                }

            }
        }
    }
 
    // point addition
    pub fn add(&self, p: Point, q: Point) -> Point {
        match (p, q) {
            (q, Point::Inf) | (Point::Inf, q) => q,
            (Point::Pt{x: px, y: py}, Point::Pt{x: qx, y: qy}) => {
                if px == qx {
                    // point doubling case                      
                    Curve::double(&self, Point::Pt{x: px, y: py})
                } else {
                    // point add case, using tangent
                    let p: &BigUint = &self.p;

                    let s: BigUint = Curve::modular_division(&(self.p.to_biguint().unwrap()), &(&py + p - qy), &(&qx + p - &qx));

                    // y coord of intersection point 
                    let i: BigUint = (py + p - (&s * &px) % p) % p;

                    let rx: BigUint = (s.pow(2) + p - px + p - qx) % p;
                    let ry: BigUint = (p - (s * &rx) % p + p - i) % p;

                    Point::Pt{x: rx, y: ry}
                }
            }
        }
    }

    // point scalar multiplication (double and add)
    pub fn mul(&self, p: Point, m: i128) -> Point {
        let mut result: Point = Point::Pt{x: Zero::zero(), y: Zero::zero()};
        let mut curr = p;
        let binary_scalar = format!("{:b}", m);

        for c in binary_scalar.chars() {
            result = Curve::double(self, result);

            if c == '1' {
                result = Curve::add(self, result, curr.clone());
            }

            curr = Curve::double(self, curr);
        }
        
        return result
    }

}
