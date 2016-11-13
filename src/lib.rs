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
mod zsys;

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
pub use zmq::{Mechanism, SocketType};
pub use zmsg::ZMsg;
pub use zpoller::ZPoller;
pub use zsock::ZSock;
pub use zsys::ZSys;

use std::os::raw::c_void;
use std::result;

pub type Result<T> = result::Result<T, Error>;

pub trait RawInterface<P> {
    unsafe fn from_raw(ptr: *mut P, owned: bool) -> Self;
    fn into_raw(self) -> *mut P;
    fn as_mut_ptr(&mut self) -> *mut P;
}

pub trait Sockish : RawInterface<c_void> {}
