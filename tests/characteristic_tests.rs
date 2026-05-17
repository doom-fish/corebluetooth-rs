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
