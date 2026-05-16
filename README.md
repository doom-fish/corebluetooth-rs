# corebluetooth

Safe, idiomatic Rust bindings for Apple's [CoreBluetooth](https://developer.apple.com/documentation/corebluetooth) framework on macOS.

Version `0.2.0` extends the original central/client surface with peripheral/server APIs, mutable GATT builders, typed advertisement and UUID helpers, ATT request wrappers, and L2CAP channel metadata.

## Features

- **Central manager APIs** — `CentralManager`, `CentralManagerOptions`, `ScanOptions`, `ConnectOptions`, restore-state callbacks, and connection lifecycle delegates.
- **Peripheral manager APIs** — `PeripheralManager`, `PeripheralManagerOptions`, advertising control, service publication, ATT responses, subscriber updates, and L2CAP publication.
- **Remote peripheral APIs** — `Peripheral`, `Service`, `Characteristic`, and `Descriptor` wrappers covering discovery, reads, writes, notify state, descriptor access, and L2CAP opening.
- **Local GATT builders** — `MutableService`, `MutableCharacteristic`, and `MutableDescriptor` for building publishable services entirely from Rust.
- **Typed helpers** — `BluetoothUuid`, `AdvertisementData`, `AttRequest`, `AttError`, `Central`, `L2capChannel`, `Peer`, `InputStreamHandle`, and `OutputStreamHandle`.
- **Headless examples and tests** — 13 examples and 12 integration tests that run successfully on a CLI macOS host without publishing or requiring a GUI window.

## Requirements

- macOS 10.13 or newer
- Xcode with the macOS SDK and Swift toolchain
- For real scanning, connecting, advertising, or GATT server use in GUI apps, the appropriate Bluetooth usage description/background modes in your app bundle

## Installation

```toml
[dependencies]
corebluetooth-rs = "0.2.0"
```

## Quick examples

```bash
cargo run --example 01_smoke
cargo run --example 03_peripheral_manager_state
cargo run --example 12_mutable_service_build
```

Representative examples:

- `01_smoke` — create a central manager and print state/authorization.
- `03_peripheral_manager_state` — create a peripheral manager and inspect server-role state.
- `05_service_roundtrip` / `06_characteristic_roundtrip` / `07_descriptor_roundtrip` — exercise immutable GATT wrappers using local mutable builders.
- `08_att_constants` / `09_l2cap_channel_types` / `10_advertisement_builder` / `11_uuid_roundtrip` — cover helper areas without requiring BLE hardware.
- `12_mutable_service_build` / `13_mutable_characteristic_build` — build publishable local services and characteristics from Rust.

## Testing

```bash
cargo clippy --all-targets -- -D warnings
cargo test
for ex in examples/*.rs; do cargo run --example "$(basename "$ex" .rs)"; done
```

## Coverage notes

See [`COVERAGE.md`](COVERAGE.md) for the framework-by-framework audit, including implemented APIs, intentionally skipped iOS-only members, and deprecated macOS-only symbols left out of the safe surface.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
