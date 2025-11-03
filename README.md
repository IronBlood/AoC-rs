# AoC-rs

This project contains Java translations of my [Advent of Code solutions in JavaScript](https://github.com/IronBlood/AoC-js/), with the goal of exploring Rust features.

## Prerequisites

- Rust and cargo
- Input files (e.g., `src/y2017/d01/input.txt`)

## Usage

```bash
# Scaffold files for a specific day
cargo run -- 2017-01 s

# Run unit tests for a specific day
cargo test y2017::d01

# Run unit tests for a year
cargo test y2017

# Run a solution
cargo run -- 2017-01
```

## License

MIT
