use digest::{Digest, common::BlockSizeUser};

use crate::{Digestible, StructuredHasher};

#[must_use = "call finish() to complete the sequence encoding"]
pub struct DigestSequence<'a, T: Digest + BlockSizeUser> {
    pub(super) hasher: &'a mut StructuredHasher<T>,
}

impl<T: Digest + BlockSizeUser> Hasher for DigestSequence<'_, T> {
    fn item<I: Digestible + Sized>(self, value: I) -> Self {
        self.hasher.add_item(value);
        self
    }

    fn iter<'a, I: Digestible + Sized + 'a>(mut self, values: impl Iterator<Item = I>) -> Self {
        for value in values {
            self = self.item(value);
        }
        self
    }

    fn finish(self) -> crate::Finished {
        self.hasher.add_raw_item(b"end");
        crate::Finished(())
    }
}

#[must_use = "call finish() to complete the sequence encoding"]
pub trait Hasher {
    fn item<I: Digestible + Sized>(self, value: I) -> Self;
    fn iter<'a, I: Digestible + Sized + 'a>(self, values: impl Iterator<Item = I>) -> Self;
    fn finish(self) -> crate::Finished;
}
