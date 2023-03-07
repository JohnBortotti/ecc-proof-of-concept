pub use num_bigint::BigInt;
use std::marker::PhantomData;

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
        Self { n: n % T::get_mod(), modulo: PhantomData }

    }

}

impl <T: ModField>std::fmt::Debug for ModN<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ModN: [n: {}]", self.n)
    }

}

#[cfg(test)]
    mod tests {
    use super::*;
    use num_bigint::{BigInt, ToBigInt};
    use std::str::FromStr;

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
}
