//! Module: czmq-zactor

use {czmq_sys, ZMsg};
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
    pub fn new(task: czmq_sys::zactor_fn) -> ZActor {
        ZActor {
            zactor: unsafe { czmq_sys::zactor_new(task, ptr::null_mut()) },
        }
    }

    pub fn send(&self, msg: &mut ZMsg) -> Result<()> {
        let rc = unsafe { czmq_sys::zactor_send(self.zactor, &mut msg.borrow_raw()) };

        // XXX Look up error message
        if rc == -1i32 { Err(()) } else { Ok(()) }
    }

    // pub fn recv(_self: *mut zactor_t) -> *mut zmsg_t;
    pub fn recv(&self) -> ZMsg {
        let zmsg_ptr = unsafe { czmq_sys::zactor_recv(self.zactor) };
    }

    // pub fn is(_self: *mut ::std::os::raw::c_void) -> u8;
    // pub fn resolve(_self: *mut ::std::os::raw::c_void)
    //  -> *mut ::std::os::raw::c_void;
    // pub fn sock(_self: *mut zactor_t) -> *mut zsock_t;
}

#[cfg(test)]
mod tests {
    use czmq_sys;
    use super::*;

    // #[test]
    // fn test_() {
    //     let zauth = create_zauth();
    //     zauth.allow("127.0.0.1");
    // }
    //
    // fn create_zauth() -> ZAuth {
    //     let ctx = ZCtx::new();
    //     ZAuth::new(&ctx).unwrap()
    // }
}
