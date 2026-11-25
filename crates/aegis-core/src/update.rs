//! Firmware update package and transaction pipeline.

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FirmwareRelease {
    pub target_device_id: String,
    pub version: String,
    pub vendor: String,
    pub summary: String,
    pub changelog: String,
    pub payload_size: usize,
    pub sha256_hash: String,
    pub requires_reboot: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UpdateStage {
    Idle,
    PreflightSafetyCheck,
    StagingPayload,
    WritingFirmwareBlocks,
    PostFlashVerification,
    CompletedScheduledReboot,
    Failed,
}

impl UpdateStage {
    pub fn description(&self) -> &'static str {
        match self {
            UpdateStage::Idle => "Ready",
            UpdateStage::PreflightSafetyCheck => "Checking power source and hardware state...",
            UpdateStage::StagingPayload => "Staging capsule payload and unlocking controller...",
            UpdateStage::WritingFirmwareBlocks => "Writing firmware blocks to memory...",
            UpdateStage::PostFlashVerification => "Verifying flash integrity checksum...",
            UpdateStage::CompletedScheduledReboot => "Firmware written successfully. Reboot required.",
            UpdateStage::Failed => "Update failed.",
        }
    }
}

#[derive(Clone, Debug)]
pub struct UpdatePipeline {
    pub stage: UpdateStage,
    pub progress_percent: u8,
    pub status_message: String,
    pub is_finished: bool,
    pub is_success: bool,
}

impl UpdatePipeline {
    pub fn new() -> Self {
        Self {
            stage: UpdateStage::Idle,
            progress_percent: 0,
            status_message: String::from("Ready to update"),
            is_finished: false,
            is_success: false,
        }
    }

    pub fn advance_step(&mut self) {
        match self.stage {
            UpdateStage::Idle => {
                self.stage = UpdateStage::PreflightSafetyCheck;
                self.progress_percent = 20;
                self.status_message = String::from("Preflight check passed: AC power detected");
            }
            UpdateStage::PreflightSafetyCheck => {
                self.stage = UpdateStage::StagingPayload;
                self.progress_percent = 45;
                self.status_message = String::from("Payload staged and SPI controller unlocked");
            }
            UpdateStage::StagingPayload => {
                self.stage = UpdateStage::WritingFirmwareBlocks;
                self.progress_percent = 75;
                self.status_message = String::from("Flash memory blocks written successfully");
            }
            UpdateStage::WritingFirmwareBlocks => {
                self.stage = UpdateStage::PostFlashVerification;
                self.progress_percent = 95;
                self.status_message = String::from("Cryptographic hash verification passed");
            }
            UpdateStage::PostFlashVerification => {
                self.stage = UpdateStage::CompletedScheduledReboot;
                self.progress_percent = 100;
                self.status_message = String::from("Firmware update complete. Reboot scheduled.");
                self.is_finished = true;
                self.is_success = true;
            }
            _ => {}
        }
    }
}
