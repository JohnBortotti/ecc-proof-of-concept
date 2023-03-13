pub use num_bigint::{BigInt, ToBigInt};
use std::marker::PhantomData;
use num_traits::{Zero, One};
use std::ops::{Mul, Add, Sub};

pub fn egcd(a: &BigInt, b: &BigInt) -> (BigInt, BigInt, BigInt) {
    if a.is_zero() {
        return (b.to_bigint().unwrap(), Zero::zero(), One::one());
    }

    let (gcd, x1, y1) = egcd(&(b % a), a);
    let x = y1 - (b / a) * &x1;
    let y = x1;

    (gcd, x, y)
}

pub fn inverse_mod<T: ModField> (a: &ModN<T>) -> Option<ModN<T>> {
    let m = T::get_mod();
    let (g, x, _) = egcd(&a.n, &m);

    if g == One::one() {
        Some(ModN::new(x))
    }
    else {
        None
    }

}

pub fn modular_division<T: ModField>(num: ModN<T>, den: ModN<T>) -> ModN<T> {
    let inverse_denominator = inverse_mod(&den).unwrap();
    num * inverse_denominator
}

pub fn rem(a: &BigInt, b: &BigInt) -> BigInt {
    ((a % b) + b) % b
}

pub trait ModField {
    fn get_mod() -> &'static BigInt;
}

#[macro_export]
macro_rules! mod_field {
    ($name:ident => $num:literal) => {
        
        lazy_static::lazy_static! {
            pub static ref $name: $crate::arithmetic::BigInt = {
                <$crate::arithmetic::BigInt as std::str::FromStr>::from_str($num).unwrap()
            };
        }

        impl $crate::arithmetic::ModField for $name {
            fn get_mod() -> &'static $crate::arithmetic::BigInt {
                &*$name
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", $name)
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}: [n: {}]", $name, $num)
            }
        }
    };
    () => {};
}

pub struct ModN<T: ModField> {
    pub n: BigInt,
    modulo: PhantomData<T>
}

impl<T: ModField> ModN<T> {
    pub fn new(n: BigInt) -> Self {
        Self { n: rem(&n, T::get_mod()), modulo: PhantomData }
    }
}

impl<T: ModField> Clone for ModN<T> {
    fn clone(&self) -> Self {
        ModN::new(self.n.clone())
    }
}

impl<T: ModField> std::fmt::Debug for ModN<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ModN: [n: {}]", self.n)
    }

}

impl<T: ModField> Mul for ModN<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let res: BigInt = (self.n * rhs.n) % T::get_mod();

        Self::new(res)
    }
}

impl<T: ModField> Add for ModN<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let res: BigInt = (self.n + rhs.n) % T::get_mod();

        Self::new(res)
    }
}

impl<T: ModField> Sub for ModN<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let res: BigInt = (self.n - rhs.n) % T::get_mod();

        Self::new(res)
    }
}

impl<T: ModField> PartialEq for ModN<T> {
    fn eq(&self, other: &Self) -> bool {
        self.n == other.n
    }
}

impl<T: ModField> Eq for ModN<T> {}

impl<T: ModField> num_traits::identities::Zero for ModN<T> {
    fn zero() -> Self {
        Self::new(0.to_bigint().unwrap())
    }

    fn is_zero(&self) -> bool {
        self.n == 0.to_bigint().unwrap()
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::{BigInt, ToBigInt};
    use std::str::FromStr;

    #[test]
    fn test_rem() {
        let r1 = rem(&80.to_bigint().unwrap(), &17.to_bigint().unwrap());
        let r2 = rem(&-5.to_bigint().unwrap(), &3.to_bigint().unwrap());

        assert_eq!(r1, 12.to_bigint().unwrap());
        assert_eq!(r2, 1.to_bigint().unwrap());
    }

    #[test]
    fn test_egcd() {
        let x: BigInt = 180.to_bigint().unwrap();
        let y: BigInt = 150.to_bigint().unwrap();

        assert_eq!(egcd(&x, &y), (30.to_bigint().unwrap(), 1.to_bigint().unwrap(), -1.to_bigint().unwrap()))
    }

    #[test]
    fn test_inverse_mod() {
        mod_field!(MF_26 => "26");

        let x: ModN<MF_26> = ModN::new(3.to_bigint().unwrap());

        let r = inverse_mod(&x).unwrap();

        let expected: ModN<MF_26> = ModN::new(9.to_bigint().unwrap());

        assert_eq!(r, expected);
    }

    #[test]
    fn test_modular_division() {
        mod_field!(MF_3 => "3");

        let x: ModN<MF_3> = ModN::new(26.to_bigint().unwrap());
        let z: ModN<MF_3> = ModN::new(4.to_bigint().unwrap());

        let expected: ModN<MF_3> = ModN::new(2.to_bigint().unwrap());

        assert_eq!(modular_division(x, z), expected);
    }

    #[test]
    fn mod_field_modular_bounds() {
        mod_field!(MF_5 => "5");

        let x = BigInt::from_str(&"123123123").unwrap();
        let y = BigInt::from_str(&"567459").unwrap();

        let w: ModN<MF_5> = ModN::new(x); 
        let z: ModN<MF_5> = ModN::new(y); 

        assert_eq!(w.n, 3.to_bigint().unwrap());
        assert_eq!(z.n, 4.to_bigint().unwrap());
    }

    #[test]
    fn modular_sum() {
        mod_field!(MF_5 => "2");

        let x: ModN<MF_5> = ModN::new(BigInt::from_str(&"10").unwrap());
        let y: ModN<MF_5> = ModN::new(BigInt::from_str(&"5").unwrap());

        let r = x + y;

        assert_eq!(r.n, 1.to_bigint().unwrap());
    }

    #[test]
    fn modular_mul() {
        mod_field!(MF_7 => "7");

        let x: ModN<MF_7> = ModN::new(BigInt::from_str(&"5").unwrap());
        let y: ModN<MF_7> = ModN::new(BigInt::from_str(&"15").unwrap());

        let r = x * y;

        assert_eq!(r.n, 5.to_bigint().unwrap());
    }

    #[test]
    fn modular_sub() {
        mod_field!(MF_7 => "7");

        let x: ModN<MF_7> = ModN::new(BigInt::from_str(&"13").unwrap());
        let y: ModN<MF_7> = ModN::new(BigInt::from_str(&"3").unwrap());

        let r = x - y;

        assert_eq!(r.n, 3.to_bigint().unwrap());
    }
}
