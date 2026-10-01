# wc

A lightweight command-line `wc` (word count) clone written in Rust.

## Usage

```bash
cargo run -- [OPTIONS] <FILE>
```

### Options

- `-l`: Count lines
- `-w`: Count words
- `-c`: Count bytes

If no options are specified, lines, words, and bytes are all counted.
