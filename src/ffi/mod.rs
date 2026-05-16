#![allow(missing_docs)]

use core::ffi::{c_char, c_void};

pub type CentralManagerEventCallback =
    unsafe extern "C" fn(user_info: *mut c_void, payload_json: *const c_char);
pub type PeripheralEventCallback =
    unsafe extern "C" fn(user_info: *mut c_void, payload_json: *const c_char);

extern "C" {
    pub fn cb_object_release(ptr: *mut c_void);
    pub fn cb_pointer_array_free(array: *mut c_void, count: usize);

    pub fn cb_manager_new(
        options_json: *const c_char,
        callback: Option<CentralManagerEventCallback>,
        user_info: *mut c_void,
        out_manager: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn cb_manager_state(manager: *mut c_void) -> i32;
    pub fn cb_manager_authorization(manager: *mut c_void) -> i32;
    pub fn cb_manager_is_scanning(manager: *mut c_void) -> bool;
    pub fn cb_manager_scan_for_peripherals(
        manager: *mut c_void,
        service_uuids_json: *const c_char,
        allow_duplicates: bool,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn cb_manager_stop_scan(manager: *mut c_void);
    pub fn cb_manager_connect(
        manager: *mut c_void,
        peripheral: *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn cb_manager_cancel_peripheral_connection(manager: *mut c_void, peripheral: *mut c_void);
    pub fn cb_manager_retrieve_connected_peripherals(
        manager: *mut c_void,
        service_uuids_json: *const c_char,
        out_array: *mut *mut c_void,
        out_count: *mut usize,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn cb_manager_retrieve_peripherals_with_identifiers(
        manager: *mut c_void,
        identifiers_json: *const c_char,
        out_array: *mut *mut c_void,
        out_count: *mut usize,
        error_out: *mut *mut c_char,
    ) -> i32;

    pub fn cb_peripheral_set_delegate(
        peripheral: *mut c_void,
        callback: Option<PeripheralEventCallback>,
        user_info: *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn cb_peripheral_clear_delegate(peripheral: *mut c_void);
    pub fn cb_peripheral_name(peripheral: *mut c_void) -> *mut c_char;
    pub fn cb_peripheral_identifier(peripheral: *mut c_void) -> *mut c_char;
    pub fn cb_peripheral_state(peripheral: *mut c_void) -> i32;
    pub fn cb_peripheral_services(
        peripheral: *mut c_void,
        out_array: *mut *mut c_void,
        out_count: *mut usize,
    );
    pub fn cb_peripheral_discover_services(
        peripheral: *mut c_void,
        service_uuids_json: *const c_char,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn cb_peripheral_read_rssi(peripheral: *mut c_void, error_out: *mut *mut c_char) -> i32;
    pub fn cb_peripheral_discover_characteristics(
        peripheral: *mut c_void,
        service: *mut c_void,
        characteristic_uuids_json: *const c_char,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn cb_peripheral_read_value_for_characteristic(
        peripheral: *mut c_void,
        characteristic: *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn cb_peripheral_write_value_for_characteristic(
        peripheral: *mut c_void,
        characteristic: *mut c_void,
        bytes: *const u8,
        length: usize,
        with_response: bool,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn cb_peripheral_set_notify_value(
        peripheral: *mut c_void,
        characteristic: *mut c_void,
        enabled: bool,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn cb_peripheral_discover_descriptors(
        peripheral: *mut c_void,
        characteristic: *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;

    pub fn cb_service_uuid(service: *mut c_void) -> *mut c_char;
    pub fn cb_service_is_primary(service: *mut c_void) -> bool;
    pub fn cb_service_included_services(
        service: *mut c_void,
        out_array: *mut *mut c_void,
        out_count: *mut usize,
    );
    pub fn cb_service_characteristics(
        service: *mut c_void,
        out_array: *mut *mut c_void,
        out_count: *mut usize,
    );

    pub fn cb_characteristic_uuid(characteristic: *mut c_void) -> *mut c_char;
    pub fn cb_characteristic_properties(characteristic: *mut c_void) -> u64;
    pub fn cb_characteristic_value_json(characteristic: *mut c_void) -> *mut c_char;
    pub fn cb_characteristic_is_notifying(characteristic: *mut c_void) -> bool;
    pub fn cb_characteristic_descriptors(
        characteristic: *mut c_void,
        out_array: *mut *mut c_void,
        out_count: *mut usize,
    );

    pub fn cb_descriptor_uuid(descriptor: *mut c_void) -> *mut c_char;
    pub fn cb_descriptor_value_json(descriptor: *mut c_void) -> *mut c_char;
}

pub mod status {
    pub const OK: i32 = 0;
    pub const INVALID_ARGUMENT: i32 = -1;
    pub const FRAMEWORK_ERROR: i32 = -2;
    pub const UNKNOWN: i32 = -99;
}
