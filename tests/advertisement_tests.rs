use corebluetooth::prelude::*;
use serde_json::json;

#[test]
fn advertisement_parsing() -> Result<(), Box<dyn std::error::Error>> {
    let advertisement = AdvertisementData::from_json_value(json!({
        "kCBAdvDataLocalName": "Demo Peripheral",
        "kCBAdvDataServiceUUIDs": ["180D"],
        "kCBAdvDataManufacturerData": [1, 2, 3],
        "kCBAdvDataIsConnectable": true,
    }))?;
    assert_eq!(advertisement.local_name(), Some("Demo Peripheral"));
    assert_eq!(advertisement.service_uuids().len(), 1);
    assert_eq!(advertisement.manufacturer_data(), Some(&[1, 2, 3][..]));
    assert_eq!(advertisement.is_connectable(), Some(true));
    Ok(())
}
