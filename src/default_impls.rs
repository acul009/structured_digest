use std::{ops::Deref, rc::Rc, sync::Arc};

use crate::{Digestible, structured::Hasher};

macro_rules! impl_digestible_numbers {
    ($($ty:ty),* $(,)?) => {
        $(
            impl Digestible for $ty {
                fn digest(&self, hasher: &mut impl crate::Hasher) -> crate::Finished {
                    hasher.digest_scalar(&self.to_le_bytes())
                }
            }
        )*
    };
}

// Floating-point values preserve their exact bits, including signed zero and NaN payloads.
impl_digestible_numbers!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);

impl Digestible for [u8] {
    fn digest(&self, hasher: &mut impl crate::Hasher) -> crate::Finished {
        hasher.digest_scalar(self)
    }
}

impl Digestible for &Vec<u8> {
    fn digest(&self, hasher: &mut impl crate::Hasher) -> crate::Finished {
        hasher.digest_scalar(self.as_slice())
    }
}

impl<T: Digestible> Digestible for Option<T> {
    fn digest(&self, hasher: &mut impl crate::Hasher) -> crate::Finished {
        match self {
            Some(value) => hasher.digest_enum("some").field("0", value),
            None => hasher.digest_enum("none"),
        }
        .finish()
    }
}

impl<T: Digestible + ?Sized> Digestible for &T {
    fn digest(&self, hasher: &mut impl crate::Hasher) -> crate::Finished {
        T::digest(*self, hasher)
    }
}

impl<T: Digestible + ?Sized> Digestible for Box<T> {
    fn digest(&self, hasher: &mut impl crate::Hasher) -> crate::Finished {
        T::digest(self.deref(), hasher)
    }
}

impl<T: Digestible + ?Sized> Digestible for Arc<T> {
    fn digest(&self, hasher: &mut impl crate::Hasher) -> crate::Finished {
        T::digest(self.deref(), hasher)
    }
}

impl<T: Digestible + ?Sized> Digestible for Rc<T> {
    fn digest(&self, hasher: &mut impl crate::Hasher) -> crate::Finished {
        T::digest(self.deref(), hasher)
    }
}
