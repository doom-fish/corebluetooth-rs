use corebluetooth::prelude::*;

fn main() {
    println!("error_domain = {}", AttRequest::error_domain());
    println!("att_error_domain = {}", AttRequest::att_error_domain());
    println!("att_error = {:?}", AttError::InsufficientResources);
}
