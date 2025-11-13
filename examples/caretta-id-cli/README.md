# caretta-id-cli
Reference tool to generate/encode/decode [caretta-id](https://github.com/fluo10/caretta-id).

## Installation

```
cargo install caretta-id-cli
```

## Usage

```
Reference tool to generate/encode/decode caretta-id, Multi-length Triplet ID

Usage: caretta-id-cli <COMMAND>

Commands:
  decode    Decode caretta-id string to integer
  encode    Encode integer to caretta-id string
  generate  Generate random caretta-id
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

```
Length options:
  -s, --single     Use CarettaIdS (Single-length Caretta ID)
  -d, --double     Use CarettaIdD (Double-length Caretta ID)
  -t, --triple     Use CarettaIdT (Triple-length Caretta ID)
  -q, --quadruple  Use CarettaIdQ (Quadruple-length Caretta ID)
```

### Generate new CarettaIdS

```
$ caretta-id-cli generate --single
abc
```

### Encode CarettaIdD

```
$ caretta-id-cli encode --double 0
000-000
```

### Decode CarettaIdQ

```
$ caretta-id-cli decode --quadruple 000-000-000-000
0
```

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.
