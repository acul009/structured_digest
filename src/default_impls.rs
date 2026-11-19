use std::collections::BTreeMap;

use digest::{Digest, common::BlockSizeUser};

use crate::{Digestable, StructuredHasher};

macro_rules! impl_digestable_numbers {
    ($($ty:ty),* $(,)?) => {
        $(
            impl Digestable for $ty {
                fn digest<T: Digest + BlockSizeUser>(&self, hasher: &mut StructuredHasher<T>) {
                    hasher.digest_scalar(&self.to_le_bytes());
                }
            }
        )*
    };
}

// Floating-point values preserve their exact bits, including signed zero and NaN payloads.
impl_digestable_numbers!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);

impl Digestable for str {
    fn digest<T: Digest + BlockSizeUser>(&self, hasher: &mut StructuredHasher<T>) {
        hasher.digest_scalar(self.as_bytes());
    }
}

impl Digestable for [u8] {
    fn digest<T: Digest + BlockSizeUser>(&self, hasher: &mut StructuredHasher<T>) {
        hasher.digest_scalar(self);
    }
}

impl<T: Digestable> Digestable for Vec<T> {
    fn digest<I: Digest + BlockSizeUser>(&self, hasher: &mut StructuredHasher<I>) {
        hasher.digest_sequence().iter(self.iter()).finish();
    }
}

impl<T: Digestable, U: Digestable> Digestable for BTreeMap<T, U> {
    fn digest<I: Digest + BlockSizeUser>(&self, hasher: &mut StructuredHasher<I>) {
        let mut sequence = hasher.digest_sequence();
        for (key, value) in self.iter() {
            sequence = sequence.item(key).item(value);
        }
        sequence.finish();
    }
}
