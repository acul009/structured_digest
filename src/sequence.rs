use digest::{Digest, common::BlockSizeUser};

use crate::{Digestable, StructuredHasher};

#[must_use = "call finish() to complete the sequence encoding"]
pub struct DigestSequence<'a, T: Digest + BlockSizeUser> {
    pub(super) hasher: &'a mut StructuredHasher<T>,
}

impl<T: Digest + BlockSizeUser> DigestSequence<'_, T> {
    pub fn item<I: Digestable + ?Sized>(self, value: &I) -> Self {
        self.hasher.add_item(value);
        self
    }

    pub fn iter<'a, I: Digestable + ?Sized + 'a>(
        mut self,
        values: impl Iterator<Item = &'a I>,
    ) -> Self {
        for value in values {
            self = self.item(value);
        }
        self
    }

    pub fn finish(self) {
        self.hasher.add_raw_item(b"end");
    }
}
