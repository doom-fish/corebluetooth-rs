use corebluetooth::prelude::*;

#[test]
fn service_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
    let uuid = BluetoothUuid::from_string("180D")?;
    let service = MutableService::new(&uuid, true)?;
    let service_view = service.as_service();
    assert_eq!(service_view.uuid(), "180D");
    assert!(service_view.is_primary());
    Ok(())
}
