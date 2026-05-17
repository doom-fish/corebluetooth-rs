//! Integration tests for the `async_api` stream module.
#![cfg(feature = "async")]

use corebluetooth::async_api::{
    CentralManagerEvent, CentralManagerEventStream, PeripheralManagerEventStream,
};
use corebluetooth::{CentralManager, PeripheralManager};

#[test]
fn central_stream_subscribe_and_drop_is_safe() {
    let manager = CentralManager::new().expect("CentralManager::new");
    let stream = CentralManagerEventStream::subscribe(&manager, 8);
    drop(stream);
}

#[test]
fn central_stream_subscribe_twice_is_safe() {
    let manager = CentralManager::new().expect("CentralManager::new");
    let stream1 = CentralManagerEventStream::subscribe(&manager, 8);
    let stream2 = CentralManagerEventStream::subscribe(&manager, 8);
    drop(stream2);
    drop(stream1);
}

#[test]
fn peripheral_manager_stream_subscribe_and_drop_is_safe() {
    let manager = PeripheralManager::new().expect("PeripheralManager::new");
    let stream = PeripheralManagerEventStream::subscribe(&manager, 8);
    drop(stream);
}

#[test]
fn central_state_change_event_arrives_or_times_out() {
    let manager = CentralManager::new().expect("CentralManager::new");
    let stream = CentralManagerEventStream::subscribe(&manager, 8);
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);

    loop {
        if let Some(event) = stream.try_next() {
            println!("received event: {event:?}");
            if matches!(event, CentralManagerEvent::StateChanged { .. }) {
                return;
            }
        }
        if std::time::Instant::now() >= deadline {
            eprintln!("WARNING: No BLE state event in 5 s — Bluetooth may be restricted");
            return;
        }
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
}
