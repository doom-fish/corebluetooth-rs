use corebluetooth::prelude::*;
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let advertisement = AdvertisementData::from_json_value(json!({
        "kCBAdvDataLocalName": "Demo Peripheral",
        "kCBAdvDataServiceUUIDs": ["180D"],
        "kCBAdvDataIsConnectable": true,
    }))?;
    println!("local_name = {:?}", advertisement.local_name());
    println!("service_uuid_count = {}", advertisement.service_uuids().len());
    println!("is_connectable = {:?}", advertisement.is_connectable());
    Ok(())
}
