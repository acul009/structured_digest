# Structured digest

This is a quite simple rust crate based on RustCryptos [digest](https://crates.io/crates/digest)

It uses the SequenceHash format and provides helpers for you to hash structs, enums and collection.
To be able to use it, you just have to implement the `Digestible` trait on your types.

## Dependencies

```toml
[dependencies]
structured_digest = "0.1"
sha2 = "0.11"
```

## Scalars and collections

```rust
use sha2::Sha256;
use structured_digest::prelude::*;

let mut hasher = StructuredHasher::<Sha256>::new();
hasher.add_item("hello".as_bytes());
hasher.add_item(42u32);
hasher.digest_sequence().iter([1u32, 2, 3].into_iter()).finish();
let digest = hasher.finalize();
assert_eq!(digest.len(), 32);
```

## Structs

```rust
use sha2::Sha256;
use structured_digest::{Finished, Hasher, prelude::*};

struct Person {
    name: String,
    age: u32,
}

impl Digestible for Person {
    fn digest(&self, hasher: &mut impl Hasher) -> Finished {
        hasher.digest_struct()
            .field("name", self.name.as_bytes())
            .field("age", self.age)
            .finish()
    }
}

let person = Person { name: "Alice".into(), age: 42 };
let mut hasher = StructuredHasher::<Sha256>::new();
hasher.add_item(&person);
let digest = hasher.finalize();
```

## Enums

```rust
use sha2::Sha256;
use structured_digest::{Finished, Hasher, prelude::*};

enum Message {
    Ping,
    Text(String),
}

impl Digestible for Message {
    fn digest(&self, hasher: &mut impl Hasher) -> Finished {
        match self {
            Self::Ping => hasher.digest_enum("Ping").finish(),
            Self::Text(text) => hasher.digest_enum("Text")
                .field("text", text.as_bytes())
                .finish(),
        }
    }
}

let mut hasher = StructuredHasher::<Sha256>::new();
hasher.add_item(&Message::Ping);
hasher.add_item(&Message::Text("hello".into()));
let digest = hasher.finalize();
```
