#[derive(Debug)]
pub enum Point {
    Pt { x:i64, y:i64 },
    Inf
}

// weierstrass form
pub struct Curve {
    pub a: i64,
    pub b: i64,
    pub p: i64,
}

impl Curve {
    fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
        if a == 0 {
            return (b, 0, 1);
        }

        let (gcd, x1, y1) = Curve::egcd(b % a, a);
        let x = y1 - (b / a) * x1;
        let y = x1;

        (gcd, x, y)
    }

    fn inverse_mod(a: i64, m: i64) -> Option<i64> {
        let (g, x, _) = Curve::egcd(a, m);

        if g == 1 {
            Some((x % m + m) % m)
        }
        else {
            None
        }

    }

    fn modular_division(modulo: i64, num: i64, den: i64) -> i64 {
        let inverse_denominator = Curve::inverse_mod(den % modulo, modulo).unwrap();
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
                if y == 0 { 
                    Point::Inf
                } else {
                    let a: i64 = self.a.try_into().unwrap();
                    let x: i64 = x.try_into().unwrap();
                    let y: i64 = y.try_into().unwrap();

                    // tangent slope (Weierstrass curve derivation with respect to y)
                    // division isn't well defined on modular arithmetic, so we use inverse_mod
                    let s: i64 = Curve::modular_division(self.p, 3 * (x.pow(2)) + a, 2*y);

                    // y coord of intersection point 
                    let i: i64 = (y + self.p - (s * x) % self.p) % self.p;

                    let rx: i64 = (s.pow(2) - 2i64 * x) % self.p;
                    let ry: i64 = (self.p - (s * rx) % self.p + self.p - i) % self.p;

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
                    Curve::double(&self, Point::Pt{x:px, y:py})
                } else {
                    // point add case, using tangent
                    let s: i64 = Curve::modular_division(self.p, py + self.p - qy, qx + self.p - qx);

                    // y coord of intersection point 
                    let i: i64 = (py + self.p - (s * px) % self.p) % self.p;

                    let rx: i64 = (s.pow(2) + self.p - px + self.p - qx) % self.p;
                    let ry: i64 = (self.p - (s * rx) % self.p + self.p - i) % self.p;

                    Point::Pt{x: rx, y: ry}
                }
            }
        }
    }

}
