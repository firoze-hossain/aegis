//! Hardware GUID generator and formatter.

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::{format, string::String};

pub fn generate_hardware_guid(seed: &str) -> String {
    let mut h1: u32 = 0x811c9dc5;
    let mut h2: u32 = 0x5bd1e995;
    for (i, b) in seed.bytes().enumerate() {
        if i % 2 == 0 {
            h1 = h1.wrapping_mul(0x01000193) ^ (b as u32);
        } else {
            h2 = h2.wrapping_mul(0x5bd1e995) ^ (b as u32);
        }
    }
    format!(
        "{:08x}-{:04x}-5{:03x}-9{:03x}-{:08x}{:04x}",
        h1,
        (h2 >> 16) & 0xffff,
        (h1 >> 16) & 0x0fff,
        (h2 & 0x0fff),
        h2.wrapping_mul(31),
        (h1 & 0xffff)
    )
}

pub fn is_valid_guid(s: &str) -> bool {
    if s.len() != 36 {
        return false;
    }
    let parts: Vec<&str> = s.split('-').collect();
    if parts.len() != 5 {
        return false;
    }
    parts[0].len() == 8
        && parts[1].len() == 4
        && parts[2].len() == 4
        && parts[3].len() == 4
        && parts[4].len() == 12
        && parts.iter().all(|p| p.chars().all(|c| c.is_ascii_hexdigit()))
}
