//! Module: trait impl for ZMQ Socket

use {RawInterface, Sockish};
use std::os::raw::c_void;
use zmq::Socket;

impl RawInterface<c_void> for Socket {
    fn from_raw(ptr: *mut c_void, _owned: bool) -> Socket {
        Socket::from_raw(ptr)
    }

    fn into_raw(self) -> *mut c_void {
        self.to_raw()
    }

    fn borrow_raw(&self) -> *mut c_void {
        self.borrow_raw()
    }
}

impl Sockish for Socket {}
