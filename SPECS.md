# caretta-id Specification (Multi-length Triplet ID)

## Overview

caretta-id is a human-friendly unique identifier format designed for readability, memorability, and compactness. It encodes integer values into a string representation using 3-character blocks, with multiple fixed-length variants available. caretta-id is suitable for use in distributed system, logging, URLs, and other contexts where short, unique identifiers are beneficial.

### Motivation
When I considering implementing IDs for users(not for internal system) to specify items, such as GitHub commit hashes or issue numbers in the distributed system using P2P, the following issues arose.

- Sequential numbers like Git issues are difficult to implement in distributed systems because collitions are unavoidable.
- Exists random number like UUID is too long for users
- Short random number like 7-digit commit hash seems good but it is not standardized specification.

So I decided to make my own ID specifications.

## Structure

Each caretta-id consists of one or more 3-character blocks separated by hyphens(`-`). Each block encode a portion of the underlying integer value using a custom base encoding.

### Block Format

- Each block contains exactly 3 characters.
- Characters are selected from a custom base alphabet (see Encoding section).
- Blocks are ordered from most significant to least significant.

### Examples

- `123` (Single block)
- `456-789` (Double block)
- `abc-def-ghj` (Triple block)
- `jkm-npq-rst-uwx` (Quadruple block)

## Encoding/Decoding

caretta-id uses a custom base encoding/decoding to convert integer value into/from character blocks.

### Alphabet
- Based on BASE32.
- In encoding, visually ambiguous characters: `I`, `L`, `O` and `V` are excluded.
- In decoding, visually ambiguous characters works as alias of valid character.


| value | Encode Digit | Decode Digit
|------:|:------------:|:------------:
|     0 |            0 |        0 o O
|     1 |            1 |    1 i I l L
|     2 |            2 |            2
|     3 |            3 |            3
|     4 |            4 |            4
|     5 |            5 |            5
|     6 |            6 |            6
|     7 |            7 |            7
|     8 |            8 |            8
|     9 |            9 |            9
|    10 |            a |          a A
|    11 |            b |          b B
|    12 |            c |          c C
|    13 |            d |          d D
|    14 |            e |          e E
|    15 |            f |          f F
|    16 |            g |          g G
|    17 |            h |          h H
|    18 |            j |          j J
|    19 |            k |          k K
|    20 |            m |          m M
|    21 |            n |          n N
|    22 |            p |          p P
|    23 |            q |          q Q
|    24 |            r |          r R
|    25 |            s |          s S
|    26 |            t |          t T
|    27 |            u |      u U v V
|    28 |            w |          w W
|    29 |            x |          x X
|    30 |            y |          y Y
|    31 |            z |          z Z

### Bit Width
- Each character encode approximately 5 bits(BASE32).
- Each 3-character block encodes 15 bits.
- Double block caretta-id encodes 45 bits of entropy

### Encoding Process
1. Split integer value by 15 bits.
1. Convert 15 bits integer value to BASE32 representation 3-character blocks.
3. Insert hyphens between blocks.

## Variants

caretta-id supports multiple fixed-length variants:

| Name                                       | Format            | Bit Width |
|:-------------------------------------------|:------------------|----------:|
| `CarettaIdS` (Single length Caretta ID)    | `abc`             | 15 bits
| `CarettaIdD` (Double length Caretta ID)    | `abc-def`         | 30 bits
| `CarettaIdT` (Triple length Caretta ID)    | `abc-def-ghj`     | 45 bits
| `CarettaIdQ` (Quadruple length Caretta ID) | `abc-def-ghj-kmn` | 60 bits

## Examples

|Integer                      | CarettaIdS | CarettaIdD | CarettaIdT    | CarettaIdQ        |
|----------------------------:|:----------:|:----------:|:-------------:|:-----------------:|
|                         `0` | `000`      | `000-000`  | `000-000-000` | `000-000-000-000` |
|                    `32_767` | `zzz`      | `000-zzz`  | `000-000-zzz` | `000-000-000-zzz` |
|             `1_073_741_823` | `-`        | `zzz-zzz`  | `000-zzz-zzz` | `000-000-zzz-zzz` |
|        `35_184_372_088_831` | `-`        | `-`        | `zzz-zzz-zzz` | `000-zzz-zzz-zzz` |
| `1_152_921_504_606_846_975` | `-`        | `-`        | `-`           | `zzz-zzz-zzz-zzz` |

## Implementation Notes

- caretta-id is language-agnostic and can be implemented in any language with integer and string manipulation capabilities.
- Rust implemention provides `CarettaIdS`, `CarettaIdD`, `CarettaIdT` and `CarettaIdQ` structs with common conversion trait.
- Parsing and formatting functions should validate character sets and block length.
- Lossy conversion from oversized interger is allowed. In this case, higher bits should be lost.

## License

This specification is open and free to use under the same license as the caretta-id reference implementation.
