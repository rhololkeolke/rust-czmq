//! Module: czmq-zmsg

use {czmq_sys, Error, ErrorKind, RawInterface, Result, Sockish, ZFrame};
use std::{error, fmt, mem, ptr, result};
use std::ffi::{CStr, CString};

pub struct ZMsg {
    zmsg: *mut czmq_sys::zmsg_t,
    owned: bool,
}

unsafe impl Send for ZMsg {}
unsafe impl Sync for ZMsg {}

impl Drop for ZMsg {
    fn drop(&mut self) {
        if self.owned {
            unsafe { czmq_sys::zmsg_destroy(&mut self.zmsg) };
        }
    }
}

impl ZMsg {
    pub fn new() -> ZMsg {
        ZMsg {
            zmsg: unsafe { czmq_sys::zmsg_new() },
            owned: true,
        }
    }

    pub fn recv<S: Sockish>(source: &S) -> Result<ZMsg> {
        let zmsg = unsafe { czmq_sys::zmsg_recv(source.borrow_raw()) };

        if zmsg == ptr::null_mut() {
            Err(Error::new(ErrorKind::NullPtr, ZMsgError::CmdFailed))
        } else {
            Ok(ZMsg {
                zmsg: zmsg,
                owned: true,
            })
        }
    }

    // XXX We'll have to roll our own here as we can't imitate a C
    // file handle without C boilerplate...which we're not doing!
    // pub fn zmsg_load(file: *mut FILE) -> *mut zmsg_t;

    pub fn encode(&self) -> Result<ZFrame> {
        let zframe = unsafe { czmq_sys::zmsg_encode(self.zmsg) };

        if zframe == ptr::null_mut() {
            Err(Error::new(ErrorKind::NullPtr, ZMsgError::CmdFailed))
        } else {
            Ok(ZFrame::from_raw(zframe, true))
        }
    }

    pub fn decode(frame: &ZFrame) -> Result<ZMsg> {
        let zmsg = unsafe { czmq_sys::zmsg_decode(frame.borrow_raw()) };

        if zmsg == ptr::null_mut() {
            Err(Error::new(ErrorKind::NullPtr, ZMsgError::CmdFailed))
        } else {
            Ok(ZMsg {
                zmsg: zmsg,
                owned: true,
            })
        }
    }

    pub fn new_signal(status: u8) -> Result<ZMsg> {
        let zmsg = unsafe { czmq_sys::zmsg_new_signal(status) };

        if zmsg == ptr::null_mut() {
            Err(Error::new(ErrorKind::NullPtr, ZMsgError::CmdFailed))
        } else {
            Ok(ZMsg {
                zmsg: zmsg,
                owned: true,
            })
        }
    }

    pub fn send<D: Sockish>(self, dest: &D) -> Result<()> {
        let mut zmsg = self;

        let rc = unsafe { czmq_sys::zmsg_send(&mut zmsg.zmsg, dest.borrow_raw()) };
        if rc == -1 {
            Err(Error::new(ErrorKind::NonZero, ZMsgError::CmdFailed))
        } else {
            Ok(())
        }
    }

    // pub fn zmsg_sendm(self_p: *mut *mut zmsg_t,
    //                   dest: *mut ::std::os::raw::c_void)
    //  -> ::std::os::raw::c_int;

    pub fn size(&self) -> usize {
        unsafe { czmq_sys::zmsg_size(self.zmsg) as usize }
    }

    // pub fn zmsg_content_size(_self: *mut zmsg_t) -> size_t;

    pub fn prepend(&self, frame: ZFrame) -> Result<()> {
        let rc = unsafe { czmq_sys::zmsg_prepend(self.zmsg, &mut frame.into_raw()) };

        if rc == 0 {
            Ok(())
        } else {
            Err(Error::new(ErrorKind::NonZero, ZMsgError::CmdFailed))
        }
    }

    pub fn append(&self, frame: ZFrame) -> Result<()> {
        let rc = unsafe { czmq_sys::zmsg_append(self.zmsg, &mut frame.into_raw()) };

        if rc == 0 {
            Ok(())
        } else {
            Err(Error::new(ErrorKind::NonZero, ZMsgError::CmdFailed))
        }
    }

    pub fn pop(&self) -> Option<ZFrame> {
        let ptr = unsafe { czmq_sys::zmsg_pop(self.zmsg) };

        if ptr == ptr::null_mut() {
            None
        } else {
            Some(ZFrame::from_raw(ptr, true))
        }
    }

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

        if rc == -1 {
            Err(Error::new(ErrorKind::NonZero, ZMsgError::CmdFailed))
        } else {
            Ok(())
        }
    }

    // pub fn zmsg_pushstrf(_self: *mut zmsg_t,
    //                      format: *const ::std::os::raw::c_char, ...)
    //  -> ::std::os::raw::c_int;
    // pub fn zmsg_addstrf(_self: *mut zmsg_t,
    //                     format: *const ::std::os::raw::c_char, ...)
    //  -> ::std::os::raw::c_int;

    pub fn popstr(&self) -> Option<result::Result<String, Vec<u8>>> {
        let ptr = unsafe { czmq_sys::zmsg_popstr(self.zmsg) };

        if ptr == ptr::null_mut() {
            None
        } else {
            let c_string = unsafe { CStr::from_ptr(ptr).to_owned() };
            let bytes = c_string.as_bytes().to_vec();
            match c_string.into_string() {
                Ok(s) => Some(Ok(s)),
                Err(_) => Some(Err(bytes)),
            }
        }
    }

    pub fn popbytes(&self) -> Option<Vec<u8>> {
        let ptr = unsafe { czmq_sys::zmsg_popstr(self.zmsg) };

        if ptr == ptr::null_mut() {
            None
        } else {
            let c_string = unsafe { CStr::from_ptr(ptr).to_owned() };
            Some(c_string.to_bytes().to_vec())
        }
    }

    // pub fn zmsg_addmsg(_self: *mut zmsg_t, msg_p: *mut *mut zmsg_t)
    //  -> ::std::os::raw::c_int;
    // pub fn zmsg_popmsg(_self: *mut zmsg_t) -> *mut zmsg_t;
    // pub fn zmsg_remove(_self: *mut zmsg_t, frame: *mut zframe_t);

    pub fn first(&self) -> Option<ZFrame> {
        let ptr = unsafe { czmq_sys::zmsg_first(self.zmsg) };

        if ptr == ptr::null_mut() {
            None
        } else {
            Some(ZFrame::from_raw(ptr, false))
        }
    }

    pub fn next(&self) -> Option<ZFrame> {
        let ptr = unsafe { czmq_sys::zmsg_next(self.zmsg) };

        if ptr == ptr::null_mut() {
            None
        } else {
            Some(ZFrame::from_raw(ptr, false))
        }
    }

    // We can't call this fn last() as it conflicts with
    // Iterator::last(), which is implemented for ZMsg.
    pub fn ref_last(&self) -> Option<ZFrame> {
        let ptr = unsafe { czmq_sys::zmsg_last(self.zmsg) };

        if ptr == ptr::null_mut() {
            None
        } else {
            Some(ZFrame::from_raw(ptr, false))
        }
    }

    // pub fn zmsg_save(_self: *mut zmsg_t, file: *mut FILE)
    //  -> ::std::os::raw::c_int;
    // pub fn zmsg_dup(_self: *mut zmsg_t) -> *mut zmsg_t;
    // pub fn zmsg_print(_self: *mut zmsg_t);
    // pub fn zmsg_eq(_self: *mut zmsg_t, other: *mut zmsg_t) -> u8;

    pub fn signal(&self) -> Result<u8> {
        let signal = unsafe { czmq_sys::zmsg_signal(self.zmsg) };
        if signal == -1 {
            Err(Error::new(ErrorKind::NonZero, ZMsgError::CmdFailed))
        } else {
            Ok(signal as u8)
        }
    }

    // pub fn zmsg_test(verbose: u8);

    pub fn borrow_raw(&self) -> *mut czmq_sys::zmsg_t {
        self.zmsg
    }
}

impl RawInterface<czmq_sys::zmsg_t> for ZMsg {
    fn from_raw(ptr: *mut czmq_sys::zmsg_t, owned: bool) -> ZMsg {
        ZMsg {
            zmsg: ptr,
            owned: owned,
        }
    }

    fn into_raw(mut self) -> *mut czmq_sys::zmsg_t {
        self.owned = false;
        self.zmsg
    }

    fn borrow_raw(&self) -> *mut czmq_sys::zmsg_t {
        self.zmsg
    }
}

impl Iterator for ZMsg {
    type Item = ZFrame;

    fn next(&mut self) -> Option<Self::Item> {
        ZMsg::next(self)
    }
}

#[derive(Debug)]
pub enum ZMsgError {
    CmdFailed,
}

impl fmt::Display for ZMsgError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ZMsgError::CmdFailed => write!(f, "ZMsg command failed"),
        }
    }
}

impl error::Error for ZMsgError {
    fn description(&self) -> &str {
        match *self {
            ZMsgError::CmdFailed => "ZMsg command failed",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use {zmq, ZFrame, ZSock, zsys_init};

    #[test]
    fn test_sendrecv_zmq() {
        let mut ctx = zmq::Context::new();

        let mut server = ctx.socket(zmq::REP).unwrap();
        server.bind("inproc://zmsg_sendrecv_zmq").unwrap();

        let mut client = ctx.socket(zmq::REQ).unwrap();
        client.connect("inproc://zmsg_sendrecv_zmq").unwrap();

        let zmsg = ZMsg::new();
        zmsg.addstr("Hello world!").unwrap();
        zmsg.send(&client).unwrap();

        let zmsg_recv = ZMsg::recv(&server).unwrap();
        assert_eq!(zmsg_recv.popstr().unwrap().unwrap(), "Hello world!");
    }

    #[test]
    fn test_sendrecv_zsock() {
        zsys_init();

        let server = ZSock::new_rep("inproc://zmsg_sendrecv_zsock").unwrap();
        let client = ZSock::new_req("inproc://zmsg_sendrecv_zsock").unwrap();

        let zmsg = ZMsg::new();
        zmsg.addstr("Hello world!").unwrap();
        zmsg.send(&client).unwrap();

        let msg = ZMsg::recv(&server).unwrap();
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

    #[test]
    fn test_size() {
        let msg = ZMsg::new();
        msg.addstr("123").unwrap();
        assert_eq!(msg.size(), 1);
    }

    #[test]
    fn test_prepend() {
        let msg = ZMsg::new();
        msg.prepend(ZFrame::from("123").unwrap()).unwrap();
        assert_eq!(msg.popstr().unwrap().unwrap(), "123");
    }

    #[test]
    fn test_append() {
        let msg = ZMsg::new();
        msg.append(ZFrame::from("123").unwrap()).unwrap();
        assert_eq!(msg.popstr().unwrap().unwrap(), "123");
    }

    #[test]
    fn test_pop() {
        let msg = ZMsg::new();
        msg.addstr("123").unwrap();
        assert_eq!(msg.pop().unwrap().data().unwrap().unwrap(), "123");
    }

    #[test]
    fn test_popstr() {
        let msg = ZMsg::new();
        msg.addstr("123").unwrap();
        assert_eq!(msg.popstr().unwrap().unwrap(), "123");
    }

    #[test]
    fn test_popbytes() {
        let msg = ZMsg::new();
        msg.addstr("123").unwrap();
        assert_eq!(msg.popbytes().unwrap(), "123".as_bytes());
    }

    #[test]
    fn test_iter_fns() {
        let msg = ZMsg::new();
        msg.addstr("1").unwrap();
        msg.addstr("2").unwrap();
        msg.addstr("3").unwrap();

        let frame = msg.first().unwrap();
        assert_eq!(frame.data().unwrap().unwrap(), "1");

        let frame = msg.next().unwrap();
        assert_eq!(frame.data().unwrap().unwrap(), "2");

        let frame = msg.ref_last().unwrap();
        assert_eq!(frame.data().unwrap().unwrap(), "3");
    }

    #[test]
    fn test_iter() {
        let msg = ZMsg::new();
        msg.addstr("1").unwrap();
        msg.addstr("2").unwrap();
        msg.addstr("3").unwrap();

        let mut i = 1;
        for x in msg {
            assert_eq!(x.data().unwrap().unwrap(), i.to_string());
            i += 1;
        }
    }
}
