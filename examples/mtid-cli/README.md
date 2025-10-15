# mtid-cli
Reference tool to generate/encode/decode [MTID](https://github.com/fluo10/mtid).

## Installation

```
cargo install mtid-cli
```

## Usage

```
Reference tool to generate/encode/decode MTID, Multi-length Triplet ID

Usage: mtid-cli <COMMAND>

Commands:
  decode    Decode MTID string to integer
  encode    Encode integer to MTID string
  generate  Generate random MTID
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

```
Length options:
  -s, --single     Use STID (Single-length Triplet ID) [aliases: --stid]
  -d, --double     Use DTID (Double-length Triplet ID) [aliases: --dtid]
  -t, --triple     Use TTID (Triple-length Triplet ID) [aliases: --ttid]
  -q, --quadruple  Use QTID (Quadruple-length Triplet ID) [aliases: --qtid]
```

### Generate new STID

```
$ mtid-cli generate --single
abc
```

### Encode DTID

```
$ mtid-cli encode --double 0
000-000
```

### Decode QTID

```
$ mtid-cli decode --quadruple 000-000-000-000
0
```


## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.
