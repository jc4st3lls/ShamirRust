# Shamir Algorithm

A Rust implementation of Shamir's Secret Sharing algorithm using Galois Field arithmetic over GF(256).

## Description

Shamir's Secret Sharing is a cryptographic algorithm that allows a secret to be divided into multiple shares such that:
- The secret can be reconstructed from a minimum threshold of shares
- Fewer than the threshold shares reveal no information about the secret

This implementation splits secrets byte-by-byte using polynomial interpolation over the finite field GF(256).

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
shamir-algorithm = "0.1.0"
```

### Example

```rust
use shamir_algorithm::ShamirSS;
use std::collections::BTreeMap;

fn main() {
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
}
```

## API

### `ShamirSS::split(n: i32, k: i32, secret: Vec<u8>) -> Result<BTreeMap<i32, Vec<u8>>, String>`

Splits a secret into `n` shares, requiring at least `k` shares to reconstruct.

- `n`: Total number of shares (1 ≤ k ≤ n ≤ 255)
- `k`: Threshold number of shares needed (k > 1)
- `secret`: The secret as bytes

Returns a map of share indices to share data.

### `ShamirSS::join(parts: BTreeMap<i32, Vec<u8>>) -> Result<Vec<u8>, String>`

Reconstructs the secret from a set of shares.

- `parts`: Map of share indices to share data

Returns the reconstructed secret.

## License

MIT
