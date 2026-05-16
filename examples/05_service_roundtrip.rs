use corebluetooth::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let uuid = BluetoothUuid::from_string("180D")?;
    let service = MutableService::new(&uuid, true)?;
    let service_view = service.as_service();
    println!("service_uuid = {}", service_view.uuid());
    println!("is_primary = {}", service_view.is_primary());
    Ok(())
}
