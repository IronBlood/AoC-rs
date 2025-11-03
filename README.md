# AoC-rs

This repo is a Rust version of my 2017 Advent of Code solutions, ported from my own [JavaScript answers](https://github.com/IronBlood/AoC-js/). Other years’ solutions are pulled in (and tweaked to work with my `src/main.rs`) from various folks - each one credits the original author in a comment.

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
