//! Module: czmq-zmsg

use {czmq_sys, ZSock};
use std::{ptr, result};
use std::ffi::CStr;
use std::os::raw::c_void;
use std::str::Utf8Error;
use zmq;

// Generic error code "-1" doesn't map to an error message, so just
// return an empty tuple.
pub type Result<T> = result::Result<T, ()>;

pub struct ZMsg {
    zmsg: *mut czmq_sys::zmsg_t,
}

impl Drop for ZMsg {
    fn drop(&mut self) {
        unsafe { czmq_sys::zmsg_destroy(&mut self.zmsg) };
    }
}

impl ZMsg {
    pub fn new() -> ZMsg {
        ZMsg {
            zmsg: unsafe { czmq_sys::zmsg_new() },
        }
    }

    pub fn recv(source: &mut zmq::Socket) -> Result<ZMsg> {
        Self::do_recv(source.borrow_raw())
    }

    pub fn zrecv(source: &mut ZSock) -> Result<ZMsg> {
        Self::do_recv(source.borrow_raw() as *mut c_void)
    }

    fn do_recv(source: *mut c_void) -> Result<ZMsg> {
        let zmsg = unsafe { czmq_sys::zmsg_recv(source) };

        if zmsg == ptr::null_mut() {
            return Err(());
        }

        Ok(ZMsg {
            zmsg: zmsg,
        })
    }
    // pub fn zmsg_load(file: *mut FILE) -> *mut zmsg_t;
    // pub fn zmsg_decode(frame: *mut zframe_t) -> *mut zmsg_t;
    // pub fn zmsg_new_signal(status: byte) -> *mut zmsg_t;
    // pub fn zmsg_destroy(self_p: *mut *mut zmsg_t);
    // pub fn zmsg_send(self_p: *mut *mut zmsg_t,
    //                  dest: *mut ::std::os::raw::c_void)
    //  -> ::std::os::raw::c_int;
    // pub fn zmsg_sendm(self_p: *mut *mut zmsg_t,
    //                   dest: *mut ::std::os::raw::c_void)
    //  -> ::std::os::raw::c_int;
    // pub fn zmsg_size(_self: *mut zmsg_t) -> size_t;
    // pub fn zmsg_content_size(_self: *mut zmsg_t) -> size_t;
    // pub fn zmsg_prepend(_self: *mut zmsg_t, frame_p: *mut *mut zframe_t)
    //  -> ::std::os::raw::c_int;
    // pub fn zmsg_append(_self: *mut zmsg_t, frame_p: *mut *mut zframe_t)
    //  -> ::std::os::raw::c_int;
    // pub fn zmsg_pop(_self: *mut zmsg_t) -> *mut zframe_t;
    // pub fn zmsg_pushmem(_self: *mut zmsg_t,
    //                     src: *const ::std::os::raw::c_void, size: size_t)
    //  -> ::std::os::raw::c_int;
    // pub fn zmsg_addmem(_self: *mut zmsg_t, src: *const ::std::os::raw::c_void,
    //                    size: size_t) -> ::std::os::raw::c_int;
    // pub fn zmsg_pushstr(_self: *mut zmsg_t,
    //                     string: *const ::std::os::raw::c_char)
    //  -> ::std::os::raw::c_int;
    // pub fn zmsg_addstr(_self: *mut zmsg_t,
    //                    string: *const ::std::os::raw::c_char)
    //  -> ::std::os::raw::c_int;
    // pub fn zmsg_pushstrf(_self: *mut zmsg_t,
    //                      format: *const ::std::os::raw::c_char, ...)
    //  -> ::std::os::raw::c_int;
    // pub fn zmsg_addstrf(_self: *mut zmsg_t,
    //                     format: *const ::std::os::raw::c_char, ...)
    //  -> ::std::os::raw::c_int;

    pub fn popstr<'a>(&'a self) -> Result<result::Result<&'a str, Utf8Error>> {
        let ptr = unsafe { czmq_sys::zmsg_popstr(self.zmsg) };

        if ptr == ptr::null_mut() {
            Err(())
        } else {
            let c_str = unsafe { CStr::from_ptr(ptr) };
            Ok(c_str.to_str())
        }
    }

    // pub fn zmsg_addmsg(_self: *mut zmsg_t, msg_p: *mut *mut zmsg_t)
    //  -> ::std::os::raw::c_int;
    // pub fn zmsg_popmsg(_self: *mut zmsg_t) -> *mut zmsg_t;
    // pub fn zmsg_remove(_self: *mut zmsg_t, frame: *mut zframe_t);
    // pub fn zmsg_first(_self: *mut zmsg_t) -> *mut zframe_t;
    // pub fn zmsg_next(_self: *mut zmsg_t) -> *mut zframe_t;
    // pub fn zmsg_last(_self: *mut zmsg_t) -> *mut zframe_t;
    // pub fn zmsg_save(_self: *mut zmsg_t, file: *mut FILE)
    //  -> ::std::os::raw::c_int;
    // pub fn zmsg_encode(_self: *mut zmsg_t) -> *mut zframe_t;
    // pub fn zmsg_dup(_self: *mut zmsg_t) -> *mut zmsg_t;
    // pub fn zmsg_print(_self: *mut zmsg_t);
    // pub fn zmsg_eq(_self: *mut zmsg_t, other: *mut zmsg_t) -> u8;
    // pub fn zmsg_signal(_self: *mut zmsg_t) -> ::std::os::raw::c_int;
    // pub fn zmsg_is(_self: *mut ::std::os::raw::c_void) -> u8;
    // pub fn zmsg_test(verbose: u8);
    // pub fn zmsg_unwrap(_self: *mut zmsg_t) -> *mut zframe_t;
    // pub fn zmsg_recv_nowait(source: *mut ::std::os::raw::c_void)
    //  -> *mut zmsg_t;
    // pub fn zmsg_wrap(_self: *mut zmsg_t, frame: *mut zframe_t);
    // pub fn zmsg_push(_self: *mut zmsg_t, frame: *mut zframe_t)
    //  -> ::std::os::raw::c_int;
    // pub fn zmsg_add(_self: *mut zmsg_t, frame: *mut zframe_t)
    //  -> ::std::os::raw::c_int;
    // pub fn zmsg_fprint(_self: *mut zmsg_t, file: *mut FILE);

    pub fn borrow_raw(&self) -> *mut czmq_sys::zmsg_t {
        self.zmsg
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use zmq;

    #[test]
    fn test_recv() {
        let mut ctx = zmq::Context::new();

        let mut server = ctx.socket(zmq::REP).unwrap();
        server.bind("inproc://test").unwrap();

        let mut client = ctx.socket(zmq::REQ).unwrap();
        client.connect("inproc://test").unwrap();

        client.send_str("Hello world!", 0).unwrap();

        let msg = ZMsg::recv(&mut server).unwrap();
        assert_eq!(msg.popstr().unwrap().unwrap(), "Hello world!");
    }

    // #[test]
    // fn test_zrecv() {
    //     let mut server = ZSock::new_rep("inproc://test").unwrap();
    //     let mut client = ZSock::new_req("inproc://test").unwrap();
    //
    //     client.send_str("Hello world!", 0).unwrap();
    //
    //     let msg = ZMsg::zrecv(&mut server).unwrap();
    //     assert_eq!(msg.popstr().unwrap().unwrap(), "Hello world!");
    // }
}
