use core::ffi::c_void;

use crate::error::take_owned_c_string;
use crate::ffi;
use crate::private::{retain_raw, retained_handle_to_raw};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum StreamStatus {
    NotOpen = 0,
    Opening = 1,
    Open = 2,
    Reading = 3,
    Writing = 4,
    AtEnd = 5,
    Closed = 6,
    Error = 7,
}

impl StreamStatus {
    pub const fn from_raw(raw: i32) -> Self {
        match raw {
            1 => Self::Opening,
            2 => Self::Open,
            3 => Self::Reading,
            4 => Self::Writing,
            5 => Self::AtEnd,
            6 => Self::Closed,
            7 => Self::Error,
            _ => Self::NotOpen,
        }
    }
}

pub struct Peer {
    raw: *mut c_void,
}

impl Peer {
    pub(crate) fn from_retained_raw(raw: *mut c_void) -> Self {
        Self { raw }
    }

    pub fn identifier(&self) -> String {
        take_owned_c_string(unsafe { ffi::cb_peer_identifier(self.raw) })
    }
}

impl Clone for Peer {
    fn clone(&self) -> Self {
        Self::from_retained_raw(retain_raw(self.raw))
    }
}

impl Drop for Peer {
    fn drop(&mut self) {
        unsafe { ffi::cb_object_release(self.raw) };
    }
}

pub struct InputStreamHandle {
    raw: *mut c_void,
}

impl InputStreamHandle {
    pub(crate) fn from_retained_raw(raw: *mut c_void) -> Self {
        Self { raw }
    }

    pub fn status(&self) -> StreamStatus {
        StreamStatus::from_raw(unsafe { ffi::cb_stream_status(self.raw) })
    }

    pub fn has_bytes_available(&self) -> bool {
        unsafe { ffi::cb_input_stream_has_bytes_available(self.raw) }
    }

    pub fn open(&self) {
        unsafe { ffi::cb_stream_open(self.raw) };
    }

    pub fn close(&self) {
        unsafe { ffi::cb_stream_close(self.raw) };
    }
}

impl Clone for InputStreamHandle {
    fn clone(&self) -> Self {
        Self::from_retained_raw(retain_raw(self.raw))
    }
}

impl Drop for InputStreamHandle {
    fn drop(&mut self) {
        unsafe { ffi::cb_object_release(self.raw) };
    }
}

pub struct OutputStreamHandle {
    raw: *mut c_void,
}

impl OutputStreamHandle {
    pub(crate) fn from_retained_raw(raw: *mut c_void) -> Self {
        Self { raw }
    }

    pub fn status(&self) -> StreamStatus {
        StreamStatus::from_raw(unsafe { ffi::cb_stream_status(self.raw) })
    }

    pub fn has_space_available(&self) -> bool {
        unsafe { ffi::cb_output_stream_has_space_available(self.raw) }
    }

    pub fn open(&self) {
        unsafe { ffi::cb_stream_open(self.raw) };
    }

    pub fn close(&self) {
        unsafe { ffi::cb_stream_close(self.raw) };
    }
}

impl Clone for OutputStreamHandle {
    fn clone(&self) -> Self {
        Self::from_retained_raw(retain_raw(self.raw))
    }
}

impl Drop for OutputStreamHandle {
    fn drop(&mut self) {
        unsafe { ffi::cb_object_release(self.raw) };
    }
}

pub struct L2capChannel {
    raw: *mut c_void,
}

impl L2capChannel {
    pub(crate) fn from_retained_raw(raw: *mut c_void) -> Self {
        Self { raw }
    }

    pub(crate) fn from_retained_handle(handle: u64) -> Self {
        Self::from_retained_raw(retained_handle_to_raw(handle))
    }

    pub fn peer(&self) -> Peer {
        Peer::from_retained_raw(unsafe { ffi::cb_l2cap_channel_peer(self.raw) })
    }

    pub fn psm(&self) -> u16 {
        unsafe { ffi::cb_l2cap_channel_psm(self.raw) }
    }

    pub fn input_stream(&self) -> InputStreamHandle {
        InputStreamHandle::from_retained_raw(unsafe { ffi::cb_l2cap_channel_input_stream(self.raw) })
    }

    pub fn output_stream(&self) -> OutputStreamHandle {
        OutputStreamHandle::from_retained_raw(unsafe { ffi::cb_l2cap_channel_output_stream(self.raw) })
    }
}

impl Clone for L2capChannel {
    fn clone(&self) -> Self {
        Self::from_retained_raw(retain_raw(self.raw))
    }
}

impl Drop for L2capChannel {
    fn drop(&mut self) {
        unsafe { ffi::cb_object_release(self.raw) };
    }
}
