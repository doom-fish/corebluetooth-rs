# corebluetooth

Safe, idiomatic Rust bindings for Apple's [CoreBluetooth](https://developer.apple.com/documentation/corebluetooth) framework — create BLE central managers, inspect peripheral/service/characteristic state, and receive delegate-driven events on macOS.

## Features

- **Central manager control** — `CentralManager` wraps state inspection, scanning, connect/cancel flows, and peripheral retrieval APIs.
- **Peripheral access** — `Peripheral`, `Service`, `Characteristic`, and `Descriptor` expose names, UUIDs, connection state, services, characteristic properties, values, and descriptor values.
- **Delegate callbacks** — `CentralManagerDelegate` / `CentralManagerCallbacks` and `PeripheralDelegate` / `PeripheralCallbacks` translate `CoreBluetooth` delegate events into Rust closures.
- **Read / write / notify** — discover services and characteristics, read RSSI, read characteristic values, write characteristic values, toggle notifications, and discover descriptors.
- **Queue-backed managers** — the Swift bridge creates a dedicated dispatch queue by default so CLI programs can observe state changes without an app run loop.

## Requirements

- macOS 10.13 or newer
- Xcode 15+ with the macOS SDK
- For scanning or connecting in GUI apps, `NSBluetoothAlwaysUsageDescription` in your app's `Info.plist`

## Installation

```toml
[dependencies]
corebluetooth-rs = "0.1.0"
```

```rust,no_run
use corebluetooth::prelude::*;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = CentralManager::new()?;
    thread::sleep(Duration::from_secs(2));
    println!("state: {:?}", manager.state());
    println!("authorization: {:?}", manager.authorization());
    Ok(())
}
```

## Smoke example

```bash
cargo run --example 01_smoke
```

The smoke example creates a `CentralManager`, waits two seconds for `centralManagerDidUpdateState`, prints the resulting state and authorization, and exits without scanning or requesting Bluetooth permission.

## Notes

- Discovery, connection, and characteristic updates arrive asynchronously on the dispatch queue used to create the `CentralManager`.
- `CBPeripheralManager` (peripheral/server role) is intentionally deferred to a future release.
- Advertisement data is surfaced as `serde_json::Value` so the bridge can preserve Apple's heterogenous dictionary structure without losing information.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
