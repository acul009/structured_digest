use digest::{Digest, common::BlockSizeUser};

use crate::{Digestable, StructuredHasher};

#[must_use = "call finish() to complete the struct encoding"]
pub struct DigestStruct<'a, T: Digest + BlockSizeUser> {
    pub(super) hasher: &'a mut StructuredHasher<T>,
}

impl<T: Digest + BlockSizeUser> Hasher for DigestStruct<'_, T> {
    type Hasher = crate::StructuredHasher<T>;

    fn field<I: Digestable + Sized>(self, name: &str, value: I) -> Self {
        self.hasher.add_raw_item(b"field");
        self.hasher.add_raw_item(name.as_bytes());
        self.hasher.add_item(value);
        self
    }

    fn field_with<F: FnOnce(&mut Self::Hasher) -> crate::Finished>(self, name: &str, f: F) -> Self {
        self.hasher.add_raw_item(b"field");
        self.hasher.add_raw_item(name.as_bytes());
        self.hasher.add_item_with(f);
        self
    }

    fn finish(self) -> crate::Finished {
        self.hasher.add_raw_item(b"end");
        crate::Finished(())
    }
}

#[must_use = "call finish() to complete the struct encoding"]
pub trait Hasher {
    type Hasher: crate::Hasher;

    fn field<I: Digestable + Sized>(self, name: &str, value: I) -> Self;
    fn field_with<F: FnOnce(&mut Self::Hasher) -> crate::Finished>(self, name: &str, f: F) -> Self;
    fn finish(self) -> crate::Finished;
}
