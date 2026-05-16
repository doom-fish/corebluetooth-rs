use core::ffi::{c_char, c_void};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::{from_swift, BluetoothErrorInfo, CoreBluetoothError};
use crate::ffi;
use crate::peripheral::Peripheral;
use crate::private::{encode_string_slice, take_retained_pointer_array, to_cstring};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[repr(i32)]
pub enum CentralManagerState {
    Unknown = 0,
    Resetting = 1,
    Unsupported = 2,
    Unauthorized = 3,
    PoweredOff = 4,
    PoweredOn = 5,
}

impl CentralManagerState {
    #[must_use]
    pub const fn from_raw(raw: i32) -> Self {
        match raw {
            1 => Self::Resetting,
            2 => Self::Unsupported,
            3 => Self::Unauthorized,
            4 => Self::PoweredOff,
            5 => Self::PoweredOn,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[repr(i32)]
pub enum ManagerAuthorization {
    NotDetermined = 0,
    Restricted = 1,
    Denied = 2,
    AllowedAlways = 3,
}

impl ManagerAuthorization {
    #[must_use]
    pub const fn from_raw(raw: i32) -> Self {
        match raw {
            1 => Self::Restricted,
            2 => Self::Denied,
            3 => Self::AllowedAlways,
            _ => Self::NotDetermined,
        }
    }

    #[must_use]
    pub const fn is_authorized(self) -> bool {
        matches!(self, Self::AllowedAlways)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScanOptions {
    pub allow_duplicates: bool,
}

impl ScanOptions {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            allow_duplicates: false,
        }
    }

    #[must_use]
    pub const fn with_allow_duplicates(mut self, allow_duplicates: bool) -> Self {
        self.allow_duplicates = allow_duplicates;
        self
    }
}

impl Default for ScanOptions {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Deserialize, Serialize)]
struct CentralManagerOptionsPayload {
    queue_label: Option<String>,
}

#[derive(Deserialize)]
struct CentralManagerEventPayload {
    event: String,
    state: Option<i32>,
    authorization: Option<i32>,
    peripheral_handle: Option<u64>,
    rssi: Option<i32>,
    advertisement_data: Option<Value>,
    error: Option<BluetoothErrorInfo>,
}

mod private {
    pub trait Sealed {}
}

pub trait CentralManagerDelegate: Send + private::Sealed {
    fn did_update_state(
        &mut self,
        state: CentralManagerState,
        authorization: ManagerAuthorization,
    ) {
        let _ = (state, authorization);
    }

    fn did_discover_peripheral(
        &mut self,
        peripheral: Peripheral,
        rssi: i32,
        advertisement_data: Value,
    ) {
        let _ = (peripheral, rssi, advertisement_data);
    }

    fn did_connect_peripheral(&mut self, peripheral: Peripheral) {
        let _ = peripheral;
    }

    fn did_fail_to_connect_peripheral(
        &mut self,
        peripheral: Peripheral,
        error: Option<BluetoothErrorInfo>,
    ) {
        let _ = (peripheral, error);
    }

    fn did_disconnect_peripheral(
        &mut self,
        peripheral: Peripheral,
        error: Option<BluetoothErrorInfo>,
    ) {
        let _ = (peripheral, error);
    }
}

type StateHandler = Box<dyn FnMut(CentralManagerState, ManagerAuthorization) + Send + 'static>;
type DiscoveryHandler = Box<dyn FnMut(Peripheral, i32, Value) + Send + 'static>;
type ConnectionHandler = Box<dyn FnMut(Peripheral) + Send + 'static>;
type ErrorHandler = Box<dyn FnMut(Peripheral, Option<BluetoothErrorInfo>) + Send + 'static>;

#[allow(clippy::type_complexity)]
pub struct CentralManagerCallbacks {
    state: Option<StateHandler>,
    discover: Option<DiscoveryHandler>,
    connect: Option<ConnectionHandler>,
    fail_to_connect: Option<ErrorHandler>,
    disconnect: Option<ErrorHandler>,
}

impl CentralManagerCallbacks {
    #[must_use]
    pub fn new() -> Self {
        Self {
            state: None,
            discover: None,
            connect: None,
            fail_to_connect: None,
            disconnect: None,
        }
    }

    #[must_use]
    pub fn on_state(
        mut self,
        callback: impl FnMut(CentralManagerState, ManagerAuthorization) + Send + 'static,
    ) -> Self {
        self.state = Some(Box::new(callback));
        self
    }

    #[must_use]
    pub fn on_discover(
        mut self,
        callback: impl FnMut(Peripheral, i32, Value) + Send + 'static,
    ) -> Self {
        self.discover = Some(Box::new(callback));
        self
    }

    #[must_use]
    pub fn on_connect(mut self, callback: impl FnMut(Peripheral) + Send + 'static) -> Self {
        self.connect = Some(Box::new(callback));
        self
    }

    #[must_use]
    pub fn on_fail_to_connect(
        mut self,
        callback: impl FnMut(Peripheral, Option<BluetoothErrorInfo>) + Send + 'static,
    ) -> Self {
        self.fail_to_connect = Some(Box::new(callback));
        self
    }

    #[must_use]
    pub fn on_disconnect(
        mut self,
        callback: impl FnMut(Peripheral, Option<BluetoothErrorInfo>) + Send + 'static,
    ) -> Self {
        self.disconnect = Some(Box::new(callback));
        self
    }
}

impl Default for CentralManagerCallbacks {
    fn default() -> Self {
        Self::new()
    }
}

impl private::Sealed for CentralManagerCallbacks {}
impl CentralManagerDelegate for CentralManagerCallbacks {
    fn did_update_state(
        &mut self,
        state: CentralManagerState,
        authorization: ManagerAuthorization,
    ) {
        if let Some(callback) = &mut self.state {
            callback(state, authorization);
        }
    }

    fn did_discover_peripheral(
        &mut self,
        peripheral: Peripheral,
        rssi: i32,
        advertisement_data: Value,
    ) {
        if let Some(callback) = &mut self.discover {
            callback(peripheral, rssi, advertisement_data);
        }
    }

    fn did_connect_peripheral(&mut self, peripheral: Peripheral) {
        if let Some(callback) = &mut self.connect {
            callback(peripheral);
        }
    }

    fn did_fail_to_connect_peripheral(
        &mut self,
        peripheral: Peripheral,
        error: Option<BluetoothErrorInfo>,
    ) {
        if let Some(callback) = &mut self.fail_to_connect {
            callback(peripheral, error);
        }
    }

    fn did_disconnect_peripheral(
        &mut self,
        peripheral: Peripheral,
        error: Option<BluetoothErrorInfo>,
    ) {
        if let Some(callback) = &mut self.disconnect {
            callback(peripheral, error);
        }
    }
}

struct CallbackState {
    delegate: Mutex<Box<dyn CentralManagerDelegate>>,
}

pub struct CentralManager {
    raw: *mut c_void,
    callback_state: Option<Box<CallbackState>>,
}

unsafe extern "C" fn central_manager_event_trampoline(
    user_info: *mut c_void,
    payload_json: *const c_char,
) {
    if user_info.is_null() || payload_json.is_null() {
        return;
    }

    let _ = catch_unwind(AssertUnwindSafe(|| {
        let state = unsafe { &*user_info.cast::<CallbackState>() };
        let payload_json = unsafe { core::ffi::CStr::from_ptr(payload_json) }
            .to_string_lossy()
            .into_owned();
        let Ok(payload): Result<CentralManagerEventPayload, _> =
            serde_json::from_str(&payload_json)
        else {
            return;
        };

        let mut delegate = match state.delegate.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };

        match payload.event.as_str() {
            "didUpdateState" => delegate.did_update_state(
                CentralManagerState::from_raw(payload.state.unwrap_or_default()),
                ManagerAuthorization::from_raw(payload.authorization.unwrap_or_default()),
            ),
            "didDiscoverPeripheral" => {
                if let Some(handle) = payload.peripheral_handle {
                    delegate.did_discover_peripheral(
                        Peripheral::from_retained_handle(handle),
                        payload.rssi.unwrap_or_default(),
                        payload.advertisement_data.unwrap_or(Value::Null),
                    );
                }
            }
            "didConnectPeripheral" => {
                if let Some(handle) = payload.peripheral_handle {
                    delegate.did_connect_peripheral(Peripheral::from_retained_handle(handle));
                }
            }
            "didFailToConnectPeripheral" => {
                if let Some(handle) = payload.peripheral_handle {
                    delegate.did_fail_to_connect_peripheral(
                        Peripheral::from_retained_handle(handle),
                        payload.error,
                    );
                }
            }
            "didDisconnectPeripheral" => {
                if let Some(handle) = payload.peripheral_handle {
                    delegate.did_disconnect_peripheral(
                        Peripheral::from_retained_handle(handle),
                        payload.error,
                    );
                }
            }
            _ => {}
        }
    }));
}

impl CentralManager {
    pub fn new() -> Result<Self, CoreBluetoothError> {
        Self::new_inner(None, None)
    }

    pub fn with_delegate<D>(delegate: D) -> Result<Self, CoreBluetoothError>
    where
        D: CentralManagerDelegate + 'static,
    {
        Self::new_inner(None, Some(Box::new(delegate)))
    }

    pub fn with_callbacks(callbacks: CentralManagerCallbacks) -> Result<Self, CoreBluetoothError> {
        Self::with_delegate(callbacks)
    }

    pub fn with_queue_label(queue_label: &str) -> Result<Self, CoreBluetoothError> {
        Self::new_inner(Some(queue_label), None)
    }

    pub fn with_queue_label_and_delegate<D>(
        queue_label: &str,
        delegate: D,
    ) -> Result<Self, CoreBluetoothError>
    where
        D: CentralManagerDelegate + 'static,
    {
        Self::new_inner(Some(queue_label), Some(Box::new(delegate)))
    }

    fn new_inner(
        queue_label: Option<&str>,
        delegate: Option<Box<dyn CentralManagerDelegate>>,
    ) -> Result<Self, CoreBluetoothError> {
        let options_json = CentralManagerOptionsPayload {
            queue_label: queue_label.map(ToOwned::to_owned),
        };
        let options_json = serde_json::to_string(&options_json).map_err(|error| {
            CoreBluetoothError::FrameworkError(format!("failed to encode manager options: {error}"))
        })?;
        let options_json = to_cstring(&options_json)?;

        let mut raw = core::ptr::null_mut();
        let mut error = core::ptr::null_mut();
        let mut callback_state = delegate.map(|delegate| {
            Box::new(CallbackState {
                delegate: Mutex::new(delegate),
            })
        });
        let user_info = callback_state
            .as_deref_mut()
            .map_or(core::ptr::null_mut(), |state| {
                std::ptr::from_mut::<CallbackState>(state).cast::<c_void>()
            });
        let callback = if callback_state.is_some() {
            Some(central_manager_event_trampoline as ffi::CentralManagerEventCallback)
        } else {
            None
        };

        let status = unsafe {
            ffi::cb_manager_new(
                options_json.as_ptr(),
                callback,
                user_info,
                &mut raw,
                &mut error,
            )
        };
        if status == ffi::status::OK {
            Ok(Self {
                raw,
                callback_state,
            })
        } else {
            Err(from_swift(status, error))
        }
    }

    #[must_use]
    pub fn state(&self) -> CentralManagerState {
        CentralManagerState::from_raw(unsafe { ffi::cb_manager_state(self.raw) })
    }

    #[must_use]
    pub fn authorization(&self) -> ManagerAuthorization {
        ManagerAuthorization::from_raw(unsafe { ffi::cb_manager_authorization(self.raw) })
    }

    #[must_use]
    pub fn is_scanning(&self) -> bool {
        unsafe { ffi::cb_manager_is_scanning(self.raw) }
    }

    pub fn scan_for_peripherals(
        &self,
        service_uuids: Option<&[&str]>,
        options: ScanOptions,
    ) -> Result<(), CoreBluetoothError> {
        let service_uuids = match service_uuids {
            Some(service_uuids) => Some(encode_string_slice(service_uuids)?),
            None => None,
        };
        let mut error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cb_manager_scan_for_peripherals(
                self.raw,
                service_uuids
                    .as_ref()
                    .map_or(core::ptr::null(), |value| value.as_ptr()),
                options.allow_duplicates,
                &mut error,
            )
        };
        if status == ffi::status::OK {
            Ok(())
        } else {
            Err(from_swift(status, error))
        }
    }

    pub fn stop_scan(&self) {
        unsafe { ffi::cb_manager_stop_scan(self.raw) };
    }

    pub fn connect(&self, peripheral: &Peripheral) -> Result<(), CoreBluetoothError> {
        let mut error = core::ptr::null_mut();
        let status = unsafe { ffi::cb_manager_connect(self.raw, peripheral.raw, &mut error) };
        if status == ffi::status::OK {
            Ok(())
        } else {
            Err(from_swift(status, error))
        }
    }

    pub fn cancel_peripheral_connection(&self, peripheral: &Peripheral) {
        unsafe { ffi::cb_manager_cancel_peripheral_connection(self.raw, peripheral.raw) };
    }

    pub fn retrieve_connected_peripherals(
        &self,
        service_uuids: &[&str],
    ) -> Result<Vec<Peripheral>, CoreBluetoothError> {
        let service_uuids = encode_string_slice(service_uuids)?;
        let mut array = core::ptr::null_mut();
        let mut count = 0;
        let mut error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cb_manager_retrieve_connected_peripherals(
                self.raw,
                service_uuids.as_ptr(),
                &mut array,
                &mut count,
                &mut error,
            )
        };
        if status == ffi::status::OK {
            Ok(take_retained_pointer_array(array, count)
                .into_iter()
                .map(Peripheral::from_retained_raw)
                .collect())
        } else {
            Err(from_swift(status, error))
        }
    }

    pub fn retrieve_peripherals_with_identifiers(
        &self,
        identifiers: &[&str],
    ) -> Result<Vec<Peripheral>, CoreBluetoothError> {
        let identifiers = encode_string_slice(identifiers)?;
        let mut array = core::ptr::null_mut();
        let mut count = 0;
        let mut error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cb_manager_retrieve_peripherals_with_identifiers(
                self.raw,
                identifiers.as_ptr(),
                &mut array,
                &mut count,
                &mut error,
            )
        };
        if status == ffi::status::OK {
            Ok(take_retained_pointer_array(array, count)
                .into_iter()
                .map(Peripheral::from_retained_raw)
                .collect())
        } else {
            Err(from_swift(status, error))
        }
    }
}

impl Drop for CentralManager {
    fn drop(&mut self) {
        unsafe { ffi::cb_object_release(self.raw) };
        let _ = self.callback_state.take();
    }
}
