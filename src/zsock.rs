//! Module: czmq-zsock

use {czmq_sys, Error, ErrorKind, RawInterface, Result, Sockish, ZMonitor};
use std::{error, fmt, mem, ptr, result};
use std::ffi::{CStr, CString};
use std::os::raw::c_void;

// Duplicate this from rust-zmq to avoid users having to depend on
// both libs.
#[derive(Clone, Debug, PartialEq)]
pub enum ZSockType {
    PAIR   = 0,
    PUB    = 1,
    SUB    = 2,
    REQ    = 3,
    REP    = 4,
    DEALER = 5,
    ROUTER = 6,
    PULL   = 7,
    PUSH   = 8,
    XPUB   = 9,
    XSUB   = 10,
}

// Duplicate this from rust-zmq to avoid users having to depend on
// both libs.
#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq)]
pub enum ZSockMechanism {
    ZMQ_NULL   = 0,
    ZMQ_PLAIN  = 1,
    ZMQ_CURVE  = 2,
    ZMQ_GSSAPI = 3,
}

impl Copy for ZSockMechanism {}

pub struct ZSock {
    zsock: *mut czmq_sys::zsock_t,
    owned: bool,
}

impl Drop for ZSock {
    fn drop(&mut self) {
        if self.owned {
            unsafe { czmq_sys::zsock_destroy(&mut self.zsock) };
        }
    }
}

impl PartialEq for ZSock {
    fn eq(&self, other: &ZSock) -> bool {
        self.zsock == other.zsock
    }
}

unsafe impl Send for ZSock {}

impl ZSock {
    pub fn new(sock_type: ZSockType) -> ZSock {
        ZSock {
            zsock: unsafe { czmq_sys::zsock_new(sock_type as i32) },
            owned: true,
        }
    }

    pub fn new_pub(endpoint: &str) -> Result<ZSock> {
        let zsock = unsafe { czmq_sys::zsock_new_pub(CString::new(endpoint).unwrap().as_ptr()) };

        if zsock == ptr::null_mut() {
            Err(Error::new(ErrorKind::NullPtr, ZSockError::CreateSock))
        } else {
            Ok(ZSock {
                zsock: zsock,
                owned: true,
            })
        }
    }

    pub fn new_sub(endpoint: &str, subscribe: Option<&str>) -> Result<ZSock> {
        let subscribe_ptr = match subscribe {
            Some(s) => try!(CString::new(s)).into_raw(),
            None => ptr::null_mut(),
        };

        let zsock = unsafe { czmq_sys::zsock_new_sub(CString::new(endpoint).unwrap().as_ptr(), subscribe_ptr as *const ::std::os::raw::c_char) };

        if subscribe_ptr != ptr::null_mut() {
            unsafe { CString::from_raw(subscribe_ptr) };
        }

        if zsock == ptr::null_mut() {
            Err(Error::new(ErrorKind::NullPtr, ZSockError::CreateSock))
        } else {
            Ok(ZSock {
                zsock: zsock,
                owned: true,
            })
        }
    }

    pub fn new_req(endpoint: &str) -> Result<ZSock> {
        let zsock = unsafe { czmq_sys::zsock_new_req(CString::new(endpoint).unwrap().as_ptr()) };

        if zsock == ptr::null_mut() {
            Err(Error::new(ErrorKind::NullPtr, ZSockError::CreateSock))
        } else {
            Ok(ZSock {
                zsock: zsock,
                owned: true,
            })
        }
    }

    pub fn new_rep(endpoint: &str) -> Result<ZSock> {
        let zsock = unsafe { czmq_sys::zsock_new_rep(CString::new(endpoint).unwrap().as_ptr()) };

        if zsock == ptr::null_mut() {
            Err(Error::new(ErrorKind::NullPtr, ZSockError::CreateSock))
        } else {
            Ok(ZSock {
                zsock: zsock,
                owned: true,
            })
        }
    }

    pub fn new_dealer(endpoint: &str) -> Result<ZSock> {
        let zsock = unsafe { czmq_sys::zsock_new_dealer(CString::new(endpoint).unwrap().as_ptr()) };

        if zsock == ptr::null_mut() {
            Err(Error::new(ErrorKind::NullPtr, ZSockError::CreateSock))
        } else {
            Ok(ZSock {
                zsock: zsock,
                owned: true,
            })
        }
    }

    pub fn new_router(endpoint: &str) -> Result<ZSock> {
        let zsock = unsafe { czmq_sys::zsock_new_router(CString::new(endpoint).unwrap().as_ptr()) };

        if zsock == ptr::null_mut() {
            Err(Error::new(ErrorKind::NullPtr, ZSockError::CreateSock))
        } else {
            Ok(ZSock {
                zsock: zsock,
                owned: true,
            })
        }
    }

    pub fn new_push(endpoint: &str) -> Result<ZSock> {
        let zsock = unsafe { czmq_sys::zsock_new_push(CString::new(endpoint).unwrap().as_ptr()) };

        if zsock == ptr::null_mut() {
            Err(Error::new(ErrorKind::NullPtr, ZSockError::CreateSock))
        } else {
            Ok(ZSock {
                zsock: zsock,
                owned: true,
            })
        }
    }

    pub fn new_pull(endpoint: &str) -> Result<ZSock> {
        let zsock = unsafe { czmq_sys::zsock_new_pull(CString::new(endpoint).unwrap().as_ptr()) };

        if zsock == ptr::null_mut() {
            Err(Error::new(ErrorKind::NullPtr, ZSockError::CreateSock))
        } else {
            Ok(ZSock {
                zsock: zsock,
                owned: true,
            })
        }
    }

    pub fn new_xpub(endpoint: &str) -> Result<ZSock> {
        let zsock = unsafe { czmq_sys::zsock_new_xpub(CString::new(endpoint).unwrap().as_ptr()) };

        if zsock == ptr::null_mut() {
            Err(Error::new(ErrorKind::NullPtr, ZSockError::CreateSock))
        } else {
            Ok(ZSock {
                zsock: zsock,
                owned: true,
            })
        }
    }

    pub fn new_xsub(endpoint: &str) -> Result<ZSock> {
        let zsock = unsafe { czmq_sys::zsock_new_xsub(CString::new(endpoint).unwrap().as_ptr()) };

        if zsock == ptr::null_mut() {
            Err(Error::new(ErrorKind::NullPtr, ZSockError::CreateSock))
        } else {
            Ok(ZSock {
                zsock: zsock,
                owned: true,
            })
        }
    }

    pub fn new_pair(endpoint: &str) -> Result<ZSock> {
        let zsock = unsafe { czmq_sys::zsock_new_pair(CString::new(endpoint).unwrap().as_ptr()) };

        if zsock == ptr::null_mut() {
            Err(Error::new(ErrorKind::NullPtr, ZSockError::CreateSock))
        } else {
            Ok(ZSock {
                zsock: zsock,
                owned: true,
            })
        }
    }

    pub fn new_stream(endpoint: &str) -> Result<ZSock> {
        let zsock = unsafe { czmq_sys::zsock_new_stream(CString::new(endpoint).unwrap().as_ptr()) };

        if zsock == ptr::null_mut() {
            Err(Error::new(ErrorKind::NullPtr, ZSockError::CreateSock))
        } else {
            Ok(ZSock {
                zsock: zsock,
                owned: true,
            })
        }
    }

    pub fn bind(&self, endpoint: &str) -> Result<i32> {
        let rc = unsafe { czmq_sys::zsock_bind(self.zsock, "%s\0".as_ptr() as *const i8, try!(CString::new(endpoint)).as_ptr() as *const i8) };
        if rc == -1 {
            Err(Error::new(ErrorKind::NonZero, ZSockError::CmdFailed))
        } else {
            Ok(rc)
        }
    }

    pub fn endpoint<'a>(&'a self) -> Result<&'a str> {
        let endpoint_c = unsafe { czmq_sys::zsock_endpoint(self.zsock) };

        if endpoint_c == ptr::null() {
            Err(Error::new(ErrorKind::NonZero, ZSockError::CmdFailed))
        } else {
            let s = try!(unsafe { CStr::from_ptr(endpoint_c) }.to_str());
            Ok(s)
        }
    }

    pub fn unbind(&self, endpoint: &str) -> Result<()> {
        let rc = unsafe { czmq_sys::zsock_unbind(self.zsock, "%s\0".as_ptr() as *const i8, endpoint.as_ptr() as *const i8) };
        if rc == -1 {
            Err(Error::new(ErrorKind::NonZero, ZSockError::CmdFailed))
        } else {
            Ok(())
        }
    }

    pub fn connect(&self, endpoint: &str) -> Result<()> {
        let rc = unsafe { czmq_sys::zsock_connect(self.zsock, "%s\0".as_ptr() as *const i8, try!(CString::new(endpoint)).as_ptr() as *const i8) };
        if rc == -1 {
            Err(Error::new(ErrorKind::NonZero, ZSockError::CmdFailed))
        } else {
            Ok(())
        }
    }

    pub fn disconnect(&self, endpoint: &str) -> Result<()> {
        let rc = unsafe { czmq_sys::zsock_disconnect(self.zsock, "%s\0".as_ptr() as *const i8, endpoint.as_ptr() as *const i8) };
        if rc == -1 {
            Err(Error::new(ErrorKind::NonZero, ZSockError::CmdFailed))
        } else {
            Ok(())
        }
    }

    pub fn attach(&self, endpoints: &[&str], serverish: bool) -> Result<()> {
        let endpoints_c = CString::new(Self::concat_endpoints(endpoints)).unwrap_or(CString::new("").unwrap());

        let rc = unsafe { czmq_sys::zsock_attach(self.zsock, endpoints_c.as_ptr(), if serverish { 1 } else { 0 }) };
        if rc == -1 {
            Err(Error::new(ErrorKind::NonZero, ZSockError::CmdFailed))
        } else {
            Ok(())
        }
    }

    pub fn type_str(&self) -> Result<result::Result<String, Vec<u8>>> {
        let ptr = unsafe { czmq_sys::zsock_type_str(self.zsock) };

        if ptr == ptr::null_mut() {
            Err(Error::new(ErrorKind::NullPtr, ZSockError::CmdFailed))
        } else {
            let c_str = unsafe { CStr::from_ptr(ptr) };
            let bytes = c_str.to_bytes();

            match String::from_utf8(bytes.to_vec()) {
                Ok(s) => Ok(Ok(s)),
                Err(_) => Ok(Err(bytes.to_vec())),
            }
        }
    }

    pub fn send_str(&self, data: &str) -> Result<()> {
        let data_c = CString::new(data).unwrap_or(CString::new("").unwrap());

        let rc = unsafe { czmq_sys::zsock_send(self.zsock as *mut c_void, "s\0".as_ptr() as *const i8, data_c.as_ptr()) };
        if rc == -1 {
            Err(Error::new(ErrorKind::NonZero, ZSockError::CmdFailed))
        } else {
            Ok(())
        }
    }

    pub fn recv_str(&self) -> Result<result::Result<String, Vec<u8>>> {
        let mut ptr = ptr::null();

        let rc = unsafe { czmq_sys::zsock_recv(self.zsock as *mut c_void, "s\0".as_ptr() as *const i8, &mut ptr) };
        if rc == -1 {
            Err(Error::new(ErrorKind::NonZero, ZSockError::CmdFailed))
        } else {
            let c_str = unsafe { CStr::from_ptr(ptr) };
            let bytes = c_str.to_bytes();

            match String::from_utf8(bytes.to_vec()) {
                Ok(s) => Ok(Ok(s)),
                Err(_) => Ok(Err(bytes.to_vec())),
            }
        }
    }

    // pub fn zsock_bsend(_self: *mut ::std::os::raw::c_void,
    //                    picture: *const ::std::os::raw::c_char, ...)
    //  -> ::std::os::raw::c_int;
    // pub fn zsock_brecv(_self: *mut ::std::os::raw::c_void,
    //                    picture: *const ::std::os::raw::c_char, ...)
    //  -> ::std::os::raw::c_int;
    // pub fn zsock_set_unbounded(_self: *mut ::std::os::raw::c_void);
    // pub fn zsock_signal(_self: *mut ::std::os::raw::c_void, status: byte)
    //  -> ::std::os::raw::c_int;
    pub fn signal(&self, status: u8) -> Result<()> {
        let rc = unsafe { czmq_sys::zsock_signal(self.zsock as *mut c_void, status) };
        if rc == -1 {
            Err(Error::new(ErrorKind::NonZero, ZSockError::CmdFailed))
        } else {
            Ok(())
        }
    }

    pub fn wait(&self) -> Result<()> {
        let rc = unsafe { czmq_sys::zsock_wait(self.zsock as *mut c_void) };
        if rc == -1 {
            Err(Error::new(ErrorKind::NonZero, ZSockError::CmdFailed))
        } else {
            Ok(())
        }
    }

    pub fn flush(&self) {
        unsafe { czmq_sys::zsock_flush(self.zsock as *mut c_void) };
    }

    // pub fn zsock_is(_self: *mut ::std::os::raw::c_void) -> u8;
    // pub fn zsock_resolve(_self: *mut ::std::os::raw::c_void)
    //  -> *mut ::std::os::raw::c_void;
    // pub fn zsock_tos(_self: *mut ::std::os::raw::c_void)
    //  -> ::std::os::raw::c_int;
    // pub fn zsock_set_tos(_self: *mut ::std::os::raw::c_void,
    //                      tos: ::std::os::raw::c_int);
    // pub fn zsock_set_router_handover(_self: *mut ::std::os::raw::c_void,
    //                                  router_handover: ::std::os::raw::c_int);
    // pub fn zsock_set_router_mandatory(_self: *mut ::std::os::raw::c_void,
    //                                   router_mandatory:
    //                                       ::std::os::raw::c_int);
    // pub fn zsock_set_probe_router(_self: *mut ::std::os::raw::c_void,
    //                               probe_router: ::std::os::raw::c_int);
    // pub fn zsock_set_req_relaxed(_self: *mut ::std::os::raw::c_void,
    //                              req_relaxed: ::std::os::raw::c_int);
    // pub fn zsock_set_req_correlate(_self: *mut ::std::os::raw::c_void,
    //                                req_correlate: ::std::os::raw::c_int);
    // pub fn zsock_set_conflate(_self: *mut ::std::os::raw::c_void,
    //                           conflate: ::std::os::raw::c_int);

    pub fn zap_domain<'a>(&'a self) -> Result<result::Result<String, Vec<u8>>> {
        let domain = unsafe { czmq_sys::zsock_zap_domain(self.zsock as *mut c_void) };

        if domain == ptr::null_mut() {
            Err(Error::new(ErrorKind::NullPtr, ZSockError::CmdFailed))
        } else {
            let c_str = unsafe { CStr::from_ptr(domain) };
            let bytes = c_str.to_bytes();

            match String::from_utf8(bytes.to_vec()) {
                Ok(s) => Ok(Ok(s)),
                Err(_) => Ok(Err(bytes.to_vec())),
            }
        }
    }

    pub fn set_zap_domain(&self, zap_domain: &str) {
        let zap_domain_c = CString::new(zap_domain).unwrap_or(CString::new("").unwrap()).into_raw();
        unsafe {
            czmq_sys::zsock_set_zap_domain(self.zsock as *mut c_void, zap_domain_c);
            CString::from_raw(zap_domain_c);
        }
    }

    pub fn mechanism(&self) -> Result<ZSockMechanism> {
        let mechanism = unsafe { czmq_sys::zsock_mechanism(self.zsock as *mut c_void) };

        if mechanism == -1 {
            Err(Error::new(ErrorKind::NonZero, ZSockError::CmdFailed))
        } else {
            match mechanism {
                0 => Ok(ZSockMechanism::ZMQ_NULL),
                1 => Ok(ZSockMechanism::ZMQ_PLAIN),
                2 => Ok(ZSockMechanism::ZMQ_CURVE),
                3 => Ok(ZSockMechanism::ZMQ_GSSAPI),
                _ => unimplemented!(),
            }
        }
    }

    pub fn plain_server(&self) -> bool {
        unsafe { czmq_sys::zsock_plain_server(self.zsock as *mut c_void) == 1 }
    }

    pub fn set_plain_server(&self, plain: bool) {
        unsafe { czmq_sys::zsock_set_plain_server(self.zsock as *mut c_void, if plain { 1 } else { 0 }) };
    }

    pub fn plain_username<'a>(&'a self) -> Result<result::Result<String, Vec<u8>>> {
        let username = unsafe { czmq_sys::zsock_plain_username(self.zsock as *mut c_void) };

        if username == ptr::null_mut() {
            Err(Error::new(ErrorKind::NullPtr, ZSockError::CmdFailed))
        } else {
            let c_str = unsafe { CStr::from_ptr(username) };
            let bytes = c_str.to_bytes();

            match String::from_utf8(bytes.to_vec()) {
                Ok(s) => Ok(Ok(s)),
                Err(_) => Ok(Err(bytes.to_vec())),
            }
        }
    }

    pub fn set_plain_username(&self, username: &str) {
        let username_c = CString::new(username).unwrap_or(CString::new("").unwrap()).into_raw();
        unsafe {
            czmq_sys::zsock_set_plain_username(self.zsock as *mut c_void, username_c);
            CString::from_raw(username_c);
        }
    }

    pub fn plain_password<'a>(&'a self) -> Result<result::Result<String, Vec<u8>>> {
        let password = unsafe { czmq_sys::zsock_plain_password(self.zsock as *mut c_void) };

        if password == ptr::null_mut() {
            Err(Error::new(ErrorKind::NullPtr, ZSockError::CmdFailed))
        } else {
            let c_str = unsafe { CStr::from_ptr(password) };
            let bytes = c_str.to_bytes();

            match String::from_utf8(bytes.to_vec()) {
                Ok(s) => Ok(Ok(s)),
                Err(_) => Ok(Err(bytes.to_vec())),
            }
        }
    }

    pub fn set_plain_password(&self, password: &str) {
        let password_c = CString::new(password).unwrap_or(CString::new("").unwrap()).into_raw();
        unsafe {
            czmq_sys::zsock_set_plain_password(self.zsock as *mut c_void, password_c);
            CString::from_raw(password_c);
        }
    }

    pub fn curve_server(&self) -> bool {
        unsafe { czmq_sys::zsock_curve_server(self.zsock as *mut c_void) == 1 }
    }

    pub fn set_curve_server(&self, curve: bool) {
        unsafe { czmq_sys::zsock_set_curve_server(self.zsock as *mut c_void, if curve { 1 } else { 0 }) };
    }

    pub fn curve_publickey<'a>(&'a self) -> Result<result::Result<String, Vec<u8>>> {
        let key = unsafe { czmq_sys::zsock_curve_publickey(self.zsock as *mut c_void) };

        if key == ptr::null_mut() {
            Err(Error::new(ErrorKind::NullPtr, ZSockError::CmdFailed))
        } else {
            let c_str = unsafe { CStr::from_ptr(key) };
            let bytes = c_str.to_bytes();

            match String::from_utf8(bytes.to_vec()) {
                Ok(s) => Ok(Ok(s)),
                Err(_) => Ok(Err(bytes.to_vec())),
            }
        }
    }

    pub fn set_curve_publickey(&self, key: &str) {
        let key_c = CString::new(key).unwrap_or(CString::new("").unwrap()).into_raw();
        unsafe {
            czmq_sys::zsock_set_curve_publickey(self.zsock as *mut c_void, key_c);
            CString::from_raw(key_c);
        }
    }

    pub fn set_curve_publickey_bin(&self, key: &[u8]) {
        unsafe { czmq_sys::zsock_set_curve_publickey_bin(self.zsock as *mut c_void, key.as_ptr()) };
    }

    pub fn curve_secretkey<'a>(&'a self) -> Result<result::Result<String, Vec<u8>>> {
        let key = unsafe { czmq_sys::zsock_curve_secretkey(self.zsock as *mut c_void) };

        if key == ptr::null_mut() {
            Err(Error::new(ErrorKind::NullPtr, ZSockError::CmdFailed))
        } else {
            let c_str = unsafe { CStr::from_ptr(key) };
            let bytes = c_str.to_bytes();

            match String::from_utf8(bytes.to_vec()) {
                Ok(s) => Ok(Ok(s)),
                Err(_) => Ok(Err(bytes.to_vec())),
            }
        }
    }

    pub fn set_curve_secretkey(&self, key: &str) {
        let key_c = CString::new(key).unwrap_or(CString::new("").unwrap()).into_raw();
        unsafe {
            czmq_sys::zsock_set_curve_secretkey(self.zsock as *mut c_void, key_c);
            CString::from_raw(key_c);
        }
    }

    pub fn set_curve_secretkey_bin(&self, key: &[u8]) {
        unsafe { czmq_sys::zsock_set_curve_secretkey_bin(self.zsock as *mut c_void, key.as_ptr()) };
    }

    pub fn curve_serverkey<'a>(&'a self) -> Result<result::Result<String, Vec<u8>>> {
        let key = unsafe { czmq_sys::zsock_curve_serverkey(self.zsock as *mut c_void) };

        if key == ptr::null_mut() {
            Err(Error::new(ErrorKind::NullPtr, ZSockError::CmdFailed))
        } else {
            let c_str = unsafe { CStr::from_ptr(key) };
            let bytes = c_str.to_bytes();

            match String::from_utf8(bytes.to_vec()) {
                Ok(s) => Ok(Ok(s)),
                Err(_) => Ok(Err(bytes.to_vec())),
            }
        }
    }

    pub fn set_curve_serverkey(&self, key: &str) {
        let key_c = CString::new(key).unwrap_or(CString::new("").unwrap()).into_raw();
        unsafe {
            czmq_sys::zsock_set_curve_serverkey(self.zsock as *mut c_void, key_c);
            CString::from_raw(key_c);
        }
    }

    pub fn set_curve_serverkey_bin(&self, key: &[u8]) {
        unsafe { czmq_sys::zsock_set_curve_serverkey_bin(self.zsock as *mut c_void, key.as_ptr()) };
    }

    // pub fn gssapi_server(&self) -> bool {
    //     unsafe { czmq_sys::zsock_gssapi_server(self.zsock as *mut c_void) == 1 }
    // }
    //
    // pub fn set_gssapi_server(&self, gssapi: bool) {
    //     unsafe { czmq_sys::zsock_set_gssapi_server(self.zsock as *mut c_void, if gssapi { 1 } else { 0 }) };
    // }

    // pub fn zsock_gssapi_plaintext(_self: *mut ::std::os::raw::c_void)
    //  -> ::std::os::raw::c_int;
    // pub fn zsock_set_gssapi_plaintext(_self: *mut ::std::os::raw::c_void,
    //                                   gssapi_plaintext:
    //                                       ::std::os::raw::c_int);
    // pub fn zsock_gssapi_principal(_self: *mut ::std::os::raw::c_void)
    //  -> *mut ::std::os::raw::c_char;
    // pub fn zsock_set_gssapi_principal(_self: *mut ::std::os::raw::c_void,
    //                                   gssapi_principal:
    //                                       *const ::std::os::raw::c_char);
    // pub fn zsock_gssapi_service_principal(_self: *mut ::std::os::raw::c_void)
    //  -> *mut ::std::os::raw::c_char;
    // pub fn zsock_set_gssapi_service_principal(_self:
    //                                               *mut ::std::os::raw::c_void,
    //                                           gssapi_service_principal:
    //                                               *const ::std::os::raw::c_char);
    // pub fn zsock_ipv6(_self: *mut ::std::os::raw::c_void)
    //  -> ::std::os::raw::c_int;
    // pub fn zsock_set_ipv6(_self: *mut ::std::os::raw::c_void,
    //                       ipv6: ::std::os::raw::c_int);
    // pub fn zsock_immediate(_self: *mut ::std::os::raw::c_void)
    //  -> ::std::os::raw::c_int;
    // pub fn zsock_set_immediate(_self: *mut ::std::os::raw::c_void,
    //                            immediate: ::std::os::raw::c_int);
    // pub fn zsock_set_router_raw(_self: *mut ::std::os::raw::c_void,
    //                             router_raw: ::std::os::raw::c_int);
    // pub fn zsock_ipv4only(_self: *mut ::std::os::raw::c_void)
    //  -> ::std::os::raw::c_int;
    // pub fn zsock_set_ipv4only(_self: *mut ::std::os::raw::c_void,
    //                           ipv4only: ::std::os::raw::c_int);
    // pub fn zsock_set_delay_attach_on_connect(_self:
    //                                              *mut ::std::os::raw::c_void,
    //                                          delay_attach_on_connect:
    //                                              ::std::os::raw::c_int);
    // pub fn zsock_type(_self: *mut ::std::os::raw::c_void)
    //  -> ::std::os::raw::c_int;

    pub fn sndhwm(&self) -> Result<i32> {
        let sndhwm = unsafe { czmq_sys::zsock_sndhwm(self.zsock as *mut c_void) };

        if sndhwm == -1 {
            Err(Error::new(ErrorKind::NonZero, ZSockError::CmdFailed))
        } else {
            Ok(sndhwm)
        }
    }

    pub fn set_sndhwm(&self, sndhwm: i32) {
        unsafe { czmq_sys::zsock_set_sndhwm(self.zsock as *mut c_void, sndhwm) };
    }

    pub fn rcvhwm(&self) -> Result<i32> {
        let rcvhwm = unsafe { czmq_sys::zsock_rcvhwm(self.zsock as *mut c_void) };

        if rcvhwm == -1 {
            Err(Error::new(ErrorKind::NonZero, ZSockError::CmdFailed))
        } else {
            Ok(rcvhwm)
        }
    }

    pub fn set_rcvhwm(&self, rcvhwm: i32) {
        unsafe { czmq_sys::zsock_set_rcvhwm(self.zsock as *mut c_void, rcvhwm) };
    }

    // pub fn zsock_affinity(_self: *mut ::std::os::raw::c_void)
    //  -> ::std::os::raw::c_int;
    // pub fn zsock_set_affinity(_self: *mut ::std::os::raw::c_void,
    //                           affinity: ::std::os::raw::c_int);

    pub fn set_subscribe(&self, subscribe: &str) {
        let subscribe_c = CString::new(subscribe).unwrap_or(CString::new("").unwrap()).into_raw();
        unsafe {
            czmq_sys::zsock_set_subscribe(self.zsock as *mut c_void, subscribe_c);
            CString::from_raw(subscribe_c);
        }
    }

    pub fn set_unsubscribe(&self, unsubscribe: &str) {
        let unsubscribe_c = CString::new(unsubscribe).unwrap_or(CString::new("").unwrap()).into_raw();
        unsafe {
            czmq_sys::zsock_set_unsubscribe(self.zsock as *mut c_void, unsubscribe_c);
            CString::from_raw(unsubscribe_c);
        }
    }

    pub fn identity<'a>(&'a self) -> Result<result::Result<&'a str, &'a [u8]>> {
        let ptr = unsafe { czmq_sys::zsock_identity(self.zsock as *mut c_void) };

        if ptr == ptr::null_mut() {
            Err(Error::new(ErrorKind::NullPtr, ZSockError::CmdFailed))
        } else {
            let c_str = unsafe { CStr::from_ptr(ptr) };
            let bytes = c_str.to_bytes();
            match c_str.to_str() {
                Ok(s) => Ok(Ok(s)),
                Err(_) => Ok(Err(bytes)),
            }
        }
    }

    pub fn set_identity(&self, identity: &str) -> Result<()> {
        let identity_c = try!(CString::new(identity));
        unsafe { czmq_sys::zsock_set_identity(self.zsock as *mut c_void, identity_c.as_ptr()) };

        // Deliberately leak this memory, which will be managed by C
        mem::forget(identity_c);
        Ok(())
    }

    // pub fn zsock_rate(_self: *mut ::std::os::raw::c_void)
    //  -> ::std::os::raw::c_int;
    // pub fn zsock_set_rate(_self: *mut ::std::os::raw::c_void,
    //                       rate: ::std::os::raw::c_int);
    // pub fn zsock_recovery_ivl(_self: *mut ::std::os::raw::c_void)
    //  -> ::std::os::raw::c_int;
    // pub fn zsock_set_recovery_ivl(_self: *mut ::std::os::raw::c_void,
    //                               recovery_ivl: ::std::os::raw::c_int);
    // pub fn zsock_sndbuf(_self: *mut ::std::os::raw::c_void)
    //  -> ::std::os::raw::c_int;
    // pub fn zsock_set_sndbuf(_self: *mut ::std::os::raw::c_void,
    //                         sndbuf: ::std::os::raw::c_int);
    // pub fn zsock_rcvbuf(_self: *mut ::std::os::raw::c_void)
    //  -> ::std::os::raw::c_int;
    // pub fn zsock_set_rcvbuf(_self: *mut ::std::os::raw::c_void,
    //                         rcvbuf: ::std::os::raw::c_int);

    pub fn linger(&self) -> Result<i32> {
        let linger = unsafe { czmq_sys::zsock_linger(self.zsock as *mut c_void) };

        if linger == -1 {
            Err(Error::new(ErrorKind::NonZero, ZSockError::CmdFailed))
        } else {
            Ok(linger)
        }
    }

    pub fn set_linger(&self, linger: i32) {
        unsafe { czmq_sys::zsock_set_linger(self.zsock as *mut c_void, linger) };
    }

    // pub fn zsock_reconnect_ivl(_self: *mut ::std::os::raw::c_void)
    //  -> ::std::os::raw::c_int;
    // pub fn zsock_set_reconnect_ivl(_self: *mut ::std::os::raw::c_void,
    //                                reconnect_ivl: ::std::os::raw::c_int);
    // pub fn zsock_reconnect_ivl_max(_self: *mut ::std::os::raw::c_void)
    //  -> ::std::os::raw::c_int;
    // pub fn zsock_set_reconnect_ivl_max(_self: *mut ::std::os::raw::c_void,
    //                                    reconnect_ivl_max:
    //                                        ::std::os::raw::c_int);
    // pub fn zsock_backlog(_self: *mut ::std::os::raw::c_void)
    //  -> ::std::os::raw::c_int;
    // pub fn zsock_set_backlog(_self: *mut ::std::os::raw::c_void,
    //                          backlog: ::std::os::raw::c_int);
    // pub fn zsock_maxmsgsize(_self: *mut ::std::os::raw::c_void)
    //  -> ::std::os::raw::c_int;
    // pub fn zsock_set_maxmsgsize(_self: *mut ::std::os::raw::c_void,
    //                             maxmsgsize: ::std::os::raw::c_int);
    // pub fn zsock_multicast_hops(_self: *mut ::std::os::raw::c_void)
    //  -> ::std::os::raw::c_int;
    // pub fn zsock_set_multicast_hops(_self: *mut ::std::os::raw::c_void,
    //                                 multicast_hops: ::std::os::raw::c_int);

    pub fn rcvtimeo(&self) -> Option<i32> {
        let timeout = unsafe { czmq_sys::zsock_rcvtimeo(self.zsock as *mut c_void) };

        if timeout == -1 {
            None
        } else {
            Some(timeout)
        }
    }

    pub fn set_rcvtimeo(&self, timeout: Option<i32>) {
        unsafe { czmq_sys::zsock_set_rcvtimeo(self.zsock as *mut c_void, timeout.unwrap_or(-1)) };
    }

    pub fn sndtimeo(&self) -> Option<i32> {
        let timeout = unsafe { czmq_sys::zsock_sndtimeo(self.zsock as *mut c_void) };

        if timeout == -1 {
            None
        } else {
            Some(timeout)
        }
    }

    pub fn set_sndtimeo(&self, timeout: Option<i32>) {
        unsafe { czmq_sys::zsock_set_sndtimeo(self.zsock as *mut c_void, timeout.unwrap_or(-1)) };
    }

    pub fn set_xpub_verbose(&self, verbose: bool) {
        unsafe { czmq_sys::zsock_set_xpub_verbose(self.zsock as *mut c_void, if verbose { 1 } else { 0 }) };
    }

    // pub fn zsock_tcp_keepalive(_self: *mut ::std::os::raw::c_void)
    //  -> ::std::os::raw::c_int;
    // pub fn zsock_set_tcp_keepalive(_self: *mut ::std::os::raw::c_void,
    //                                tcp_keepalive: ::std::os::raw::c_int);
    // pub fn zsock_tcp_keepalive_idle(_self: *mut ::std::os::raw::c_void)
    //  -> ::std::os::raw::c_int;
    // pub fn zsock_set_tcp_keepalive_idle(_self: *mut ::std::os::raw::c_void,
    //                                     tcp_keepalive_idle:
    //                                         ::std::os::raw::c_int);
    // pub fn zsock_tcp_keepalive_cnt(_self: *mut ::std::os::raw::c_void)
    //  -> ::std::os::raw::c_int;
    // pub fn zsock_set_tcp_keepalive_cnt(_self: *mut ::std::os::raw::c_void,
    //                                    tcp_keepalive_cnt:
    //                                        ::std::os::raw::c_int);
    // pub fn zsock_tcp_keepalive_intvl(_self: *mut ::std::os::raw::c_void)
    //  -> ::std::os::raw::c_int;
    // pub fn zsock_set_tcp_keepalive_intvl(_self: *mut ::std::os::raw::c_void,
    //                                      tcp_keepalive_intvl:
    //                                          ::std::os::raw::c_int);
    // pub fn zsock_tcp_accept_filter(_self: *mut ::std::os::raw::c_void)
    //  -> *mut ::std::os::raw::c_char;
    // pub fn zsock_set_tcp_accept_filter(_self: *mut ::std::os::raw::c_void,
    //                                    tcp_accept_filter:
    //                                        *const ::std::os::raw::c_char);

    pub fn rcvmore(&self) -> bool {
        unsafe { czmq_sys::zsock_rcvmore(self.zsock as *mut c_void) == 1 }
    }

    // pub fn zsock_fd(_self: *mut ::std::os::raw::c_void) -> SOCKET;
    // pub fn zsock_events(_self: *mut ::std::os::raw::c_void)
    //  -> ::std::os::raw::c_int;
    // pub fn zsock_last_endpoint(_self: *mut ::std::os::raw::c_void)
    //  -> *mut ::std::os::raw::c_char;

    pub fn monitor(&self) -> Result<ZMonitor> {
        ZMonitor::new(self)
    }

    fn concat_endpoints(endpoints: &[&str]) -> String {
        let mut endpoint_str = String::new();
        let mut iter = 0;
        for e in endpoints {
            endpoint_str.push_str(e);

            if iter < endpoints.len() {
                endpoint_str.push_str(",");
            }

            iter += 1;
        }

        endpoint_str
    }
}

impl RawInterface<c_void> for ZSock {
    fn from_raw(ptr: *mut c_void, owned: bool) -> ZSock {
        ZSock {
            zsock: ptr as *mut czmq_sys::zsock_t,
            owned: owned,
        }
    }

    fn into_raw(mut self) -> *mut c_void {
        self.owned = false;
        self.zsock as *mut c_void
    }

    fn borrow_raw(&self) -> *mut c_void {
        self.zsock as *mut c_void
    }
}

impl Sockish for ZSock {}

#[derive(Debug)]
pub enum ZSockError {
    CreateSock,
    CmdFailed,
}

impl fmt::Display for ZSockError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ZSockError::CreateSock => write!(f, "Could not create socket"),
            ZSockError::CmdFailed => write!(f, "Socket command failed"),
        }
    }
}

impl error::Error for ZSockError {
    fn description(&self) -> &str {
        match *self {
            ZSockError::CreateSock => "Could not create socket",
            ZSockError::CmdFailed => "Socket command failed",
        }
    }
}

#[cfg(test)]
mod tests {
    use std::thread::sleep;
    use std::time::Duration;
    use super::*;
    use {zmq, ZFrame, ZMsg, ZSys};

    #[test]
    fn test_new_pub() {
        ZSys::init();

        let zsock = ZSock::new_pub("inproc://test_pub");
        assert!(zsock.is_ok());
    }

    #[test]
    fn test_new_sub() {
        ZSys::init();

        let zsock = ZSock::new_sub("inproc://test_sub1", None);
        assert!(zsock.is_ok());
        let zsock = ZSock::new_sub("inproc://test_sub2", Some("moo"));
        assert!(zsock.is_ok());
    }

    #[test]
    fn test_pubsub() {
        ZSys::init();

        let zpub = ZSock::new_pub("inproc://zsock_test_pubsub").unwrap();
        let zsub = ZSock::new_sub("inproc://zsock_test_pubsub", Some("moo")).unwrap();

        sleep(Duration::from_millis(200));

        let outgoing = ZMsg::new();
        outgoing.addstr("moo").unwrap();
        outgoing.addstr("cow").unwrap();
        outgoing.send(&zpub).unwrap();

        sleep(Duration::from_millis(200));

        let incoming = ZMsg::recv(&zsub).unwrap();
        assert_eq!(incoming.popstr().unwrap().unwrap(), "moo");
        assert_eq!(incoming.popstr().unwrap().unwrap(), "cow");
    }

    #[test]
    fn test_new_req() {
        ZSys::init();

        let zsock = ZSock::new_req("inproc://test_req");
        assert!(zsock.is_ok());
    }

    #[test]
    fn test_new_rep() {
        ZSys::init();

        let zsock = ZSock::new_rep("inproc://test_rep");
        assert!(zsock.is_ok());
    }

    #[test]
    fn test_new_dealer() {
        ZSys::init();

        let zsock = ZSock::new_dealer("inproc://test_dealer");
        assert!(zsock.is_ok());
    }

    #[test]
    fn test_new_router() {
        ZSys::init();

        let zsock = ZSock::new_router("inproc://test_router");
        assert!(zsock.is_ok());
    }

    #[test]
    fn test_new_push() {
        ZSys::init();

        let zsock = ZSock::new_push("inproc://test_push");
        assert!(zsock.is_ok());
    }

    #[test]
    fn test_new_pull() {
        ZSys::init();

        let zsock = ZSock::new_pull("inproc://test_pull");
        assert!(zsock.is_ok());
    }

    #[test]
    fn test_new_xpub() {
        ZSys::init();

        let zsock = ZSock::new_xpub("inproc://test_xpub");
        assert!(zsock.is_ok());
    }

    #[test]
    fn test_new_xsub() {
        ZSys::init();
        let zsock = ZSock::new_xsub("inproc://test_xsub");
        assert!(zsock.is_ok());
    }

    #[test]
    fn test_new_pair() {
        ZSys::init();

        let zsock = ZSock::new_pair("inproc://test_pair");
        assert!(zsock.is_ok());
    }

    #[test]
    fn test_new_stream() {
        ZSys::init();

        let zsock = ZSock::new_stream("inproc://test_stream");
        assert!(zsock.is_ok());
    }

    #[test]
    fn test_bind() {
        ZSys::init();

        let zsock = ZSock::new(ZSockType::REP);
        assert!(zsock.bind("inproc://test_").is_ok());
    }

    #[test]
    fn test_endpoint() {
        ZSys::init();

        let zsock = ZSock::new_rep("inproc://test_endpoint").unwrap();
        assert_eq!(zsock.endpoint().unwrap(), "inproc://test_endpoint");
    }

    #[test]
    fn test_unbind() {
        ZSys::init();

        let zsock = ZSock::new_rep("inproc://test_unbind").unwrap();
        assert!(zsock.unbind("inproc://test_unbind").is_ok());
    }

    #[test]
    fn test_connect() {
        ZSys::init();

        ZSock::new_rep("inproc://test_connect").unwrap();
        let client = ZSock::new(ZSockType::REQ);
        assert!(client.connect("inproc://test_connect").is_ok());
    }

    #[test]
    fn test_disconnect() {
        ZSys::init();

        let zsock = ZSock::new_req("inproc://test_disconnect").unwrap();
        zsock.connect("inproc://test_disconnect1").unwrap();
        assert!(zsock.disconnect("inproc://test_disconnect").is_ok());
    }

    #[test]
    fn test_attach() {
        ZSys::init();

        let zsock = ZSock::new(ZSockType::REP);
        let result = zsock.attach(&["inproc://test_attach1", "inproc://test_attach2", "inproc://test_attach3"], true);
        assert!(result.is_ok());
    }

    #[test]
    fn test_type_str() {
        ZSys::init();

        let zsock = ZSock::new(ZSockType::PUB);
        assert_eq!(zsock.type_str().unwrap().unwrap(), "PUB");
    }

    #[test]
    fn test_sendrecv() {
        ZSys::init();

        let server = ZSock::new_rep("inproc://test_send").unwrap();
        let client = ZSock::new_req("inproc://test_send").unwrap();

        assert!(client.send_str("This is a test string.").is_ok());
        assert_eq!(server.recv_str().unwrap().unwrap(), "This is a test string.");
    }

    #[test]
    fn test_zap_domain() {
        ZSys::init();

        let zsock = ZSock::new(ZSockType::REP);
        zsock.set_zap_domain("test");
        assert_eq!(zsock.zap_domain().unwrap().unwrap(), "test");
    }

    #[test]
    fn test_signal_wait() {
        ZSys::init();

        let server = ZSock::new_pull("inproc://zsock_wait").unwrap();
        let client = ZSock::new_push("inproc://zsock_wait").unwrap();

        client.signal(0).unwrap();
        assert!(server.wait().is_ok());
    }

    #[test]
    fn test_flush() {
        ZSys::init();

        let server = ZSock::new_rep("inproc://zsock_flush").unwrap();
        let client = ZSock::new_req("inproc://zsock_flush").unwrap();

        let msg = ZMsg::new();
        msg.addstr("one").unwrap();
        msg.addstr("two").unwrap();
        msg.send(&client).unwrap();

        ZFrame::recv(&server).unwrap();
        server.flush();
        assert!(server.recv_str().is_err());
    }

    #[test]
    fn test_rcvtimeo() {
        ZSys::init();

        let zsock = ZSock::new(ZSockType::REP);
        zsock.set_rcvtimeo(Some(2000));
        assert_eq!(zsock.rcvtimeo().unwrap(), 2000);
    }

    #[test]
    fn test_sndtimeo() {
        ZSys::init();

        let zsock = ZSock::new(ZSockType::REP);
        zsock.set_sndtimeo(Some(2000));
        assert_eq!(zsock.sndtimeo().unwrap(), 2000);
    }

    #[test]
    fn test_sndhwm() {
        ZSys::init();

        let zsock = ZSock::new(ZSockType::REP);
        zsock.set_sndhwm(2000);
        assert_eq!(zsock.sndhwm().unwrap(), 2000);
    }

    #[test]
    fn test_rcvhwm() {
        ZSys::init();

        let zsock = ZSock::new(ZSockType::REP);
        zsock.set_rcvhwm(2000);
        assert_eq!(zsock.rcvhwm().unwrap(), 2000);
    }

    #[test]
    fn test_subscribe() {
        ZSys::init();

        let publisher = ZSock::new_pub("inproc://zsock_test_subscribe").unwrap();
        let subscriber = ZSock::new(ZSockType::SUB);
        subscriber.set_rcvtimeo(Some(200));
        subscriber.connect("inproc://zsock_test_subscribe").unwrap();

        // Wait for subscriber to connect
        sleep(Duration::from_millis(200));

        // Test subscribe prefix
        subscriber.set_subscribe("m");

        let msg = ZMsg::new();
        msg.addstr("moo").unwrap();
        msg.send(&publisher).unwrap();

        assert_eq!(subscriber.recv_str().unwrap().unwrap(), "moo");

        // Test no subscription
        subscriber.set_unsubscribe("m");

        let msg = ZMsg::new();
        msg.addstr("moo").unwrap();
        msg.send(&publisher).unwrap();

        assert!(subscriber.recv_str().is_err());

        // Test blank subscription (thus subscribe to all)
        subscriber.set_subscribe("");

        let msg = ZMsg::new();
        msg.addstr("moo").unwrap();
        msg.send(&publisher).unwrap();

        assert_eq!(subscriber.recv_str().unwrap().unwrap(), "moo");
    }

    #[test]
    fn test_identity() {
        ZSys::init();

        let zsock = ZSock::new(ZSockType::REP);
        zsock.set_identity("moo").unwrap();
        assert_eq!(zsock.identity().unwrap().unwrap(), "moo");
    }

    #[test]
    fn test_linger() {
        ZSys::init();

        let zsock = ZSock::new(ZSockType::REP);
        zsock.set_linger(2000);
        assert_eq!(zsock.linger().unwrap(), 2000);
    }

    #[test]
    fn test_mechanism() {
        ZSys::init();

        let zsock = ZSock::new(ZSockType::REP);
        assert_eq!(zsock.mechanism().unwrap(), ZSockMechanism::ZMQ_NULL);

        zsock.set_plain_server(true);
        assert_eq!(zsock.mechanism().unwrap(), ZSockMechanism::ZMQ_PLAIN);

        zsock.set_curve_server(true);
        assert_eq!(zsock.mechanism().unwrap(), ZSockMechanism::ZMQ_CURVE);

        // zsock.set_gssapi_server(true);
        // assert_eq!(zsock.mechanism().unwrap(), ZSockMechanism::ZMQ_GSSAPI);
    }

    #[test]
    fn test_plain_server() {
        ZSys::init();

        let zsock = ZSock::new(ZSockType::REP);
        assert!(!zsock.plain_server());
        zsock.set_plain_server(true);
        assert!(zsock.plain_server());
    }

    #[test]
    fn test_plain_username() {
        ZSys::init();

        let zsock = ZSock::new(ZSockType::REP);
        zsock.set_plain_username("jnrvicepresident");
        assert_eq!(zsock.plain_username().unwrap().unwrap(), "jnrvicepresident");
    }

    #[test]
    fn test_plain_password() {
        ZSys::init();

        let zsock = ZSock::new(ZSockType::REP);
        zsock.set_plain_password("ohtheinternet'soncomputersnow");
        assert_eq!(zsock.plain_password().unwrap().unwrap(), "ohtheinternet'soncomputersnow");
    }

    #[test]
    fn test_curve_server() {
        ZSys::init();

        let zsock = ZSock::new(ZSockType::REP);
        assert!(!zsock.curve_server());
        zsock.set_curve_server(true);
        assert!(zsock.curve_server());
    }

    #[test]
    fn test_curve_keys() {
        ZSys::init();

        let zsock = ZSock::new(ZSockType::REP);
        let keypair = zmq::CurveKeypair::new().unwrap();

        zsock.set_curve_publickey(&keypair.public_key);
        assert_eq!(&zsock.curve_publickey().unwrap().unwrap(), &keypair.public_key);
        zsock.set_curve_publickey_bin(&zmq::z85_decode(&keypair.public_key));
        assert_eq!(&zsock.curve_publickey().unwrap().unwrap(), &keypair.public_key);

        zsock.set_curve_secretkey(&keypair.secret_key);
        assert_eq!(&zsock.curve_secretkey().unwrap().unwrap(), &keypair.secret_key);
        zsock.set_curve_secretkey_bin(&zmq::z85_decode(&keypair.secret_key));
        assert_eq!(&zsock.curve_secretkey().unwrap().unwrap(), &keypair.secret_key);

        zsock.set_curve_serverkey(&keypair.secret_key);
        assert_eq!(&zsock.curve_serverkey().unwrap().unwrap(), &keypair.secret_key);
        zsock.set_curve_serverkey_bin(&zmq::z85_decode(&keypair.secret_key));
        assert_eq!(&zsock.curve_serverkey().unwrap().unwrap(), &keypair.secret_key);
    }

    #[test]
    fn test_xpub_verbose() {
        ZSys::init();

        let xpub = ZSock::new(ZSockType::XPUB);
        xpub.set_rcvtimeo(Some(10));
        xpub.bind("inproc://zsock_test_xpub_verbose_broken").unwrap();
        let _sub1 = ZSock::new_sub("inproc://zsock_test_xpub_verbose_broken", Some("a")).unwrap();
        let _sub2 = ZSock::new_sub("inproc://zsock_test_xpub_verbose_broken", Some("a")).unwrap();

        let mut x = 0;
        while xpub.recv_str().is_ok() {
            x += 1;
        }

        assert_eq!(x, 1);

        let xpub = ZSock::new(ZSockType::XPUB);
        xpub.set_rcvtimeo(Some(10));
        xpub.set_xpub_verbose(true);
        xpub.bind("inproc://zsock_test_xpub_verbose").unwrap();
        let _sub1 = ZSock::new_sub("inproc://zsock_test_xpub_verbose", Some("a")).unwrap();
        let _sub2 = ZSock::new_sub("inproc://zsock_test_xpub_verbose", Some("a")).unwrap();

        let mut x = 0;
        while xpub.recv_str().is_ok() {
            x += 1;
        }

        assert_eq!(x, 2);
    }

    #[test]
    fn test_monitor() {
        ZSys::init();

        let zsock = ZSock::new(ZSockType::REP);
        assert!(zsock.monitor().is_ok());
    }
}
