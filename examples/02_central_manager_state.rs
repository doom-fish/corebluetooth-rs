use corebluetooth::prelude::*;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = CentralManager::with_options(
        CentralManagerOptions::new().with_queue_label("corebluetooth-rs.example.central"),
    )?;
    thread::sleep(Duration::from_millis(200));
    println!("state = {:?}", manager.state());
    println!("authorization = {:?}", manager.authorization());
    println!(
        "current_authorization = {:?}",
        CentralManager::current_authorization()
    );
    Ok(())
}
