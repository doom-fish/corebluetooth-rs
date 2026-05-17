use corebluetooth::prelude::*;

#[test]
fn uuid_roundtrip_and_constants() -> Result<(), Box<dyn std::error::Error>> {
    let uuid = BluetoothUuid::from_string("180D")?;
    let bytes_uuid = BluetoothUuid::from_bytes(&[0x18, 0x0D]);
    assert_eq!(uuid.uuid_string(), bytes_uuid.uuid_string());
    assert!(!BluetoothUuid::characteristic_user_description()
        .uuid_string()
        .is_empty());
    Ok(())
}
