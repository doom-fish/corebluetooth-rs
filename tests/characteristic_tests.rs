use corebluetooth::prelude::*;

#[test]
fn characteristic_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
    let uuid = BluetoothUuid::from_string("2A37")?;
    let characteristic = MutableCharacteristic::new(
        &uuid,
        CharacteristicProperties::READ,
        Some(&[1, 2, 3]),
        AttributePermissions::READABLE,
    )?;
    let characteristic_view = characteristic.as_characteristic();
    assert_eq!(characteristic_view.uuid(), "2A37");
    assert!(characteristic_view
        .properties()
        .contains(CharacteristicProperties::READ));
    assert_eq!(characteristic_view.value()?, Some(vec![1, 2, 3]));
    Ok(())
}

#[test]
fn characteristic_values_keep_empty_and_missing_values_apart(
) -> Result<(), Box<dyn std::error::Error>> {
    let uuid = BluetoothUuid::from_string("2A38")?;
    let empty = MutableCharacteristic::new(
        &uuid,
        CharacteristicProperties::READ,
        Some(&[]),
        AttributePermissions::READABLE,
    )?;
    assert_eq!(empty.as_characteristic().value()?, Some(Vec::new()));

    let missing = MutableCharacteristic::new(
        &uuid,
        CharacteristicProperties::NOTIFY,
        None,
        AttributePermissions::READABLE,
    )?;
    assert_eq!(missing.as_characteristic().value()?, None);

    let bytes: Vec<u8> = (0..=255).collect();
    let full = MutableCharacteristic::new(
        &uuid,
        CharacteristicProperties::READ,
        Some(&bytes),
        AttributePermissions::READABLE,
    )?;
    assert_eq!(full.as_characteristic().value()?, Some(bytes));
    Ok(())
}
