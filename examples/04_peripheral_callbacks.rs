use corebluetooth::prelude::*;

fn main() {
    let _callbacks = PeripheralCallbacks::new()
        .on_name_update(|| println!("name updated"))
        .on_ready_to_send(|| println!("ready to send"))
        .on_rssi(|rssi, _| println!("rssi={rssi}"));

    println!("peripheral_state = {:?}", PeripheralState::from_raw(2));
    println!("write_type = {:?}", CharacteristicWriteType::WithResponse);
}
