//! Module: czmq

#[macro_use]
extern crate bitflags;
extern crate czmq_sys;
#[cfg(test)]
extern crate tempdir;
#[cfg(test)]
extern crate tempfile;
extern crate zmq;

mod error;
mod socket;
mod zactor;
mod zauth;
mod zcert;
mod zcertstore;
mod zframe;
mod zhashx;
mod zlist;
mod zmonitor;
mod zmsg;
mod zsock;

pub use error::{Error, ErrorKind};
pub use czmq_sys::zcertstore_t as ZCertStoreRaw;
pub use zactor::ZActor;
pub use zauth::ZAuth;
pub use zcert::ZCert;
pub use zcertstore::ZCertStore;
pub use zframe::{ZFrame, ZFRAME_MORE, ZFRAME_REUSE, ZFRAME_DONTWAIT};
pub use zhashx::ZHashX;
pub use zlist::ZList;
pub use zmonitor::{ZMonitor, ZMonitorEvents};
pub use zmsg::ZMsg;
pub use zsock::{ZSock, ZSockMechanism, ZSockType};

use std::result;
pub type Result<T> = result::Result<T, Error>;

// Each new ZSock calls zsys_init(), which is a non-threadsafe
// fn. To mitigate the race condition, wrap it in a Once struct.
// Currently this is only necessary in tests, as they are the only
// multithreaded component.
use std::sync::{Once, ONCE_INIT};

static INIT_ZSYS: Once = ONCE_INIT;

pub fn zsys_init() {
    INIT_ZSYS.call_once(|| {
        unsafe { czmq_sys::zsys_init() };
    });
}

// Wrapper around Box type that deliberately leaks its memory instead
// of destroying it. This is useful for reading borrowed void ptrs
// whose memory is managed by C.
use std::borrow::{Borrow, BorrowMut};
use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::ops::{Deref, DerefMut};
use std::ptr;

pub struct Colander<T> {
    inner: Option<Box<T>>,
}

impl<T> Colander<T> {
    unsafe fn from_raw(raw: *mut T) -> Colander<T> {
        Colander {
            inner: Some(Box::from_raw(raw)),
        }
    }

    pub fn into_raw(mut self) -> *mut T {
        if let Some(b) = self.inner.take() {
            Box::into_raw(b)
        } else {
            ptr::null_mut()
        }
    }

    pub fn into_inner(mut self) -> Option<Box<T>> {
        self.inner.take()
    }
}

impl<T> AsRef<T> for Colander<T> {
    fn as_ref(&self) -> &T {
        self.inner.as_ref().unwrap().as_ref()
    }
}

impl<T> AsMut<T> for Colander<T> {
    fn as_mut(&mut self) -> &mut T {
        self.inner.as_mut().unwrap().as_mut()
    }
}

impl<T> Borrow<T> for Colander<T> {
    fn borrow(&self) -> &T {
        self.inner.as_ref().unwrap().borrow()
    }
}

impl<T> BorrowMut<T> for Colander<T> {
    fn borrow_mut(&mut self) -> &mut T {
        self.inner.as_mut().unwrap().borrow_mut()
    }
}

impl<T> Debug for Colander<T> where T: Debug {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.inner.as_ref().unwrap().fmt(f)
    }
}

impl<T> Deref for Colander<T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.inner.as_ref().unwrap().deref()
    }
}

impl<T> DerefMut for Colander<T> {
    fn deref_mut(&mut self) -> &mut T {
        self.inner.as_mut().unwrap().deref_mut()
    }
}

impl<T> Drop for Colander<T> {
    fn drop(&mut self) {
        if let Some(b) = self.inner.take() {
            Box::into_raw(b);
        }
    }
}
