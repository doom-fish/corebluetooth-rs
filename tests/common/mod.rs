pub fn live_tests_enabled() -> bool {
    let enabled = std::env::var("COREBLUETOOTH_LIVE_TESTS").as_deref() == Ok("1");
    if !enabled {
        eprintln!(
            "skip: set COREBLUETOOTH_LIVE_TESTS=1 to run tests that create Bluetooth managers"
        );
    }
    enabled
}
