//! Module: trait impl for ZMQ Socket

use {RawInterface, Sockish};
use std::os::raw::c_void;
use zmq::Socket;

impl RawInterface<c_void> for Socket {
    unsafe fn from_raw(ptr: *mut c_void, _owned: bool) -> Socket {
        Socket::from_raw(ptr)
    }

    fn into_raw(self) -> *mut c_void {
        self.into_raw()
    }

    fn as_mut_ptr(&mut self) -> *mut c_void {
        self.as_mut_ptr()
    }
}

impl Sockish for Socket {}
