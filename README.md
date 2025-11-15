# caretta-id

<!-- cargo-rdme start -->

A human-friendly 7 characters identifier format (e.g. `123abcd`).

For a language agnostic specification of the caretta-id format, see [SPECS.md](https://github.com/fluo10/caretta-id/blob/main/SPECS.md)

## Quick Start

```rust
use caretta_id::CarettaId;

let id = CarettaId::random();
println!("{}", id); // e.g. "123abcd"
```

## Why caretta-id?

Traditional identifier systems face challenges in distributed environments:

- **Sequential numbers** (like GitHub issue numbers) cause collisions in distributed systems
- **UUIDs** are too long and not human-friendly
- **Short hashes** (like Git commit hashes) lack standardization

caretta-id bridges the gap between human readability and technical requirements.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
caretta-id = "0.8"

# With optional features
caretta-id = { version = "0.8", features = ["arbitrary", "serde", "rusqlite", "sea-orm", "prost", "redb"] }
```

### For no_std Environments

This crate support `no_std`.
For `no_std` environment, you'll need to disable default features.

```toml
[dependencies]
caretta-id = { version = "0.8", default-features = false }
```

## Features

- **Human-friendly**: Easy to read, type, and communicate
- **Collision-resistant**: Sufficient entropy for personal distributed systems
- **Compact**: Shorter than UUIDs while maintaining uniqueness
- **Type-safe**: Rust implementation with strong typing
- **Multiple integrations**: Support for serde, rusqlite, sea-orm, and protobuf

### Optional Feature Flags

- `arbitrary`: `arbitrary::Arbitrary` support for fuzzing tests.
- `serde`: Serialization/deserialization support
- `rusqlite`: SQLite database integration
- `sea-orm`: SeaORM ORM integration  
- `prost`: Protocol Buffers support
- `redb`: `redb` integration

## Examples

```rust
use caretta_id::CarettaId;
// Generate random caretta-id
let caretta_id = CarettaId::random();

// e.g. `123abcd`
println!("'{}'", caretta_id);

// Parse from string
let valid_id: CarettaId = "012atvw".parse()?;

// When decoding from BASE32, ambiguous characters (1/l/I, 0/o, v/u) are treated as 1, 0 and v respectively, so they do not cause errors.
let also_valid_id: CarettaId = "ol2atuw".parse()?;
assert_eq!(valid_id, also_valid_id);

// Convert to/from integer
let num: u64 = valid_id.into();
let id_from_int: CarettaId = num.try_into()?;
assert_eq!(valid_id, id_from_int);

// Lossy conversion from oversized int is allowed.
let id_from_overflowed_int = CarettaId::from_u64_lossy(CarettaId::CAPACITY + num);
assert_eq!(valid_id, id_from_overflowed_int);

```

<!-- cargo-rdme end -->

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.
