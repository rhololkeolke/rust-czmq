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
mod zframe;
mod zlist;
mod zmonitor;
mod zmsg;
mod zsock;

pub use error::{Error, ErrorKind};
pub use zactor::ZActor;
pub use zauth::ZAuth;
pub use zcert::ZCert;
pub use zframe::{ZFrame, ZFRAME_MORE, ZFRAME_REUSE, ZFRAME_DONTWAIT};
pub use zlist::ZList;
pub use zmonitor::{ZMonitor, ZMonitorEvents};
pub use zmsg::ZMsg;
pub use zsock::ZSock;

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
