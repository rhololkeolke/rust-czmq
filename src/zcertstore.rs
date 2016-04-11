//! Module: czmq-zcert

use {czmq_sys, Error, ErrorKind, Result, ZCert};
use std::ffi::CString;
use std::{error, fmt, ptr};

pub struct ZCertStore {
    zcertstore: *mut czmq_sys::zcertstore_t,
}

unsafe impl Send for ZCertStore {}
unsafe impl Sync for ZCertStore {}

impl Drop for ZCertStore {
    fn drop(&mut self) {
        unsafe { czmq_sys::zcertstore_destroy(&mut self.zcertstore) };
    }
}

impl ZCertStore {
    pub fn new(location: &str) -> Result<ZCertStore> {
        let location_c = try!(CString::new(location));
        let zcertstore = unsafe { czmq_sys::zcertstore_new(location_c.as_ptr()) };

        if zcertstore == ptr::null_mut() {
            return Err(Error::new(ErrorKind::NullPtr, ZCertStoreError::Instantiate));
        }

        Ok(ZCertStore {
            zcertstore: zcertstore,
        })
    }

    pub fn lookup(&self, public_key: &str) -> Result<Option<ZCert>> {
        let public_key_c = try!(CString::new(public_key));
        let zcert = unsafe { czmq_sys::zcertstore_lookup(self.zcertstore, public_key_c.as_ptr()) };

        if zcert == ptr::null_mut() { Ok(None) } else { Ok(Some(ZCert::from_raw(zcert, true))) }
    }

    pub fn insert(&self, zcert: ZCert) {
        unsafe { czmq_sys::zcertstore_insert(self.zcertstore, &mut zcert.into_raw()); }
    }

    pub fn print(&self) {
        unsafe { czmq_sys::zcertstore_print(self.zcertstore) };
    }
}

#[derive(Debug)]
pub enum ZCertStoreError {
    Instantiate,
}

impl fmt::Display for ZCertStoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ZCertStoreError::Instantiate => write!(f, "Could not instantiate new ZCertStore struct"),
        }
    }
}

impl error::Error for ZCertStoreError {
    fn description(&self) -> &str {
        match *self {
            ZCertStoreError::Instantiate => "Could not instantiate new ZCertStore struct",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempdir::TempDir;
    use ZCert;

    #[test]
    fn test_new() {
        let dir = TempDir::new("zcertstore").unwrap();
        assert!(ZCertStore::new(dir.path().to_str().unwrap()).is_ok());
    }

    #[test]
    fn test_lookup() {
        let dir = TempDir::new("zcertstore").unwrap();

        let store = ZCertStore::new(dir.path().to_str().unwrap()).unwrap();
        assert!(store.lookup("nonexistent_key").unwrap().is_none());

        let cert = ZCert::new().unwrap();
        cert.save_public(&format!("{}/testcert.crt", dir.path().to_str().unwrap())).unwrap();
        assert_eq!(store.lookup(cert.public_txt()).unwrap().unwrap().public_txt(), cert.public_txt());
    }

    #[test]
    fn test_insert() {
        let dir = TempDir::new("zcertstore").unwrap();

        let store = ZCertStore::new(dir.path().to_str().unwrap()).unwrap();
        let cert = ZCert::new().unwrap();
        let public_txt = cert.public_txt().to_string(); // Make sure we own this

        store.insert(cert);
        assert_eq!(store.lookup(&public_txt).unwrap().unwrap().public_txt(), public_txt);

    }
}
