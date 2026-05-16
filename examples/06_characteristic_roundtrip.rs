use corebluetooth::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let uuid = BluetoothUuid::from_string("2A37")?;
    let characteristic = MutableCharacteristic::new(
        &uuid,
        CharacteristicProperties::READ,
        Some(&[1, 2, 3]),
        AttributePermissions::READABLE,
    )?;
    let characteristic_view = characteristic.as_characteristic();
    println!("characteristic_uuid = {}", characteristic_view.uuid());
    println!("properties = {}", characteristic_view.properties().bits());
    println!("value = {:?}", characteristic_view.value()?);
    Ok(())
}
