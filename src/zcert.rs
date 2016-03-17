//! Module: czmq-zcert

use {czmq_sys, ZList};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::{ptr, result, slice};
use zmq::Socket;

const KEY_SIZE: usize = 32;

// Generic error code "-1" doesn't map to an error message, so just
// return an empty tuple.
pub type Result<T> = result::Result<T, ()>;

pub struct ZCert {
    zcert: *mut czmq_sys::zcert_t,
}

impl Drop for ZCert {
    fn drop(&mut self) {
        unsafe { czmq_sys::zcert_destroy(&mut self.zcert) };
    }
}

impl ZCert {
    pub fn new() -> Result<ZCert> {
        let zcert = unsafe { czmq_sys::zcert_new() };

        if zcert == ptr::null_mut() {
            return Err(());
        }

        Ok(ZCert {
            zcert: zcert,
        })
    }

    pub fn from_keys(public_key: &[u8], secret_key: &[u8]) -> ZCert {
        ZCert {
            zcert: unsafe { czmq_sys::zcert_new_from(public_key.as_ptr(), secret_key.as_ptr()) },
        }
    }

    pub fn load(filename: &str) -> ZCert {
        let filename_c = CString::new(filename).unwrap_or(CString::new("").unwrap());

        ZCert {
            zcert: unsafe { czmq_sys::zcert_load(filename_c.as_ptr()) },
        }
    }

    pub fn public_key<'a>(&'a self) -> &'a mut [u8] {
        unsafe {
            let ptr = czmq_sys::zcert_public_key(self.zcert);
            slice::from_raw_parts_mut(ptr, KEY_SIZE)
        }
    }

    pub fn secret_key<'a>(&'a self) -> &'a mut [u8] {
        unsafe {
            let ptr = czmq_sys::zcert_secret_key(self.zcert);
            slice::from_raw_parts_mut(ptr, KEY_SIZE)
        }
    }

    pub fn public_txt<'a>(&'a self) -> &'a str {
        unsafe {
            let ptr = czmq_sys::zcert_public_txt(self.zcert);
            CStr::from_ptr(ptr as *const c_char).to_str().unwrap_or("")
        }
    }

    pub fn secret_txt<'a>(&'a self) -> &'a str {
        unsafe {
            let ptr = czmq_sys::zcert_secret_txt(self.zcert);
            CStr::from_ptr(ptr as *const c_char).to_str().unwrap_or("")
        }
    }

    pub fn set_meta(&self, key: &str, value: &str) {
        let key_c = CString::new(key).unwrap_or(CString::new("").unwrap());
        let value_c = CString::new(value).unwrap_or(CString::new("").unwrap());
        let format_c = CString::new("%s").unwrap();

        unsafe { czmq_sys::zcert_set_meta(self.zcert, key_c.as_ptr(), format_c.as_ptr(), value_c.as_ptr()) };
    }

    pub fn meta<'a>(&'a self, key: &str) -> &'a str {
        let key_c = CString::new(key).unwrap_or(CString::new("").unwrap());

        unsafe {
            let ptr = czmq_sys::zcert_meta(self.zcert, key_c.as_ptr());
            CStr::from_ptr(ptr as *const c_char).to_str().unwrap_or("")
        }
    }

    pub fn meta_keys<'a>(&'a self) -> ZList {
        let ptr = unsafe { czmq_sys::zcert_meta_keys(self.zcert) };
        ZList::from_raw(ptr)
    }

    pub fn save(&self, filename: &str) -> Result<()> {
        let filename_c = CString::new(filename).unwrap_or(CString::new("").unwrap());

        unsafe {
            let rc = czmq_sys::zcert_save(self.zcert, filename_c.as_ptr());
            if rc == -1i32 { Err(()) } else { Ok(()) }
        }
    }

    pub fn save_public(&self, filename: &str) -> Result<()> {
        let filename_c = CString::new(filename).unwrap_or(CString::new("").unwrap());

        unsafe {
            let rc = czmq_sys::zcert_save_public(self.zcert, filename_c.as_ptr());
            if rc == -1i32 { Err(()) } else { Ok(()) }
        }
    }

    pub fn save_secret(&self, filename: &str) -> Result<()> {
        let filename_c = CString::new(filename).unwrap_or(CString::new("").unwrap());

        unsafe {
            let rc = czmq_sys::zcert_save_secret(self.zcert, filename_c.as_ptr());
            if rc == -1i32 { Err(()) } else { Ok(()) }
        }
    }

    pub fn apply(&self, sock: Socket) {
        unsafe { czmq_sys::zcert_apply(self.zcert, sock.sock) };
    }

    pub fn dup(&self) {}

    pub fn eq(&self) {}

    pub fn print(&self) {
        unsafe { czmq_sys::zcert_print(self.zcert) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use zmq;

    const PUBLIC_TXT: &'static str = "Ko9/&3Uw)$U]Zyp>+4$-i/yaDea2QqlDPGl-&V1s";
    const SECRET_TXT: &'static str = "[MfOo!1^1N}zZY/x{[A6^9>VRC.+O6vX&]zYvDC-";

    #[test]
    fn test_public_key() {
        let cert = create_cert();
        let key = cert.public_key();
        let test_key = zmq::z85_decode(PUBLIC_TXT);

        let mut iter = 0;
        for _ in key.iter() {
            assert_eq!(key[iter], test_key[iter]);
            iter += 1;
        }
    }

    #[test]
    fn test_secret_key() {
        let cert = create_cert();
        let key = cert.secret_key();
        let test_key = zmq::z85_decode(SECRET_TXT);

        let mut iter = 0;
        for _ in key.iter() {
            assert_eq!(key[iter], test_key[iter]);
            iter += 1;
        }
    }

    #[test]
    fn test_public_txt() {
        let cert = create_cert();
        assert_eq!(cert.public_txt(), PUBLIC_TXT);
    }

    #[test]
    fn test_secret_txt() {
        let cert = create_cert();
        assert_eq!(cert.secret_txt(), SECRET_TXT);
    }

    #[test]
    fn test_getset_meta() {
        let cert = create_cert();
        cert.set_meta("moo", "cow");
        assert_eq!(cert.meta("moo"), "cow");
    }

    #[test]
    fn test_meta_keys() {
        let cert = create_cert();
        assert!(cert.meta_keys().to_vec().len() == 0);
        cert.set_meta("moo", "cow");
        let keys = cert.meta_keys();
        assert_eq!(keys.to_vec().first().unwrap(), &"moo");
    }

    fn create_cert() -> ZCert {
        let public_key = zmq::z85_decode(PUBLIC_TXT);
        let secret_key = zmq::z85_decode(SECRET_TXT);
        ZCert::from_keys(&public_key, &secret_key)
    }
}
