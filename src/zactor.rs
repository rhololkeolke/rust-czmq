//! Module: czmq-zactor

use {czmq_sys, ZMsg, ZSock};
use std::{ptr, result};

// Generic error code "-1" doesn't map to an error message, so just
// return an empty tuple.
pub type Result<T> = result::Result<T, ()>;

pub struct ZActor {
    zactor: *mut czmq_sys::zactor_t,
}

impl Drop for ZActor {
    fn drop(&mut self) {
        unsafe { czmq_sys::zactor_destroy(&mut self.zactor) };
    }
}

impl ZActor {
    // @todo Missing `args` argument
    pub fn new(task: czmq_sys::zactor_fn) -> Result<ZActor> {
        let zactor = unsafe { czmq_sys::zactor_new(task, ptr::null_mut()) };

        if zactor == ptr::null_mut() {
            Err(())
        } else {
            Ok(ZActor {
                zactor: zactor,
            })
        }
    }

    pub fn send(&self, msg: &mut ZMsg) -> Result<()> {
        let rc = unsafe { czmq_sys::zactor_send(self.zactor, &mut msg.borrow_raw()) };
        if rc == -1 { Err(()) } else { Ok(()) }
    }

    pub fn send_str(&self, string: &str) -> Result<()> {
        unimplemented!();
    }

    pub fn recv(&self) -> Result<ZMsg> {
        let zmsg_ptr = unsafe { czmq_sys::zactor_recv(self.zactor) };

        if zmsg_ptr == ptr::null_mut() {
            Err(())
        } else {
            Ok(ZMsg::from_raw(zmsg_ptr))
        }
    }

    pub fn sock(&self) -> ZSock {
        ZSock::from_raw(unsafe { czmq_sys::zactor_sock(self.zactor) })
    }
}
