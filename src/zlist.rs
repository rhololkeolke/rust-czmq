//! Module: czmq-zlist

use czmq_sys;
use std::ptr;
use std::ffi::CStr;

pub struct ZList<T> {
    zlist: *mut czmq_sys::zlist_t,
    _list_type: Option<T>,
}

impl<T> ZList<T> {
    pub fn from_raw(ptr: *mut czmq_sys::zlist_t) -> ZList<T> {
        ZList {
            zlist: ptr,
            _list_type: None,
        }
    }
}

impl Iterator for ZList<&'static str> {
    type Item = &'static str;

    fn next(&mut self) -> Option<Self::Item> {
        let ptr = unsafe { czmq_sys::zlist_next(self.zlist) };

        if ptr == ptr::null_mut() {
            None
        } else {
            Some(unsafe { CStr::from_ptr(ptr as *const i8) }.to_str().unwrap_or(""))
        }
    }
}
