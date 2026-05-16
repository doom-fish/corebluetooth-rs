use corebluetooth::prelude::*;

#[test]
fn peripheral_state_mapping_and_callbacks_builder() {
    let _callbacks = PeripheralCallbacks::new()
        .on_name_update(|| {})
        .on_ready_to_send(|| {})
        .on_rssi(|_, _| {});

    assert_eq!(PeripheralState::from_raw(0), PeripheralState::Disconnected);
    assert_eq!(PeripheralState::from_raw(2), PeripheralState::Connected);
}
