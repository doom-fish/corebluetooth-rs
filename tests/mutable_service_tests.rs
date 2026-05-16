use corebluetooth::prelude::*;

#[test]
fn mutable_service_builds_characteristics() -> Result<(), Box<dyn std::error::Error>> {
    let service_uuid = BluetoothUuid::from_string("180D")?;
    let characteristic_uuid = BluetoothUuid::from_string("2A37")?;
    let included_uuid = BluetoothUuid::from_string("180F")?;

    let included = MutableService::new(&included_uuid, false)?;
    let characteristic = MutableCharacteristic::new(
        &characteristic_uuid,
        CharacteristicProperties::READ,
        Some(&[0x01]),
        AttributePermissions::READABLE,
    )?;
    let mut service = MutableService::new(&service_uuid, true)?;
    service.set_included_services(&[&included])?;
    service.set_characteristics(&[&characteristic])?;

    assert_eq!(service.included_services().len(), 1);
    assert_eq!(service.characteristics().len(), 1);
    Ok(())
}
