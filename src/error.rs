use core::ffi::c_char;
use core::fmt;

use libc::free;
use serde::Deserialize;

use crate::ffi;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct BluetoothErrorInfo {
    pub domain: String,
    pub code: i32,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum BluetoothErrorCode {
    Unknown = 0,
    InvalidParameters = 1,
    InvalidHandle = 2,
    NotConnected = 3,
    OutOfSpace = 4,
    OperationCancelled = 5,
    ConnectionTimeout = 6,
    PeripheralDisconnected = 7,
    UuidNotAllowed = 8,
    AlreadyAdvertising = 9,
    ConnectionFailed = 10,
    ConnectionLimitReached = 11,
    UnknownDevice = 12,
    OperationNotSupported = 13,
    PeerRemovedPairingInformation = 14,
    EncryptionTimedOut = 15,
    TooManyLePairedDevices = 16,
}

impl BluetoothErrorCode {
    #[must_use]
    pub const fn from_raw(raw: i32) -> Self {
        match raw {
            1 => Self::InvalidParameters,
            2 => Self::InvalidHandle,
            3 => Self::NotConnected,
            4 => Self::OutOfSpace,
            5 => Self::OperationCancelled,
            6 => Self::ConnectionTimeout,
            7 => Self::PeripheralDisconnected,
            8 => Self::UuidNotAllowed,
            9 => Self::AlreadyAdvertising,
            10 => Self::ConnectionFailed,
            11 => Self::ConnectionLimitReached,
            12 => Self::UnknownDevice,
            13 => Self::OperationNotSupported,
            14 => Self::PeerRemovedPairingInformation,
            15 => Self::EncryptionTimedOut,
            16 => Self::TooManyLePairedDevices,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum CoreBluetoothError {
    InvalidArgument(String),
    FrameworkError(String),
    Unknown { code: i32, message: String },
}

impl CoreBluetoothError {
    #[must_use]
    pub const fn code(&self) -> i32 {
        match self {
            Self::InvalidArgument(_) => ffi::status::INVALID_ARGUMENT,
            Self::FrameworkError(_) => ffi::status::FRAMEWORK_ERROR,
            Self::Unknown { code, .. } => *code,
        }
    }

    #[must_use]
    pub fn message(&self) -> &str {
        match self {
            Self::InvalidArgument(message)
            | Self::FrameworkError(message)
            | Self::Unknown { message, .. } => message,
        }
    }
}

impl fmt::Display for CoreBluetoothError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (code {})", self.message(), self.code())
    }
}

impl std::error::Error for CoreBluetoothError {}

pub(crate) fn take_owned_c_string(ptr: *mut c_char) -> String {
    if ptr.is_null() {
        return String::new();
    }

    let string = unsafe { core::ffi::CStr::from_ptr(ptr) }
        .to_string_lossy()
        .into_owned();
    unsafe { free(ptr.cast()) };
    string
}

pub(crate) fn from_swift(status: i32, error_str: *mut c_char) -> CoreBluetoothError {
    from_status_message(status, take_owned_c_string(error_str))
}

pub(crate) fn from_status_message(status: i32, message: String) -> CoreBluetoothError {
    match status {
        ffi::status::INVALID_ARGUMENT => CoreBluetoothError::InvalidArgument(message),
        ffi::status::FRAMEWORK_ERROR => CoreBluetoothError::FrameworkError(message),
        code => CoreBluetoothError::Unknown { code, message },
    }
}
