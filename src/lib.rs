//! Module: czmq
#![doc(html_root_url = "https://petehayes102.github.io/rust-czmq/")]

#[macro_use]
extern crate bitflags;
extern crate czmq_sys;
#[cfg(test)]
extern crate tempdir;
#[cfg(test)]
extern crate tempfile;
extern crate zmq;

mod colander;
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
mod zpoller;
mod zsock;

pub use colander::Colander;
pub use czmq_sys::zcertstore_t as ZCertStoreRaw;
pub use error::{Error, ErrorKind};
pub use zactor::ZActor;
pub use zauth::ZAuth;
pub use zcert::ZCert;
pub use zcertstore::ZCertStore;
pub use zframe::{ZFrame, ZFRAME_MORE, ZFRAME_REUSE, ZFRAME_DONTWAIT};
pub use zhashx::ZHashX;
pub use zlist::ZList;
pub use zmonitor::{ZMonitor, ZMonitorEvents};
pub use zmsg::ZMsg;
pub use zpoller::ZPoller;
pub use zsock::{ZSock, ZSockMechanism, ZSockType};

use std::os::raw::c_void;
use std::result;
use std::sync::{Once, ONCE_INIT};

pub type Result<T> = result::Result<T, Error>;

// Each new ZSock calls zsys_init(), which is a non-threadsafe
// fn. To mitigate the race condition, wrap it in a Once struct.
// Currently this is only necessary in tests, as they are the only
// multithreaded component.
static INIT_ZSYS: Once = ONCE_INIT;

pub fn zsys_init() {
    INIT_ZSYS.call_once(|| {
        unsafe { czmq_sys::zsys_init() };
    });
}

pub trait RawInterface<P> {
    fn from_raw(ptr: *mut P, owned: bool) -> Self;
    fn into_raw(self) -> *mut P;
    fn borrow_raw(&self) -> *mut P;
}

pub trait Sockish : RawInterface<c_void> {}
