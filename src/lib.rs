//! Module: czmq

#[macro_use]
extern crate bitflags;
extern crate czmq_sys;
extern crate zmq;

// mod zactor;
// mod zauth;
mod zcert;
mod zframe;
mod zlist;
mod zmsg;
mod zsock;
// mod zstr;

// pub use zactor::ZActor;
// pub use zauth::ZAuth;
pub use zcert::ZCert;
pub use zframe::{ZFrame, ZFRAME_MORE, ZFRAME_REUSE, ZFRAME_DONTWAIT};
pub use zlist::ZList;
pub use zmsg::ZMsg;
pub use zsock::ZSock;
// pub use zstr::ZStr;

// Each new ZSock calls zsys_init(), which is a non-threadsafe
// fn. To mitigate the race condition, wrap it in a Once struct.
// Currently this is only necessary in tests, as they are the only
// multithreaded component.
#[cfg(test)]
use std::sync::{Once, ONCE_INIT};

#[cfg(test)]
static INIT_ZSYS: Once = ONCE_INIT;

#[cfg(test)]
fn zsys_init() {
    INIT_ZSYS.call_once(|| {
        unsafe { czmq_sys::zsys_init() };
    });
}
