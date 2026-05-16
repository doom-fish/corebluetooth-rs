# Changelog

## [0.1.0] - 2026-05-16

### Added

- `CentralManager` with state / authorization inspection, scanning, connect / cancel, and peripheral retrieval APIs.
- `Peripheral`, `Service`, `Characteristic`, and `Descriptor` wrappers for the BLE central/client surface Apple exposes through `CoreBluetooth.framework`.
- Delegate-to-Rust callback bridging for `CBCentralManagerDelegate` and `CBPeripheralDelegate`, including service discovery, characteristic discovery, value updates, writes, notification-state changes, and RSSI reads.
- `CharacteristicProperties` and `CharacteristicWriteType` helpers for common read / write / notify flows.
- SwiftPM bridge under `swift-bridge/` that links `CoreBluetooth.framework` and `Foundation.framework` into a static library built from `build.rs`.
- Smoke example `examples/01_smoke.rs` that creates a central manager, waits for state propagation, and exits without triggering a Bluetooth permission prompt.
