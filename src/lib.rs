#![doc = include_str!("../README.md")]
//!
//! ---
//!
//! # API documentation
//!
//! Safe Rust bindings for Apple's
//! [CoreBluetooth](https://developer.apple.com/documentation/corebluetooth)
//! framework.
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(
    clippy::missing_const_for_fn,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::new_without_default
)]

pub mod central;
pub mod error;
pub mod ffi;
pub mod peripheral;
mod private;

pub use central::{
    CentralManager, CentralManagerCallbacks, CentralManagerDelegate, CentralManagerState,
    ManagerAuthorization, ScanOptions,
};
pub use error::{BluetoothErrorInfo, CoreBluetoothError};
pub use peripheral::{
    Characteristic, CharacteristicProperties, CharacteristicWriteType, Descriptor, Peripheral,
    PeripheralCallbacks, PeripheralDelegate, PeripheralState, Service,
};

/// Common imports.
pub mod prelude {
    pub use crate::central::{
        CentralManager, CentralManagerCallbacks, CentralManagerDelegate, CentralManagerState,
        ManagerAuthorization, ScanOptions,
    };
    pub use crate::error::{BluetoothErrorInfo, CoreBluetoothError};
    pub use crate::peripheral::{
        Characteristic, CharacteristicProperties, CharacteristicWriteType, Descriptor, Peripheral,
        PeripheralCallbacks, PeripheralDelegate, PeripheralState, Service,
    };
}
