//! Module: czmq-zlist

use {czmq_sys, RawInterface};
use std::ffi::CStr;
#[cfg(test)]
use std::ffi::CString;
use std::os::raw::c_char;
#[cfg(test)]
use std::os::raw::c_void;
use std::ptr;

pub struct ZList {
    zlist: *mut czmq_sys::zlist_t,
    owned: bool,
}

unsafe impl Send for ZList {}
unsafe impl Sync for ZList {}

impl Drop for ZList {
    fn drop(&mut self) {
        if self.owned {
            unsafe { czmq_sys::zlist_destroy(&mut self.zlist) };
        }
    }
}

impl ZList {
    #[cfg(test)]
    fn new() -> ZList {
        ZList {
            zlist: unsafe { czmq_sys::zlist_new() },
            owned: true,
        }
    }

    pub fn to_vec<'a>(&'a self) -> Vec<&'a str> {
        let mut v: Vec<&str> = Vec::new();

        loop {
            if let Some(s) = self.next() {
                v.push(s);
            } else {
                break;
            }
        }

        v
    }

    fn next<'a>(&self) -> Option<&'a str> {
        unsafe {
            let ptr = czmq_sys::zlist_next(self.zlist);
            if ptr != ptr::null_mut() {
                Some(CStr::from_ptr(ptr as *const c_char).to_str().unwrap_or(""))
            } else {
                None
            }
        }
    }

    #[cfg(test)]
    fn append(&self, value: &str) -> Result<(), ()> {
        let value_c = CString::new(value).unwrap_or(CString::new("").unwrap());
        let rc = unsafe { czmq_sys::zlist_append(self.zlist, value_c.into_raw() as *mut c_void) };
        if rc == -1 { Err(()) } else { Ok(()) }
    }
}

impl RawInterface<czmq_sys::zlist_t> for ZList {
    fn from_raw(ptr: *mut czmq_sys::zlist_t, owned: bool) -> ZList {
        ZList {
            zlist: ptr,
            owned: owned,
        }
    }

    fn into_raw(mut self) -> *mut czmq_sys::zlist_t {
        self.owned = false;
        self.zlist
    }

    fn borrow_raw(&self) -> *mut czmq_sys::zlist_t {
        self.zlist
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next() {
        let zlist = ZList::new();
        assert!(zlist.next().is_none());
        zlist.append("moo").unwrap();
        assert_eq!(zlist.next().unwrap(), "moo");
    }

    #[test]
    fn test_to_vec() {
        let zlist = ZList::new();
        assert!(zlist.to_vec().len() == 0);
        zlist.append("moo").unwrap();
        let vec = zlist.to_vec();
        assert_eq!(vec.first().unwrap(), &"moo");
    }
}
