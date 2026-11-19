use digest::{Digest, common::BlockSizeUser, typenum::Unsigned};

use crate::{sequence::DigestSequence, structured::DigestStruct};

mod default_impls;
mod sequence;
mod structured;

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

    fn add_item<I: Digestable + ?Sized>(&mut self, item: &I) {
        item.digest(self);
    }

    /// Adds a typed scalar, keeping payload bytes separate from structural tags.
    pub fn digest_scalar(&mut self, bytes: &[u8]) {
        self.add_raw_item(b"scalar");
        self.add_raw_item(bytes);
    }

    pub fn digest_sequence(&mut self) -> DigestSequence<'_, T> {
        self.add_raw_item(b"sequence");
        DigestSequence { hasher: self }
    }

    /// Starts a struct. Names and field order are part of its encoding.
    pub fn digest_struct(&mut self) -> DigestStruct<'_, T> {
        self.add_raw_item(b"struct");
        DigestStruct { hasher: self }
    }

    pub fn digest_enum(&mut self, variant: &str) -> DigestStruct<'_, T> {
        self.add_raw_item(b"enum");
        self.add_raw_item(variant.as_bytes());
        DigestStruct { hasher: self }
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

/// Encodes a value using typed scalars or finished struct builders.
///
/// Implementations must use a stable encoding; raw items bypass this structure.
pub trait Digestable {
    fn digest<T: Digest + BlockSizeUser>(&self, hasher: &mut StructuredHasher<T>);
}
