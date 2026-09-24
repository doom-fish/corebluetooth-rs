# corebluetooth

Safe, idiomatic Rust bindings for Apple's [CoreBluetooth](https://developer.apple.com/documentation/corebluetooth) framework on macOS.

It covers the central/client and peripheral/server surfaces, an optional `async` event-stream layer, mutable GATT builders, typed advertisement and UUID helpers, ATT request wrappers, and L2CAP channels.

## Features

- **Central manager APIs** — `CentralManager`, `CentralManagerOptions`, `ScanOptions`, `ConnectOptions`, restore-state callbacks, and connection lifecycle delegates.
- **Peripheral manager APIs** — `PeripheralManager`, `PeripheralManagerOptions`, advertising control, service publication, ATT responses, subscriber updates, and L2CAP publication.
- **Remote peripheral APIs** — `Peripheral`, `Service`, `Characteristic`, and `Descriptor` wrappers covering discovery, reads, writes, notify state, descriptor access, and L2CAP opening.
- **Optional async streams** — enable the `async` cargo feature for `async_api::{CentralManagerEventStream, PeripheralEventStream, PeripheralManagerEventStream}` backed by `doom-fish-utils`.
- **Local GATT builders** — `MutableService`, `MutableCharacteristic`, and `MutableDescriptor` for building publishable services entirely from Rust.
- **Typed helpers** — `BluetoothUuid`, `AdvertisementData`, `AttRequest`, `AttError`, `Central`, `L2capChannel`, `Peer`, `InputStreamHandle`, and `OutputStreamHandle`.
- **L2CAP channels** — open or publish a channel, then move bytes with the blocking `InputStreamHandle::read` and `OutputStreamHandle::write` after opening both streams; poll `has_bytes_available` / `has_space_available` to avoid blocking. The streams are not scheduled on a run loop.
- **Headless examples and tests** — 14 examples and 14 integration test files that run on a CLI macOS host without a GUI window. The event tests add and remove a local GATT service and skip themselves when Bluetooth is off or not authorized.

## Requirements

- macOS 10.13 or newer (L2CAP channels need 10.14; the bridge checks at run time)
- Xcode with the macOS SDK and Swift toolchain
- Bluetooth permission: an app bundle must declare `NSBluetoothAlwaysUsageDescription`, and a command-line tool uses the permission of the terminal that launches it. Creating the first `CentralManager` or `PeripheralManager` can show the consent prompt; until access is granted the managers report `Unauthorized`.

## Installation

```toml
[dependencies]
corebluetooth-rs = "0.4"
# or, for async delegate streams:
corebluetooth-rs = { version = "0.4", features = ["async"] }
```

## Delegates and streams

- Each `CBCentralManager`, `CBPeripheralManager` and `CBPeripheral` has one bridge delegate that forwards every event to the Rust delegate and to every async stream, so delegates and streams can be added and dropped in any order.
- A stream keeps its manager or peripheral alive. Dropping it waits for an event that is being delivered, then stops; dropping a manager stops its Rust delegate even while streams keep the manager alive.
- Stream events leave the manager's dispatch queue as plain data and retained object handles. `next` and `try_next` build the `Peripheral`, `Service`, `Characteristic` and other wrappers on the thread that polls the stream; streams and their `next` futures are not `Send`, so that is the thread that owns the manager or peripheral.
- `Peripheral::clear_delegate` and dropping a `Peripheral` remove only the delegate that handle installed.
- `Peripheral::write_value_for_descriptor` refuses the Client Characteristic Configuration descriptor (0x2902); use `Peripheral::set_notify_value` to switch notifications.

## Quick examples

```bash
cargo run --example 01_smoke
cargo run --example 03_peripheral_manager_state
cargo run --example 12_mutable_service_build
cargo run --example 14_async_central --features async
```

Representative examples:

- `01_smoke` — create a central manager and print state/authorization.
- `03_peripheral_manager_state` — create a peripheral manager and inspect server-role state.
- `05_service_roundtrip` / `06_characteristic_roundtrip` / `07_descriptor_roundtrip` — exercise immutable GATT wrappers using local mutable builders.
- `08_att_constants` / `09_l2cap_channel_types` / `10_advertisement_builder` / `11_uuid_roundtrip` — cover helper areas without requiring BLE hardware.
- `12_mutable_service_build` / `13_mutable_characteristic_build` — build publishable local services and characteristics from Rust.
- `14_async_central` — subscribe to the async central-manager stream and print the first state-change event.

## Testing

```bash
cargo clippy --all-features --all-targets -- -D warnings
cargo test --all-features
cargo run --example 14_async_central --features async
```

## Coverage notes

See [`COVERAGE.md`](COVERAGE.md) for the framework-by-framework audit, including implemented APIs, intentionally skipped iOS-only members, and deprecated macOS-only symbols left out of the safe surface.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
