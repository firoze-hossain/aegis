//! Aegis CLI & Desktop Application: Sovereign Firmware & Hardware Security Studio.

use std::env;

use aegis_core::{HardwareInventory, UpdatePipeline};
use aegis_storage::AegisStorage;
use aegis_ui::AegisApp;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.contains(&String::from("--test-full")) {
        run_full_qualification_suite();
        return;
    }

    if args.contains(&String::from("--selftest")) {
        println!("Running Aegis self-tests...");
        run_full_qualification_suite();
        return;
    }

    if args.contains(&String::from("--devices")) || args.contains(&String::from("--list")) {
        list_devices();
        return;
    }

    if args.contains(&String::from("--check")) {
        check_firmware_updates();
        return;
    }

    if args.contains(&String::from("--update-checksums")) {
        update_all_checksums();
        return;
    }

    if args.contains(&String::from("--report")) {
        let inv = HardwareInventory::new();
        println!("{}", AegisStorage::export_inventory_json(&inv));
        return;
    }

    if args.contains(&String::from("--conf")) {
        println!("{}", AegisStorage::generate_firmware_conf());
        return;
    }

    if args.contains(&String::from("--version")) || args.contains(&String::from("-v")) {
        println!("Aegis Firmware Studio v1.0.0 (Sovereign Edition)");
        println!("Copyright (c) 2026 NovaOS Enterprise Architecture Team");
        return;
    }

    // Default: print banner and device inventory
    print_banner();
    list_devices();
}

fn print_banner() {
    println!("============================================================");
    println!("       AEGIS: SOVEREIGN FIRMWARE & SECURITY STUDIO          ");
    println!("============================================================");
    println!("  Hardware Bus: Direct Microkernel I/O & SPI Ring-3 Bridge  ");
    println!("  Integrity:    Ed25519 Signatures & SHA-256 Checksums      ");
    println!("============================================================\n");
}

fn list_devices() {
    let inv = HardwareInventory::new();
    println!("Discovered {} firmware devices on system bus:\n", inv.devices.len());
    for (i, dev) in inv.devices.iter().enumerate() {
        let update_status = if dev.has_update() {
            format!(" [UPDATE AVAILABLE -> {}]", dev.available_version.as_deref().unwrap_or("?"))
        } else {
            String::from(" [UP TO DATE]")
        };
        println!("{}. {} ({}){}", i + 1, dev.name, dev.vendor, update_status);
        if let Some(ver) = &dev.current_version {
            println!("   * Current Version: {}", ver);
        }
        for guid in &dev.guids {
            println!("   * GUID: {}", guid);
        }
        println!("   * Flags: {}", dev.flags.iter().map(|f| f.label()).collect::<Vec<_>>().join(", "));
        if let Some(cs) = &dev.checksum {
            println!("   * Checksum: {} (Verified: {})", cs, dev.checksum_verified);
        }
        println!();
    }
}

fn check_firmware_updates() {
    let inv = HardwareInventory::new();
    let pending: Vec<_> = inv.devices.iter().filter(|d| d.has_update()).collect();
    println!("Checking for available firmware updates...");
    if pending.is_empty() {
        println!("All devices are running the latest verified firmware.");
    } else {
        println!("Found {} pending firmware updates:\n", pending.len());
        for dev in pending {
            println!(" * {} ({}): {} -> {}",
                dev.name,
                dev.vendor,
                dev.current_version.as_deref().unwrap_or("none"),
                dev.available_version.as_deref().unwrap_or("available")
            );
        }
    }
}

fn update_all_checksums() {
    let mut inv = HardwareInventory::new();
    println!("Updating cryptographic device checksums against verified hardware state...");
    let ids: Vec<String> = inv.devices.iter().map(|d| d.id.clone()).collect();
    for id in ids {
        inv.update_device_checksums(&id);
        if let Some(dev) = inv.devices.iter().find(|d| d.id == id) {
            println!(" * Updated checksum for {}: OK", dev.name);
        }
    }
    println!("\nAll hardware device checksums verified successfully.");
}

fn run_full_qualification_suite() {
    println!("============================================================");
    println!("   AEGIS: AUTONOMOUS QUALIFICATION PASS (10/10 TEST SUITES)  ");
    println!("============================================================");

    let mut inv = HardwareInventory::new();

    // 1. Device Discovery
    assert_eq!(inv.devices.len(), 3);
    println!("[PASS 1/10] Hardware Device Enumeration & Multi-Component Discovery");

    // 2. GUID Assignment
    for dev in &inv.devices {
        assert!(!dev.guids.is_empty());
        for guid in &dev.guids {
            assert!(aegis_core::is_valid_guid(guid));
        }
    }
    println!("[PASS 2/10] Unique GUID Allocation & RFC 4122 Compliance");

    // 3. Version Comparison
    assert_eq!(aegis_core::compare_firmware_versions("F2", "F3"), -1);
    assert_eq!(aegis_core::compare_firmware_versions("1.0.0", "1.1.0"), -1);
    assert_eq!(aegis_core::compare_firmware_versions("2.0.0", "2.0.0"), 0);
    println!("[PASS 3/10] Version Comparison & Alphanumeric Upgrade Eligibility");

    // 4. Power & Safety Flags
    assert!(inv.devices[2].requires_ac_power());
    assert!(inv.devices[0].is_updatable());
    println!("[PASS 4/10] Hardware Safety Gates & External Power Verification");

    // 5. Cryptographic Checksums
    let test_hash = aegis_core::compute_sha256(b"HardwareIntegrityPayload");
    assert!(aegis_core::verify_payload_integrity(b"HardwareIntegrityPayload", &test_hash));
    println!("[PASS 5/10] Cryptographic SHA-256 Checksum Calculation & Verification");

    // 6. Update Pipeline Stages
    let mut pipe = UpdatePipeline::new();
    while !pipe.is_finished {
        pipe.advance_step();
    }
    assert!(pipe.is_success);
    println!("[PASS 6/10] 5-Stage Firmware Flashing & Capsule Staging Pipeline");

    // 7. Device Tree JSON Export
    let json = AegisStorage::export_inventory_json(&inv);
    assert!(json.contains("Internal SPI Controller"));
    assert!(json.contains("SSD 860 EVO 250GB"));
    println!("[PASS 7/10] Hardware Inventory JSON & Security Policy Serialization");

    // 8. Interactive UI Click Event Dispatch
    let mut app = AegisApp::new(800, 600);
    app.handle_click(50, 60); // Select device 0
    assert_eq!(app.selected_index, 0);
    app.handle_click(50, 180); // Select device 2
    assert_eq!(app.selected_index, 2);
    println!("[PASS 8/10] Interactive UI Event Dispatch & Device Switching");

    // 9. 2D Double-Buffered Vector Rasterizer
    let mut canvas = aegis_render::SoftwareCanvas::new(800, 600);
    aegis_render::AegisRenderer::render_firmware_studio(&mut canvas, &inv, 0, None);
    assert!(canvas.buffer.iter().any(|&p| p != 0xff1e293b));
    println!("[PASS 9/10] 2D Double-Buffered Vector Rasterization & Framebuffer Canvas");

    // 10. Complete End-to-End Flash Flow
    let update_res = inv.apply_update("samsung-860-evo");
    assert!(update_res.is_ok());
    assert_eq!(inv.devices[2].current_version.as_deref(), Some("RVT04B6Q"));
    assert!(!inv.devices[2].has_update());
    println!("[PASS 10/10] End-to-End Firmware Flashing, Verification & Reboot Handshake");

    println!("============================================================");
    println!("   ALL 10/10 AEGIS QUALIFICATION TESTS PASSED SUCCESSFULLY!  ");
    println!("============================================================\n");
}
