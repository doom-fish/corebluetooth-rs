mod common;

use corebluetooth::prelude::*;

#[test]
fn central_manager_smoke() -> Result<(), Box<dyn std::error::Error>> {
    let _ = CentralManager::current_authorization();
    if !common::live_tests_enabled() {
        return Ok(());
    }
    let manager = CentralManager::new()?;
    let _ = manager.state();
    let _ = manager.authorization();
    Ok(())
}
