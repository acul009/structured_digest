use std::{ops::Deref, rc::Rc, sync::Arc};

use crate::{Digestable, structured::Hasher};

macro_rules! impl_digestable_numbers {
    ($($ty:ty),* $(,)?) => {
        $(
            impl Digestable for $ty {
                fn digest(&self, hasher: &mut impl crate::Hasher) -> crate::Finished {
                    hasher.digest_scalar(&self.to_le_bytes())
                }
            }
        )*
    };
}

// Floating-point values preserve their exact bits, including signed zero and NaN payloads.
impl_digestable_numbers!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);

impl Digestable for [u8] {
    fn digest(&self, hasher: &mut impl crate::Hasher) -> crate::Finished {
        hasher.digest_scalar(self)
    }
}

impl Digestable for &Vec<u8> {
    fn digest(&self, hasher: &mut impl crate::Hasher) -> crate::Finished {
        hasher.digest_scalar(self.as_slice())
    }
}

impl<T: Digestable> Digestable for Option<T> {
    fn digest(&self, hasher: &mut impl crate::Hasher) -> crate::Finished {
        match self {
            Some(value) => hasher.digest_enum("some").field("0", value),
            None => hasher.digest_enum("none"),
        }
        .finish()
    }
}

impl<T: Digestable + ?Sized> Digestable for &T {
    fn digest(&self, hasher: &mut impl crate::Hasher) -> crate::Finished {
        T::digest(*self, hasher)
    }
}

impl<T: Digestable + ?Sized> Digestable for Box<T> {
    fn digest(&self, hasher: &mut impl crate::Hasher) -> crate::Finished {
        T::digest(self.deref(), hasher)
    }
}

impl<T: Digestable + ?Sized> Digestable for Arc<T> {
    fn digest(&self, hasher: &mut impl crate::Hasher) -> crate::Finished {
        T::digest(self.deref(), hasher)
    }
}

impl<T: Digestable + ?Sized> Digestable for Rc<T> {
    fn digest(&self, hasher: &mut impl crate::Hasher) -> crate::Finished {
        T::digest(self.deref(), hasher)
    }
}
