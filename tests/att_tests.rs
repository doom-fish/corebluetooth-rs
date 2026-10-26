use corebluetooth::prelude::*;

#[test]
fn att_error_and_domains() {
    assert_eq!(AttError::from_raw(0x11), AttError::InsufficientResources);
    assert_ne!(AttRequest::error_domain(), "");
    assert_ne!(AttRequest::att_error_domain(), "");
}
