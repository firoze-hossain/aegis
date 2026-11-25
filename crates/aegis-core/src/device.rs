//! Hardware firmware device model and descriptors.

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeviceCategory {
    SpiControllerBios,
    SpiControllerIfd,
    StorageDrive,
    EmbeddedController,
    TpmSecurityChip,
    NetworkController,
}

impl DeviceCategory {
    pub fn display_name(&self) -> &'static str {
        match self {
            DeviceCategory::SpiControllerBios => "System Firmware",
            DeviceCategory::SpiControllerIfd => "Chipset Region",
            DeviceCategory::StorageDrive => "Storage Controller",
            DeviceCategory::EmbeddedController => "Embedded Controller",
            DeviceCategory::TpmSecurityChip => "Security Device",
            DeviceCategory::NetworkController => "Network Adapter",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeviceFlag {
    InternalDevice,
    Updatable,
    NeedsRebootAfterInstall,
    CryptoHashVerificationAvailable,
    SystemRequiresExternalPower,
    DeviceUsableDuringUpdate,
}

impl DeviceFlag {
    pub fn label(&self) -> &'static str {
        match self {
            DeviceFlag::InternalDevice => "Internal device",
            DeviceFlag::Updatable => "Updatable",
            DeviceFlag::NeedsRebootAfterInstall => "Needs a reboot after installation",
            DeviceFlag::CryptoHashVerificationAvailable => "Cryptographic hash verification is available",
            DeviceFlag::SystemRequiresExternalPower => "System requires external power source",
            DeviceFlag::DeviceUsableDuringUpdate => "Device is usable for the duration of the update",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FirmwareDevice {
    pub id: String,
    pub name: String,
    pub subtype: Option<String>,
    pub vendor: String,
    pub current_version: Option<String>,
    pub available_version: Option<String>,
    pub guids: Vec<String>,
    pub flags: Vec<DeviceFlag>,
    pub checksum: Option<String>,
    pub checksum_verified: bool,
    pub icon_type: u8,
}

impl FirmwareDevice {
    pub fn new(
        id: &str,
        name: &str,
        subtype: Option<&str>,
        vendor: &str,
        current_version: Option<&str>,
        available_version: Option<&str>,
        guids: Vec<String>,
        flags: Vec<DeviceFlag>,
        icon_type: u8,
    ) -> Self {
        Self {
            id: String::from(id),
            name: String::from(name),
            subtype: subtype.map(String::from),
            vendor: String::from(vendor),
            current_version: current_version.map(String::from),
            available_version: available_version.map(String::from),
            guids,
            flags,
            checksum: None,
            checksum_verified: false,
            icon_type,
        }
    }

    pub fn has_update(&self) -> bool {
        match (&self.current_version, &self.available_version) {
            (Some(cur), Some(avail)) => cur != avail,
            (None, Some(_)) => true,
            _ => false,
        }
    }

    pub fn is_updatable(&self) -> bool {
        self.flags.contains(&DeviceFlag::Updatable)
    }

    pub fn requires_ac_power(&self) -> bool {
        self.flags.contains(&DeviceFlag::SystemRequiresExternalPower)
    }
}
