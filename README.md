# ShamirRust Project

This repository contains a Rust implementation of Shamir's Secret Sharing algorithm, split into two main components:

## Projects

### 1. shamir-algorithm 📚
[![Crates.io](https://img.shields.io/crates/v/shamir-algorithm.svg)](https://crates.io/crates/shamir-algorithm)
[![Documentation](https://docs.rs/shamir-algorithm/badge.svg)](https://docs.rs/shamir-algorithm)

A reusable Rust library that implements Shamir's Secret Sharing algorithm using Galois Field arithmetic over GF(256).

**Key Features:**
- Split secrets into multiple shares with configurable thresholds
- Reconstruct secrets from minimum required shares
- Pure Rust implementation with no external dependencies (except rand for randomness)
- Comprehensive documentation and examples

**Location:** [shamir-algorithm/](shamir-algorithm/)

**Documentation:** [README](shamir-algorithm/README.md)

### 2. ShamirRust 🚀
A command-line application that demonstrates the usage of the shamir-algorithm library.

**Features:**
- Interactive command-line interface for secret sharing
- Base64 encoding of shares for easy handling
- Complete example implementation
- Educational demonstrations of the algorithm

**Location:** [ShamirRust/](ShamirRust/)

**Documentation:** [README](ShamirRust/README.md) (in Catalan)

## What is Shamir's Secret Sharing?

Shamir's Secret Sharing is a cryptographic algorithm that allows a secret to be divided into multiple shares such that:
- The secret can be reconstructed from a minimum threshold of shares
- Fewer than the threshold shares reveal no information about the secret

This is particularly useful in scenarios requiring distributed trust, such as:
- Multi-signature wallets in blockchain
- Secure key management systems
- Distributed access control
- Backup systems with redundancy

## Installation

### Library Usage
Add to your `Cargo.toml`:
```toml
[dependencies]
shamir-algorithm = "0.1.0"
```

### Building from Source
```bash
git clone https://github.com/yourusername/ShamirRust.git
cd ShamirRust
cargo build --release
```

## Usage Example

```rust
use shamir_algorithm::ShamirSS;
use std::collections::BTreeMap;

let secret = b"Hello, world!";
let n = 5; // Total shares
let k = 3; // Threshold

// Split the secret
let shares = ShamirSS::split(n, k, secret.to_vec()).unwrap();

// Reconstruct using k shares
let mut parts = BTreeMap::new();
for i in 1..=k {
    parts.insert(i, shares[&i].clone());
}
let reconstructed = ShamirSS::join(parts).unwrap();

assert_eq!(reconstructed, secret);
```

## References

- [Wikipedia: Shamir's Secret Sharing](https://en.wikipedia.org/wiki/Shamir%27s_Secret_Sharing)
- [Original Paper by Adi Shamir](https://en.wikipedia.org/wiki/Shamir%27s_Secret_Sharing#cite_note-Shamir79-1)

## License

MIT License - see [LICENSE](LICENSE) for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

[!["Buy Me A Coffee"](https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png)](https://www.buymeacoffee.com/jcastellsgH)