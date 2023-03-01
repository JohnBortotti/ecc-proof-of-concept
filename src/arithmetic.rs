pub use num_bigint::BigUint;

use std::marker::PhantomData;

pub trait ModField {
    fn get_mod() -> &'static BigUint;
}

#[macro_export]
macro_rules! mod_field {
    ($name:ident => $num:literal) => {
        
        lazy_static::lazy_static! {
            pub static ref $name: $crate::arithmetic::BigUint = {
                <$crate::arithmetic::BigUint as std::str::FromStr>::from_str($num).unwrap()
            };
        }

        impl $crate::arithmetic::ModField for $name {
            fn get_mod() -> &'static $crate::arithmetic::BigUint {
                &*$name
            }
        }

    };
    () => {};
}

pub struct ModN<P: ModField> {
    pub n: BigUint,
    modulo: PhantomData<P>
}

impl<M: ModField> ModN<M> {
    pub fn new(n: BigUint) -> Self {
        Self { n: n % M::get_mod(), modulo: PhantomData }

    }

}
