//! Module: czmq-zframe

use {czmq_sys, Error, ErrorKind, Result};
use std::{error, fmt, ptr, result, slice};
use std::ffi::{CStr, CString};
use std::str::{self, Utf8Error};
use std::os::raw::c_void;
use zmsg::ZMsgable;

bitflags! {
    pub flags Flags: i32 {
        const ZFRAME_MORE     = 0b00000001,
        const ZFRAME_REUSE    = 0b00000010,
        const ZFRAME_DONTWAIT = 0b00000100,
    }
}

pub struct ZFrame {
    zframe: *mut czmq_sys::zframe_t,
}

unsafe impl Send for ZFrame {}
unsafe impl Sync for ZFrame {}

impl Drop for ZFrame {
    fn drop(&mut self) {
        unsafe { czmq_sys::zframe_destroy(&mut self.zframe) };
    }
}

impl ZFrame {
    pub fn new(data: &[u8]) -> Result<ZFrame> {
        let data_c = CString::new(data).unwrap().into_raw();
        let zframe = unsafe { czmq_sys::zframe_new(data_c as *const c_void, data.len() as u64) };
        unsafe { CString::from_raw(data_c) };

        if zframe == ptr::null_mut() {
            return Err(Error::new(ErrorKind::NullPtr, ZFrameError::Instantiate));
        }

        Ok(ZFrame {
            zframe: zframe,
        })
    }

    pub fn empty() -> Result<ZFrame> {
        let zframe = unsafe { czmq_sys::zframe_new_empty() };

        if zframe == ptr::null_mut() {
            return Err(Error::new(ErrorKind::NullPtr, ZFrameError::Instantiate));
        }

        Ok(ZFrame {
            zframe: zframe,
        })
    }

    // The correlating C fn zframe_from wraps zframe_new(), which is
    // already wrapped by ZFrame::new, thus we can call our new()
    // instead.
    pub fn from(frame: &str) -> Result<ZFrame> {
        Self::new(frame.as_bytes())
    }

    pub fn from_raw(zframe: *mut czmq_sys::zframe_t) -> ZFrame {
        ZFrame {
            zframe: zframe,
        }
    }

    pub fn recv<S: ZMsgable>(source: &S) -> Result<ZFrame> {
        let zframe = unsafe { czmq_sys::zframe_recv(source.borrow_raw()) };

        if zframe == ptr::null_mut() {
            return Err(Error::new(ErrorKind::NullPtr, ZFrameError::CmdFailed));
        }

        Ok(ZFrame {
            zframe: zframe,
        })
    }

    // This fn consumes the ZFrame, implying no REUSE flag
    pub fn send<D: ZMsgable>(self, dest: &D, flags: Option<Flags>) -> Result<i32> {
        let mut zframe = self;
        zframe.do_send(dest.borrow_raw(), flags)
    }

    // This fn doesn't consume the ZFrame, which implies REUSE flag
    pub fn send_reuse<D: ZMsgable>(&mut self, dest: &D, flags: Option<Flags>) -> Result<i32> {
        let flags = if let Some(f) = flags {
            f | ZFRAME_REUSE
        } else {
            ZFRAME_REUSE
        };
        self.do_send(dest.borrow_raw(), Some(flags))
    }

    fn do_send(&mut self, dest: *mut c_void, flags: Option<Flags>) -> Result<i32> {
        let flags_c = if let Some(f) = flags { f.bits() } else { 0 };
        let size = unsafe { czmq_sys::zframe_send(&mut self.zframe as *mut *mut czmq_sys::zframe_t, dest, flags_c) };
        if size == -1 {
            Err(Error::new(ErrorKind::NonZero, ZFrameError::CmdFailed))
        } else {
            Ok(size)
        }
    }

    pub fn size(&self) -> usize {
        unsafe { czmq_sys::zframe_size(self.zframe) as usize }
    }

    pub fn data<'a>(&'a self) -> Result<result::Result<&'a str, Utf8Error>> {
        let data = unsafe { czmq_sys::zframe_data(self.zframe) };

        if data == ptr::null_mut() {
            Err(Error::new(ErrorKind::NullPtr, ZFrameError::CmdFailed))
        } else {
            let s = unsafe { slice::from_raw_parts(data, self.size()) };
            Ok(str::from_utf8(s))
        }
    }

    pub fn dup(&self) -> Result<ZFrame> {
        let zframe = unsafe { czmq_sys::zframe_dup(self.zframe) };

        if zframe == ptr::null_mut() {
            Err(Error::new(ErrorKind::NullPtr, ZFrameError::CmdFailed))
        } else {
            Ok(ZFrame {
                zframe: zframe,
            })
        }
    }

    pub fn strhex<'a>(&'a self) -> Result<result::Result<&'a str, Utf8Error>> {
        let hex = unsafe { czmq_sys::zframe_strhex(self.zframe) };

        if hex == ptr::null_mut() {
            Err(Error::new(ErrorKind::NullPtr, ZFrameError::CmdFailed))
        } else {
            Ok(unsafe { CStr::from_ptr(hex) }.to_str())
        }
    }

    pub fn strdup(&self) -> Result<result::Result<String, Utf8Error>> {
        let string = unsafe { czmq_sys::zframe_strdup(self.zframe) };

        if string == ptr::null_mut() {
            Err(Error::new(ErrorKind::NullPtr, ZFrameError::CmdFailed))
        } else {
            let cstr = unsafe { CStr::from_ptr(string) }.to_str();
            match cstr {
                Ok(s) => Ok(Ok(s.to_string())),
                Err(e) => Ok(Err(e))
            }
        }
    }

    pub fn streq(&self, string: &str) -> bool {
        unsafe { czmq_sys::zframe_streq(self.zframe, CString::new(string).unwrap_or(CString::new("").unwrap()).as_ptr()) == 1 }
    }

    pub fn more(&self) -> bool {
        unsafe { czmq_sys::zframe_more(self.zframe) == 1 }
    }

    pub fn set_more(&self, more: bool) {
        unsafe { czmq_sys::zframe_set_more(self.zframe, if more { 1 } else { 0 }) }
    }

    pub fn eq(&self, other: &ZFrame) -> bool {
        unsafe { czmq_sys::zframe_eq(self.zframe, other.zframe) == 1 }
    }

    pub fn reset(&self, data: &[u8]) {
        unsafe { czmq_sys::zframe_reset(self.zframe, data.as_ptr() as *const c_void, data.len() as u64) };
    }

    pub fn print(&self, prefix: Option<&str>) {
        let prefix_ptr = match prefix {
            Some(p) => CString::new(p).unwrap_or(CString::new("").unwrap()).as_ptr(),
            None => ptr::null(),
        };
        unsafe { czmq_sys::zframe_print(self.zframe, prefix_ptr) };
    }

    pub fn borrow_raw(&self) -> *mut czmq_sys::zframe_t {
        self.zframe
    }
}

#[derive(Debug)]
pub enum ZFrameError {
    Instantiate,
    CmdFailed,
}

impl fmt::Display for ZFrameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ZFrameError::Instantiate => write!(f, "Could not instantiate new ZFrame struct"),
            ZFrameError::CmdFailed => write!(f, "ZFrame command failed"),
        }
    }
}

impl error::Error for ZFrameError {
    fn description(&self) -> &str {
        match *self {
            ZFrameError::Instantiate => "Could not instantiate new ZFrame struct",
            ZFrameError::CmdFailed => "ZFrame command failed",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use {zmq, ZSock, zsys_init};

    #[test]
    fn test_sendrecv_zmq() {
        let mut ctx = zmq::Context::new();

        let mut server = ctx.socket(zmq::REP).unwrap();
        server.bind("inproc://zframe_sendrecv_zmq").unwrap();

        let mut client = ctx.socket(zmq::REQ).unwrap();
        client.connect("inproc://zframe_sendrecv_zmq").unwrap();

        let mut zframe1 = ZFrame::from("Hello world!").unwrap();
        zframe1.send_reuse(&client, Some(ZFRAME_MORE)).unwrap();
        zframe1.send(&client, None).unwrap();

        for _ in 1..2 {
            let zframe = ZFrame::recv(&server).unwrap();
            assert_eq!(zframe.data().unwrap().unwrap(), "Hello world!");
        }
    }

    #[test]
    fn test_sendrecv_zsock() {
        zsys_init();

        let server = ZSock::new_rep("inproc://zframe_sendrecv_zsock").unwrap();
        let client = ZSock::new_req("inproc://zframe_sendrecv_zsock").unwrap();

        let mut zframe1 = ZFrame::from("Hello world!").unwrap();
        zframe1.send_reuse(&client, Some(ZFRAME_MORE)).unwrap();
        zframe1.send(&client, None).unwrap();

        for _ in 1..2 {
            let zframe = ZFrame::recv(&server).unwrap();
            assert_eq!(zframe.data().unwrap().unwrap(), "Hello world!");
        }
    }

    #[test]
    fn test_dup() {
        let zframe = ZFrame::from("moo cow").unwrap();
        let zframe_dup = zframe.dup().unwrap();
        assert_eq!(zframe_dup.data().unwrap().unwrap(), "moo cow");
    }

    #[test]
    fn test_strhex() {
        let zframe = ZFrame::from("Oh Tobias, you blowhard!").unwrap();
        assert_eq!(zframe.strhex().unwrap().unwrap(), "4F6820546F626961732C20796F7520626C6F776861726421");
    }

    #[test]
    fn test_strdup() {
        let zframe = ZFrame::from("Because that's how you get ants, Lana!").unwrap();
        assert_eq!(zframe.strdup().unwrap().unwrap(), "Because that's how you get ants, Lana!");
    }

    #[test]
    fn test_streq() {
        let zframe = ZFrame::from("And that's why you always leave a note.").unwrap();
        assert!(zframe.streq("And that's why you always leave a note."));
    }

    #[test]
    fn test_more() {
        let zframe = ZFrame::empty().unwrap();
        zframe.set_more(true);
        assert!(zframe.more());
        zframe.set_more(false);
        assert!(!zframe.more());
    }

    #[test]
    fn test_eq() {
        let zframe1 = ZFrame::from("Steve Holt!").unwrap();
        let zframe2 = ZFrame::from("Steve Holt!").unwrap();
        assert!(zframe1.eq(&zframe2));
    }

    #[test]
    fn test_reset() {
        let zframe = ZFrame::from("Phrasing!").unwrap();
        zframe.reset("Holy shitsnacks!".as_bytes());
        assert_eq!(zframe.data().unwrap().unwrap(), "Holy shitsnacks!");
    }

    // XXX We need to capture output here before we can test.
    // #[test]
    // fn test_print() {
    //     let zframe = ZFrame::from("You're not my supervisor!").unwrap();
    //     zframe.print(Some("prefix_"));
    //     zframe.print(None);
    // }
}
