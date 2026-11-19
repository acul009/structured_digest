use digest::{Digest, common::BlockSizeUser, typenum::Unsigned};

use crate::{sequence::DigestSequence, structured::DigestStruct};

mod default_impls;
mod sequence;
mod structured;

pub mod prelude {
    pub use crate::Digestible;
    pub use crate::Hasher as _;
    pub use crate::StructuredHasher;
    pub use crate::sequence::Hasher as _;
    pub use crate::structured::Hasher as _;
}

#[must_use = "call finalize to get the hash value"]
pub struct StructuredHasher<T: Digest + BlockSizeUser> {
    inner: T,
    item_count: u128,
}

impl<T: Digest + BlockSizeUser> StructuredHasher<T> {
    /// Creates an empty sequence hasher with an empty customization string.
    pub fn new() -> Self {
        let block_size = T::BlockSize::USIZE;

        let mut key_block = vec![0u8; block_size];
        key_block[0] = 0x55;
        let mut inner = T::new();
        inner.update(&key_block);

        // Inner header identifier
        let mut inner_header = Vec::from(&b"SEQHSH_I"[..]);
        // Function indicator
        inner_header.extend_from_slice(&2u128.to_be_bytes());
        // Key length
        inner_header.extend_from_slice(&0u128.to_be_bytes());
        inner_header.resize(inner_header.len().div_ceil(block_size) * block_size, 0);
        inner.update(&inner_header);

        Self {
            inner,
            item_count: 0,
        }
    }

    /// Appends one item, preserving its boundary even when it is empty.
    fn add_raw_item(&mut self, item: &[u8]) {
        let next_count = self
            .item_count
            .checked_add(1)
            .expect("SequenceHash item count overflow");

        self.inner.update(item);
        self.inner.update((item.len() as u128).to_le_bytes());
        self.item_count = next_count;
    }

    pub fn add_item<I: Digestible + Sized>(&mut self, item: I) {
        item.digest(self);
    }

    fn add_item_with<F: FnOnce(&mut Self) -> Finished>(&mut self, f: F) {
        f(self);
    }

    /// Consumes the sequence hasher and returns its digest.
    pub fn finalize(self) -> digest::Output<T> {
        let block_size = T::BlockSize::USIZE;
        let mut outer = T::new();

        let mut key_block = vec![0u8; block_size];
        key_block[0] = 0xaa;
        outer.update(&key_block);

        // Outer header identifier
        let mut outer_header = Vec::from(&b"SEQHSH_O"[..]);
        // Function indicator
        outer_header.extend_from_slice(&2u128.to_be_bytes());
        // Customization string length
        outer_header.extend_from_slice(&0u128.to_be_bytes());
        // Key length
        outer_header.extend_from_slice(&0u128.to_be_bytes());
        outer_header.resize(outer_header.len().div_ceil(block_size) * block_size, 0);
        outer.update(&outer_header);

        // The empty customization string derives to one block of zeroes.

        key_block[0] = 0;
        outer.update(&key_block);
        // Item count
        outer.update(self.item_count.to_be_bytes());
        // Output length
        outer.update((<T as Digest>::output_size() as u128).to_be_bytes());
        // Inner digest
        outer.update(self.inner.finalize());
        outer.finalize()
    }
}

impl<T: Digest + BlockSizeUser> Hasher for StructuredHasher<T> {
    /// Adds a typed scalar, keeping payload bytes separate from structural tags.
    fn digest_scalar(&mut self, bytes: &[u8]) -> Finished {
        self.add_raw_item(b"scalar");
        self.add_raw_item(bytes);
        Finished(())
    }

    fn digest_sequence(&mut self) -> impl sequence::Hasher {
        self.add_raw_item(b"sequence");
        DigestSequence { hasher: self }
    }

    /// Starts a struct. Names and field order are part of its encoding.
    fn digest_struct(&mut self) -> impl structured::Hasher {
        self.add_raw_item(b"struct");
        DigestStruct { hasher: self }
    }

    fn digest_enum(&mut self, variant: &str) -> impl structured::Hasher {
        self.add_raw_item(b"enum");
        self.add_raw_item(variant.as_bytes());
        DigestStruct { hasher: self }
    }
}

pub trait Hasher {
    fn digest_scalar(&mut self, bytes: &[u8]) -> Finished;
    fn digest_sequence(&mut self) -> impl sequence::Hasher;
    fn digest_struct(&mut self) -> impl structured::Hasher;
    fn digest_enum(&mut self, variant: &str) -> impl structured::Hasher;
}

pub struct Finished(pub(crate) ());

/// Encodes a value using typed scalars or finished struct builders.
///
/// Implementations must use a stable encoding; raw items bypass this structure.
pub trait Digestible {
    fn digest(&self, hasher: &mut impl Hasher) -> Finished;
}
