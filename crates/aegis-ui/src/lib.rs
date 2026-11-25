//! Aegis UI: Interactive hardware firmware state machine and event handling.

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::{string::String, vec::Vec};

use aegis_core::{HardwareInventory, UpdatePipeline};
use aegis_render::{AegisRenderer, SoftwareCanvas};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UiAction {
    SelectDevice(usize),
    UpdateChecksums,
    InstallUpdate,
    DismissDialog,
    None,
}

pub struct AegisApp {
    pub inventory: HardwareInventory,
    pub selected_index: usize,
    pub pipeline: Option<UpdatePipeline>,
    pub is_checksum_updating: bool,
    pub canvas: SoftwareCanvas,
}

impl Default for AegisApp {
    fn default() -> Self {
        Self::new(800, 600)
    }
}

impl AegisApp {
    pub fn new(width: i32, height: i32) -> Self {
        let mut app = Self {
            inventory: HardwareInventory::new(),
            selected_index: 0,
            pipeline: None,
            is_checksum_updating: false,
            canvas: SoftwareCanvas::new(width, height),
        };
        app.render();
        app
    }

    pub fn select_device(&mut self, idx: usize) {
        if idx < self.inventory.devices.len() {
            self.selected_index = idx;
            self.render();
        }
    }

    pub fn trigger_checksum_update(&mut self) -> bool {
        if let Some(dev) = self.inventory.devices.get(self.selected_index) {
            let id = dev.id.clone();
            let res = self.inventory.update_device_checksums(&id);
            self.is_checksum_updating = false;
            self.render();
            res
        } else {
            false
        }
    }

    pub fn trigger_install_update(&mut self) -> bool {
        if let Some(dev) = self.inventory.devices.get(self.selected_index) {
            if dev.has_update() {
                let mut pipe = UpdatePipeline::new();
                pipe.advance_step(); // moves to PreflightSafetyCheck
                self.pipeline = Some(pipe);
                self.render();
                return true;
            }
        }
        false
    }

    pub fn step_update_pipeline(&mut self) {
        if let Some(ref mut pipe) = self.pipeline {
            if !pipe.is_finished {
                pipe.advance_step();
                if pipe.is_finished && pipe.is_success {
                    if let Some(dev) = self.inventory.devices.get(self.selected_index) {
                        let id = dev.id.clone();
                        let _ = self.inventory.apply_update(&id);
                    }
                }
            }
        }
        self.render();
    }

    pub fn render(&mut self) {
        AegisRenderer::render_firmware_studio(
            &mut self.canvas,
            &self.inventory,
            self.selected_index,
            self.pipeline.as_ref(),
        );
    }

    pub fn handle_click(&mut self, x: i32, y: i32) -> UiAction {
        let sb_w = 280.min(self.canvas.width / 3);

        // 1. If clicking inside sidebar device list
        if x < sb_w && y >= 56 {
            let item_idx = ((y - 56) / 58) as usize;
            if item_idx < self.inventory.devices.len() {
                self.select_device(item_idx);
                return UiAction::SelectDevice(item_idx);
            }
        }

        // 2. If clicking [ Update Checksums ] button
        let cx = sb_w + 30;
        let btn_y = 20 + 50 + 200;
        if x >= cx + 80 && x <= cx + 220 && y >= btn_y && y <= btn_y + 32 {
            self.trigger_checksum_update();
            return UiAction::UpdateChecksums;
        }

        // 3. If clicking [ Install Update ] button
        if let Some(dev) = self.inventory.devices.get(self.selected_index) {
            if dev.has_update() {
                let card_y = btn_y + 45;
                let card_w = (self.canvas.width - cx - 40).max(100);
                let inst_btn_x = cx + card_w - 130;
                let inst_btn_y = card_y + 20;
                if x >= inst_btn_x && x <= inst_btn_x + 115 && y >= inst_btn_y && y <= inst_btn_y + 34 {
                    self.trigger_install_update();
                    return UiAction::InstallUpdate;
                }
            }
        }

        UiAction::None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ui_initialization_and_selection() {
        let mut app = AegisApp::new(800, 600);
        assert_eq!(app.selected_index, 0);

        app.select_device(2);
        assert_eq!(app.selected_index, 2);
        assert_eq!(app.inventory.devices[2].name, "SSD 860 EVO 250GB");
    }

    #[test]
    fn test_ui_checksum_and_update_triggers() {
        let mut app = AegisApp::new(800, 600);
        assert!(app.trigger_checksum_update());
        assert!(app.inventory.devices[0].checksum_verified);

        assert!(app.trigger_install_update());
        assert!(app.pipeline.is_some());

        // Step pipeline through completion
        while !app.pipeline.as_ref().unwrap().is_finished {
            app.step_update_pipeline();
        }
        assert!(app.pipeline.as_ref().unwrap().is_success);
        assert_eq!(app.inventory.devices[0].current_version.as_deref(), Some("F3"));
    }
}
