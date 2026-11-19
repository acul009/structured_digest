use digest::{Digest, common::BlockSizeUser};

use crate::{Digestable, StructuredHasher};

#[must_use = "call finish() to complete the struct encoding"]
pub struct DigestStruct<'a, T: Digest + BlockSizeUser> {
    pub(super) hasher: &'a mut StructuredHasher<T>,
}

impl<T: Digest + BlockSizeUser> DigestStruct<'_, T> {
    pub fn field<I: Digestable + ?Sized>(self, name: &str, value: &I) -> Self {
        self.hasher.add_raw_item(b"field");
        self.hasher.add_raw_item(name.as_bytes());
        self.hasher.add_item(value);
        self
    }

    pub fn finish(self) {
        self.hasher.add_raw_item(b"end");
    }
}
