use corebluetooth::prelude::*;

#[test]
fn att_error_and_domains() {
    assert_eq!(AttError::from_raw(0x11), AttError::InsufficientResources);
    assert!(!AttRequest::error_domain().is_empty());
    assert!(!AttRequest::att_error_domain().is_empty());
}
