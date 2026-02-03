/// # Shamir Secret Sharing Library
///
/// This library implements Shamir's Secret Sharing algorithm, allowing a secret to be split into multiple shares
/// such that a minimum number of shares are required to reconstruct the original secret.
/// The implementation uses Galois Field arithmetic over GF(256) for the polynomial operations.
///
/// # Example
///
/// ```rust
/// use shamir_rust::ShamirSS;
/// use std::collections::BTreeMap;
///
/// let secret = b"Hello, world!";
/// let n = 5;
/// let k = 3;
///
/// let shares = ShamirSS::split(n, k, secret.to_vec()).unwrap();
///
/// // Use at least k shares to reconstruct
/// let mut parts = BTreeMap::new();
/// for i in 1..=k {
///     parts.insert(i, shares[&i].clone());
/// }
///
/// let reconstructed = ShamirSS::join(parts).unwrap();
/// assert_eq!(reconstructed, secret);
/// ```
mod tables;
use std::{
    collections::{BTreeMap, HashSet},
    fmt::Debug,
};
use rand::distr::StandardUniform;
use rand::Rng;
use tables::{EXP, LOG};

/// Shamir Secret Sharing implementation.
/// This struct provides static methods for splitting and joining secrets using Shamir's algorithm.
#[derive(Debug, Clone)]
pub struct ShamirSS;

impl ShamirSS {
    /// Splits a secret into `n` shares, requiring at least `k` shares to reconstruct the secret.
    ///
    /// # Arguments
    ///
    /// * `n` - Total shares to generate (up to 255).
    /// * `k` - Threshold needed to reconstruct (must be > 1).
    /// * `secret` - The secret data as a byte vector.
    pub fn split(n: i32, k: i32, secret: Vec<u8>) -> Result<BTreeMap<i32, Vec<u8>>, String> {
        if k <= 1 { return Err("Threshold k must be greater than 1".to_string()); }
        if n < k { return Err("Total shares n must be greater than or equal to k".to_string()); }
        if n > 255 { return Err("Total shares n cannot exceed 255".to_string()); }
        if secret.is_empty() { return Err("Secret cannot be empty".to_string()); }

        let seclen = secret.len();
        let mut values: Vec<Vec<u8>> = vec![vec![0u8; seclen]; n as usize];
        let degree = (k - 1) as i32;

        for (i, &byte) in secret.iter().enumerate() {
            let p = GFC256::generate(degree, byte);
            for x in 1..=n {
                values[(x - 1) as usize][i] = GFC256::eval(&p, x as u8);
            }
        }

        let mut parts = BTreeMap::new();
        for i in 1..=n {
            parts.insert(i, values[(i - 1) as usize].clone());
        }

        Ok(parts)
    }

    /// Reconstructs the original secret from a set of shares.
    pub fn join(parts: BTreeMap<i32, Vec<u8>>) -> Result<Vec<u8>, String> {
        if parts.is_empty() {
            return Err("No parts provided".to_string());
        }

        let lengths: HashSet<usize> = parts.values().map(|v| v.len()).collect();
        if lengths.len() != 1 {
            return Err("Varying lengths of part values".to_string());
        }

        let secret_len = *lengths.iter().next().unwrap();
        let mut secret = vec![0u8; secret_len];

        for i in 0..secret_len {
            let points: Vec<Vec<u8>> = parts.iter()
                .map(|(&idx, data)| vec![idx as u8, data[i]])
                .collect();

            secret[i] = GFC256::interpolate(points);
        }

        Ok(secret)
    }
}

/// Galois Field operations over GF(256).
struct GFC256;

impl GFC256 {
    #[inline]
    fn add(a: u8, b: u8) -> u8 { a ^ b }

    #[inline]
    fn sub(a: u8, b: u8) -> u8 { a ^ b }

    fn mul(a: u8, b: u8) -> u8 {
        if a == 0 || b == 0 { return 0; }
        let log_sum = LOG[a as usize] as usize + LOG[b as usize] as usize;
        EXP[log_sum % 255]
    }

    fn div(a: u8, b: u8) -> u8 {
        if b == 0 { panic!("Division by zero in GF(256)"); }
        if a == 0 { return 0; }
        let log_diff = (LOG[a as usize] as i32 - LOG[b as usize] as i32 + 255) % 255;
        EXP[log_diff as usize]
    }

    fn eval(p: &[u8], x: u8) -> u8 {
        let mut result = 0u8;
        for &coeff in p.iter().rev() {
            result = Self::add(Self::mul(result, x), coeff);
        }
        result
    }

    fn generate(degree: i32, secret_byte: u8) -> Vec<u8> {
        let mut rng = rand::rng();
        let mut p = vec![0u8; (degree + 1) as usize];
        p[0] = secret_byte;
        for i in 1..=degree as usize {
            p[i] = rng.sample(StandardUniform);
        }
        // Ensure the leading coefficient is non-zero to maintain the degree
        while p[degree as usize] == 0 {
            p[degree as usize] = rng.sample(StandardUniform);
        }
        p
    }

    fn interpolate(points: Vec<Vec<u8>>) -> u8 {
        let mut y = 0u8;
        let len = points.len();
        for i in 0..len {
            let mut li = 1u8;
            for j in 0..len {
                if i != j {
                    let num = points[j][0];
                    let den = Self::sub(points[i][0], points[j][0]);
                    li = Self::mul(li, Self::div(num, den));
                }
            }
            y = Self::add(y, Self::mul(li, points[i][1]));
        }
        y
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let secret = b"Hello Shamir Shared Secret!!!!!";
        let numparts = 5;
        let miniumparts = 3;



        let keys = ShamirSS::split(numparts, miniumparts, secret.to_vec());
        assert!(keys.is_ok());
        let keys = keys.unwrap();
        let mut parts:BTreeMap<i32,Vec<u8>>=BTreeMap::new();
        for (key, value) in &keys {
            // Copy only entries with keys less than or equal to 3
            if *key <= miniumparts {
                parts.insert(*key, value.clone());
            }
        }
        let nshared = ShamirSS::join(parts);
        assert!(nshared.is_ok());
        let shared = nshared.unwrap();
        assert_eq!(shared, secret.to_vec());

    }
}
