//! Aegis Core library entrypoint.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

pub mod checksum;
pub mod device;
pub mod guid;
pub mod inventory;
pub mod update;
pub mod version;

pub use checksum::*;
pub use device::*;
pub use guid::*;
pub use inventory::*;
pub use update::*;
pub use version::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_inventory_defaults() {
        let inv = HardwareInventory::new();
        assert_eq!(inv.devices.len(), 3);
        assert_eq!(inv.devices[0].name, "Internal SPI Controller (BIOS)");
        assert_eq!(inv.devices[1].name, "Internal SPI Controller (IFD descriptor region)");
        assert_eq!(inv.devices[2].name, "SSD 860 EVO 250GB");
        assert!(inv.devices[0].has_update());
        assert!(inv.devices[2].has_update());
    }

    #[test]
    fn test_guid_generation() {
        let guid = generate_hardware_guid("spi-controller-bios");
        assert!(is_valid_guid(&guid));
    }

    #[test]
    fn test_checksum_computation() {
        let hash = compute_sha256(b"AegisFirmwarePayload");
        assert_eq!(hash.len(), 64);
        assert!(verify_payload_integrity(b"AegisFirmwarePayload", &hash));
    }

    #[test]
    fn test_firmware_update_pipeline() {
        let mut pipeline = UpdatePipeline::new();
        assert_eq!(pipeline.stage, UpdateStage::Idle);
        pipeline.advance_step();
        assert_eq!(pipeline.stage, UpdateStage::PreflightSafetyCheck);
        pipeline.advance_step();
        assert_eq!(pipeline.stage, UpdateStage::StagingPayload);
        pipeline.advance_step();
        assert_eq!(pipeline.stage, UpdateStage::WritingFirmwareBlocks);
        pipeline.advance_step();
        assert_eq!(pipeline.stage, UpdateStage::PostFlashVerification);
        pipeline.advance_step();
        assert_eq!(pipeline.stage, UpdateStage::CompletedScheduledReboot);
        assert!(pipeline.is_finished);
        assert!(pipeline.is_success);
    }
}
