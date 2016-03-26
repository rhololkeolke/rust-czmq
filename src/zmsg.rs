//! Module: czmq-zmsg

use {czmq_sys, ZFrame};
use std::{mem, ptr, result};
use std::ffi::{CStr, CString};
use std::os::raw::c_void;
use zmq;

// Generic error code "-1" doesn't map to an error message, so just
// return an empty tuple.
pub type Result<T> = result::Result<T, ()>;

pub struct ZMsg {
    zmsg: *mut czmq_sys::zmsg_t,
    destroyed: bool,
}

impl Drop for ZMsg {
    fn drop(&mut self) {
        if !self.destroyed {
            unsafe { czmq_sys::zmsg_destroy(&mut self.zmsg) };
        }
    }
}

impl ZMsg {
    pub fn new() -> ZMsg {
        ZMsg {
            zmsg: unsafe { czmq_sys::zmsg_new() },
            destroyed: false,
        }
    }

    pub fn from_raw(zmsg: *mut czmq_sys::zmsg_t) -> ZMsg {
        ZMsg {
            zmsg: zmsg,
            destroyed: false,
        }
    }

    pub fn into_raw(self) -> *mut czmq_sys::zmsg_t {
        let mut msg = self;
        msg.destroyed = true;
        msg.zmsg
    }

    pub fn recv(source: &mut zmq::Socket) -> Result<ZMsg> {
        Self::do_recv(source.borrow_raw())
    }

    pub fn zrecv(source: &ZMsgable) -> Result<ZMsg> {
        Self::do_recv(source.borrow_raw())
    }

    fn do_recv(source: *mut c_void) -> Result<ZMsg> {
        let zmsg = unsafe { czmq_sys::zmsg_recv(source) };

        if zmsg == ptr::null_mut() {
            Err(())
        } else {
            Ok(ZMsg {
                zmsg: zmsg,
                destroyed: false,
            })
        }
    }

    // XXX We'll have to roll our own here as we can't imitate a C
    // file handle without C boilerplate...which we're not doing!
    // pub fn zmsg_load(file: *mut FILE) -> *mut zmsg_t;

    pub fn encode(&self) -> Result<ZFrame> {
        let zframe = unsafe { czmq_sys::zmsg_encode(self.zmsg) };

        if zframe == ptr::null_mut() {
            Err(())
        } else {
            Ok(ZFrame::from_raw(zframe))
        }
    }

    pub fn decode(frame: &ZFrame) -> Result<ZMsg> {
        let zmsg = unsafe { czmq_sys::zmsg_decode(frame.borrow_raw()) };

        if zmsg == ptr::null_mut() {
            Err(())
        } else {
            Ok(ZMsg {
                zmsg: zmsg,
                destroyed: false,
            })
        }
    }

    pub fn new_signal(status: u8) -> Result<ZMsg> {
        let zmsg = unsafe { czmq_sys::zmsg_new_signal(status) };

        if zmsg == ptr::null_mut() {
            Err(())
        } else {
            Ok(ZMsg {
                zmsg: zmsg,
                destroyed: false,
            })
        }
    }

    pub fn send(self, dest: &mut zmq::Socket) -> Result<()> {
        let mut zmsg = self;
        zmsg.do_send(dest.borrow_raw())
    }

    pub fn zsend(self, dest: &mut ZMsgable) -> Result<()> {
        let mut zmsg = self;
        zmsg.do_send(dest.borrow_raw() as *mut c_void)
    }

    fn do_send(&mut self, dest: *mut c_void) -> Result<()> {
        let rc = unsafe { czmq_sys::zmsg_send(&mut self.zmsg, dest) };
        if rc == -1 { Err(()) } else { Ok(()) }
    }

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

    pub fn addstr(&self, string: &str) -> Result<()> {
        let string_c = CString::new(string).unwrap_or(CString::new("").unwrap());
        let rc = unsafe { czmq_sys::zmsg_addstr(self.zmsg, string_c.as_ptr()) };

        // Deliberately leak this memory, which will be managed by C
        mem::forget(string_c);

        if rc == -1 { Err(()) } else { Ok(()) }
    }

    // pub fn zmsg_pushstrf(_self: *mut zmsg_t,
    //                      format: *const ::std::os::raw::c_char, ...)
    //  -> ::std::os::raw::c_int;
    // pub fn zmsg_addstrf(_self: *mut zmsg_t,
    //                     format: *const ::std::os::raw::c_char, ...)
    //  -> ::std::os::raw::c_int;

    pub fn popstr(&self) -> Result<result::Result<String, Vec<u8>>> {
        let ptr = unsafe { czmq_sys::zmsg_popstr(self.zmsg) };

        if ptr == ptr::null_mut() {
            Err(())
        } else {
            let c_string = unsafe { CStr::from_ptr(ptr).to_owned() };
            let bytes = c_string.as_bytes().to_vec();
            match c_string.into_string() {
                Ok(s) => Ok(Ok(s)),
                Err(_) => Ok(Err(bytes))
            }
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
    // pub fn zmsg_dup(_self: *mut zmsg_t) -> *mut zmsg_t;
    // pub fn zmsg_print(_self: *mut zmsg_t);
    // pub fn zmsg_eq(_self: *mut zmsg_t, other: *mut zmsg_t) -> u8;

    pub fn signal(&self) -> Result<u8> {
        let signal = unsafe { czmq_sys::zmsg_signal(self.zmsg) };
        if signal == -1 { Err(()) } else { Ok(signal as u8) }
    }

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

pub trait ZMsgable {
    fn borrow_raw(&self) -> *mut c_void;
}

#[cfg(test)]
mod tests {
    use super::*;
    use {zmq, ZSock, zsys_init};

    #[test]
    fn test_sendrecv() {
        let mut ctx = zmq::Context::new();

        let mut server = ctx.socket(zmq::REP).unwrap();
        server.bind("inproc://test").unwrap();

        let mut client = ctx.socket(zmq::REQ).unwrap();
        client.connect("inproc://test").unwrap();

        let zmsg = ZMsg::new();
        zmsg.addstr("Hello world!").unwrap();
        zmsg.send(&mut client).unwrap();

        let zmsg_recv = ZMsg::recv(&mut server).unwrap();
        assert_eq!(zmsg_recv.popstr().unwrap().unwrap(), "Hello world!");
    }

    #[test]
    fn test_zsendrecv() {
        zsys_init();

        let mut server = ZSock::new_rep("inproc://zmsg_zsendrecv").unwrap();
        let client = ZSock::new_req("inproc://zmsg_zsendrecv").unwrap();

        client.send_str("Hello world!").unwrap();

        let msg = ZMsg::zrecv(&mut server).unwrap();
        assert_eq!(msg.popstr().unwrap().unwrap(), "Hello world!");
    }

    #[test]
    fn test_encode_decode() {
        let msg = ZMsg::new();
        msg.addstr("moo").unwrap();
        let msg_decoded = ZMsg::decode(&msg.encode().unwrap()).unwrap();
        assert_eq!(msg_decoded.popstr().unwrap().unwrap(), "moo");
    }

    #[test]
    fn test_signal() {
        let msg = ZMsg::new_signal(97).unwrap();
        assert_eq!(msg.signal().unwrap(), 97);
    }
}
