# hashdenticon

generate identicons from hashed seed strings

## Features

- deterministic
- mostly-unique, symmetric, github-style patterns (different generation algorithm)
- fast and lightweight
- customizable image size, grid size, padding
- simple CLI interface

## Installation

```bash
cargo install --git https://github.com/patrickarmengol/hashdenticon
```

## Usage

### Syntax

```
hashdenticon [OPTIONS] <SEED>

Arguments:
  <SEED>  Seed text (username, email, etc.) to generate identicon from

Options:
  -o, --output <OUTPUT>      Output file path [default: <seed>.png]
  -s, --size <SIZE>          Size of the identicon in pixels [default: 420]
  -g, --grid <GRID>          Grid size for the pattern [default: 5]
  -p, --padding <PADDING>    Padding as a percentage of size [default: 8]
  -h, --help                 Print help
  -V, --version              Print version
```

### Examples

```bash
# generate an identicon for a username
hashdenticon alice
# creates: alice.png

# generate from an email (output uses hash as filename)
hashdenticon "bob@example.com"
# creates: 5ff860bf1190596c7188ab851db691f0f3169c453936e9e1eba2f9a47f7a0018.png

# custom output path
hashdenticon "charlie" -o avatars/charlie-avatar.png

# custom size and grid
hashdenticon "david" -s 256 -g 7

# no padding
hashdenticon "eve" -p 0
```

## How it Works

1. sha256 hash from seed string
2. first 3 bytes for RGB color (constrained for good contrast)
3. remaining 29 bytes for pattern generation, filling half grid bit-by-bit, then mirroring
4. rendering with padding as png image

## TODO

- output as SVG for scalable icons
- output to stdout
- batch generation
- library crate
- alternative hash algorithms
- loop through hash bytes to support larger grid sizes
- non-square-grid patterns?
- multiple colors? color schemes?

## License

This project is dual-licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
