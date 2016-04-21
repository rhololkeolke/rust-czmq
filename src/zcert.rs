//! Module: czmq-zcert

use {czmq_sys, Error, ErrorKind, Result, ZList};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::{error, fmt, ptr, slice};
use zmsg::ZMsgable;

const KEY_SIZE: usize = 32;

pub struct ZCert {
    zcert: *mut czmq_sys::zcert_t,
    persistent: bool,
}

unsafe impl Send for ZCert {}
unsafe impl Sync for ZCert {}

impl Drop for ZCert {
    fn drop(&mut self) {
        if !self.persistent {
            unsafe { czmq_sys::zcert_destroy(&mut self.zcert) };
        }
    }
}

impl ZCert {
    pub fn new() -> Result<ZCert> {
        let zcert = unsafe { czmq_sys::zcert_new() };

        if zcert == ptr::null_mut() {
            return Err(Error::new(ErrorKind::NullPtr, ZCertError::Instantiate));
        }

        Ok(ZCert {
            zcert: zcert,
            persistent: false,
        })
    }

    pub fn from_keys(public_key: &[u8], secret_key: &[u8]) -> ZCert {
        ZCert {
            zcert: unsafe { czmq_sys::zcert_new_from(public_key.as_ptr(), secret_key.as_ptr()) },
            persistent: false,
        }
    }

    pub fn from_raw(zcert: *mut czmq_sys::zcert_t, persistent: bool) -> ZCert {
        ZCert {
            zcert: zcert,
            persistent: persistent,
        }
    }

    pub fn into_raw(mut self) -> *mut czmq_sys::zcert_t {
        self.persistent = true;
        self.zcert
    }

    pub fn load(filename: &str) -> Result<ZCert> {
        let filename_c = try!(CString::new(filename));
        let zcert = unsafe { czmq_sys::zcert_load(filename_c.as_ptr()) };

        if zcert == ptr::null_mut() {
            return Err(Error::new(ErrorKind::NullPtr, ZCertError::InvalidCert(filename_c.into_string().unwrap())));
        }

        Ok(ZCert {
            zcert: zcert,
            persistent: false,
        })
    }

    pub fn public_key<'a>(&'a self) -> &'a [u8] {
        unsafe {
            let ptr = czmq_sys::zcert_public_key(self.zcert);
            slice::from_raw_parts(ptr, KEY_SIZE)
        }
    }

    pub fn secret_key<'a>(&'a self) -> &'a [u8] {
        unsafe {
            let ptr = czmq_sys::zcert_secret_key(self.zcert);
            slice::from_raw_parts(ptr, KEY_SIZE)
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

        unsafe { czmq_sys::zcert_set_meta(self.zcert, key_c.as_ptr(), "%s\0".as_ptr() as *const i8, value_c.as_ptr()) };
    }

    pub fn meta<'a>(&'a self, key: &str) -> Result<Option<&'a str>> {
        let key_c = try!(CString::new(key));

        let value = unsafe {
            let ptr = czmq_sys::zcert_meta(self.zcert, key_c.as_ptr());
            CStr::from_ptr(ptr as *const c_char)
        };

        match value.to_str() {
            Ok(s) => Ok(Some(s)),
            Err(_) => Ok(None),
        }
    }

    pub fn meta_keys<'a>(&'a self) -> ZList {
        let ptr = unsafe { czmq_sys::zcert_meta_keys(self.zcert) };
        ZList::from_raw(ptr)
    }

    pub fn save(&self, filename: &str) -> Result<()> {
        let filename_c = try!(CString::new(filename));

        unsafe {
            let rc = czmq_sys::zcert_save(self.zcert, filename_c.as_ptr());
            if rc == -1 {
                Err(Error::new(ErrorKind::NonZero, ZCertError::SavePath(filename_c.into_string().unwrap())))
            } else {
                Ok(())
            }
        }
    }

    pub fn save_public(&self, filename: &str) -> Result<()> {
        let filename_c = try!(CString::new(filename));

        unsafe {
            let rc = czmq_sys::zcert_save_public(self.zcert, filename_c.as_ptr());
            if rc == -1 {
                Err(Error::new(ErrorKind::NonZero, ZCertError::SavePath(filename_c.into_string().unwrap())))
            } else {
                Ok(())
            }
        }
    }

    pub fn save_secret(&self, filename: &str) -> Result<()> {
        let filename_c = try!(CString::new(filename));

        unsafe {
            let rc = czmq_sys::zcert_save_secret(self.zcert, filename_c.as_ptr());
            if rc == -1 {
                Err(Error::new(ErrorKind::NonZero, ZCertError::SavePath(filename_c.into_string().unwrap())))
            } else {
                Ok(())
            }
        }
    }

    pub fn apply<S: ZMsgable>(&self, sock: &S) {
        unsafe { czmq_sys::zcert_apply(self.zcert, sock.borrow_raw()) };
    }

    pub fn dup(&self) -> ZCert {
        let ptr = unsafe { czmq_sys::zcert_dup(self.zcert) };

        ZCert {
            zcert: ptr,
            persistent: false,
        }
    }

    pub fn eq(&self, cert: &ZCert) -> bool {
        let result = unsafe { czmq_sys::zcert_eq(self.zcert, cert.zcert) };
        result == 1
    }

    pub fn print(&self) {
        unsafe { czmq_sys::zcert_print(self.zcert) };
    }
}

#[derive(Debug)]
pub enum ZCertError {
    Instantiate,
    InvalidCert(String),
    SavePath(String),
}

impl fmt::Display for ZCertError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ZCertError::Instantiate => write!(f, "Could not instantiate new ZCert struct"),
            ZCertError::InvalidCert(ref e) => write!(f, "Could not open certificate at path: {}", e),
            ZCertError::SavePath(ref e) => write!(f, "Could not save certificate file to path: {}", e),
        }
    }
}

impl error::Error for ZCertError {
    fn description(&self) -> &str {
        match *self {
            ZCertError::Instantiate => "Could not instantiate new ZCert struct",
            ZCertError::InvalidCert(_) => "Certificate was invalid or non-existent",
            ZCertError::SavePath(_) => "Could not save certificate file to given path",
        }
    }
}

#[cfg(test)]
mod tests {
    use {ZSock, zsys_init};
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
        assert_eq!(cert.meta("moo").unwrap().unwrap(), "cow");
    }

    #[test]
    fn test_meta_keys() {
        let cert = create_cert();
        assert!(cert.meta_keys().to_vec().len() == 0);
        cert.set_meta("moo", "cow");
        let keys = cert.meta_keys();
        assert_eq!(keys.to_vec().first().unwrap(), &"moo");
    }

    #[test]
    fn test_apply_zmq() {
        let cert = create_cert();
        let mut ctx = zmq::Context::new();
        let sock = ctx.socket(zmq::REQ).unwrap();
        cert.apply(&sock);
        assert_eq!(sock.get_curve_publickey().unwrap().unwrap(), PUBLIC_TXT);
        assert_eq!(sock.get_curve_secretkey().unwrap().unwrap(), SECRET_TXT);
    }

    #[test]
    fn test_apply_zsock() {
        zsys_init();

        let cert = create_cert();
        let sock = ZSock::new_rep("inproc://zcert_test_apply_zsock").unwrap();
        cert.apply(&sock);
        assert_eq!(sock.curve_publickey().unwrap().unwrap(), PUBLIC_TXT);
        assert_eq!(sock.curve_secretkey().unwrap().unwrap(), SECRET_TXT);
    }

    #[test]
    fn test_dup() {
        let cert = create_cert();
        let dup = cert.dup();
        assert_eq!(cert.secret_txt(), dup.secret_txt());
    }

    #[test]
    fn test_eq() {
        let c1 = create_cert();
        let c2 = create_cert();
        assert!(c1.eq(&c2));
    }

    fn create_cert() -> ZCert {
        let public_key = zmq::z85_decode(PUBLIC_TXT);
        let secret_key = zmq::z85_decode(SECRET_TXT);
        ZCert::from_keys(&public_key, &secret_key)
    }
}
