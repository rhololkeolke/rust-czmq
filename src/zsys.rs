//! Module: czmq-zsys

use {czmq_sys, RawInterface, Result};
use error::{Error, ErrorKind};
use std::{error, fmt, ptr};
use std::os::raw::c_void;
use std::sync::{Once, ONCE_INIT};
use zsock::ZSock;

static INIT_ZSYS: Once = ONCE_INIT;

pub struct ZSys;

impl ZSys {
    // Each new ZSock calls zsys_init(), which is a non-threadsafe
    // fn. To mitigate the race condition, wrap it in a Once struct.
    pub fn init() {
        INIT_ZSYS.call_once(|| {
            unsafe { czmq_sys::zsys_init() };
        });
    }

    /// Create a pipe, which consists of two PAIR sockets connected
    /// over inproc.
    pub fn create_pipe() -> Result<(ZSock, ZSock)> {
        let mut backend_raw: *mut czmq_sys::zsock_t = ptr::null_mut();
        let frontend_raw = unsafe { czmq_sys::zsys_create_pipe(&mut backend_raw) };

        if frontend_raw == ptr::null_mut() {
            Err(Error::new(ErrorKind::NullPtr, ZSysError::CreatePipe))
        } else {
            Ok((ZSock::from_raw(frontend_raw as *mut c_void, true), ZSock::from_raw(backend_raw as *mut c_void, true)))
        }
    }
}

#[derive(Debug)]
pub enum ZSysError {
    CreatePipe,
}

impl fmt::Display for ZSysError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ZSysError::CreatePipe => write!(f, "Could not create pipe"),
        }
    }
}

impl error::Error for ZSysError {
    fn description(&self) -> &str {
        match *self {
            ZSysError::CreatePipe => "Could not create pipe",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_pipe() {
        ZSys::init();

        let (frontend, backend) = ZSys::create_pipe().unwrap();
        frontend.send_str("I changed my iPod’s name to Titanic...now it’s syncing.").unwrap();
        assert_eq!(backend.recv_str().unwrap().unwrap(), "I changed my iPod’s name to Titanic...now it’s syncing.");
        backend.send_str("My family laughed when I told them I was going to be a comedian. Well...they aren't laughing now!").unwrap();
        assert_eq!(frontend.recv_str().unwrap().unwrap(), "My family laughed when I told them I was going to be a comedian. Well...they aren't laughing now!");
    }
}
