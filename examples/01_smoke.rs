use corebluetooth::prelude::*;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = CentralManager::new()?;
    thread::sleep(Duration::from_secs(2));
    let state = manager.state();
    println!("state = {state:?}");
    println!("authorization = {:?}", manager.authorization());
    println!("✅ corebluetooth central created (state={state:?})");
    Ok(())
}
