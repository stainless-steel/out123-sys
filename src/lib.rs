//! Bindings to [out123][1].
//!
//! [1]: https://www.mpg123.de/

#![allow(non_camel_case_types)]

extern crate libc;
extern crate mpg123_sys;

use libc::{c_char, c_double, c_int, c_long, c_void, size_t};
use mpg123_sys::mpg123_fmt;

pub enum out123_handle {}

#[derive(Clone, Copy)]
#[repr(C)]
pub enum out123_flags {
    OUT123_HEADPHONES = 0x01,
    OUT123_INTERNAL_SPEAKER = 0x02,
    OUT123_LINE_OUT = 0x04,
    OUT123_QUIET = 0x08,
    OUT123_KEEP_PLAYING = 0x10,
}
pub use out123_flags::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub enum out123_error {
    OUT123_ERR = -1,
    OUT123_OK = 0,
    OUT123_DOOM,
    OUT123_BAD_DRIVER_NAME,
    OUT123_BAD_DRIVER,
    OUT123_NO_DRIVER,
    OUT123_NOT_LIVE,
    OUT123_DEV_PLAY,
    OUT123_DEV_OPEN,
    OUT123_BUFFER_ERROR,
    OUT123_MODULE_ERROR,
    OUT123_ARG_ERROR,
    OUT123_BAD_PARAM,
    OUT123_SET_RO_PARAM,
    OUT123_BAD_HANDLE,
    OUT123_ERRCOUNT,
}
pub use out123_error::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub enum out123_parms {
    OUT123_FLAGS = 1,
    OUT123_PRELOAD,
    OUT123_GAIN,
    OUT123_VERBOSE,
    OUT123_DEVICEBUFFER,
    OUT123_PROPFLAGS,
    OUT123_NAME,
}
pub use out123_parms::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub enum out123_propflags {
    OUT123_PROP_LIVE = 0x01,
    OUT123_PROP_PERSISTENT = 0x02,
}
pub use out123_propflags::*;

extern {
    pub fn out123_new() -> *mut out123_handle;
    pub fn out123_del(ao: *mut out123_handle);
    pub fn out123_strerror(ao: *mut out123_handle) -> *const c_char;
    pub fn out123_errcode(ao: *mut out123_handle) -> c_int;
    pub fn out123_plain_strerror(errcode: c_int) -> *const c_char;
    pub fn out123_set_buffer(ao: *mut out123_handle, buffer_bytes: size_t) -> c_int;
    pub fn out123_param(ao: *mut out123_handle,
                        code: out123_parms,
                        value: c_long,
                        fvalue: c_double,
                        svalue: *const c_char)
                        -> c_int;
    pub fn out123_getparam(ao: *mut out123_handle,
                           code: out123_parms,
                           ret_value: *mut c_long,
                           ret_fvalue: *mut c_double,
                           ret_svalue: *mut *mut c_char)
                           -> c_int;
    pub fn out123_param_from(ao: *mut out123_handle, from_ao: *mut out123_handle) -> c_int;
    pub fn out123_drivers(ao: *mut out123_handle,
                          names: *mut *mut *mut c_char,
                          descr: *mut *mut *mut c_char)
                          -> c_int;
    pub fn out123_open(ao: *mut out123_handle,
                       driver: *const c_char,
                       device: *const c_char)
                       -> c_int;
    pub fn out123_driver_info(ao: *mut out123_handle,
                              driver: *mut *mut c_char,
                              device: *mut *mut c_char)
                              -> c_int;
    pub fn out123_close(ao: *mut out123_handle);
    pub fn out123_encodings(ao: *mut out123_handle, rate: c_long, channels: c_int) -> c_int;
    pub fn out123_encsize(encoding: c_int) -> c_int;
    pub fn out123_formats(ao: *mut out123_handle,
                          rates: *const c_long,
                          ratecount: c_int,
                          minchannels: c_int,
                          maxchannels: c_int,
                          fmtlist: *mut *mut mpg123_fmt)
                          -> c_int;
    pub fn out123_enc_list(enclist: *mut *mut c_int) -> c_int;
    pub fn out123_enc_byname(name: *const c_char) -> c_int;
    pub fn out123_enc_name(encoding: c_int) -> *const c_char;
    pub fn out123_enc_longname(encoding: c_int) -> *const c_char;
    pub fn out123_start(ao: *mut out123_handle,
                        rate: c_long,
                        channels: c_int,
                        encoding: c_int)
                        -> c_int;
    pub fn out123_pause(ao: *mut out123_handle);
    pub fn out123_continue(ao: *mut out123_handle);
    pub fn out123_stop(ao: *mut out123_handle);
    pub fn out123_play(ao: *mut out123_handle, buffer: *mut c_void, bytes: size_t) -> size_t;
    pub fn out123_drop(ao: *mut out123_handle);
    pub fn out123_drain(ao: *mut out123_handle);
    pub fn out123_ndrain(ao: *mut out123_handle, bytes: size_t);
    pub fn out123_buffered(ao: *mut out123_handle) -> size_t;
    pub fn out123_getformat(ao: *mut out123_handle,
                            rate: *mut c_long,
                            channels: *mut c_int,
                            encoding: *mut c_int,
                            framesize: *mut c_int)
                            -> c_int;
}
