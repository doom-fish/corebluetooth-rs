#![cfg(feature = "async")]

use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use corebluetooth::async_api::{
    CentralManagerEventStream, PeripheralManagerEvent, PeripheralManagerEventStream,
};
use corebluetooth::{
    BluetoothUuid, CentralManager, MutableService, PeripheralManager, PeripheralManagerCallbacks,
    PeripheralManagerState,
};

fn powered_on(manager: &PeripheralManager) -> bool {
    let deadline = Instant::now() + Duration::from_secs(5);
    while Instant::now() < deadline {
        if manager.state() == PeripheralManagerState::PoweredOn {
            return true;
        }
        thread::sleep(Duration::from_millis(20));
    }
    eprintln!("skipping: Bluetooth is not powered on or not authorized");
    false
}

fn service(uuid: &str) -> MutableService {
    let uuid = BluetoothUuid::from_string(uuid).expect("service UUID");
    MutableService::new(&uuid, true).expect("service")
}

fn wait_for_added_service(stream: &PeripheralManagerEventStream) -> bool {
    let deadline = Instant::now() + Duration::from_secs(5);
    while Instant::now() < deadline {
        while let Some(event) = stream.try_next() {
            if matches!(event, PeripheralManagerEvent::DidAddService { .. }) {
                return true;
            }
        }
        thread::sleep(Duration::from_millis(10));
    }
    false
}

#[test]
fn streams_outlive_the_manager_they_were_subscribed_to() {
    let central = CentralManager::new().expect("central manager");
    let central_stream = CentralManagerEventStream::subscribe(&central, 8);
    let peripheral = PeripheralManager::new().expect("peripheral manager");
    let peripheral_stream = PeripheralManagerEventStream::subscribe(&peripheral, 8);

    drop(central);
    drop(peripheral);
    thread::sleep(Duration::from_millis(50));

    drop(central_stream);
    drop(peripheral_stream);
}

#[test]
fn dropped_streams_are_never_reattached_as_delegates() -> Result<(), Box<dyn std::error::Error>> {
    let manager = PeripheralManager::new()?;
    if !powered_on(&manager) {
        return Ok(());
    }

    let first = PeripheralManagerEventStream::subscribe(&manager, 16);
    let second = PeripheralManagerEventStream::subscribe(&manager, 16);
    drop(first);
    let third = PeripheralManagerEventStream::subscribe(&manager, 16);
    drop(second);

    manager.add_service(&service("A0C1B2D3-0001-4E5F-8000-00805F9B34FB"))?;
    assert!(wait_for_added_service(&third));
    manager.remove_all_services();
    Ok(())
}

#[test]
fn the_delegate_and_every_stream_receive_each_event() -> Result<(), Box<dyn std::error::Error>> {
    let (added, added_rx) = mpsc::channel();
    let manager = PeripheralManager::with_callbacks(
        PeripheralManagerCallbacks::new().on_add_service(move |_, _| {
            let _ = added.send(());
        }),
    )?;
    if !powered_on(&manager) {
        return Ok(());
    }

    let first = PeripheralManagerEventStream::subscribe(&manager, 16);
    let second = PeripheralManagerEventStream::subscribe(&manager, 16);
    manager.add_service(&service("A0C1B2D3-0002-4E5F-8000-00805F9B34FB"))?;
    assert!(added_rx.recv_timeout(Duration::from_secs(5)).is_ok());
    assert!(wait_for_added_service(&first));
    assert!(wait_for_added_service(&second));

    drop(first);
    manager.add_service(&service("A0C1B2D3-0003-4E5F-8000-00805F9B34FB"))?;
    assert!(added_rx.recv_timeout(Duration::from_secs(5)).is_ok());
    assert!(wait_for_added_service(&second));

    drop(second);
    manager.add_service(&service("A0C1B2D3-0004-4E5F-8000-00805F9B34FB"))?;
    assert!(added_rx.recv_timeout(Duration::from_secs(5)).is_ok());
    manager.remove_all_services();
    Ok(())
}

#[test]
fn dropping_a_stream_while_events_are_in_flight_is_safe() -> Result<(), Box<dyn std::error::Error>>
{
    let manager = PeripheralManager::new()?;
    if !powered_on(&manager) {
        return Ok(());
    }

    for round in 0..20_u32 {
        let stream = PeripheralManagerEventStream::subscribe(&manager, 2);
        for index in 0..4_u32 {
            let uuid = format!("A0C1B2D3-{round:04X}-4E5F-8{index:03X}-00805F9B34FB");
            manager.add_service(&service(&uuid))?;
        }
        thread::sleep(Duration::from_millis(u64::from(round % 3)));
        drop(stream);
        manager.remove_all_services();
    }

    let stream = PeripheralManagerEventStream::subscribe(&manager, 16);
    manager.add_service(&service("A0C1B2D3-FFFF-4E5F-8000-00805F9B34FB"))?;
    assert!(wait_for_added_service(&stream));
    manager.remove_all_services();
    Ok(())
}
