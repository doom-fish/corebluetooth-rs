use corebluetooth::prelude::*;
use serde_json::json;

#[test]
fn descriptor_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
    let uuid = BluetoothUuid::characteristic_user_description();
    let descriptor = MutableDescriptor::new(&uuid, DescriptorValue::string("Heart Rate"))?;
    let descriptor_view = descriptor.as_descriptor();
    assert_eq!(descriptor_view.uuid(), uuid.uuid_string());
    assert_eq!(descriptor_view.value()?, Some(json!("Heart Rate")));
    Ok(())
}
