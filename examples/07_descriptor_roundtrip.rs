use corebluetooth::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let uuid = BluetoothUuid::characteristic_user_description();
    let descriptor = MutableDescriptor::new(&uuid, DescriptorValue::string("Heart Rate"))?;
    let descriptor_view = descriptor.as_descriptor();
    println!("descriptor_uuid = {}", descriptor_view.uuid());
    println!("descriptor_value = {:?}", descriptor_view.value()?);
    Ok(())
}
