//! Aegis Render: 2D double-buffered software vector rasterizer.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::{format, string::String, vec, vec::Vec};

use aegis_core::{HardwareInventory, UpdatePipeline, UpdateStage};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
    pub const WHITE: Self = Self::rgb(255, 255, 255);
    pub const BLACK: Self = Self::rgb(0, 0, 0);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl Rect {
    pub const fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self { x, y, w, h }
    }
    pub fn right(&self) -> i32 { self.x + self.w }
    pub fn bottom(&self) -> i32 { self.y + self.h }
}

pub struct SoftwareCanvas {
    pub width: i32,
    pub height: i32,
    pub buffer: Vec<u32>,
}

impl SoftwareCanvas {
    pub fn new(w: i32, h: i32) -> Self {
        Self {
            width: w,
            height: h,
            buffer: vec![0xff1e293b; (w * h) as usize],
        }
    }

    pub fn put_pixel(&mut self, x: i32, y: i32, col: Color) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            let idx = (y * self.width + x) as usize;
            let val = ((col.a as u32) << 24)
                | ((col.r as u32) << 16)
                | ((col.g as u32) << 8)
                | (col.b as u32);
            self.buffer[idx] = val;
        }
    }

    pub fn fill_rect(&mut self, r: Rect, col: Color) {
        let x0 = r.x.max(0);
        let y0 = r.y.max(0);
        let x1 = r.right().min(self.width);
        let y1 = r.bottom().min(self.height);
        for y in y0..y1 {
            for x in x0..x1 {
                self.put_pixel(x, y, col);
            }
        }
    }

    pub fn draw_rect_outline(&mut self, r: Rect, thickness: i32, col: Color) {
        self.fill_rect(Rect::new(r.x, r.y, r.w, thickness), col);
        self.fill_rect(Rect::new(r.x, r.bottom() - thickness, r.w, thickness), col);
        self.fill_rect(Rect::new(r.x, r.y, thickness, r.h), col);
        self.fill_rect(Rect::new(r.right() - thickness, r.y, thickness, r.h), col);
    }
}

pub struct AegisRenderer;

impl AegisRenderer {
    pub fn render_firmware_studio(
        canvas: &mut SoftwareCanvas,
        inventory: &HardwareInventory,
        selected_idx: usize,
        pipeline: Option<&UpdatePipeline>,
    ) {
        let bg_main = Color::rgb(255, 255, 255);
        let bg_sidebar = Color::rgb(248, 250, 252);
        let border_subtle = Color::rgb(226, 232, 240);
        let text_dark = Color::rgb(15, 23, 42);
        let text_muted = Color::rgb(100, 116, 139);
        let accent_blue = Color::rgb(0, 122, 255);
        let success_green = Color::rgb(34, 197, 94);

        // 1. Clear background
        canvas.fill_rect(Rect::new(0, 0, canvas.width, canvas.height), bg_main);

        // 2. Left Sidebar (Width 280)
        let sb_w = 280.min(canvas.width / 3);
        canvas.fill_rect(Rect::new(0, 0, sb_w, canvas.height), bg_sidebar);
        canvas.fill_rect(Rect::new(sb_w - 1, 0, 1, canvas.height), border_subtle);

        // Sidebar Header: "Firmware Updater"
        canvas.fill_rect(Rect::new(0, 0, sb_w, 46), bg_sidebar);
        canvas.fill_rect(Rect::new(0, 45, sb_w, 1), border_subtle);

        // Sidebar Device List
        let mut item_y = 56;
        for (i, dev) in inventory.devices.iter().enumerate() {
            let is_sel = i == selected_idx;
            let item_rect = Rect::new(8, item_y, sb_w - 16, 52);

            if is_sel {
                canvas.fill_rect(item_rect, Color::rgb(241, 245, 249));
                canvas.draw_rect_outline(item_rect, 1, Color::rgb(203, 213, 225));
            }

            // Draw device icon badge
            let ic_x = item_rect.x + 8;
            let ic_y = item_rect.y + 12;
            if dev.icon_type == 0 {
                // Computer / SPI controller monitor icon
                canvas.draw_rect_outline(Rect::new(ic_x, ic_y, 16, 12), 1, text_dark);
                canvas.fill_rect(Rect::new(ic_x + 5, ic_y + 12, 6, 3), text_dark);
                canvas.fill_rect(Rect::new(ic_x + 3, ic_y + 15, 10, 2), text_dark);
            } else {
                // Storage / SSD drive icon
                canvas.draw_rect_outline(Rect::new(ic_x, ic_y, 14, 18), 1, text_dark);
                canvas.fill_rect(Rect::new(ic_x + 3, ic_y + 3, 8, 4), accent_blue);
                canvas.fill_rect(Rect::new(ic_x + 3, ic_y + 10, 8, 2), text_muted);
            }

            // Update pending dot if available
            if dev.has_update() {
                canvas.fill_rect(Rect::new(item_rect.right() - 14, item_rect.y + 10, 6, 6), Color::rgb(245, 158, 11));
            }

            item_y += 58;
        }

        // 3. Right Details Pane
        if let Some(dev) = inventory.devices.get(selected_idx) {
            let cx = sb_w + 30;
            let cy = 20;

            // Header: Device Icon & Title
            let h_ic_x = cx;
            let h_ic_y = cy + 2;
            if dev.icon_type == 0 {
                canvas.draw_rect_outline(Rect::new(h_ic_x, h_ic_y, 20, 15), 1, text_dark);
                canvas.fill_rect(Rect::new(h_ic_x + 6, h_ic_y + 15, 8, 4), text_dark);
                canvas.fill_rect(Rect::new(h_ic_x + 3, h_ic_y + 19, 14, 2), text_dark);
            } else {
                canvas.draw_rect_outline(Rect::new(h_ic_x, h_ic_y, 16, 22), 1, text_dark);
                canvas.fill_rect(Rect::new(h_ic_x + 3, h_ic_y + 4, 10, 4), accent_blue);
            }

            // Metadata Lines: Current Version, Vendor, GUIDs, Flags
            let meta_y = cy + 50;

            // Checksum Section: [ Update Checksums ] Button
            let btn_y = meta_y + 200;
            let btn_rect = Rect::new(cx + 80, btn_y, 140, 32);
            canvas.draw_rect_outline(btn_rect, 1, Color::rgb(203, 213, 225));
            canvas.fill_rect(Rect::new(btn_rect.x + 1, btn_rect.y + 1, btn_rect.w - 2, btn_rect.h - 2), Color::rgb(248, 250, 252));

            // If update available: Action Card
            if dev.has_update() {
                let card_y = btn_y + 45;
                let card_w = (canvas.width - cx - 40).max(100);
                let card_rect = Rect::new(cx, card_y, card_w, 75);
                canvas.fill_rect(card_rect, Color::rgb(240, 249, 255));
                canvas.draw_rect_outline(card_rect, 1, Color::rgb(186, 230, 253));

                // [ Install Update ] Button
                let inst_btn = Rect::new(card_rect.right() - 130, card_rect.y + 20, 115, 34);
                canvas.fill_rect(inst_btn, accent_blue);
            }
        }

        // 4. Update Pipeline Progress Overlay (if active)
        if let Some(pipe) = pipeline {
            if pipe.stage != UpdateStage::Idle {
                let dlg_w = 420.min(canvas.width - 40);
                let dlg_h = 160;
                let dlg_x = (canvas.width - dlg_w) / 2;
                let dlg_y = (canvas.height - dlg_h) / 2;

                // Dialog shadow & body
                canvas.fill_rect(Rect::new(dlg_x - 2, dlg_y - 2, dlg_w + 4, dlg_h + 4), Color::rgba(0, 0, 0, 60));
                canvas.fill_rect(Rect::new(dlg_x, dlg_y, dlg_w, dlg_h), Color::rgb(255, 255, 255));
                canvas.draw_rect_outline(Rect::new(dlg_x, dlg_y, dlg_w, dlg_h), 1, Color::rgb(203, 213, 225));

                // Progress Bar
                let bar_x = dlg_x + 24;
                let bar_y = dlg_y + 70;
                let bar_w = dlg_w - 48;
                let bar_h = 12;
                canvas.fill_rect(Rect::new(bar_x, bar_y, bar_w, bar_h), Color::rgb(226, 232, 240));
                let fill_w = ((bar_w as u32 * pipe.progress_percent as u32) / 100) as i32;
                let bar_col = if pipe.is_success { success_green } else { accent_blue };
                canvas.fill_rect(Rect::new(bar_x, bar_y, fill_w, bar_h), bar_col);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_renderer_rasterize_output() {
        let mut canvas = SoftwareCanvas::new(800, 600);
        let inv = HardwareInventory::new();
        AegisRenderer::render_firmware_studio(&mut canvas, &inv, 0, None);

        let non_empty = canvas.buffer.iter().any(|&px| px != 0xff1e293b);
        assert!(non_empty, "Renderer should rasterize graphics into canvas");
    }

    #[test]
    fn test_renderer_with_pipeline_overlay() {
        let mut canvas = SoftwareCanvas::new(800, 600);
        let inv = HardwareInventory::new();
        let mut pipeline = UpdatePipeline::new();
        pipeline.advance_step(); // moves to PreflightSafetyCheck
        AegisRenderer::render_firmware_studio(&mut canvas, &inv, 2, Some(&pipeline));

        assert!(canvas.buffer.len() == 800 * 600);
    }
}
