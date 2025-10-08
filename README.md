# MTID (Multi-length Triplet ID)

A human-friendly identifier format based on 3-character blocks ("triplets").
This crate provide multiple fixed-length variants:

- `Stid`: Single triplet (e.g. `abc`)
- `Dtid`: Double triplet (e.g. `456-789`) (Recommended)
- `Ttid`: Triple triplet (e.g. `abc-def-ghj`)

For a language agnostic specification of the MTID format, see [SPECS.md](./SPECS.md).