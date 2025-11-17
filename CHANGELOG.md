# Changelog

All notable changes to this workspace will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.8.1] - 2025-11-18

### Added

- bytes conversion

## [0.8.0] - 2025-11-16

### Added

- New `CarettaId` format like `0123abc`

### Deprecated
- All old caretta id format
  - `CarettaIdS`
  - `CarettaIdD`
  - `CarettaIdT`
  - `CarettaIdQ`

## [0.7.0] - 2025-11-14

### Added

- `redb` feature

### Changed

- **Rename `mtid` to `caretta-id`**

## [0.6.0] - 2025-10-21

### Added

- `from_proto_lossy` method for lossy conversion between rust struct and protobuf definition.

### Changed

- Rename `from_int_lossy` to `from_uint_lossy`
- Move protobuf conversion implemention to each mtid's submodule.

## [0.5.0] - 2025-10-20

### Changed

- Split protobuf definitions to [submodule](https://github.com/fluo10/mtid-proto).
- Fix field name of protobuf difinitions from `id` to `value`

## [0.4.0] - 2025-10-19

### Changed

- Refactor internal module structure.
- Rename `prost` module to `proto`

## [0.3.0] - 2025-10-18

### Added

- Public `triplet` module and `alphabet` module for references.
- Fuzzing tests
- `arbitrary` feature.

### Changed

- (`mtid-cli`) Length options become required.
- Rewrite internal character decoding function.

## [0.2.1] - 2025-10-16

### Added
- Document on docs.rs now contains all features with labels.
- (`mtid-cli`) Licenses.

### Changed
- (`mtid-cli`) Bump `mtid` to `v0.2.1`.

### Removed
- Unnecessary files like `.gitignore`, `.vscode` and `.github` are removed from published package.

## [0.2.0] - 2025-10-15

### Added

- `rand` feature (as default feature)
  - Dependencies on `rand` crate is now optional.
- Optional feature flags:
  - `std` (Default feature): Enable `std` features.
  - `rand` (Default feature): Enable random MTID generation. 
- `no_std` support by disable default `std` feature

### Changed

- Bump version of `rand` crate to `0.9.2`
- `MTID::random()` functionsnow uses thread_rng by default, so no arguments are required.
- Dependencies on `rand` crate is now optional.
- `Error` type has been almost completely rewritten to support `no_std`.
- The functions for conversion with strings has been almost completely rewitten to support `no_std`.
- (`mtid-cli`) Bump `mtid` to `v0.2.0`.

### Removed
- Dependency on `thiserror` crate

## [0.1.0] - 2025-10-13

### Added

- Initial release with multi-length triplet ID support
- `Stid`: Single triplet ID (3 characters)
- `Dtid`: Double Triplet ID (7 characters with delimiter)
- `Ttid`: Triple Triplet ID (11 characters with delimiter)
- `Qtid`: Quadruple triplet ID (15 characters with delimiter)
- Support for random ID generation
- String parsing with ambiguous character handling
- Integer conversion (to/from)
- Optional feature flags:
  - `serde`: Serialization/deserialization support
  - `rusqlite`: SQLite database integration
  - `sea-orm`: SeaORM ORM integration
  - `prost`: Protocol Buffers support
- (`mitd-cli`) Initial release example cli tool with 3 subcommands
  - `generate`: generate random id
  - `encode`: encode integer to string
  - `decode`: to decode string to integer

[Unreleased]: https://github.com/fluo10/caretta-id/compare/v0.8.0...HEAD
[0.8.0]: https://github.com/fluo10/caretta-id/compare/v0.7.0...v0.8.0
[0.7.0]: https://github.com/fluo10/caretta-id/compare/v0.6.0...v0.7.0
[0.6.0]: https://github.com/fluo10/caretta-id/compare/v0.5.0...v0.6.0
[0.5.0]: https://github.com/fluo10/caretta-id/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/fluo10/caretta-id/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/fluo10/caretta-id/compare/v0.2.1...v0.3.0
[0.2.1]: https://github.com/fluo10/caretta-id/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/fluo10/caretta-id/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/fluo10/caretta-id/releases/tag/v0.1.0
