//! Module: trait impl for ZMQ Socket

use std::os::raw::c_void;
use zmq::Socket;
use zmsg::ZMsgable;

impl ZMsgable for Socket {
    fn borrow_raw(&self) -> *mut c_void {
        self.borrow_raw()
    }
}
