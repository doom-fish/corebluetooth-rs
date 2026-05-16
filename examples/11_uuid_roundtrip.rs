use corebluetooth::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let uuid = BluetoothUuid::from_string("180D")?;
    let bytes_uuid = BluetoothUuid::from_bytes(&[0x18, 0x0D]);
    println!("uuid = {uuid}");
    println!("uuid_bytes = {:?}", uuid.data()?);
    println!("bytes_uuid = {}", bytes_uuid.uuid_string());
    Ok(())
}
