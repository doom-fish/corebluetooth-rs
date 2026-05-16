use corebluetooth::prelude::*;

#[test]
fn peripheral_manager_smoke() -> Result<(), Box<dyn std::error::Error>> {
    let manager = PeripheralManager::new()?;
    let _ = manager.state();
    let _ = manager.authorization();
    assert!(!manager.is_advertising());
    Ok(())
}
