//! Async central manager: subscribe to state changes and exit after first event or 3 s.
#[cfg(not(feature = "async"))]
fn main() {
    eprintln!("Rerun with --features async");
}

#[cfg(feature = "async")]
fn main() {
    pollster::block_on(run());
}

#[cfg(feature = "async")]
#[allow(clippy::unused_async)]
async fn run() {
    use corebluetooth::async_api::{CentralManagerEvent, CentralManagerEventStream};
    use corebluetooth::CentralManager;

    let manager = CentralManager::new().expect("CentralManager::new");
    let stream = CentralManagerEventStream::subscribe(&manager, 8);

    // CoreBluetooth fires didUpdateState shortly after manager creation.
    // Timeout after 3 s to stay headless-friendly.
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(3);
    loop {
        std::thread::sleep(std::time::Duration::from_millis(50));
        if let Some(event) = stream.try_next() {
            println!("CentralManagerEvent: {event:?}");
            if matches!(event, CentralManagerEvent::StateChanged { .. }) {
                println!("State-change received — stream works.");
                break;
            }
        }
        if std::time::Instant::now() >= deadline {
            println!("Timeout reached — no event yet (Bluetooth may be off or restricted).");
            break;
        }
    }
}
