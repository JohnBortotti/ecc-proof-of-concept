use num_bigint::ToBigInt;
use num_traits::Zero;
use crate::arithmetic::{ModN, ModField, modular_division};

#[derive(Debug)]
pub enum Point<T: ModField> {
    Inf,
    Pt { x: ModN<T>, y: ModN<T> },
}

impl<T: ModField> Clone for Point<T> {
    fn clone(&self) -> Self {
        match &self {
            Point::Inf => Point::Inf,
            Point::Pt {x, y} => {
                let xr = ModN::new(x.n.clone());
                let yr = ModN::new(y.n.clone());

                Point::Pt{x: xr, y: yr}
            }

        }
    }
}

// weierstrass form
pub struct Curve<T: ModField> {
    pub a: ModN<T>,
    pub b: ModN<T>,
}

impl<T: ModField> Curve<T> {
    // point doubling
    // for point doubling, calc the tangent slope in P, 
    // take the intersection point for slope and curve, 
    // invert the y coord of found point
    pub fn double(&self, p: Point<T>) -> Point<T> {
        match p {
            // matching identity cases
            Point::Inf => Point::Inf,
            Point::Pt {x, y} => {
                if Zero::is_zero(&y) { 
                    Point::Inf
                } else {
                    // tangent slope (Weierstrass curve derivation with respect to y)
                    // division isn't well defined on modular arithmetic, so we use inverse_mod
                    let s: ModN<T> = modular_division(
                        ModN::new(3.to_bigint().unwrap()) * (x.clone() * x.clone()) + self.a.clone(),
                        ModN::new(2.to_bigint().unwrap()) * y.clone()
                    );

                    // y coord of intersection point 
                    let i: ModN<T> = y - (s.clone() * x.clone());

                    let zero = ModN::new(0.to_bigint().unwrap());

                    let rx = (s.clone() * s.clone()) - ModN::new(2.to_bigint().unwrap()) * x;
                    let ry = zero - (s * rx.clone()) - i;

                    Point::Pt{x: rx, y: ry}
                }

            }
        }
    }
 
    // point addition
    pub fn add(&self, p: Point<T>, q: Point<T>) -> Point<T> {
        match (p, q) {
            (q, Point::Inf) | (Point::Inf, q) => q,
            (Point::Pt{x: px, y: py}, Point::Pt{x: qx, y: qy}) => {
                if px.n == qx.n {
                    // point doubling case                      
                    Curve::double(&self, Point::Pt{x: px, y: py})
                } else {
                    // point add case, using tangent

                    let s = modular_division(qy.clone() - py.clone(), qx.clone() - px.clone());

                    // y coord of intersection point 
                    let i = py - (s.clone() * px.clone());

                    let zero = ModN::new(0.to_bigint().unwrap());

                    let rx = (s.clone() * s.clone()) - px - qx;
                    let ry = zero - (s * rx.clone()) - i;

                    Point::Pt{x: rx, y: ry}
                }
            }
        }
    }

    // point scalar multiplication (double and add)
    pub fn mul(&self, p: Point<T>, m: u128) -> Point<T> {
        let mut result: Point<T> = Point::Pt{x: Zero::zero(), y: Zero::zero()};
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

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::ToBigInt;
    use crate::mod_field;

    #[test]
    fn test_curve_double() {
        mod_field!(MF_97 => "97");

        let a: ModN<MF_97> = ModN::new(2.to_bigint().unwrap());
        let b: ModN<MF_97> = ModN::new(3.to_bigint().unwrap());

        let curve = Curve{a, b};

        let x: ModN<MF_97> = ModN::new(3.to_bigint().unwrap());
        let y: ModN<MF_97> = ModN::new(6.to_bigint().unwrap());

        let q = Point::Pt{x, y};

        let double = curve.double(q);

        match double {
            Point::Inf => {},
            Point::Pt {x, y} => {
                let x_expected: ModN<MF_97> = ModN::new(80.to_bigint().unwrap());
                let y_expected: ModN<MF_97> = ModN::new(10.to_bigint().unwrap());

                assert_eq!(x, x_expected);
                assert_eq!(y, y_expected);
            }
        }

    }

    #[test]
    fn test_curve_add_1() {
        mod_field!(MF_97 => "97");

        let a: ModN<MF_97> = ModN::new(2.to_bigint().unwrap());
        let b: ModN<MF_97> = ModN::new(3.to_bigint().unwrap());

        let curve = Curve{a, b};

        let x: ModN<MF_97> = ModN::new(3.to_bigint().unwrap());
        let y: ModN<MF_97> = ModN::new(6.to_bigint().unwrap());

        let q = Point::Pt{x: x.clone(), y: y.clone()};
        let p = Point::Pt{x: x.clone(), y: y.clone()};

        let r = curve.add(p, q);

        match r {
            Point::Inf => {},
            Point::Pt {x, y} => {
                let x_expected: ModN<MF_97> = ModN::new(80.to_bigint().unwrap());
                let y_expected: ModN<MF_97> = ModN::new(10.to_bigint().unwrap());

                assert_eq!(x, x_expected);
                assert_eq!(y, y_expected);
            }
        }
    }

    #[test]
    fn test_curve_add_2() {
        mod_field!(MF_97 => "97");

        let a: ModN<MF_97> = ModN::new(2.to_bigint().unwrap());
        let b: ModN<MF_97> = ModN::new(3.to_bigint().unwrap());

        let curve = Curve{a, b};

        let p = Point::Pt{x: ModN::new(80.to_bigint().unwrap()), y: ModN::new(10.to_bigint().unwrap())};
        let q = Point::Pt{x: ModN::new(3.to_bigint().unwrap()), y: ModN::new(91.to_bigint().unwrap())};

        let r = curve.add(p, q);

        match r {
            Point::Inf => {},
            Point::Pt {x, y} => {
                let x_expected: ModN<MF_97> = ModN::new(3.to_bigint().unwrap());
                let y_expected: ModN<MF_97> = ModN::new(6.to_bigint().unwrap());

                assert_eq!(x, x_expected);
                assert_eq!(y, y_expected);
            }
        }
    }


    #[test]
    fn test_curve_mul() {
        mod_field!(MF_97 => "97");

        let a: ModN<MF_97> = ModN::new(2.to_bigint().unwrap());
        let b: ModN<MF_97> = ModN::new(3.to_bigint().unwrap());

        let curve = Curve{a, b};

        let x: ModN<MF_97> = ModN::new(3.to_bigint().unwrap());
        let y: ModN<MF_97> = ModN::new(6.to_bigint().unwrap());

        let q = Point::Pt{x, y};

        let r = curve.mul(q, 4);

        match r {
            Point::Inf => {},
            Point::Pt {x, y} => {
                let x_expected: ModN<MF_97> = ModN::new(3.to_bigint().unwrap());
                let y_expected: ModN<MF_97> = ModN::new(91.to_bigint().unwrap());

                assert_eq!(x, x_expected);
                assert_eq!(y, y_expected);
            }
        }
    }

}
