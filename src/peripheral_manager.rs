use core::ffi::{c_char, c_void};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::advertisement::AdvertisementData;
use crate::att::{AttError, AttRequest};
use crate::central_manager::ManagerAuthorization;
use crate::characteristic::Characteristic;
use crate::error::{from_swift, take_owned_c_string, BluetoothErrorInfo, CoreBluetoothError};
use crate::ffi;
use crate::l2cap_channel::L2capChannel;
use crate::mutable_characteristic::MutableCharacteristic;
use crate::mutable_service::MutableService;
use crate::private::{encode_json, retain_raw, retained_handle_to_raw};
use crate::service::Service;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[repr(i32)]
pub enum PeripheralManagerState {
    Unknown = 0,
    Resetting = 1,
    Unsupported = 2,
    Unauthorized = 3,
    PoweredOff = 4,
    PoweredOn = 5,
}

impl PeripheralManagerState {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PeripheralManagerConnectionLatency {
    Low = 0,
    Medium = 1,
    High = 2,
}

#[derive(Debug, Clone, Default)]
#[must_use]
pub struct PeripheralManagerOptions {
    pub queue_label: Option<String>,
    pub show_power_alert: Option<bool>,
    pub restore_identifier: Option<String>,
}

impl PeripheralManagerOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_queue_label(mut self, queue_label: impl Into<String>) -> Self {
        self.queue_label = Some(queue_label.into());
        self
    }

    pub fn with_show_power_alert(mut self, show_power_alert: bool) -> Self {
        self.show_power_alert = Some(show_power_alert);
        self
    }

    pub fn with_restore_identifier(mut self, restore_identifier: impl Into<String>) -> Self {
        self.restore_identifier = Some(restore_identifier.into());
        self
    }
}

pub struct Central {
    pub(crate) raw: *mut c_void,
}

impl Central {
    pub(crate) fn from_retained_raw(raw: *mut c_void) -> Self {
        Self { raw }
    }

    pub(crate) fn from_retained_handle(handle: u64) -> Self {
        Self::from_retained_raw(retained_handle_to_raw(handle))
    }

    pub fn identifier(&self) -> String {
        take_owned_c_string(unsafe { ffi::cb_central_identifier(self.raw) })
    }

    pub fn maximum_update_value_length(&self) -> usize {
        unsafe { ffi::cb_central_maximum_update_value_length(self.raw) }
    }
}

impl Clone for Central {
    fn clone(&self) -> Self {
        Self::from_retained_raw(retain_raw(self.raw))
    }
}

impl Drop for Central {
    fn drop(&mut self) {
        unsafe { ffi::cb_object_release(self.raw) };
    }
}

pub struct PeripheralManagerRestoredState {
    pub services: Vec<MutableService>,
    pub advertisement_data: Option<AdvertisementData>,
}

#[derive(Serialize)]
struct PeripheralManagerOptionsPayload {
    queue_label: Option<String>,
    show_power_alert: Option<bool>,
    restore_identifier: Option<String>,
}

#[derive(Deserialize)]
struct PeripheralManagerEventPayload {
    event: String,
    state: Option<i32>,
    authorization: Option<i32>,
    service_handle: Option<u64>,
    service_handles: Option<Vec<u64>>,
    central_handle: Option<u64>,
    characteristic_handle: Option<u64>,
    request_handle: Option<u64>,
    request_handles: Option<Vec<u64>>,
    psm: Option<u16>,
    channel_handle: Option<u64>,
    advertisement_data: Option<Value>,
    error: Option<BluetoothErrorInfo>,
}

mod private {
    pub trait Sealed {}
}

pub trait PeripheralManagerDelegate: Send + private::Sealed {
    fn did_update_state(
        &mut self,
        state: PeripheralManagerState,
        authorization: ManagerAuthorization,
    ) {
        let _ = (state, authorization);
    }

    fn will_restore_state(&mut self, restored_state: PeripheralManagerRestoredState) {
        let _ = restored_state;
    }

    fn did_start_advertising(&mut self, error: Option<BluetoothErrorInfo>) {
        let _ = error;
    }

    fn did_add_service(&mut self, service: Service, error: Option<BluetoothErrorInfo>) {
        let _ = (service, error);
    }

    fn did_subscribe_central(&mut self, central: Central, characteristic: Characteristic) {
        let _ = (central, characteristic);
    }

    fn did_unsubscribe_central(&mut self, central: Central, characteristic: Characteristic) {
        let _ = (central, characteristic);
    }

    fn is_ready_to_update_subscribers(&mut self) {}

    fn did_receive_read_request(&mut self, request: AttRequest) {
        let _ = request;
    }

    fn did_receive_write_requests(&mut self, requests: Vec<AttRequest>) {
        let _ = requests;
    }

    fn did_publish_l2cap_channel(&mut self, psm: u16, error: Option<BluetoothErrorInfo>) {
        let _ = (psm, error);
    }

    fn did_unpublish_l2cap_channel(&mut self, psm: u16, error: Option<BluetoothErrorInfo>) {
        let _ = (psm, error);
    }

    fn did_open_l2cap_channel(
        &mut self,
        channel: Option<L2capChannel>,
        error: Option<BluetoothErrorInfo>,
    ) {
        let _ = (channel, error);
    }
}

type StateHandler = Box<dyn FnMut(PeripheralManagerState, ManagerAuthorization) + Send + 'static>;
type RestoreHandler = Box<dyn FnMut(PeripheralManagerRestoredState) + Send + 'static>;
type ErrorOnlyHandler = Box<dyn FnMut(Option<BluetoothErrorInfo>) + Send + 'static>;
type AddServiceHandler = Box<dyn FnMut(Service, Option<BluetoothErrorInfo>) + Send + 'static>;
type SubscriptionHandler = Box<dyn FnMut(Central, Characteristic) + Send + 'static>;
type ReadRequestHandler = Box<dyn FnMut(AttRequest) + Send + 'static>;
type WriteRequestsHandler = Box<dyn FnMut(Vec<AttRequest>) + Send + 'static>;
type L2capPublishHandler = Box<dyn FnMut(u16, Option<BluetoothErrorInfo>) + Send + 'static>;
type L2capOpenHandler =
    Box<dyn FnMut(Option<L2capChannel>, Option<BluetoothErrorInfo>) + Send + 'static>;

#[allow(clippy::type_complexity)]
#[must_use]
pub struct PeripheralManagerCallbacks {
    state: Option<StateHandler>,
    restore_state: Option<RestoreHandler>,
    start_advertising: Option<ErrorOnlyHandler>,
    add_service: Option<AddServiceHandler>,
    subscribe: Option<SubscriptionHandler>,
    unsubscribe: Option<SubscriptionHandler>,
    ready_to_update: Option<Box<dyn FnMut() + Send + 'static>>,
    read_request: Option<ReadRequestHandler>,
    write_requests: Option<WriteRequestsHandler>,
    publish_l2cap: Option<L2capPublishHandler>,
    unpublish_l2cap: Option<L2capPublishHandler>,
    open_l2cap: Option<L2capOpenHandler>,
}

impl PeripheralManagerCallbacks {
    pub fn new() -> Self {
        Self {
            state: None,
            restore_state: None,
            start_advertising: None,
            add_service: None,
            subscribe: None,
            unsubscribe: None,
            ready_to_update: None,
            read_request: None,
            write_requests: None,
            publish_l2cap: None,
            unpublish_l2cap: None,
            open_l2cap: None,
        }
    }

    pub fn on_state(
        mut self,
        callback: impl FnMut(PeripheralManagerState, ManagerAuthorization) + Send + 'static,
    ) -> Self {
        self.state = Some(Box::new(callback));
        self
    }

    pub fn on_restore_state(
        mut self,
        callback: impl FnMut(PeripheralManagerRestoredState) + Send + 'static,
    ) -> Self {
        self.restore_state = Some(Box::new(callback));
        self
    }

    pub fn on_start_advertising(
        mut self,
        callback: impl FnMut(Option<BluetoothErrorInfo>) + Send + 'static,
    ) -> Self {
        self.start_advertising = Some(Box::new(callback));
        self
    }

    pub fn on_add_service(
        mut self,
        callback: impl FnMut(Service, Option<BluetoothErrorInfo>) + Send + 'static,
    ) -> Self {
        self.add_service = Some(Box::new(callback));
        self
    }

    pub fn on_subscribe(
        mut self,
        callback: impl FnMut(Central, Characteristic) + Send + 'static,
    ) -> Self {
        self.subscribe = Some(Box::new(callback));
        self
    }

    pub fn on_unsubscribe(
        mut self,
        callback: impl FnMut(Central, Characteristic) + Send + 'static,
    ) -> Self {
        self.unsubscribe = Some(Box::new(callback));
        self
    }

    pub fn on_ready_to_update(mut self, callback: impl FnMut() + Send + 'static) -> Self {
        self.ready_to_update = Some(Box::new(callback));
        self
    }

    pub fn on_read_request(mut self, callback: impl FnMut(AttRequest) + Send + 'static) -> Self {
        self.read_request = Some(Box::new(callback));
        self
    }

    pub fn on_write_requests(
        mut self,
        callback: impl FnMut(Vec<AttRequest>) + Send + 'static,
    ) -> Self {
        self.write_requests = Some(Box::new(callback));
        self
    }

    pub fn on_publish_l2cap_channel(
        mut self,
        callback: impl FnMut(u16, Option<BluetoothErrorInfo>) + Send + 'static,
    ) -> Self {
        self.publish_l2cap = Some(Box::new(callback));
        self
    }

    pub fn on_unpublish_l2cap_channel(
        mut self,
        callback: impl FnMut(u16, Option<BluetoothErrorInfo>) + Send + 'static,
    ) -> Self {
        self.unpublish_l2cap = Some(Box::new(callback));
        self
    }

    pub fn on_open_l2cap_channel(
        mut self,
        callback: impl FnMut(Option<L2capChannel>, Option<BluetoothErrorInfo>) + Send + 'static,
    ) -> Self {
        self.open_l2cap = Some(Box::new(callback));
        self
    }
}

impl Default for PeripheralManagerCallbacks {
    fn default() -> Self {
        Self::new()
    }
}

impl private::Sealed for PeripheralManagerCallbacks {}
impl PeripheralManagerDelegate for PeripheralManagerCallbacks {
    fn did_update_state(
        &mut self,
        state: PeripheralManagerState,
        authorization: ManagerAuthorization,
    ) {
        if let Some(callback) = &mut self.state {
            callback(state, authorization);
        }
    }

    fn will_restore_state(&mut self, restored_state: PeripheralManagerRestoredState) {
        if let Some(callback) = &mut self.restore_state {
            callback(restored_state);
        }
    }

    fn did_start_advertising(&mut self, error: Option<BluetoothErrorInfo>) {
        if let Some(callback) = &mut self.start_advertising {
            callback(error);
        }
    }

    fn did_add_service(&mut self, service: Service, error: Option<BluetoothErrorInfo>) {
        if let Some(callback) = &mut self.add_service {
            callback(service, error);
        }
    }

    fn did_subscribe_central(&mut self, central: Central, characteristic: Characteristic) {
        if let Some(callback) = &mut self.subscribe {
            callback(central, characteristic);
        }
    }

    fn did_unsubscribe_central(&mut self, central: Central, characteristic: Characteristic) {
        if let Some(callback) = &mut self.unsubscribe {
            callback(central, characteristic);
        }
    }

    fn is_ready_to_update_subscribers(&mut self) {
        if let Some(callback) = &mut self.ready_to_update {
            callback();
        }
    }

    fn did_receive_read_request(&mut self, request: AttRequest) {
        if let Some(callback) = &mut self.read_request {
            callback(request);
        }
    }

    fn did_receive_write_requests(&mut self, requests: Vec<AttRequest>) {
        if let Some(callback) = &mut self.write_requests {
            callback(requests);
        }
    }

    fn did_publish_l2cap_channel(&mut self, psm: u16, error: Option<BluetoothErrorInfo>) {
        if let Some(callback) = &mut self.publish_l2cap {
            callback(psm, error);
        }
    }

    fn did_unpublish_l2cap_channel(&mut self, psm: u16, error: Option<BluetoothErrorInfo>) {
        if let Some(callback) = &mut self.unpublish_l2cap {
            callback(psm, error);
        }
    }

    fn did_open_l2cap_channel(
        &mut self,
        channel: Option<L2capChannel>,
        error: Option<BluetoothErrorInfo>,
    ) {
        if let Some(callback) = &mut self.open_l2cap {
            callback(channel, error);
        }
    }
}

struct CallbackState {
    delegate: Mutex<Box<dyn PeripheralManagerDelegate>>,
}

pub struct PeripheralManager {
    raw: *mut c_void,
    callback_state: Option<Box<CallbackState>>,
}

unsafe extern "C" fn peripheral_manager_event_trampoline(
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
        let Ok(payload): Result<PeripheralManagerEventPayload, _> =
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
                PeripheralManagerState::from_raw(payload.state.unwrap_or_default()),
                ManagerAuthorization::from_raw(payload.authorization.unwrap_or_default()),
            ),
            "willRestoreState" => delegate.will_restore_state(PeripheralManagerRestoredState {
                services: payload
                    .service_handles
                    .unwrap_or_default()
                    .into_iter()
                    .map(MutableService::from_retained_handle)
                    .collect(),
                advertisement_data: payload
                    .advertisement_data
                    .and_then(|value| AdvertisementData::from_json_value(value).ok()),
            }),
            "didStartAdvertising" => delegate.did_start_advertising(payload.error),
            "didAddService" => {
                if let Some(handle) = payload.service_handle {
                    delegate.did_add_service(Service::from_retained_handle(handle), payload.error);
                }
            }
            "didSubscribeToCharacteristic" => {
                if let (Some(central), Some(characteristic)) =
                    (payload.central_handle, payload.characteristic_handle)
                {
                    delegate.did_subscribe_central(
                        Central::from_retained_handle(central),
                        Characteristic::from_retained_handle(characteristic),
                    );
                }
            }
            "didUnsubscribeFromCharacteristic" => {
                if let (Some(central), Some(characteristic)) =
                    (payload.central_handle, payload.characteristic_handle)
                {
                    delegate.did_unsubscribe_central(
                        Central::from_retained_handle(central),
                        Characteristic::from_retained_handle(characteristic),
                    );
                }
            }
            "isReadyToUpdateSubscribers" => delegate.is_ready_to_update_subscribers(),
            "didReceiveReadRequest" => {
                if let Some(handle) = payload.request_handle {
                    delegate.did_receive_read_request(AttRequest::from_retained_handle(handle));
                }
            }
            "didReceiveWriteRequests" => delegate.did_receive_write_requests(
                payload
                    .request_handles
                    .unwrap_or_default()
                    .into_iter()
                    .map(AttRequest::from_retained_handle)
                    .collect(),
            ),
            "didPublishL2CAPChannel" => {
                delegate.did_publish_l2cap_channel(payload.psm.unwrap_or_default(), payload.error);
            }
            "didUnpublishL2CAPChannel" => {
                delegate
                    .did_unpublish_l2cap_channel(payload.psm.unwrap_or_default(), payload.error);
            }
            "didOpenL2CAPChannel" => delegate.did_open_l2cap_channel(
                payload
                    .channel_handle
                    .map(L2capChannel::from_retained_handle),
                payload.error,
            ),
            _ => {}
        }
    }));
}

impl PeripheralManager {
    pub fn new() -> Result<Self, CoreBluetoothError> {
        Self::with_options(PeripheralManagerOptions::default())
    }

    pub fn with_options(options: PeripheralManagerOptions) -> Result<Self, CoreBluetoothError> {
        Self::new_inner(options, None)
    }

    pub fn with_delegate<D>(delegate: D) -> Result<Self, CoreBluetoothError>
    where
        D: PeripheralManagerDelegate + 'static,
    {
        Self::new_inner(
            PeripheralManagerOptions::default(),
            Some(Box::new(delegate)),
        )
    }

    pub fn with_callbacks(
        callbacks: PeripheralManagerCallbacks,
    ) -> Result<Self, CoreBluetoothError> {
        Self::with_delegate(callbacks)
    }

    pub fn with_queue_label(queue_label: &str) -> Result<Self, CoreBluetoothError> {
        Self::with_options(PeripheralManagerOptions::new().with_queue_label(queue_label))
    }

    pub fn current_authorization() -> ManagerAuthorization {
        ManagerAuthorization::from_raw(unsafe { ffi::cb_peripheral_manager_global_authorization() })
    }

    fn new_inner(
        options: PeripheralManagerOptions,
        delegate: Option<Box<dyn PeripheralManagerDelegate>>,
    ) -> Result<Self, CoreBluetoothError> {
        let options_json = encode_json(&PeripheralManagerOptionsPayload {
            queue_label: options.queue_label,
            show_power_alert: options.show_power_alert,
            restore_identifier: options.restore_identifier,
        })?;

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
            Some(peripheral_manager_event_trampoline as ffi::JsonCallback)
        } else {
            None
        };
        let status = unsafe {
            ffi::cb_peripheral_manager_new(
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

    pub(crate) const fn as_raw(&self) -> *mut c_void {
        self.raw
    }

    pub fn state(&self) -> PeripheralManagerState {
        PeripheralManagerState::from_raw(unsafe { ffi::cb_peripheral_manager_state(self.raw) })
    }

    pub fn authorization(&self) -> ManagerAuthorization {
        ManagerAuthorization::from_raw(unsafe {
            ffi::cb_peripheral_manager_authorization(self.raw)
        })
    }

    pub fn is_advertising(&self) -> bool {
        unsafe { ffi::cb_peripheral_manager_is_advertising(self.raw) }
    }

    pub fn start_advertising(
        &self,
        advertisement_data: &AdvertisementData,
    ) -> Result<(), CoreBluetoothError> {
        let advertisement_data = advertisement_data.encode_for_advertising()?;
        let mut error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cb_peripheral_manager_start_advertising(
                self.raw,
                advertisement_data.as_ptr(),
                &mut error,
            )
        };
        if status == ffi::status::OK {
            Ok(())
        } else {
            Err(from_swift(status, error))
        }
    }

    pub fn stop_advertising(&self) {
        unsafe { ffi::cb_peripheral_manager_stop_advertising(self.raw) };
    }

    pub fn set_desired_connection_latency(
        &self,
        latency: PeripheralManagerConnectionLatency,
        central: &Central,
    ) -> Result<(), CoreBluetoothError> {
        let mut error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cb_peripheral_manager_set_desired_connection_latency(
                self.raw,
                latency as i32,
                central.raw,
                &mut error,
            )
        };
        if status == ffi::status::OK {
            Ok(())
        } else {
            Err(from_swift(status, error))
        }
    }

    pub fn add_service(&self, service: &MutableService) -> Result<(), CoreBluetoothError> {
        let mut error = core::ptr::null_mut();
        let status =
            unsafe { ffi::cb_peripheral_manager_add_service(self.raw, service.raw, &mut error) };
        if status == ffi::status::OK {
            Ok(())
        } else {
            Err(from_swift(status, error))
        }
    }

    pub fn remove_service(&self, service: &MutableService) {
        unsafe { ffi::cb_peripheral_manager_remove_service(self.raw, service.raw) };
    }

    pub fn remove_all_services(&self) {
        unsafe { ffi::cb_peripheral_manager_remove_all_services(self.raw) };
    }

    pub fn respond_to_request(
        &self,
        request: &AttRequest,
        result: AttError,
    ) -> Result<(), CoreBluetoothError> {
        let mut error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cb_peripheral_manager_respond_to_request(
                self.raw,
                request.raw,
                result as i32,
                &mut error,
            )
        };
        if status == ffi::status::OK {
            Ok(())
        } else {
            Err(from_swift(status, error))
        }
    }

    pub fn update_value(
        &self,
        value: &[u8],
        characteristic: &MutableCharacteristic,
        centrals: Option<&[&Central]>,
    ) -> Result<bool, CoreBluetoothError> {
        let central_pointers: Vec<*mut c_void> = centrals
            .map(|centrals| centrals.iter().map(|central| central.raw).collect())
            .unwrap_or_default();
        let mut sent = false;
        let mut error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cb_peripheral_manager_update_value(
                self.raw,
                value.as_ptr(),
                value.len(),
                characteristic.raw,
                if central_pointers.is_empty() {
                    core::ptr::null()
                } else {
                    central_pointers.as_ptr()
                },
                central_pointers.len(),
                &mut sent,
                &mut error,
            )
        };
        if status == ffi::status::OK {
            Ok(sent)
        } else {
            Err(from_swift(status, error))
        }
    }

    pub fn publish_l2cap_channel(
        &self,
        encryption_required: bool,
    ) -> Result<(), CoreBluetoothError> {
        let mut error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cb_peripheral_manager_publish_l2cap_channel(
                self.raw,
                encryption_required,
                &mut error,
            )
        };
        if status == ffi::status::OK {
            Ok(())
        } else {
            Err(from_swift(status, error))
        }
    }

    pub fn unpublish_l2cap_channel(&self, psm: u16) -> Result<(), CoreBluetoothError> {
        let mut error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cb_peripheral_manager_unpublish_l2cap_channel(self.raw, psm, &mut error)
        };
        if status == ffi::status::OK {
            Ok(())
        } else {
            Err(from_swift(status, error))
        }
    }
}

impl Drop for PeripheralManager {
    fn drop(&mut self) {
        unsafe { ffi::cb_object_release(self.raw) };
        let _ = self.callback_state.take();
    }
}
