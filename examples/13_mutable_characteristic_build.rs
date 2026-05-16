use corebluetooth::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let characteristic_uuid = BluetoothUuid::from_string("2A39")?;
    let descriptor_uuid = BluetoothUuid::characteristic_user_description();
    let descriptor = MutableDescriptor::new(&descriptor_uuid, DescriptorValue::string("Control Point"))?;
    let mut characteristic = MutableCharacteristic::new(
        &characteristic_uuid,
        CharacteristicProperties::WRITE,
        None,
        AttributePermissions::WRITEABLE,
    )?;
    characteristic.set_descriptors(&[&descriptor])?;
    characteristic.set_value(Some(&[0x10, 0x20]));
    println!("characteristic_uuid = {}", characteristic.uuid());
    println!("permissions = {}", characteristic.permissions().bits());
    println!("descriptor_count = {}", characteristic.descriptors().len());
    Ok(())
}
