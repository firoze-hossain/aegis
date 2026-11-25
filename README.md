# Aegis - Sovereign Firmware & Hardware Security Studio

Aegis is the sovereign firmware management, hardware component discovery, cryptographic integrity verification, and firmware flashing studio designed for **NovaOS** and cross-platform desktop operating systems (Linux, macOS, Windows).

Synthesizes the functionality of modern firmware management utilities:
- **Device Enumeration**: Internal SPI controllers (UEFI/BIOS, IFD descriptor regions), storage drives (SATA/NVMe SSDs), embedded controllers, and security cryptoprocessors.
- **Hardware GUID Allocation**: RFC 4122 v5 deterministic UUID calculation across hardware device namespaces.
- **Dynamic Checksum Computation**: Live SHA-256 calculation and verification matching on-board hardware state (`[ Update Checksums ]`).
- **5-Stage Flash Pipeline**: Preflight power checks, capsule staging, flash block write verification, post-flash integrity validation, and reboot scheduling.
- **Dual-Buffered 2D Vector Rasterizer**: Zero-dependency software rendering with responsive layout for hardware lists, device specs, and flashing progress overlays.
- **Autonomous Qualification Suite**: Built-in 10/10 automated tests verification.

## Architecture

```
aegis/
├── crates/
│   ├── aegis-core/       # Hardware device models, GUID generator, checksums, and update pipeline
│   ├── aegis-storage/    # Device tree JSON export and security policy configuration
│   ├── aegis-render/     # Pure Rust 2D vector software rasterizer
│   ├── aegis-ui/         # Interactive UI event dispatch and state machine
│   └── aegis-app/        # Cross-platform CLI application and test suites
└── novaos/               # Bare-metal i686 ring-3 ELF32 binary and package manifests
```

## Building & Testing

```bash
cargo test
cargo run -p aegis-app -- --test-full
```
