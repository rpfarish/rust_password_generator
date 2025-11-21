# Password Generator

## Overview

This is a configurable command-line password generator written in Rust. It generates cryptographically random passwords with customizable character sets and length requirements.

## Features

- **Configurable Character Sets**: Enable/disable lowercase, uppercase, numbers, and symbols
- **Variable Length**: Generate passwords of any specified length
- **Secure Randomness**: Uses the `rand` crate for random character generation
- **ASCII Printable Range**: Generates characters from ASCII 33-126 (all printable characters)

## Requirements

- Rust 1.70 or later
- `rand` crate (add to `Cargo.toml`)

## Installation

```bash
cargo build --release
```

## Usage

Currently, password parameters are configured via constants in `main()`. Default configuration:

```rust
let password_length: u64 = 15;
let lowercase = true;
let uppercase = true;
let numbers = true;
let symbols = true;
```

Run the generator:

```bash
cargo run
```

## Character Sets

The generator supports four character categories:

- **Lowercase**: `a-z`
- **Uppercase**: `A-Z`
- **Numbers**: `0-9`
- **Symbols**: `! @ # $ % ^ & * ( ) [ ] { } - _ = + ' " ; : , . < > / ? \ | ~`


## Implementation Details

The generator uses a rejection sampling approach:
1. Generates random ASCII characters in the printable range (33-126)
2. Checks if the character matches the enabled character sets
3. Adds valid characters to the password string until the desired length is reached

## Future Enhancements

- [ ] CLI argument parsing for runtime configuration
- [ ] Minimum character requirements per category (e.g., at least 1 symbol, 1 number)
- [ ] Password strength estimation
- [ ] Batch generation mode
- [ ] Exclude ambiguous characters option (0/O, 1/l/I)
- [ ] Custom character set support

## Security Note

This generator is suitable for creating strong passwords for general use. For high-security applications requiring CSPRNG guarantees, verify that `rand::rng()` meets your specific requirements.

---
