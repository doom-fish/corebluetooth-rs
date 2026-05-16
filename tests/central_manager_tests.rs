use corebluetooth::prelude::*;

#[test]
fn central_manager_smoke() -> Result<(), Box<dyn std::error::Error>> {
    let manager = CentralManager::new()?;
    let _ = manager.state();
    let _ = manager.authorization();
    let _ = CentralManager::current_authorization();
    Ok(())
}
