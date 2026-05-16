use corebluetooth::prelude::*;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = PeripheralManager::with_options(
        PeripheralManagerOptions::new().with_queue_label("corebluetooth-rs.example.peripheral"),
    )?;
    thread::sleep(Duration::from_millis(200));
    println!("state = {:?}", manager.state());
    println!("authorization = {:?}", manager.authorization());
    println!("is_advertising = {}", manager.is_advertising());
    Ok(())
}
