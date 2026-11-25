//! Aegis Storage: Hardware firmware configuration and persistence.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::{format, string::String, vec::Vec};

use aegis_core::HardwareInventory;

pub struct AegisStorage;

impl AegisStorage {
    pub fn export_inventory_json(inventory: &HardwareInventory) -> String {
        let mut out = String::from("{\n  \"devices\": [\n");
        for (i, dev) in inventory.devices.iter().enumerate() {
            let comma = if i + 1 < inventory.devices.len() { "," } else { "" };
            let flags_str = dev.flags.iter()
                .map(|f| format!("\"{}\"", f.label()))
                .collect::<Vec<_>>()
                .join(", ");
            let guids_str = dev.guids.iter()
                .map(|g| format!("\"{}\"", g))
                .collect::<Vec<_>>()
                .join(", ");

            out.push_str(&format!(
                "    {{\n      \"id\": \"{}\",\n      \"name\": \"{}\",\n      \"vendor\": \"{}\",\n      \"current_version\": {},\n      \"available_version\": {},\n      \"guids\": [{}],\n      \"flags\": [{}]\n    }}{}\n",
                dev.id,
                dev.name,
                dev.vendor,
                dev.current_version.as_deref().map(|v| format!("\"{}\"", v)).unwrap_or_else(|| "null".into()),
                dev.available_version.as_deref().map(|v| format!("\"{}\"", v)).unwrap_or_else(|| "null".into()),
                guids_str,
                flags_str,
                comma
            ));
        }
        out.push_str("  ]\n}");
        out
    }

    pub fn generate_firmware_conf() -> String {
        String::from(
            "[fwupd]\n\
             # Aegis Sovereign Firmware Manager Configuration\n\
             DisabledPlugins=test\n\
             ArchiveStore=/var/lib/fwupd/pending.cab\n\
             AllowUnsigned=false\n\
             RequireACPower=true\n\
             MinBatteryPercentage=30\n\
             TrustedVendorKeys=ed25519,gpg\n"
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_export_inventory_json() {
        let inv = HardwareInventory::new();
        let json = AegisStorage::export_inventory_json(&inv);
        assert!(json.contains("Internal SPI Controller (BIOS)"));
        assert!(json.contains("SSD 860 EVO 250GB"));
        assert!(json.contains("Intel"));
        assert!(json.contains("Samsung"));
    }

    #[test]
    fn test_generate_firmware_conf() {
        let conf = AegisStorage::generate_firmware_conf();
        assert!(conf.contains("AllowUnsigned=false"));
        assert!(conf.contains("RequireACPower=true"));
    }
}
