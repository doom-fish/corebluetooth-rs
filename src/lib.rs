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

pub mod advertisement;
#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub mod async_api;
pub mod att;
pub mod central;
pub mod central_manager;
pub mod characteristic;
pub mod descriptor;
pub mod error;
pub mod ffi;
pub mod l2cap_channel;
pub mod mutable_characteristic;
pub mod mutable_service;
pub mod peripheral;
pub mod peripheral_manager;
mod private;
pub mod service;
pub mod uuid;

pub use advertisement::AdvertisementData;
pub use att::{AttError, AttRequest};
pub use central_manager::{
    CentralManager, CentralManagerCallbacks, CentralManagerDelegate, CentralManagerOptions,
    CentralManagerRestoredState, CentralManagerState, ConnectOptions, ManagerAuthorization,
    ScanOptions,
};
pub use characteristic::{
    AttributePermissions, Characteristic, CharacteristicProperties, CharacteristicWriteType,
};
pub use descriptor::{Descriptor, DescriptorValue, MutableDescriptor};
pub use error::{BluetoothErrorCode, BluetoothErrorInfo, CoreBluetoothError};
pub use l2cap_channel::{InputStreamHandle, L2capChannel, OutputStreamHandle, Peer, StreamStatus};
pub use mutable_characteristic::MutableCharacteristic;
pub use mutable_service::MutableService;
pub use peripheral::{
    Peripheral, PeripheralCallbacks, PeripheralDelegate, PeripheralState, Service,
};
pub use peripheral_manager::{
    Central, PeripheralManager, PeripheralManagerCallbacks, PeripheralManagerConnectionLatency,
    PeripheralManagerDelegate, PeripheralManagerOptions, PeripheralManagerRestoredState,
    PeripheralManagerState,
};
pub use uuid::BluetoothUuid;

/// Common imports.
pub mod prelude {
    pub use crate::advertisement::AdvertisementData;
    pub use crate::att::{AttError, AttRequest};
    pub use crate::central_manager::{
        CentralManager, CentralManagerCallbacks, CentralManagerDelegate, CentralManagerOptions,
        CentralManagerRestoredState, CentralManagerState, ConnectOptions, ManagerAuthorization,
        ScanOptions,
    };
    pub use crate::characteristic::{
        AttributePermissions, Characteristic, CharacteristicProperties, CharacteristicWriteType,
    };
    pub use crate::descriptor::{Descriptor, DescriptorValue, MutableDescriptor};
    pub use crate::error::{BluetoothErrorCode, BluetoothErrorInfo, CoreBluetoothError};
    pub use crate::l2cap_channel::{
        InputStreamHandle, L2capChannel, OutputStreamHandle, Peer, StreamStatus,
    };
    pub use crate::mutable_characteristic::MutableCharacteristic;
    pub use crate::mutable_service::MutableService;
    pub use crate::peripheral::{
        Peripheral, PeripheralCallbacks, PeripheralDelegate, PeripheralState, Service,
    };
    pub use crate::peripheral_manager::{
        Central, PeripheralManager, PeripheralManagerCallbacks, PeripheralManagerConnectionLatency,
        PeripheralManagerDelegate, PeripheralManagerOptions, PeripheralManagerRestoredState,
        PeripheralManagerState,
    };
    pub use crate::uuid::BluetoothUuid;
}
