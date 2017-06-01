//! Module: czmq-zcert

use {czmq_sys, Error, ErrorKind, RawInterface, Result, Sockish, zmq, ZList};
use std::{convert, error, fmt, ptr, result, slice, str};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::path::Path;

const KEY_SIZE: usize = 32;

#[derive(Debug, Eq)]
pub struct ZCert {
    zcert: *mut czmq_sys::zcert_t,
    owned: bool,
}

unsafe impl Send for ZCert {}
unsafe impl Sync for ZCert {}

impl Drop for ZCert {
    fn drop(&mut self) {
        if self.owned {
            unsafe { czmq_sys::zcert_destroy(&mut self.zcert) };
        }
    }
}

impl PartialEq for ZCert {
    fn eq(&self, other: &ZCert) -> bool {
        ZCert::eq(self, other)
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
            owned: true,
        })
    }

    pub fn from_keys(public_key: &[u8], secret_key: &[u8]) -> ZCert {
        ZCert {
            zcert: unsafe { czmq_sys::zcert_new_from(public_key.as_mut_ptr(), secret_key.as_mut_ptr()) },
            owned: true,
        }
    }

    pub fn from_txt(public_txt: &str, secret_txt: &str) -> Result<ZCert> {
        let public_key = try!(zmq::z85_decode(public_txt));
        let secret_key = try!(zmq::z85_decode(secret_txt));
        Ok(ZCert::from_keys(&public_key, &secret_key))
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Result<ZCert> {
        let path_c = try!(CString::new(path.as_ref().to_str().unwrap()));
        let zcert = unsafe { czmq_sys::zcert_load(path_c.as_ptr()) };

        if zcert == ptr::null_mut() {
            return Err(Error::new(ErrorKind::NullPtr, ZCertError::InvalidCert(path_c.into_string().unwrap())));
        }

        Ok(ZCert {
            zcert: zcert,
            owned: true,
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

    pub fn meta(&self, key: &str) -> Option<result::Result<String, Vec<u8>>> {
        let key_c = CString::new(key).unwrap_or(CString::new("").unwrap());

        let ptr = unsafe { czmq_sys::zcert_meta(self.zcert, key_c.as_ptr()) };

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

    pub fn meta_keys(&self) -> ZList<&'static str> {
        let ptr = unsafe { czmq_sys::zcert_meta_keys(self.zcert) };
        ZList::<&'static str>::from_raw(ptr)
    }

    /// Encode certificate metadata into ZMQ wire format.
    ///
    /// ```no_run
    /// # use czmq::{ZCert, ZFrame, ZSock};
    /// # let mut sock = ZSock::new_rep("...").unwrap();
    /// let cert = ZCert::new().unwrap();
    /// cert.set_meta("key", "value");
    ///
    /// let encoded = cert.encode_meta();
    /// let frame = ZFrame::new(&encoded).unwrap();
    /// frame.send(&mut sock, None).unwrap();
    /// ```
    pub fn encode_meta(&self) -> Vec<u8> {
        let mut encoded: Vec<u8> = Vec::new();

        for metakey in self.meta_keys() {
            if let Some(Ok(metaval)) = self.meta(metakey) {
                encoded.push(metakey.len() as u8);
                encoded.extend_from_slice(metakey.as_bytes());
                encoded.push(((metaval.len() >> 24) & 0xff) as u8);
                encoded.push(((metaval.len() >> 16) & 0xff) as u8);
                encoded.push(((metaval.len() >> 8) & 0xff) as u8);
                encoded.push((metaval.len() & 0xff) as u8);
                encoded.extend_from_slice(metaval.as_bytes());
            }
        }

        encoded
    }

    /// Decode and set certificate metadata from ZMQ wire format.
    ///
    /// ```no_run
    /// # use czmq::{ZCert, ZFrame, ZSock};
    /// # let mut sock = ZSock::new_rep("...").unwrap();
    /// let frame = ZFrame::recv(&mut sock).unwrap();
    /// let encoded = match frame.data().unwrap() {
    ///     Ok(str) => str.into_bytes(),
    ///     Err(bytes) => bytes,
    /// };
    ///
    /// let cert = ZCert::new().unwrap();
    /// cert.decode_meta(&encoded).unwrap();
    /// ```
    pub fn decode_meta(&self, encoded: &[u8]) -> Result<()> {
        let mut bytes_left = encoded.len();
        let mut index = 0;

        while bytes_left > 1 {
            let name_length: usize = *try!(encoded.get(index).ok_or(Error::new(ErrorKind::InvalidArg, ZCertError::InvalidMetaEncoded))) as usize;
            index += 1;
            bytes_left -= 1;

            if bytes_left < name_length {
                return Err(Error::new(ErrorKind::InvalidArg, ZCertError::InvalidMetaEncoded));
            }

            let name = try!(str::from_utf8(&encoded[index..index + name_length]));
            index += name_length;
            bytes_left -= name_length;

            if bytes_left < 4 {
                return Err(Error::new(ErrorKind::InvalidArg, ZCertError::InvalidMetaEncoded));
            }

            let value_length = (((encoded[index] as u32) << 24) |
                                ((encoded[index + 1] as u32) << 16) |
                                ((encoded[index + 2] as u32) << 8) |
                                  encoded[index + 3] as u32) as usize;
            index += 4;
            bytes_left -= 4;

            if bytes_left < value_length {
                return Err(Error::new(ErrorKind::InvalidArg, ZCertError::InvalidMetaEncoded));
            }

            let value = try!(str::from_utf8(&encoded[index..index + value_length]));
            index += value_length;
            bytes_left -= value_length;

            self.set_meta(name, value);
        }

        Ok(())
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let path_c = try!(CString::new(path.as_ref().to_str().unwrap()));

        unsafe {
            let rc = czmq_sys::zcert_save(self.zcert, path_c.as_ptr());
            if rc == -1 {
                Err(Error::new(ErrorKind::NonZero, ZCertError::SavePath(path_c.into_string().unwrap())))
            } else {
                Ok(())
            }
        }
    }

    pub fn save_public<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let path_c = try!(CString::new(path.as_ref().to_str().unwrap()));

        unsafe {
            let rc = czmq_sys::zcert_save_public(self.zcert, path_c.as_ptr());
            if rc == -1 {
                Err(Error::new(ErrorKind::NonZero, ZCertError::SavePath(path_c.into_string().unwrap())))
            } else {
                Ok(())
            }
        }
    }

    pub fn save_secret<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let path_c = try!(CString::new(path.as_ref().to_str().unwrap()));

        unsafe {
            let rc = czmq_sys::zcert_save_secret(self.zcert, path_c.as_ptr());
            if rc == -1 {
                Err(Error::new(ErrorKind::NonZero, ZCertError::SavePath(path_c.into_string().unwrap())))
            } else {
                Ok(())
            }
        }
    }

    pub fn apply<S: Sockish>(&self, sock: &mut S) {
        unsafe { czmq_sys::zcert_apply(self.zcert, sock.as_mut_ptr()) };
    }

    pub fn dup(&self) -> ZCert {
        let ptr = unsafe { czmq_sys::zcert_dup(self.zcert) };

        ZCert {
            zcert: ptr,
            owned: true,
        }
    }

    pub fn eq(&self, cert: &ZCert) -> bool {
        unsafe { czmq_sys::zcert_eq(self.zcert, cert.zcert) }
    }

    pub fn print(&self) {
        unsafe { czmq_sys::zcert_print(self.zcert) };
    }
}

impl RawInterface<czmq_sys::zcert_t> for ZCert {
    unsafe fn from_raw(ptr: *mut czmq_sys::zcert_t, owned: bool) -> ZCert {
        ZCert {
            zcert: ptr,
            owned: owned,
        }
    }

    fn into_raw(mut self) -> *mut czmq_sys::zcert_t {
        self.owned = false;
        self.zcert
    }

    fn as_mut_ptr(&mut self) -> *mut czmq_sys::zcert_t {
        self.zcert
    }
}

#[derive(Debug)]
pub enum ZCertError {
    Instantiate,
    InvalidCert(String),
    InvalidMetaEncoded,
    SavePath(String),
    ZmqDecode(zmq::DecodeError),
}

impl fmt::Display for ZCertError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ZCertError::Instantiate => write!(f, "Could not instantiate new ZCert struct"),
            ZCertError::InvalidCert(ref e) => write!(f, "Could not open certificate at path: {}", e),
            ZCertError::InvalidMetaEncoded => write!(f, "Encoded metadata is invalid"),
            ZCertError::SavePath(ref e) => write!(f, "Could not save certificate file to path: {}", e),
            ZCertError::ZmqDecode(ref e) => write!(f, "Could not decode Z85 string: {}", e),
        }
    }
}

impl error::Error for ZCertError {
    fn description(&self) -> &str {
        match *self {
            ZCertError::Instantiate => "Could not instantiate new ZCert struct",
            ZCertError::InvalidCert(_) => "Certificate was invalid or non-existent",
            ZCertError::InvalidMetaEncoded => "Encoded metadata is invalid",
            ZCertError::SavePath(_) => "Could not save certificate file to given path",
            ZCertError::ZmqDecode(_) => "Could not decode Z85 string",
        }
    }
}

impl convert::From<zmq::DecodeError> for Error {
    fn from(e: zmq::DecodeError) -> Error {
        Error::new(ErrorKind::StringConversion, ZCertError::ZmqDecode(e))
    }
}

#[cfg(test)]
mod tests {
    use {ZSock, ZSys};
    use super::*;
    use zmq;

    const PUBLIC_TXT: &'static str = "Ko9/&3Uw)$U]Zyp>+4$-i/yaDea2QqlDPGl-&V1s";
    const SECRET_TXT: &'static str = "[MfOo!1^1N}zZY/x{[A6^9>VRC.+O6vX&]zYvDC-";

    #[test]
    fn test_public_key() {
        let cert = create_cert();
        let key = cert.public_key();
        let test_key = zmq::z85_decode(PUBLIC_TXT).unwrap();

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
        let test_key = zmq::z85_decode(SECRET_TXT).unwrap();

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
        cert.set_meta("moo", "cow");

        let mut keys = cert.meta_keys();
        assert_eq!(keys.next().unwrap(), "moo");
    }

    #[test]
    fn test_encode_decode_meta() {
        let cert = create_cert();
        cert.set_meta("moo", "cow");
        cert.set_meta("foobar", "baz");
        cert.set_meta("ka", "BOOM!!");
        let encoded = cert.encode_meta();

        let cert = create_cert();
        cert.decode_meta(&encoded).unwrap();

        assert_eq!(cert.meta("moo").unwrap().unwrap(), "cow");
        assert_eq!(cert.meta("foobar").unwrap().unwrap(), "baz");
        assert_eq!(cert.meta("ka").unwrap().unwrap(), "BOOM!!");
    }

    #[test]
    fn test_apply_zmq() {
        let cert = create_cert();
        let ctx = zmq::Context::new();
        let mut sock = ctx.socket(zmq::REQ).unwrap();
        cert.apply(&mut sock);
        assert_eq!(sock.get_curve_publickey().unwrap().unwrap(), PUBLIC_TXT);
        assert_eq!(sock.get_curve_secretkey().unwrap().unwrap(), SECRET_TXT);
    }

    #[test]
    fn test_apply_zsock() {
        ZSys::init();

        let cert = create_cert();
        let mut sock = ZSock::new_rep("inproc://zcert_test_apply_zsock").unwrap();
        cert.apply(&mut sock);
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
        assert_eq!(c1, c2);
    }

    fn create_cert() -> ZCert {
        ZCert::from_txt(PUBLIC_TXT, SECRET_TXT).unwrap()
    }
}
