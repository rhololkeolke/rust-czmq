//! Module: czmq-zactor

use {czmq_sys, Error, ErrorKind, Result, ZMsg, ZSock};
use std::{error, fmt, ptr};
use std::os::raw::c_void;
use zmsg::ZMsgable;

pub struct ZActor {
    zactor: *mut czmq_sys::zactor_t,
}

unsafe impl Send for ZActor {}

impl Drop for ZActor {
    fn drop(&mut self) {
        unsafe { czmq_sys::zactor_destroy(&mut self.zactor) };
    }
}

impl ZActor {
    pub fn new(task: czmq_sys::zactor_fn) -> Result<ZActor> {
        let zactor = unsafe { czmq_sys::zactor_new(task, ptr::null_mut()) };

        if zactor == ptr::null_mut() {
            Err(Error::new(ErrorKind::NullPtr, ZActorError::Instantiate))
        } else {
            Ok(ZActor {
                zactor: zactor,
            })
        }
    }

    pub fn from_raw(zactor: *mut czmq_sys::zactor_t) -> ZActor {
        ZActor {
            zactor: zactor
        }
    }

    pub fn send(&self, msg: ZMsg) -> Result<()> {
        let rc = unsafe { czmq_sys::zactor_send(self.zactor, &mut msg.into_raw()) };
        if rc == -1 {
            Err(Error::new(ErrorKind::NonZero, ZActorError::CmdFailed))
        } else {
            Ok(())
        }
    }

    pub fn send_str(&self, string: &str) -> Result<()> {
        let msg = ZMsg::new();
        try!(msg.addstr(string));
        self.send(msg)
    }

    pub fn recv(&self) -> Result<ZMsg> {
        let zmsg_ptr = unsafe { czmq_sys::zactor_recv(self.zactor) };

        if zmsg_ptr == ptr::null_mut() {
            Err(Error::new(ErrorKind::NullPtr, ZActorError::CmdFailed))
        } else {
            Ok(ZMsg::from_raw(zmsg_ptr))
        }
    }

    pub fn sock(&self) -> ZSock {
        ZSock::from_raw(unsafe { czmq_sys::zactor_sock(self.zactor) }, true)
    }
}

impl ZMsgable for ZActor {
    fn borrow_raw(&self) -> *mut c_void {
        self.zactor as *mut c_void
    }
}

#[derive(Debug)]
pub enum ZActorError {
    Instantiate,
    CmdFailed,
}

impl fmt::Display for ZActorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ZActorError::Instantiate => write!(f, "Could not instantiate new ZActor struct"),
            ZActorError::CmdFailed => write!(f, "ZActor command failed"),
        }
    }
}

impl error::Error for ZActorError {
    fn description(&self) -> &str {
        match *self {
            ZActorError::Instantiate => "Could not instantiate new ZActor struct",
            ZActorError::CmdFailed => "ZActor command failed",
        }
    }
}
