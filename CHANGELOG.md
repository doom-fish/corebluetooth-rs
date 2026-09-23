# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.0] - Unreleased

### Security

- The async streams handed the Swift bridge a raw pointer to their sender
  with no retain and unsubscribed without waiting for the CoreBluetooth queue,
  so an event in flight could push into freed memory. Each stream sink now
  holds a reference to its context until it is deallocated, and dropping a
  stream waits for an event that is being delivered.
- `CentralManagerEventStream`, `PeripheralEventStream` and
  `PeripheralManagerEventStream` kept no reference to the manager or
  peripheral, so dropping it first made unsubscribing read freed memory. A
  stream now keeps its manager or peripheral alive.
- Unsubscribing re-installed the delegate captured at subscribe time, which
  could be a stream that had already been dropped; the next event then went
  to freed memory. Delegates and streams are now fanned out from one bridge
  delegate per object, and nothing is re-installed.
- `CBPeripheral`, `CBCentralManager` and `CBPeripheralManager` delegates held
  an unretained Rust context that could be freed while a callback was running
  (fixed after 0.3.6, first released here).

### Fixed

- Dropping a `CentralManager` or `PeripheralManager` stops its delegate even
  while async streams keep the manager alive.
- A `Peripheral` clears only the delegate that it installed, so dropping one
  clone no longer removes a delegate that another clone set.
- Async streams receive disconnects that macOS 14 and later report through the
  timestamped `didDisconnectPeripheral` delegate method.
- Retained handles in events that a stream ignores or receives after it was
  dropped are released instead of leaking.
- `Peripheral::write_value_for_descriptor` refuses the Client Characteristic
  Configuration descriptor (0x2902) with `InvalidArgument` and points to
  `set_notify_value`.
- Clippy 1.98's `borrow_as_ptr` warnings (`&raw mut` at the bridge calls).

### Changed

- **Breaking:** the raw `ffi` stream functions take context retain/release
  callbacks and return a sink handle; `cb_peripheral_clear_delegate` takes the
  delegate's context; `cb_characteristic_value_json` and
  `cb_att_request_value_json` are replaced by `cb_characteristic_copy_value`
  and `cb_att_request_copy_value`.
- `Characteristic::value` and `AttRequest::value` receive raw bytes from the
  bridge instead of a JSON number array.
- `rust-version` is 1.82 (was 1.76). Requires `apple-cf >=0.11, <0.12` and
  `doom-fish-utils >=0.4.1, <0.5`.

### Added

- `InputStreamHandle::read` and `OutputStreamHandle::write` for L2CAP channel
  streams.
- `ffi::cb_manager_detach_delegate` and
  `ffi::cb_peripheral_manager_detach_delegate`.

## [0.3.6] - 2026-05-20

- Clippy hygiene sweep: cleared all `-D warnings` lints across the crate. No public API change.

## [0.3.5] - 2026-05-20

- Widen `doom-fish-utils` dependency bound to `<0.4` so the 0.3.x SPSC-ring release resolves cleanly. No source changes.

## [0.3.4] - 2026-05-18

### Changed

- Added rustdoc coverage across the safe public `src/` surface so public wrappers, options, delegates, callback adapters, and async event enums now document their CoreBluetooth framework counterparts.

## [0.3.3] - 2026-05-18

- Widen apple-cf version bound to `<0.10` so 0.9.x resolves.

## [0.3.2] - 2026-05-18

- Widen apple-cf version bound to `<0.9` so the 0.8.0 nested-CGRect dep resolves. No source changes.

## [0.3.1] - 2026-05-17

### Fixed

- Added `doom_fish_utils::panic_safe::catch_user_panic` to all three async `extern "C"` event callbacks (`central_manager_event_cb`, `peripheral_event_cb`, `peripheral_manager_event_cb`). Panics inside serde deserialisation or event construction would previously unwind across the FFI boundary — undefined behaviour. The non-async trampolines already used `catch_unwind`; the async callbacks were missing it.
- Added `SAFETY:` comments to every `unsafe` block in `src/async_api.rs` (subscribe calls, Drop unsubscribe calls, Box::from_raw calls, and ctx/payload pointer dereferences in the callbacks).
- Widened `doom-fish-utils` version requirement from `"0.1"` to `">=0.1, <0.3"` to allow the next minor release without a forced upgrade.

## [0.3.0] - 2026-05-17

### Added

- `async` cargo feature enabling executor-agnostic `BoundedAsyncStream`-based event streams.
- `async_api::CentralManagerEventStream` — streams `CBCentralManagerDelegate` events: state changes, peripheral discovered/connected/failed/disconnected.
- `async_api::PeripheralEventStream` — streams all `CBPeripheralDelegate` events: service/characteristic/descriptor discovery, value updates, write confirmations, RSSI reads, and L2CAP channels.
- `async_api::PeripheralManagerEventStream` — streams `CBPeripheralManagerDelegate` events: state changes, advertising, service add, ATT requests, and L2CAP publish/open events.
- Example `examples/14_async_central.rs` demonstrating async state-change streaming.
- Integration tests in `tests/async_stream_tests.rs`.

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
