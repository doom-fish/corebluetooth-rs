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
