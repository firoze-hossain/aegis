//! Firmware version parsing and comparison utilities.

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::string::String;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VersionType {
    Alphanumeric(String),
    Triple(u32, u32, u32),
}

pub fn compare_firmware_versions(current: &str, candidate: &str) -> i32 {
    if current == candidate {
        return 0;
    }
    if let (Some(c), Some(t)) = (parse_semver(current), parse_semver(candidate)) {
        return match c.cmp(&t) {
            core::cmp::Ordering::Less => -1,
            core::cmp::Ordering::Equal => 0,
            core::cmp::Ordering::Greater => 1,
        };
    }
    if current < candidate {
        -1
    } else {
        1
    }
}

fn parse_semver(s: &str) -> Option<(u32, u32, u32)> {
    let parts: Vec<&str> = s.split('.').collect();
    if parts.len() == 3 {
        let p0 = parts[0].parse::<u32>().ok()?;
        let p1 = parts[1].parse::<u32>().ok()?;
        let p2 = parts[2].parse::<u32>().ok()?;
        Some((p0, p1, p2))
    } else {
        None
    }
}
