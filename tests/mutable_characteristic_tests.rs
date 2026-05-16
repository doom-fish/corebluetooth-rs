use corebluetooth::prelude::*;

#[test]
fn mutable_characteristic_builds_descriptors() -> Result<(), Box<dyn std::error::Error>> {
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

    assert_eq!(characteristic.permissions().bits(), AttributePermissions::WRITEABLE.bits());
    assert_eq!(characteristic.value()?, Some(vec![0x10, 0x20]));
    assert_eq!(characteristic.descriptors().len(), 1);
    Ok(())
}
