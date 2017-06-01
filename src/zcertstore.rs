//! Module: czmq-zcertstore

use {czmq_sys, Error, ErrorKind, RawInterface, Result, ZCert};
use std::{error, fmt, ptr};
#[cfg(feature = "draft")]
use std::any::Any;
use std::ffi::CString;
#[cfg(feature = "draft")]
use std::os::raw::c_void;

pub struct ZCertStore {
    zcertstore: *mut czmq_sys::zcertstore_t,
    owned: bool,
}

unsafe impl Send for ZCertStore {}
unsafe impl Sync for ZCertStore {}

impl Drop for ZCertStore {
    fn drop(&mut self) {
        if self.owned {
            unsafe { czmq_sys::zcertstore_destroy(&mut self.zcertstore) };
        }
    }
}

impl ZCertStore {
    pub fn new(location: Option<&str>) -> Result<ZCertStore> {
        let zcertstore = match location {
            Some(l) => {
                let location_c = try!(CString::new(l));
                unsafe { czmq_sys::zcertstore_new(location_c.as_ptr()) }
            },
            None => unsafe { czmq_sys::zcertstore_new(ptr::null()) },
        };

        if zcertstore == ptr::null_mut() {
            return Err(Error::new(ErrorKind::NullPtr, ZCertStoreError::Instantiate));
        }

        Ok(ZCertStore {
            zcertstore: zcertstore,
            owned: true,
        })
    }

    #[cfg(feature = "draft")]
    pub fn set_loader(&self, loader: czmq_sys::zcertstore_loader) {
        unsafe { czmq_sys::zcertstore_set_loader(self.zcertstore, loader, Some(default_destructor), ptr::null_mut::<c_void>()) };
    }

    #[cfg(feature = "draft")]
    pub fn set_loader_with_state(&self, loader: czmq_sys::zcertstore_loader, state: Box<Any>, destructor: Option<czmq_sys::zcertstore_destructor>) {
        let destructor_ptr = destructor.or(Some(Some(default_destructor))).unwrap();

        unsafe { czmq_sys::zcertstore_set_loader(self.zcertstore, loader, destructor_ptr, Box::into_raw(state) as *mut c_void) };
    }

    pub fn lookup(&self, public_key: &str) -> Result<Option<ZCert>> {
        let public_key_c = try!(CString::new(public_key));
        let zcert = unsafe { czmq_sys::zcertstore_lookup(self.zcertstore, public_key_c.as_ptr()) };

        if zcert == ptr::null_mut() {
            Ok(None)
        } else {
            Ok(Some(unsafe { ZCert::from_raw(zcert, false) }))
        }
    }

    pub fn insert(&self, zcert: ZCert) {
        unsafe { czmq_sys::zcertstore_insert(self.zcertstore, &mut zcert.into_raw()); }
    }

    #[cfg(feature = "draft")]
    pub fn empty(&self) {
        unsafe { czmq_sys::zcertstore_empty(self.zcertstore) };
    }

    pub fn print(&self) {
        unsafe { czmq_sys::zcertstore_print(self.zcertstore) };
    }
}

impl RawInterface<czmq_sys::zcertstore_t> for ZCertStore {
    unsafe fn from_raw(ptr: *mut czmq_sys::zcertstore_t, owned: bool) -> ZCertStore {
        ZCertStore {
            zcertstore: ptr,
            owned: owned,
        }
    }

    fn into_raw(mut self) -> *mut czmq_sys::zcertstore_t {
        self.owned = false;
        self.zcertstore
    }

    fn as_mut_ptr(&mut self) -> *mut czmq_sys::zcertstore_t {
        self.zcertstore
    }
}

#[cfg(feature = "draft")]
unsafe extern "C" fn default_destructor(state: *mut *mut c_void) {
    if state != ptr::null_mut() && *state != ptr::null_mut() {
        Box::from_raw(*state);
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
    use ZCert;
    #[cfg(feature = "draft")]
    use {ZCertStoreRaw, ZSys};
    use super::*;
    use tempdir::TempDir;
    #[cfg(feature = "draft")]
    use zmq::z85_decode;

    #[test]
    fn test_new() {
        let dir = TempDir::new("zcertstore").unwrap();
        assert!(ZCertStore::new(Some(dir.path().to_str().unwrap())).is_ok());
    }

    #[cfg(feature = "draft")]
    #[test]
    fn test_loader() {
        ZSys::init();

        let store = ZCertStore::new(None).unwrap();
        assert!(store.lookup("nonexistent_key").unwrap().is_none()); // Idiot check

        store.set_loader(Some(test_loader_fn));

        let public_key = "abcdefghijklmnopqrstuvwxyzabcdefghijklmn";
        assert_eq!(store.lookup(public_key).unwrap().unwrap().public_txt(), public_key);
    }

    #[test]
    fn test_lookup() {
        let dir = TempDir::new("zcertstore").unwrap();

        let store = ZCertStore::new(Some(dir.path().to_str().unwrap())).unwrap();
        assert!(store.lookup("nonexistent_key").unwrap().is_none());

        let cert = ZCert::new().unwrap();
        cert.save_public(&format!("{}/testcert.crt", dir.path().to_str().unwrap())).unwrap();
        assert_eq!(store.lookup(cert.public_txt()).unwrap().unwrap().public_txt(), cert.public_txt());
    }

    #[test]
    fn test_insert() {
        let dir = TempDir::new("zcertstore").unwrap();

        let store = ZCertStore::new(Some(dir.path().to_str().unwrap())).unwrap();
        let cert = ZCert::new().unwrap();
        let public_txt = cert.public_txt().to_string(); // Make sure we own this

        store.insert(cert);
        assert_eq!(store.lookup(&public_txt).unwrap().unwrap().public_txt(), public_txt);
    }

    #[cfg(feature = "draft")]
    #[test]
    fn test_empty() {
        let dir = TempDir::new("zcertstore").unwrap();

        let store = ZCertStore::new(Some(dir.path().to_str().unwrap())).unwrap();
        let cert = ZCert::new().unwrap();
        let public_txt = cert.public_txt().to_string(); // Make sure we own this

        store.insert(cert);
        assert_eq!(store.lookup(&public_txt).unwrap().unwrap().public_txt(), public_txt);

        store.empty();
        assert!(store.lookup(&public_txt).unwrap().is_none());
    }

    #[cfg(feature = "draft")]
    struct TestState {
        index: u32,
    }

    #[cfg(feature = "draft")]
    unsafe extern "C" fn test_loader_fn(raw: *mut ZCertStoreRaw) {
        let store = ZCertStore::from_raw(raw, false);
        store.empty();
        store.insert(ZCert::new().unwrap());

        let public_key = z85_decode("abcdefghijklmnopqrstuvwxyzabcdefghijklmn").unwrap();
        let secret_key = z85_decode("abcdefghijklmnopqrstuvwxyzabcdefghijklmn").unwrap();

        let cert = ZCert::from_keys(&public_key, &secret_key);
        store.insert(cert);

        // if let Some(mut state) = store.get_state::<TestState>() {
        //     state.index += 1;
        // }
    }
}
