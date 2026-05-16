# Changelog

## [0.2.0] - 2026-05-16

### Added

- `PeripheralManager`, `PeripheralManagerOptions`, `PeripheralManagerCallbacks`, and `PeripheralManagerDelegate` for the CoreBluetooth peripheral/server role.
- `MutableService`, `MutableCharacteristic`, and `MutableDescriptor` for building local GATT databases from Rust.
- `BluetoothUuid`, `AdvertisementData`, `AttRequest`, `AttError`, `Central`, `L2capChannel`, `Peer`, `InputStreamHandle`, and `OutputStreamHandle` helpers.
- Restore-state, advertising, ATT read/write request, descriptor, included-service, ready-to-send, and L2CAP delegate/event coverage across the Swift bridge and safe Rust wrappers.
- 12 new examples (`02_*` through `13_*`) and 12 integration test files covering every logical area.
- `COVERAGE.md` documenting framework coverage, implemented wrappers, and skipped iOS-only/deprecated APIs.

### Changed

- Expanded `CentralManager` with builder-style options, connect options, restore-state callbacks, and detailed disconnect metadata.
- Split the Swift bridge and Rust safe wrappers by logical area (`CentralManager`, `PeripheralManager`, `Peripheral`, `Service`, `Characteristic`, `Descriptor`, `ATT`, `L2CAPChannel`, `Advertisement`, `UUID`, `MutableService`, `MutableCharacteristic`).
- Updated the crate description and README to reflect full central + peripheral CoreBluetooth support.

## [0.1.0] - 2026-05-16

### Added

- `CentralManager` with state / authorization inspection, scanning, connect / cancel, and peripheral retrieval APIs.
- `Peripheral`, `Service`, `Characteristic`, and `Descriptor` wrappers for the BLE central/client surface Apple exposes through `CoreBluetooth.framework`.
- Delegate-to-Rust callback bridging for `CBCentralManagerDelegate` and `CBPeripheralDelegate`, including service discovery, characteristic discovery, value updates, writes, notification-state changes, and RSSI reads.
- `CharacteristicProperties` and `CharacteristicWriteType` helpers for common read / write / notify flows.
- SwiftPM bridge under `swift-bridge/` that links `CoreBluetooth.framework` and `Foundation.framework` into a static library built from `build.rs`.
- Smoke example `examples/01_smoke.rs` that creates a central manager, waits for state propagation, and exits without triggering a Bluetooth permission prompt.
