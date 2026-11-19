# Stuctured digest

This is a quite simple rust crate based on RustCryptos [digest](https://crates.io/crates/digest)

It uses the SequenceHash format and provides helpers for you to hash structs, enums and collection.
To be able to use it, you just have to implement the `Digestable` trait on your types.
