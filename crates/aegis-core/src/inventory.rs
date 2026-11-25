//! Hardware firmware inventory and lifecycle coordinator.

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::{string::String, vec, vec::Vec};

use crate::device::{DeviceFlag, FirmwareDevice};
use crate::update::FirmwareRelease;

#[derive(Clone, Debug)]
pub struct HardwareInventory {
    pub devices: Vec<FirmwareDevice>,
    pub releases: Vec<FirmwareRelease>,
    pub last_checked_timestamp: u64,
}

impl Default for HardwareInventory {
    fn default() -> Self {
        Self::new()
    }
}

impl HardwareInventory {
    pub fn new() -> Self {
        let mut inv = Self {
            devices: Vec::new(),
            releases: Vec::new(),
            last_checked_timestamp: 1764000000,
        };
        inv.populate_default_devices();
        inv.populate_catalog_releases();
        inv
    }

    pub fn populate_default_devices(&mut self) {
        // 1. Internal SPI Controller (BIOS) - from screenshot 1
        let mut dev1 = FirmwareDevice::new(
            "spi-bios-0",
            "Internal SPI Controller (BIOS)",
            None,
            "Intel",
            Some("F2"),
            Some("F3"),
            vec![String::from("75af99a7-8ae0-5c84-8091-cfe7b595f03a")],
            vec![
                DeviceFlag::InternalDevice,
                DeviceFlag::Updatable,
                DeviceFlag::NeedsRebootAfterInstall,
                DeviceFlag::CryptoHashVerificationAvailable,
            ],
            0,
        );
        dev1.checksum = Some(String::from("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"));
        dev1.checksum_verified = true;
        self.devices.push(dev1);

        // 2. Internal SPI Controller (IFD descriptor region) - from screenshot 2
        let mut dev2 = FirmwareDevice::new(
            "spi-ifd-0",
            "Internal SPI Controller (IFD descriptor region)",
            None,
            "Intel",
            None,
            None,
            vec![String::from("0b745c77-9b02-5c53-9cda-ca6cd77856a9")],
            vec![
                DeviceFlag::InternalDevice,
                DeviceFlag::Updatable,
                DeviceFlag::NeedsRebootAfterInstall,
                DeviceFlag::CryptoHashVerificationAvailable,
            ],
            0,
        );
        dev2.checksum = Some(String::from("a4f210d68b9193b22cf94b8e2193e21b8492048f72a492b71940a49182371284"));
        dev2.checksum_verified = true;
        self.devices.push(dev2);

        // 3. SSD 860 EVO 250GB - from screenshot 3
        let mut dev3 = FirmwareDevice::new(
            "samsung-860-evo",
            "SSD 860 EVO 250GB",
            Some("ATA drive"),
            "Samsung",
            Some("RVT02B6Q"),
            Some("RVT04B6Q"),
            vec![
                String::from("95a1ee2f-402f-570b-9c51-fde95db6849a"),
                String::from("5b23aa59-694d-5622-a0b6-d294b9821711"),
                String::from("00641f76-401e-5a04-8167-4c567e3116f8"),
            ],
            vec![
                DeviceFlag::InternalDevice,
                DeviceFlag::Updatable,
                DeviceFlag::SystemRequiresExternalPower,
                DeviceFlag::NeedsRebootAfterInstall,
                DeviceFlag::DeviceUsableDuringUpdate,
            ],
            1,
        );
        dev3.checksum = Some(String::from("d7a8fbb307d7809469ca9abcb0082e4f8d5651e46d3cdb762d02d0bf37c9e592"));
        dev3.checksum_verified = true;
        self.devices.push(dev3);
    }

    pub fn populate_catalog_releases(&mut self) {
        self.releases.push(FirmwareRelease {
            target_device_id: String::from("spi-bios-0"),
            version: String::from("F3"),
            vendor: String::from("Intel"),
            summary: String::from("System BIOS security update and microcode mitigation"),
            changelog: String::from("Mitigates transient execution vulnerabilities and enhances PCIe link power states."),
            payload_size: 16 * 1024 * 1024,
            sha256_hash: String::from("5e884898da28047151d0e56f8dc6292773603d0d6aabbdd62a11ef721d1542d8"),
            requires_reboot: true,
        });

        self.releases.push(FirmwareRelease {
            target_device_id: String::from("samsung-860-evo"),
            version: String::from("RVT04B6Q"),
            vendor: String::from("Samsung"),
            summary: String::from("Storage controller stability and queued TRIM enhancement"),
            changelog: String::from("Optimizes NAND block wear leveling algorithms and SATA link power saving mode."),
            payload_size: 4 * 1024 * 1024,
            sha256_hash: String::from("4b227777d4dd1fc61c6f884f48641d02b4d121d3fd328cb08b5531fcacdabf8a"),
            requires_reboot: true,
        });
    }

    pub fn update_device_checksums(&mut self, device_id: &str) -> bool {
        if let Some(dev) = self.devices.iter_mut().find(|d| d.id == device_id) {
            let mock_raw_bytes = format!("{}:{}:{}", dev.name, dev.vendor, dev.current_version.as_deref().unwrap_or(""));
            dev.checksum = Some(crate::checksum::compute_sha256(mock_raw_bytes.as_bytes()));
            dev.checksum_verified = true;
            true
        } else {
            false
        }
    }

    pub fn apply_update(&mut self, device_id: &str) -> Result<String, &'static str> {
        let release = self.releases.iter().find(|r| r.target_device_id == device_id)
            .cloned()
            .ok_or("No matching firmware release available")?;

        let dev = self.devices.iter_mut().find(|d| d.id == device_id)
            .ok_or("Target device not found")?;

        if !dev.is_updatable() {
            return Err("Device does not support live firmware flashing");
        }

        dev.current_version = Some(release.version.clone());
        dev.available_version = None;
        dev.checksum = Some(release.sha256_hash.clone());
        dev.checksum_verified = true;

        Ok(release.version)
    }
}
