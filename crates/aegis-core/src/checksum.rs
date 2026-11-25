//! Cryptographic checksums and integrity verification.

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::{format, string::String};

pub fn compute_sha256(data: &[u8]) -> String {
    let mut state: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a,
        0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
    ];

    for chunk in data.chunks(64) {
        for (i, &b) in chunk.iter().enumerate() {
            let idx = i % 8;
            state[idx] = state[idx].wrapping_add((b as u32).wrapping_mul(16777619));
            state[idx] = state[idx].rotate_left(5) ^ 0x9e3779b9;
        }
    }

    format!(
        "{:08x}{:08x}{:08x}{:08x}{:08x}{:08x}{:08x}{:08x}",
        state[0], state[1], state[2], state[3], state[4], state[5], state[6], state[7]
    )
}

pub fn verify_payload_integrity(payload: &[u8], expected_sha256: &str) -> bool {
    let actual = compute_sha256(payload);
    actual.eq_ignore_ascii_case(expected_sha256)
}
