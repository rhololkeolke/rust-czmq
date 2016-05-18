//! Module: czmq-zpoller

use {czmq_sys, Error, ErrorKind, Result, Sockish};
use std::{error, fmt, ptr};
use std::os::raw::c_int;

pub struct ZPoller {
    zpoller: *mut czmq_sys::zpoller_t,
}

unsafe impl Send for ZPoller {}

impl Drop for ZPoller {
    fn drop(&mut self) {
        unsafe { czmq_sys::zpoller_destroy(&mut self.zpoller) };
    }
}

impl ZPoller {
    pub fn new() -> Result<ZPoller> {
        // zpoller_new() can take one or more readers, though Rust
        // doesn't support variadic fns except through macros.
        let zpoller = unsafe { czmq_sys::zpoller_new(ptr::null_mut()) };

        if zpoller == ptr::null_mut() {
            return Err(Error::new(ErrorKind::NullPtr, ZPollerError::Instantiate));
        }

        Ok(ZPoller {
            zpoller: zpoller,
        })
    }

    pub fn add<S: Sockish>(&mut self, reader: &S) -> Result<()> {
        let rc = unsafe { czmq_sys::zpoller_add(self.zpoller, reader.borrow_raw()) };

        if rc == -1 {
            Err(Error::new(ErrorKind::NonZero, ZPollerError::CmdFailed))
        } else {
            Ok(())
        }
    }

    pub fn remove<S: Sockish>(&mut self, reader: &S) -> Result<()> {
        let rc = unsafe { czmq_sys::zpoller_remove(self.zpoller, reader.borrow_raw()) };

        if rc == -1 {
            Err(Error::new(ErrorKind::NonZero, ZPollerError::CmdFailed))
        } else {
            Ok(())
        }
    }

    pub fn wait<S: Sockish>(&self, timeout: Option<u32>) -> Result<Option<S>> {
        let t = match timeout {
            Some(time) => time as c_int,
            None => -1 as c_int,
        };

        let ptr = unsafe { czmq_sys::zpoller_wait(self.zpoller, t) };

        if ptr == ptr::null_mut() {
            Ok(None)
        } else {
            Ok(Some(S::from_raw(ptr, false)))
        }
    }

    pub fn expired(&self) -> bool {
        unsafe { czmq_sys::zpoller_expired(self.zpoller) == 1 }
    }

    pub fn terminated(&self) -> bool {
        unsafe { czmq_sys::zpoller_terminated(self.zpoller) == 1 }
    }

    pub fn set_nonstop(&self, nonstop: bool) {
        unsafe { czmq_sys::zpoller_set_nonstop(self.zpoller, if nonstop { 1 } else { 0 }) }
    }
}

#[derive(Debug)]
pub enum ZPollerError {
    CmdFailed,
    Instantiate,
}

impl fmt::Display for ZPollerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ZPollerError::CmdFailed => write!(f, "ZPoller command failed"),
            ZPollerError::Instantiate => write!(f, "Could not instantiate new ZPoller struct"),
        }
    }
}

impl error::Error for ZPollerError {
    fn description(&self) -> &str {
        match *self {
            ZPollerError::CmdFailed => "ZPoller command failed",
            ZPollerError::Instantiate => "Could not instantiate new ZPoller struct",
        }
    }
}

#[cfg(test)]
mod tests {
    use {ZSock, ZSockType, zsys_init};
    use super::*;

    #[test]
    fn test_new() {
        assert!(ZPoller::new().is_ok());
    }

    #[test]
    fn test_add_remove() {
        zsys_init();

        let sock = ZSock::new(ZSockType::PAIR);
        let mut poller = ZPoller::new().unwrap();
        assert!(poller.add(&sock).is_ok());
        assert!(poller.remove(&sock).is_ok());
    }

    #[test]
    fn test_wait() {
        zsys_init();

        let server1 = ZSock::new_rep("inproc://zpoller_test_wait1").unwrap();
        let client1 = ZSock::new_req("inproc://zpoller_test_wait1").unwrap();
        client1.send_str("moo").unwrap();

        let server2 = ZSock::new_rep("inproc://zpoller_test_wait2").unwrap();
        let client2 = ZSock::new_req("inproc://zpoller_test_wait2").unwrap();
        client2.send_str("cow").unwrap();

        let mut poller = ZPoller::new().unwrap();
        poller.add(&server1).unwrap();
        poller.add(&server2).unwrap();

        let sock: ZSock = poller.wait(Some(500)).unwrap().unwrap();
        assert_eq!(sock.endpoint().unwrap(), "inproc://zpoller_test_wait1");
        server1.recv_str().unwrap().unwrap();

        let sock: ZSock = poller.wait(Some(500)).unwrap().unwrap();
        assert_eq!(sock.endpoint().unwrap(), "inproc://zpoller_test_wait2");
        server2.recv_str().unwrap().unwrap();

        let sock: Option<ZSock> = poller.wait(Some(0)).unwrap();
        assert!(sock.is_none());
        assert!(poller.expired());
        assert!(!poller.terminated());
    }
}
