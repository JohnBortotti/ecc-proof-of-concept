pub use num_bigint::BigUint;

use std::marker::PhantomData;

pub trait PrimeMod {
    fn get_mod() -> &'static BigUint;
}

#[macro_export]
macro_rules! prime_mod_field {
    ($name:ident => $num:literal) => {
        
        lazy_static::lazy_static! {
            pub static ref $name: $crate::arithmetic::BigUint = {
                <$crate::arithmetic::BigUint as std::str::FromStr>::from_str($num).unwrap()
            };
        }

        impl $crate::arithmetic::PrimeMod for $name {
            fn get_mod() -> &'static $crate::arithmetic::BigUint {
                &*$name
            }
        }

    };
    () => {};
}

pub struct ModN<P: PrimeMod> {
    pub n: BigUint,
    modulo: PhantomData<P>
}

impl<M: PrimeMod> ModN<M> {
    pub fn new(n: BigUint) -> Self {
        Self { n: n % M::get_mod(), modulo: PhantomData }

    }

}
