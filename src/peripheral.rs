use core::ffi::{c_char, c_void};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::Mutex;

use serde::Deserialize;
use serde_json::Value;

use crate::error::{from_swift, take_owned_c_string, BluetoothErrorInfo, CoreBluetoothError};
use crate::ffi;
use crate::private::{decode_optional_json, encode_string_slice, take_retained_pointer_array};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[repr(i32)]
pub enum PeripheralState {
    Disconnected = 0,
    Connecting = 1,
    Connected = 2,
    Disconnecting = 3,
}

impl PeripheralState {
    #[must_use]
    pub const fn from_raw(raw: i32) -> Self {
        match raw {
            1 => Self::Connecting,
            2 => Self::Connected,
            3 => Self::Disconnecting,
            _ => Self::Disconnected,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum CharacteristicWriteType {
    WithResponse = 0,
    WithoutResponse = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CharacteristicProperties(u64);

impl CharacteristicProperties {
    pub const BROADCAST: Self = Self(0x01);
    pub const READ: Self = Self(0x02);
    pub const WRITE_WITHOUT_RESPONSE: Self = Self(0x04);
    pub const WRITE: Self = Self(0x08);
    pub const NOTIFY: Self = Self(0x10);
    pub const INDICATE: Self = Self(0x20);
    pub const AUTHENTICATED_SIGNED_WRITES: Self = Self(0x40);
    pub const EXTENDED_PROPERTIES: Self = Self(0x80);
    pub const NOTIFY_ENCRYPTION_REQUIRED: Self = Self(0x100);
    pub const INDICATE_ENCRYPTION_REQUIRED: Self = Self(0x200);

    #[must_use]
    pub const fn from_bits(bits: u64) -> Self {
        Self(bits)
    }

    #[must_use]
    pub const fn bits(self) -> u64 {
        self.0
    }

    #[must_use]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
}

#[derive(Deserialize)]
struct PeripheralEventPayload {
    event: String,
    service_handles: Option<Vec<u64>>,
    service_handle: Option<u64>,
    characteristic_handles: Option<Vec<u64>>,
    characteristic_handle: Option<u64>,
    rssi: Option<i32>,
    error: Option<BluetoothErrorInfo>,
}

fn retained_handle_to_raw(handle: u64) -> *mut c_void {
    usize::try_from(handle).unwrap_or_else(|_| {
        unreachable!("retained handles must fit into usize on supported targets")
    }) as *mut c_void
}

mod private {
    pub trait Sealed {}
}

pub trait PeripheralDelegate: Send + private::Sealed {
    fn did_discover_services(&mut self, services: Vec<Service>, error: Option<BluetoothErrorInfo>) {
        let _ = (services, error);
    }

    fn did_discover_characteristics_for_service(
        &mut self,
        service: Service,
        characteristics: Vec<Characteristic>,
        error: Option<BluetoothErrorInfo>,
    ) {
        let _ = (service, characteristics, error);
    }

    fn did_update_value_for_characteristic(
        &mut self,
        characteristic: Characteristic,
        error: Option<BluetoothErrorInfo>,
    ) {
        let _ = (characteristic, error);
    }

    fn did_write_value_for_characteristic(
        &mut self,
        characteristic: Characteristic,
        error: Option<BluetoothErrorInfo>,
    ) {
        let _ = (characteristic, error);
    }

    fn did_update_notification_state_for_characteristic(
        &mut self,
        characteristic: Characteristic,
        error: Option<BluetoothErrorInfo>,
    ) {
        let _ = (characteristic, error);
    }

    fn did_read_rssi(&mut self, rssi: i32, error: Option<BluetoothErrorInfo>) {
        let _ = (rssi, error);
    }
}

type ServicesHandler = Box<dyn FnMut(Vec<Service>, Option<BluetoothErrorInfo>) + Send + 'static>;
type CharacteristicsHandler =
    Box<dyn FnMut(Service, Vec<Characteristic>, Option<BluetoothErrorInfo>) + Send + 'static>;
type CharacteristicHandler =
    Box<dyn FnMut(Characteristic, Option<BluetoothErrorInfo>) + Send + 'static>;
type RssiHandler = Box<dyn FnMut(i32, Option<BluetoothErrorInfo>) + Send + 'static>;

#[allow(clippy::type_complexity)]
pub struct PeripheralCallbacks {
    services: Option<ServicesHandler>,
    characteristics: Option<CharacteristicsHandler>,
    update_value: Option<CharacteristicHandler>,
    write_value: Option<CharacteristicHandler>,
    update_notification: Option<CharacteristicHandler>,
    rssi: Option<RssiHandler>,
}

impl PeripheralCallbacks {
    #[must_use]
    pub fn new() -> Self {
        Self {
            services: None,
            characteristics: None,
            update_value: None,
            write_value: None,
            update_notification: None,
            rssi: None,
        }
    }

    #[must_use]
    pub fn on_services(
        mut self,
        callback: impl FnMut(Vec<Service>, Option<BluetoothErrorInfo>) + Send + 'static,
    ) -> Self {
        self.services = Some(Box::new(callback));
        self
    }

    #[must_use]
    pub fn on_characteristics(
        mut self,
        callback: impl FnMut(Service, Vec<Characteristic>, Option<BluetoothErrorInfo>) + Send + 'static,
    ) -> Self {
        self.characteristics = Some(Box::new(callback));
        self
    }

    #[must_use]
    pub fn on_value_update(
        mut self,
        callback: impl FnMut(Characteristic, Option<BluetoothErrorInfo>) + Send + 'static,
    ) -> Self {
        self.update_value = Some(Box::new(callback));
        self
    }

    #[must_use]
    pub fn on_write(
        mut self,
        callback: impl FnMut(Characteristic, Option<BluetoothErrorInfo>) + Send + 'static,
    ) -> Self {
        self.write_value = Some(Box::new(callback));
        self
    }

    #[must_use]
    pub fn on_notification_state(
        mut self,
        callback: impl FnMut(Characteristic, Option<BluetoothErrorInfo>) + Send + 'static,
    ) -> Self {
        self.update_notification = Some(Box::new(callback));
        self
    }

    #[must_use]
    pub fn on_rssi(
        mut self,
        callback: impl FnMut(i32, Option<BluetoothErrorInfo>) + Send + 'static,
    ) -> Self {
        self.rssi = Some(Box::new(callback));
        self
    }
}

impl Default for PeripheralCallbacks {
    fn default() -> Self {
        Self::new()
    }
}

impl private::Sealed for PeripheralCallbacks {}
impl PeripheralDelegate for PeripheralCallbacks {
    fn did_discover_services(&mut self, services: Vec<Service>, error: Option<BluetoothErrorInfo>) {
        if let Some(callback) = &mut self.services {
            callback(services, error);
        }
    }

    fn did_discover_characteristics_for_service(
        &mut self,
        service: Service,
        characteristics: Vec<Characteristic>,
        error: Option<BluetoothErrorInfo>,
    ) {
        if let Some(callback) = &mut self.characteristics {
            callback(service, characteristics, error);
        }
    }

    fn did_update_value_for_characteristic(
        &mut self,
        characteristic: Characteristic,
        error: Option<BluetoothErrorInfo>,
    ) {
        if let Some(callback) = &mut self.update_value {
            callback(characteristic, error);
        }
    }

    fn did_write_value_for_characteristic(
        &mut self,
        characteristic: Characteristic,
        error: Option<BluetoothErrorInfo>,
    ) {
        if let Some(callback) = &mut self.write_value {
            callback(characteristic, error);
        }
    }

    fn did_update_notification_state_for_characteristic(
        &mut self,
        characteristic: Characteristic,
        error: Option<BluetoothErrorInfo>,
    ) {
        if let Some(callback) = &mut self.update_notification {
            callback(characteristic, error);
        }
    }

    fn did_read_rssi(&mut self, rssi: i32, error: Option<BluetoothErrorInfo>) {
        if let Some(callback) = &mut self.rssi {
            callback(rssi, error);
        }
    }
}

struct CallbackState {
    delegate: Mutex<Box<dyn PeripheralDelegate>>,
}

pub struct Peripheral {
    pub(crate) raw: *mut c_void,
    callback_state: Option<Box<CallbackState>>,
}

unsafe extern "C" fn peripheral_event_trampoline(
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
        let Ok(payload): Result<PeripheralEventPayload, _> = serde_json::from_str(&payload_json)
        else {
            return;
        };

        let mut delegate = match state.delegate.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };

        match payload.event.as_str() {
            "didDiscoverServices" => delegate.did_discover_services(
                payload
                    .service_handles
                    .unwrap_or_default()
                    .into_iter()
                    .map(Service::from_retained_handle)
                    .collect(),
                payload.error,
            ),
            "didDiscoverCharacteristicsForService" => {
                if let Some(service_handle) = payload.service_handle {
                    delegate.did_discover_characteristics_for_service(
                        Service::from_retained_handle(service_handle),
                        payload
                            .characteristic_handles
                            .unwrap_or_default()
                            .into_iter()
                            .map(Characteristic::from_retained_handle)
                            .collect(),
                        payload.error,
                    );
                }
            }
            "didUpdateValueForCharacteristic" => {
                if let Some(characteristic_handle) = payload.characteristic_handle {
                    delegate.did_update_value_for_characteristic(
                        Characteristic::from_retained_handle(characteristic_handle),
                        payload.error,
                    );
                }
            }
            "didWriteValueForCharacteristic" => {
                if let Some(characteristic_handle) = payload.characteristic_handle {
                    delegate.did_write_value_for_characteristic(
                        Characteristic::from_retained_handle(characteristic_handle),
                        payload.error,
                    );
                }
            }
            "didUpdateNotificationStateForCharacteristic" => {
                if let Some(characteristic_handle) = payload.characteristic_handle {
                    delegate.did_update_notification_state_for_characteristic(
                        Characteristic::from_retained_handle(characteristic_handle),
                        payload.error,
                    );
                }
            }
            "didReadRSSI" => {
                delegate.did_read_rssi(payload.rssi.unwrap_or_default(), payload.error);
            }
            _ => {}
        }
    }));
}

impl Peripheral {
    pub(crate) fn from_retained_raw(raw: *mut c_void) -> Self {
        Self {
            raw,
            callback_state: None,
        }
    }

    pub(crate) fn from_retained_handle(handle: u64) -> Self {
        Self::from_retained_raw(retained_handle_to_raw(handle))
    }

    pub fn set_delegate<D>(&mut self, delegate: D) -> Result<(), CoreBluetoothError>
    where
        D: PeripheralDelegate + 'static,
    {
        let callback_state = Box::new(CallbackState {
            delegate: Mutex::new(Box::new(delegate)),
        });
        let mut error = core::ptr::null_mut();
        let mut callback_state = Some(callback_state);
        let user_info = callback_state
            .as_deref_mut()
            .map_or(core::ptr::null_mut(), |state| {
                std::ptr::from_mut::<CallbackState>(state).cast::<c_void>()
            });
        let status = unsafe {
            ffi::cb_peripheral_set_delegate(
                self.raw,
                Some(peripheral_event_trampoline as ffi::PeripheralEventCallback),
                user_info,
                &mut error,
            )
        };
        if status == ffi::status::OK {
            self.callback_state = callback_state;
            Ok(())
        } else {
            Err(from_swift(status, error))
        }
    }

    pub fn set_callbacks(
        &mut self,
        callbacks: PeripheralCallbacks,
    ) -> Result<(), CoreBluetoothError> {
        self.set_delegate(callbacks)
    }

    pub fn clear_delegate(&mut self) {
        unsafe { ffi::cb_peripheral_clear_delegate(self.raw) };
        self.callback_state = None;
    }

    #[must_use]
    pub fn name(&self) -> String {
        let ptr = unsafe { ffi::cb_peripheral_name(self.raw) };
        take_owned_c_string(ptr)
    }

    #[must_use]
    pub fn identifier(&self) -> String {
        let ptr = unsafe { ffi::cb_peripheral_identifier(self.raw) };
        take_owned_c_string(ptr)
    }

    #[must_use]
    pub fn state(&self) -> PeripheralState {
        PeripheralState::from_raw(unsafe { ffi::cb_peripheral_state(self.raw) })
    }

    #[must_use]
    pub fn services(&self) -> Vec<Service> {
        let mut array = core::ptr::null_mut();
        let mut count = 0;
        unsafe { ffi::cb_peripheral_services(self.raw, &mut array, &mut count) };
        take_retained_pointer_array(array, count)
            .into_iter()
            .map(Service::from_retained_raw)
            .collect()
    }

    pub fn discover_services(
        &self,
        service_uuids: Option<&[&str]>,
    ) -> Result<(), CoreBluetoothError> {
        let service_uuids = match service_uuids {
            Some(service_uuids) => Some(encode_string_slice(service_uuids)?),
            None => None,
        };
        let mut error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cb_peripheral_discover_services(
                self.raw,
                service_uuids
                    .as_ref()
                    .map_or(core::ptr::null(), |value| value.as_ptr()),
                &mut error,
            )
        };
        if status == ffi::status::OK {
            Ok(())
        } else {
            Err(from_swift(status, error))
        }
    }

    pub fn read_rssi(&self) -> Result<(), CoreBluetoothError> {
        let mut error = core::ptr::null_mut();
        let status = unsafe { ffi::cb_peripheral_read_rssi(self.raw, &mut error) };
        if status == ffi::status::OK {
            Ok(())
        } else {
            Err(from_swift(status, error))
        }
    }

    pub fn discover_characteristics(
        &self,
        service: &Service,
        characteristic_uuids: Option<&[&str]>,
    ) -> Result<(), CoreBluetoothError> {
        let characteristic_uuids = match characteristic_uuids {
            Some(characteristic_uuids) => Some(encode_string_slice(characteristic_uuids)?),
            None => None,
        };
        let mut error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cb_peripheral_discover_characteristics(
                self.raw,
                service.raw,
                characteristic_uuids
                    .as_ref()
                    .map_or(core::ptr::null(), |value| value.as_ptr()),
                &mut error,
            )
        };
        if status == ffi::status::OK {
            Ok(())
        } else {
            Err(from_swift(status, error))
        }
    }

    pub fn read_value_for_characteristic(
        &self,
        characteristic: &Characteristic,
    ) -> Result<(), CoreBluetoothError> {
        let mut error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cb_peripheral_read_value_for_characteristic(
                self.raw,
                characteristic.raw,
                &mut error,
            )
        };
        if status == ffi::status::OK {
            Ok(())
        } else {
            Err(from_swift(status, error))
        }
    }

    pub fn write_value_for_characteristic(
        &self,
        characteristic: &Characteristic,
        value: &[u8],
        write_type: CharacteristicWriteType,
    ) -> Result<(), CoreBluetoothError> {
        let mut error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cb_peripheral_write_value_for_characteristic(
                self.raw,
                characteristic.raw,
                value.as_ptr(),
                value.len(),
                matches!(write_type, CharacteristicWriteType::WithResponse),
                &mut error,
            )
        };
        if status == ffi::status::OK {
            Ok(())
        } else {
            Err(from_swift(status, error))
        }
    }

    pub fn set_notify_value(
        &self,
        characteristic: &Characteristic,
        enabled: bool,
    ) -> Result<(), CoreBluetoothError> {
        let mut error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cb_peripheral_set_notify_value(self.raw, characteristic.raw, enabled, &mut error)
        };
        if status == ffi::status::OK {
            Ok(())
        } else {
            Err(from_swift(status, error))
        }
    }

    pub fn discover_descriptors(
        &self,
        characteristic: &Characteristic,
    ) -> Result<(), CoreBluetoothError> {
        let mut error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cb_peripheral_discover_descriptors(self.raw, characteristic.raw, &mut error)
        };
        if status == ffi::status::OK {
            Ok(())
        } else {
            Err(from_swift(status, error))
        }
    }
}

impl Drop for Peripheral {
    fn drop(&mut self) {
        if self.callback_state.is_some() {
            unsafe { ffi::cb_peripheral_clear_delegate(self.raw) };
        }
        unsafe { ffi::cb_object_release(self.raw) };
    }
}

pub struct Service {
    pub(crate) raw: *mut c_void,
}

impl Service {
    pub(crate) fn from_retained_raw(raw: *mut c_void) -> Self {
        Self { raw }
    }

    fn from_retained_handle(handle: u64) -> Self {
        Self::from_retained_raw(retained_handle_to_raw(handle))
    }

    #[must_use]
    pub fn uuid(&self) -> String {
        let ptr = unsafe { ffi::cb_service_uuid(self.raw) };
        take_owned_c_string(ptr)
    }

    #[must_use]
    pub fn is_primary(&self) -> bool {
        unsafe { ffi::cb_service_is_primary(self.raw) }
    }

    #[must_use]
    pub fn included_services(&self) -> Vec<Self> {
        let mut array = core::ptr::null_mut();
        let mut count = 0;
        unsafe { ffi::cb_service_included_services(self.raw, &mut array, &mut count) };
        take_retained_pointer_array(array, count)
            .into_iter()
            .map(Self::from_retained_raw)
            .collect()
    }

    #[must_use]
    pub fn characteristics(&self) -> Vec<Characteristic> {
        let mut array = core::ptr::null_mut();
        let mut count = 0;
        unsafe { ffi::cb_service_characteristics(self.raw, &mut array, &mut count) };
        take_retained_pointer_array(array, count)
            .into_iter()
            .map(Characteristic::from_retained_raw)
            .collect()
    }
}

impl Drop for Service {
    fn drop(&mut self) {
        unsafe { ffi::cb_object_release(self.raw) };
    }
}

pub struct Characteristic {
    pub(crate) raw: *mut c_void,
}

impl Characteristic {
    pub(crate) fn from_retained_raw(raw: *mut c_void) -> Self {
        Self { raw }
    }

    fn from_retained_handle(handle: u64) -> Self {
        Self::from_retained_raw(retained_handle_to_raw(handle))
    }

    #[must_use]
    pub fn uuid(&self) -> String {
        let ptr = unsafe { ffi::cb_characteristic_uuid(self.raw) };
        take_owned_c_string(ptr)
    }

    #[must_use]
    pub fn properties(&self) -> CharacteristicProperties {
        CharacteristicProperties::from_bits(unsafe { ffi::cb_characteristic_properties(self.raw) })
    }

    pub fn value(&self) -> Result<Option<Vec<u8>>, CoreBluetoothError> {
        let json = unsafe { ffi::cb_characteristic_value_json(self.raw) };
        decode_optional_json(json)
    }

    #[must_use]
    pub fn is_notifying(&self) -> bool {
        unsafe { ffi::cb_characteristic_is_notifying(self.raw) }
    }

    #[must_use]
    pub fn descriptors(&self) -> Vec<Descriptor> {
        let mut array = core::ptr::null_mut();
        let mut count = 0;
        unsafe { ffi::cb_characteristic_descriptors(self.raw, &mut array, &mut count) };
        take_retained_pointer_array(array, count)
            .into_iter()
            .map(Descriptor::from_retained_raw)
            .collect()
    }
}

impl Drop for Characteristic {
    fn drop(&mut self) {
        unsafe { ffi::cb_object_release(self.raw) };
    }
}

pub struct Descriptor {
    raw: *mut c_void,
}

impl Descriptor {
    fn from_retained_raw(raw: *mut c_void) -> Self {
        Self { raw }
    }

    #[must_use]
    pub fn uuid(&self) -> String {
        let ptr = unsafe { ffi::cb_descriptor_uuid(self.raw) };
        take_owned_c_string(ptr)
    }

    pub fn value(&self) -> Result<Option<Value>, CoreBluetoothError> {
        let json = unsafe { ffi::cb_descriptor_value_json(self.raw) };
        decode_optional_json(json)
    }
}

impl Drop for Descriptor {
    fn drop(&mut self) {
        unsafe { ffi::cb_object_release(self.raw) };
    }
}
