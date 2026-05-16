use corebluetooth::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let service_uuid = BluetoothUuid::from_string("180D")?;
    let characteristic_uuid = BluetoothUuid::from_string("2A37")?;
    let mut service = MutableService::new(&service_uuid, true)?;
    let characteristic = MutableCharacteristic::new(
        &characteristic_uuid,
        CharacteristicProperties::READ,
        Some(&[0x01]),
        AttributePermissions::READABLE,
    )?;
    service.set_characteristics(&[&characteristic])?;
    println!("service_uuid = {}", service.uuid());
    println!("characteristic_count = {}", service.characteristics().len());
    Ok(())
}
