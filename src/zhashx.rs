//! Module: czmq-zhashx

use {czmq_sys, Colander, Error, ErrorKind, Result};
use std::{error, fmt, mem, ptr};
use std::any::Any;
use std::ffi::CString;
use std::os::raw::c_void;

pub struct ZHashX {
    zhashx: *mut czmq_sys::zhashx_t,
    owned: bool
}

unsafe impl Send for ZHashX {}
unsafe impl Sync for ZHashX {}

impl Drop for ZHashX {
    fn drop(&mut self) {
        if self.owned {
            unsafe { czmq_sys::zhashx_destroy(&mut self.zhashx) };
        }
    }
}

impl ZHashX {
    pub fn new() -> ZHashX {
        ZHashX {
            zhashx: unsafe { czmq_sys::zhashx_new() },
            owned: true,
        }
    }

    pub fn from_raw(zhashx: *mut czmq_sys::zhashx_t, owned: bool) -> ZHashX {
        ZHashX {
            zhashx: zhashx,
            owned: owned,
        }
    }

    // pub fn zhashx_unpack(frame: *mut zframe_t) -> *mut zhashx_t;
    // pub fn zhashx_destroy(self_p: *mut *mut zhashx_t);

    pub fn insert(&self, key: &str, item: Box<Any>) -> Result<()> {
        self.insert_raw(key, Box::into_raw(item) as *mut c_void)
    }

    pub fn insert_raw(&self, key: &str, item: *mut c_void) -> Result<()> {
        let key_c = try!(CString::new(key));
        let rc = unsafe { czmq_sys::zhashx_insert(self.zhashx, key_c.as_ptr() as *const c_void, item) };

        // Deliberately leak this memory, which will be managed by C
        mem::forget(key_c);

        if rc == -1 {
            Err(Error::new(ErrorKind::NonZero, ZHashXError::CmdFailed))
        } else {
            Ok(())
        }
    }

    pub fn update(&self, key: &str, item: Box<Any>) {
        let key_c = CString::new(key).unwrap_or(CString::new("").unwrap());
        unsafe { czmq_sys::zhashx_update(self.zhashx, key_c.as_ptr() as *const c_void, Box::into_raw(item) as *mut c_void) };

        // Deliberately leak this memory, which will be managed by C
        mem::forget(key_c);
    }

    pub fn delete(&self, key: &str) {
        let key_c = CString::new(key).unwrap_or(CString::new("").unwrap());
        unsafe { czmq_sys::zhashx_delete(self.zhashx, key_c.as_ptr() as *const c_void) };
    }

    // pub fn zhashx_purge(_self: *mut zhashx_t);

    pub fn lookup<T>(&self, key: &str) -> Option<Colander<T>> {
        let key_c = CString::new(key).unwrap_or(CString::new("").unwrap());
        unsafe {
            let ptr = czmq_sys::zhashx_lookup(self.zhashx, key_c.as_ptr() as *const c_void);
            if ptr != ptr::null_mut() {
                Some(mem::transmute(Colander::from_raw(ptr)))
            } else {
                None
            }
        }
    }

    // pub fn zhashx_rename(_self: *mut zhashx_t,
    //                      old_key: *const ::std::os::raw::c_void,
    //                      new_key: *const ::std::os::raw::c_void)
    //  -> ::std::os::raw::c_int;
    // pub fn zhashx_freefn(_self: *mut zhashx_t,
    //                      key: *const ::std::os::raw::c_void,
    //                      free_fn: zhashx_free_fn)
    //  -> *mut ::std::os::raw::c_void;
    // pub fn zhashx_size(_self: *mut zhashx_t) -> size_t;
    // pub fn zhashx_keys(_self: *mut zhashx_t) -> *mut zhashxx_t;
    // pub fn zhashx_values(_self: *mut zhashx_t) -> *mut zhashxx_t;
    // pub fn zhashx_first(_self: *mut zhashx_t) -> *mut ::std::os::raw::c_void;
    // pub fn zhashx_next(_self: *mut zhashx_t) -> *mut ::std::os::raw::c_void;
    // pub fn zhashx_cursor(_self: *mut zhashx_t)
    //  -> *const ::std::os::raw::c_void;
    // pub fn zhashx_comment(_self: *mut zhashx_t,
    //                       format: *const ::std::os::raw::c_char, ...);
    // pub fn zhashx_save(_self: *mut zhashx_t,
    //                    filename: *const ::std::os::raw::c_char)
    //  -> ::std::os::raw::c_int;
    // pub fn zhashx_load(_self: *mut zhashx_t,
    //                    filename: *const ::std::os::raw::c_char)
    //  -> ::std::os::raw::c_int;
    // pub fn zhashx_refresh(_self: *mut zhashx_t) -> ::std::os::raw::c_int;
    // pub fn zhashx_pack(_self: *mut zhashx_t) -> *mut zframe_t;
    // pub fn zhashx_dup(_self: *mut zhashx_t) -> *mut zhashx_t;
    // pub fn zhashx_set_destructor(_self: *mut zhashx_t,
    //                              destructor: zhashx_destructor_fn);
    // pub fn zhashx_set_duplicator(_self: *mut zhashx_t,
    //                              duplicator: zhashx_duplicator_fn);
    // pub fn zhashx_set_key_destructor(_self: *mut zhashx_t,
    //                                  destructor: zhashx_destructor_fn);
    // pub fn zhashx_set_key_duplicator(_self: *mut zhashx_t,
    //                                  duplicator: zhashx_duplicator_fn);
    // pub fn zhashx_set_key_comparator(_self: *mut zhashx_t,
    //                                  comparator: zhashx_comparator_fn);
    // pub fn zhashx_set_key_hasher(_self: *mut zhashx_t,
    //                              hasher: zhashx_hash_fn);
    // pub fn zhashx_dup_v2(_self: *mut zhashx_t) -> *mut zhashx_t;
    // pub fn zhashx_autofree(_self: *mut zhashx_t);
    // pub fn zhashx_foreach(_self: *mut zhashx_t, callback: zhashx_foreach_fn,
    //                       argument: *mut ::std::os::raw::c_void)
    //  -> ::std::os::raw::c_int;
}

#[derive(Debug)]
pub enum ZHashXError {
    CmdFailed,
}

impl fmt::Display for ZHashXError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ZHashXError::CmdFailed => write!(f, "ZHashX command failed"),
        }
    }
}

impl error::Error for ZHashXError {
    fn description(&self) -> &str {
        match *self {
            ZHashXError::CmdFailed => "ZHashX command failed",
        }
    }
}

#[cfg(test)]
mod tests {
    use ZCert;
    use super::*;

    #[test]
    fn test_crud() {
        let hash = ZHashX::new();

        let test_value = ZCert::new().unwrap();
        let pubkey = test_value.public_txt().to_string();
        assert!(hash.insert("mykey", Box::new(test_value)).is_ok());

        let cert = hash.lookup::<ZCert>("mykey").unwrap();
        assert_eq!(cert.public_txt(), pubkey);

        let test_value = ZCert::new().unwrap();
        let pubkey = test_value.public_txt().to_string();
        hash.update("mykey", Box::new(test_value));

        let cert = hash.lookup::<ZCert>("mykey").unwrap();
        assert_eq!(cert.public_txt(), pubkey);

        hash.delete("mykey");
        assert!(hash.lookup::<ZCert>("mykey").is_none());
    }
}
