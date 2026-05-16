use corebluetooth::prelude::*;

fn main() {
    println!("stream_status = {:?}", StreamStatus::from_raw(2));
    println!("l2cap_uuid = {}", BluetoothUuid::l2cap_psm_characteristic());
}
