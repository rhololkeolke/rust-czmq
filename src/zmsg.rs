//! Module: czmq-zmsg

use {czmq_sys, Error, ErrorKind, RawInterface, Result, Sockish, ZFrame};
use std::{error, fmt, mem, ptr, result};
use std::ffi::{CStr, CString};

#[derive(Debug, Eq)]
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

impl Iterator for ZMsg {
    type Item = ZFrame;

    fn next(&mut self) -> Option<Self::Item> {
        ZMsg::next(self)
    }
}

impl PartialEq for ZMsg {
    fn eq(&self, other: &ZMsg) -> bool {
        ZMsg::eq(self, other)
    }
}

impl ZMsg {
    pub fn new() -> ZMsg {
        ZMsg {
            zmsg: unsafe { czmq_sys::zmsg_new() },
            owned: true,
        }
    }

    pub fn recv<S: Sockish>(source: &mut S) -> Result<ZMsg> {
        let zmsg = unsafe { czmq_sys::zmsg_recv(source.as_mut_ptr()) };

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
            Ok(unsafe { ZFrame::from_raw(zframe, true) })
        }
    }

    pub fn decode(frame: &mut ZFrame) -> Result<ZMsg> {
        let zmsg = unsafe { czmq_sys::zmsg_decode(frame.as_mut_ptr()) };

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

    pub fn send<D: Sockish>(self, dest: &mut D) -> Result<()> {
        let mut zmsg = self;

        let rc = unsafe { czmq_sys::zmsg_send(&mut zmsg.zmsg, dest.as_mut_ptr()) };
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
            Some(unsafe { ZFrame::from_raw(ptr, true) })
        }
    }

    // pub fn zmsg_pushmem(_self: *mut zmsg_t,
    //                     src: *const ::std::os::raw::c_void, size: size_t)
    //  -> ::std::os::raw::c_int;
    // pub fn zmsg_addmem(_self: *mut zmsg_t, src: *const ::std::os::raw::c_void,
    //                    size: size_t) -> ::std::os::raw::c_int;

    pub fn pushstr(&self, string: &str) -> Result<()> {
        let string_c = CString::new(string).unwrap_or(CString::new("").unwrap());
        let rc = unsafe { czmq_sys::zmsg_pushstr(self.zmsg, string_c.as_ptr()) };

        // Deliberately leak this memory, which will be managed by C
        mem::forget(string_c);

        if rc == -1 {
            Err(Error::new(ErrorKind::NonZero, ZMsgError::CmdFailed))
        } else {
            Ok(())
        }
    }

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

    pub fn pushbytes(&self, bytes: &[u8]) -> Result<()> {
        let zframe = try!(ZFrame::new(bytes));
        try!(self.prepend(zframe));
        Ok(())
    }

    pub fn addbytes(&self, bytes: &[u8]) -> Result<()> {
        let zframe = try!(ZFrame::new(bytes));
        try!(self.append(zframe));
        Ok(())
    }

    pub fn popbytes(&self) -> Result<Option<Vec<u8>>> {
        let frame = self.pop();

        if frame.is_some() {
            Ok(Some(match try!(frame.unwrap().data()) {
                Ok(s) => s.into_bytes(),
                Err(b) => b
            }))
        } else {
            Ok(None)
        }
    }

    pub fn addmsg(&self, other: ZMsg) -> Result<()> {
        let rc = unsafe { czmq_sys::zmsg_addmsg(self.zmsg, &mut other.into_raw()) };

        if rc == -1 {
            Err(Error::new(ErrorKind::NonZero, ZMsgError::CmdFailed))
        } else {
            Ok(())
        }
    }

    pub fn popmsg(&self) -> Option<ZMsg> {
        let ptr = unsafe { czmq_sys::zmsg_popmsg(self.zmsg) };

        if ptr == ptr::null_mut() {
            None
        } else {
            Some(unsafe { ZMsg::from_raw(ptr, true) })
        }
    }

    pub fn remove(&self, frame: &mut ZFrame) {
        unsafe { czmq_sys::zmsg_remove(self.zmsg, frame.as_mut_ptr()) };
    }

    // pub fn zmsg_remove(_self: *mut zmsg_t, frame: *mut zframe_t);

    pub fn first(&self) -> Option<ZFrame> {
        let ptr = unsafe { czmq_sys::zmsg_first(self.zmsg) };

        if ptr == ptr::null_mut() {
            None
        } else {
            Some(unsafe { ZFrame::from_raw(ptr, false) })
        }
    }

    pub fn next(&self) -> Option<ZFrame> {
        let ptr = unsafe { czmq_sys::zmsg_next(self.zmsg) };

        if ptr == ptr::null_mut() {
            None
        } else {
            Some(unsafe { ZFrame::from_raw(ptr, false) })
        }
    }

    // We can't call this fn last() as it conflicts with
    // Iterator::last(), which is implemented for ZMsg.
    pub fn ref_last(&self) -> Option<ZFrame> {
        let ptr = unsafe { czmq_sys::zmsg_last(self.zmsg) };

        if ptr == ptr::null_mut() {
            None
        } else {
            Some(unsafe { ZFrame::from_raw(ptr, false) })
        }
    }

    // pub fn zmsg_save(_self: *mut zmsg_t, file: *mut FILE)
    //  -> ::std::os::raw::c_int;

    pub fn dup(&self) -> Result<ZMsg> {
        let ptr = unsafe { czmq_sys::zmsg_dup(self.zmsg) };

        if ptr == ptr::null_mut() {
            Err(Error::new(ErrorKind::NullPtr, ZMsgError::CmdFailed))
        } else {
            Ok(unsafe { ZMsg::from_raw(ptr, true) })
        }
    }

    pub fn print(&self) {
        unsafe { czmq_sys::zmsg_print(self.zmsg) };
    }

    pub fn eq(&self, other: &ZMsg) -> bool {
        unsafe { czmq_sys::zmsg_eq(self.zmsg, other.zmsg) == 1 }
    }

    pub fn signal(&self) -> Result<u8> {
        let signal = unsafe { czmq_sys::zmsg_signal(self.zmsg) };
        if signal == -1 {
            Err(Error::new(ErrorKind::NonZero, ZMsgError::CmdFailed))
        } else {
            Ok(signal as u8)
        }
    }
}

impl RawInterface<czmq_sys::zmsg_t> for ZMsg {
    unsafe fn from_raw(ptr: *mut czmq_sys::zmsg_t, owned: bool) -> ZMsg {
        ZMsg {
            zmsg: ptr,
            owned: owned,
        }
    }

    fn into_raw(mut self) -> *mut czmq_sys::zmsg_t {
        self.owned = false;
        self.zmsg
    }

    fn as_mut_ptr(&mut self) -> *mut czmq_sys::zmsg_t {
        self.zmsg
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
    use {zmq, ZCert, ZFrame, ZSock, ZSys};

    #[test]
    fn test_sendrecv_zmq() {
        let mut ctx = zmq::Context::new();

        let mut server = ctx.socket(zmq::REP).unwrap();
        server.bind("inproc://zmsg_sendrecv_zmq").unwrap();

        let mut client = ctx.socket(zmq::REQ).unwrap();
        client.connect("inproc://zmsg_sendrecv_zmq").unwrap();

        let zmsg = ZMsg::new();
        zmsg.addstr("Hello world!").unwrap();
        zmsg.send(&mut client).unwrap();

        let zmsg_recv = ZMsg::recv(&mut server).unwrap();
        assert_eq!(zmsg_recv.popstr().unwrap().unwrap(), "Hello world!");
    }

    #[test]
    fn test_sendrecv_zsock() {
        ZSys::init();

        let mut server = ZSock::new_rep("inproc://zmsg_sendrecv_zsock").unwrap();
        let mut client = ZSock::new_req("inproc://zmsg_sendrecv_zsock").unwrap();

        let zmsg = ZMsg::new();
        zmsg.addstr("Hello world!").unwrap();
        zmsg.send(&mut client).unwrap();

        let msg = ZMsg::recv(&mut server).unwrap();
        assert_eq!(msg.popstr().unwrap().unwrap(), "Hello world!");
    }

    #[test]
    fn test_encode_decode() {
        let msg = ZMsg::new();
        msg.addstr("moo").unwrap();
        let msg_decoded = ZMsg::decode(&mut msg.encode().unwrap()).unwrap();
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
    fn test_push_add_popstr() {
        let msg = ZMsg::new();
        msg.addstr("456").unwrap();
        msg.pushstr("123").unwrap();
        assert_eq!(msg.popstr().unwrap().unwrap(), "123");
    }

    #[test]
    fn test_push_add_popbytes() {
        let cert = ZCert::new().unwrap();
        cert.set_meta("key", "value");
        let encoded = cert.encode_meta();

        let msg = ZMsg::new();
        msg.addbytes("123".as_bytes()).unwrap();
        msg.pushbytes(&encoded).unwrap();
        assert_eq!(msg.popbytes().unwrap().unwrap(), encoded);
    }

    #[test]
    fn test_add_popmsg() {
        let child_msg = ZMsg::new();
        child_msg.addstr("test").unwrap();

        let parent_msg = ZMsg::new();
        parent_msg.addmsg(child_msg).unwrap();
        assert_eq!(parent_msg.popmsg().unwrap().popstr().unwrap().unwrap(), "test");
    }

    #[test]
    fn test_remove() {
        let frame = ZFrame::from("baa").unwrap();
        let msg = ZMsg::new();
        msg.append(frame).unwrap();
        msg.remove(&mut msg.next().unwrap());
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

    #[test]
    fn test_dup() {
        let msg = ZMsg::new();
        msg.addstr("1").unwrap();
        msg.addstr("2").unwrap();
        msg.addstr("3").unwrap();

        assert!(msg.dup().is_ok());
    }

    #[test]
    fn test_eq() {
        let one = ZMsg::new();
        one.addstr("1").unwrap();
        one.addstr("2").unwrap();
        one.addstr("3").unwrap();

        let two = ZMsg::new();
        two.addstr("1").unwrap();
        two.addstr("2").unwrap();
        two.addstr("3").unwrap();

        assert_eq!(one, two);
    }
}
