use corebluetooth::prelude::*;

#[test]
fn l2cap_metadata_types_are_accessible() {
    assert_eq!(StreamStatus::from_raw(2), StreamStatus::Open);
    assert_eq!(StreamStatus::from_raw(7), StreamStatus::Error);
}
