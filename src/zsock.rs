//! Module: czmq-zsock

use czmq_sys;
use std::{ptr, result};
use std::ffi::{CStr, CString};
use std::os::raw::c_void;
use std::str::Utf8Error;
use zmq;

// Generic error code "-1" doesn't map to an error message, so just
// return an empty tuple.
pub type Result<T> = result::Result<T, ()>;

pub struct ZSock {
    zsock: *mut czmq_sys::zsock_t,
}

impl Drop for ZSock {
    fn drop(&mut self) {
        unsafe { czmq_sys::zsock_destroy(&mut self.zsock) };
    }
}

impl ZSock {
    pub fn new(sock_type: zmq::SocketType) -> ZSock {
        ZSock {
            zsock: unsafe { czmq_sys::zsock_new(sock_type as i32) },
        }
    }

    pub fn new_pub(endpoint: &str) -> Result<ZSock> {
        let zsock = unsafe { czmq_sys::zsock_new_pub(endpoint.as_ptr() as *const i8) };

        if zsock == ptr::null_mut() {
            Err(())
        } else {
            Ok(ZSock {
                zsock: zsock,
            })
        }
    }

    pub fn new_sub(endpoint: &str, subscribe: Option<&str>) -> Result<ZSock> {
        let subscribe_ptr = match subscribe {
            Some(p) => CString::new(p).unwrap_or(CString::new("").unwrap()).as_ptr(),
            None => ptr::null(),
        };
        let zsock = unsafe { czmq_sys::zsock_new_sub(endpoint.as_ptr() as *const i8, subscribe_ptr) };

        if zsock == ptr::null_mut() {
            Err(())
        } else {
            Ok(ZSock {
                zsock: zsock,
            })
        }
    }

    pub fn new_req(endpoint: &str) -> Result<ZSock> {
        let zsock = unsafe { czmq_sys::zsock_new_req(endpoint.as_ptr() as *const i8) };

        if zsock == ptr::null_mut() {
            Err(())
        } else {
            Ok(ZSock {
                zsock: zsock,
            })
        }
    }

    pub fn new_rep(endpoint: &str) -> Result<ZSock> {
        let zsock = unsafe { czmq_sys::zsock_new_rep(endpoint.as_ptr() as *const i8) };

        if zsock == ptr::null_mut() {
            Err(())
        } else {
            Ok(ZSock {
                zsock: zsock,
            })
        }
    }

    pub fn new_dealer(endpoint: &str) -> Result<ZSock> {
        let zsock = unsafe { czmq_sys::zsock_new_dealer(endpoint.as_ptr() as *const i8) };

        if zsock == ptr::null_mut() {
            Err(())
        } else {
            Ok(ZSock {
                zsock: zsock,
            })
        }
    }

    pub fn new_router(endpoint: &str) -> Result<ZSock> {
        let zsock = unsafe { czmq_sys::zsock_new_router(endpoint.as_ptr() as *const i8) };

        if zsock == ptr::null_mut() {
            Err(())
        } else {
            Ok(ZSock {
                zsock: zsock,
            })
        }
    }

    pub fn new_push(endpoint: &str) -> Result<ZSock> {
        let zsock = unsafe { czmq_sys::zsock_new_push(endpoint.as_ptr() as *const i8) };

        if zsock == ptr::null_mut() {
            Err(())
        } else {
            Ok(ZSock {
                zsock: zsock,
            })
        }
    }

    pub fn new_pull(endpoint: &str) -> Result<ZSock> {
        let zsock = unsafe { czmq_sys::zsock_new_pull(endpoint.as_ptr() as *const i8) };

        if zsock == ptr::null_mut() {
            Err(())
        } else {
            Ok(ZSock {
                zsock: zsock,
            })
        }
    }

    pub fn new_xpub(endpoint: &str) -> Result<ZSock> {
        let zsock = unsafe { czmq_sys::zsock_new_xpub(endpoint.as_ptr() as *const i8) };

        if zsock == ptr::null_mut() {
            Err(())
        } else {
            Ok(ZSock {
                zsock: zsock,
            })
        }
    }

    pub fn new_xsub(endpoint: &str) -> Result<ZSock> {
        let zsock = unsafe { czmq_sys::zsock_new_xsub(endpoint.as_ptr() as *const i8) };

        if zsock == ptr::null_mut() {
            Err(())
        } else {
            Ok(ZSock {
                zsock: zsock,
            })
        }
    }

    pub fn new_pair(endpoint: &str) -> Result<ZSock> {
        let zsock = unsafe { czmq_sys::zsock_new_pair(endpoint.as_ptr() as *const i8) };

        if zsock == ptr::null_mut() {
            Err(())
        } else {
            Ok(ZSock {
                zsock: zsock,
            })
        }
    }

    pub fn new_stream(endpoint: &str) -> Result<ZSock> {
        let zsock = unsafe { czmq_sys::zsock_new_stream(endpoint.as_ptr() as *const i8) };

        if zsock == ptr::null_mut() {
            Err(())
        } else {
            Ok(ZSock {
                zsock: zsock,
            })
        }
    }

    pub fn bind(&self, endpoint: &str) -> Result<i32> {
        let rc = unsafe { czmq_sys::zsock_bind(self.zsock, "%s".as_ptr() as *const i8, endpoint.as_ptr() as *const i8) };
        if rc == -1i32 { Err(()) } else { Ok(rc) }
    }

    pub fn endpoint<'a>(&'a self) -> result::Result<&'a str, Utf8Error> {
        let endpoint_c = unsafe { czmq_sys::zsock_endpoint(self.zsock) };

        if endpoint_c == ptr::null() {
            Ok("")
        } else {
            unsafe { CStr::from_ptr(endpoint_c) }.to_str()
        }
    }

    pub fn unbind(&self, endpoint: &str) -> Result<()> {
        let rc = unsafe { czmq_sys::zsock_unbind(self.zsock, "%s".as_ptr() as *const i8, endpoint.as_ptr() as *const i8) };
        if rc == -1i32 { Err(()) } else { Ok(()) }
    }

    pub fn connect(&self, endpoint: &str) -> Result<()> {
        let rc = unsafe { czmq_sys::zsock_connect(self.zsock, "%s".as_ptr() as *const i8, endpoint.as_ptr() as *const i8) };
        if rc == -1i32 { Err(()) } else { Ok(()) }
    }

    pub fn disconnect(&self, endpoint: &str) -> Result<()> {
        let rc = unsafe { czmq_sys::zsock_disconnect(self.zsock, "%s".as_ptr() as *const i8, endpoint.as_ptr() as *const i8) };
        if rc == -1i32 { Err(()) } else { Ok(()) }
    }

    pub fn attach(&self, endpoints: &[&str], serverish: bool) -> Result<()> {
        let endpoints_c = CString::new(Self::concat_endpoints(endpoints)).unwrap_or(CString::new("").unwrap());

        let rc = unsafe { czmq_sys::zsock_attach(self.zsock, endpoints_c.as_ptr(), if serverish { 1 } else { 0 }) };
        if rc == -1i32 { Err(()) } else { Ok(()) }
    }

    pub fn type_str<'a>(&'a self) -> Result<result::Result<&'a str, Utf8Error>> {
        let ptr = unsafe { czmq_sys::zsock_type_str(self.zsock) };

        if ptr == ptr::null_mut() {
            Err(())
        } else {
            let type_str = unsafe { CStr::from_ptr(ptr) };
            Ok(type_str.to_str())
        }
    }

    pub fn send(&self, picture: &str, data: &str) -> Result<()> {
        let picture_c = CString::new(picture).unwrap_or(CString::new("").unwrap());
        let data_c = CString::new(data).unwrap_or(CString::new("").unwrap());

        let rc = unsafe { czmq_sys::zsock_send(self.zsock as *mut c_void, picture_c.as_ptr(), data_c.as_ptr()) };
        if rc == -1i32 { Err(()) } else { Ok(()) }
    }

    // pub fn zsock_recv(_self: *mut ::std::os::raw::c_void,
    //                   picture: *const ::std::os::raw::c_char, ...)
    //  -> ::std::os::raw::c_int;
    // pub fn recv(&self, picture: &str, d)

    // pub fn zsock_bsend(_self: *mut ::std::os::raw::c_void,
    //                    picture: *const ::std::os::raw::c_char, ...)
    //  -> ::std::os::raw::c_int;
    // pub fn zsock_brecv(_self: *mut ::std::os::raw::c_void,
    //                    picture: *const ::std::os::raw::c_char, ...)
    //  -> ::std::os::raw::c_int;
    // pub fn zsock_set_unbounded(_self: *mut ::std::os::raw::c_void);
    // pub fn zsock_signal(_self: *mut ::std::os::raw::c_void, status: byte)
    //  -> ::std::os::raw::c_int;
    // pub fn zsock_wait(_self: *mut ::std::os::raw::c_void)
    //  -> ::std::os::raw::c_int;
    // pub fn zsock_flush(_self: *mut ::std::os::raw::c_void);
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
    // pub fn zsock_zap_domain(_self: *mut ::std::os::raw::c_void)
    //  -> *mut ::std::os::raw::c_char;
    // pub fn zsock_set_zap_domain(_self: *mut ::std::os::raw::c_void,
    //                             zap_domain: *const ::std::os::raw::c_char);
    // pub fn zsock_mechanism(_self: *mut ::std::os::raw::c_void)
    //  -> ::std::os::raw::c_int;
    // pub fn zsock_plain_server(_self: *mut ::std::os::raw::c_void)
    //  -> ::std::os::raw::c_int;
    // pub fn zsock_set_plain_server(_self: *mut ::std::os::raw::c_void,
    //                               plain_server: ::std::os::raw::c_int);
    // pub fn zsock_plain_username(_self: *mut ::std::os::raw::c_void)
    //  -> *mut ::std::os::raw::c_char;
    // pub fn zsock_set_plain_username(_self: *mut ::std::os::raw::c_void,
    //                                 plain_username:
    //                                     *const ::std::os::raw::c_char);
    // pub fn zsock_plain_password(_self: *mut ::std::os::raw::c_void)
    //  -> *mut ::std::os::raw::c_char;
    // pub fn zsock_set_plain_password(_self: *mut ::std::os::raw::c_void,
    //                                 plain_password:
    //                                     *const ::std::os::raw::c_char);
    // pub fn zsock_curve_server(_self: *mut ::std::os::raw::c_void)
    //  -> ::std::os::raw::c_int;
    // pub fn zsock_set_curve_server(_self: *mut ::std::os::raw::c_void,
    //                               curve_server: ::std::os::raw::c_int);
    // pub fn zsock_curve_publickey(_self: *mut ::std::os::raw::c_void)
    //  -> *mut ::std::os::raw::c_char;
    // pub fn zsock_set_curve_publickey(_self: *mut ::std::os::raw::c_void,
    //                                  curve_publickey:
    //                                      *const ::std::os::raw::c_char);
    // pub fn zsock_set_curve_publickey_bin(_self: *mut ::std::os::raw::c_void,
    //                                      curve_publickey: *const byte);
    // pub fn zsock_curve_secretkey(_self: *mut ::std::os::raw::c_void)
    //  -> *mut ::std::os::raw::c_char;
    // pub fn zsock_set_curve_secretkey(_self: *mut ::std::os::raw::c_void,
    //                                  curve_secretkey:
    //                                      *const ::std::os::raw::c_char);
    // pub fn zsock_set_curve_secretkey_bin(_self: *mut ::std::os::raw::c_void,
    //                                      curve_secretkey: *const byte);
    // pub fn zsock_curve_serverkey(_self: *mut ::std::os::raw::c_void)
    //  -> *mut ::std::os::raw::c_char;
    // pub fn zsock_set_curve_serverkey(_self: *mut ::std::os::raw::c_void,
    //                                  curve_serverkey:
    //                                      *const ::std::os::raw::c_char);
    // pub fn zsock_set_curve_serverkey_bin(_self: *mut ::std::os::raw::c_void,
    //                                      curve_serverkey: *const byte);
    // pub fn zsock_gssapi_server(_self: *mut ::std::os::raw::c_void)
    //  -> ::std::os::raw::c_int;
    // pub fn zsock_set_gssapi_server(_self: *mut ::std::os::raw::c_void,
    //                                gssapi_server: ::std::os::raw::c_int);
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
    // pub fn zsock_sndhwm(_self: *mut ::std::os::raw::c_void)
    //  -> ::std::os::raw::c_int;
    // pub fn zsock_set_sndhwm(_self: *mut ::std::os::raw::c_void,
    //                         sndhwm: ::std::os::raw::c_int);
    // pub fn zsock_rcvhwm(_self: *mut ::std::os::raw::c_void)
    //  -> ::std::os::raw::c_int;
    // pub fn zsock_set_rcvhwm(_self: *mut ::std::os::raw::c_void,
    //                         rcvhwm: ::std::os::raw::c_int);
    // pub fn zsock_affinity(_self: *mut ::std::os::raw::c_void)
    //  -> ::std::os::raw::c_int;
    // pub fn zsock_set_affinity(_self: *mut ::std::os::raw::c_void,
    //                           affinity: ::std::os::raw::c_int);
    // pub fn zsock_set_subscribe(_self: *mut ::std::os::raw::c_void,
    //                            subscribe: *const ::std::os::raw::c_char);
    // pub fn zsock_set_unsubscribe(_self: *mut ::std::os::raw::c_void,
    //                              unsubscribe: *const ::std::os::raw::c_char);
    // pub fn zsock_identity(_self: *mut ::std::os::raw::c_void)
    //  -> *mut ::std::os::raw::c_char;
    // pub fn zsock_set_identity(_self: *mut ::std::os::raw::c_void,
    //                           identity: *const ::std::os::raw::c_char);
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
    // pub fn zsock_linger(_self: *mut ::std::os::raw::c_void)
    //  -> ::std::os::raw::c_int;
    // pub fn zsock_set_linger(_self: *mut ::std::os::raw::c_void,
    //                         linger: ::std::os::raw::c_int);
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
    // pub fn zsock_rcvtimeo(_self: *mut ::std::os::raw::c_void)
    //  -> ::std::os::raw::c_int;
    // pub fn zsock_set_rcvtimeo(_self: *mut ::std::os::raw::c_void,
    //                           rcvtimeo: ::std::os::raw::c_int);
    // pub fn zsock_sndtimeo(_self: *mut ::std::os::raw::c_void)
    //  -> ::std::os::raw::c_int;
    // pub fn zsock_set_sndtimeo(_self: *mut ::std::os::raw::c_void,
    //                           sndtimeo: ::std::os::raw::c_int);
    // pub fn zsock_set_xpub_verbose(_self: *mut ::std::os::raw::c_void,
    //                               xpub_verbose: ::std::os::raw::c_int);
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
    // pub fn zsock_rcvmore(_self: *mut ::std::os::raw::c_void)
    //  -> ::std::os::raw::c_int;
    // pub fn zsock_fd(_self: *mut ::std::os::raw::c_void) -> SOCKET;
    // pub fn zsock_events(_self: *mut ::std::os::raw::c_void)
    //  -> ::std::os::raw::c_int;
    // pub fn zsock_last_endpoint(_self: *mut ::std::os::raw::c_void)
    //  -> *mut ::std::os::raw::c_char;

    pub fn borrow_raw(&self) -> *mut czmq_sys::zsock_t {
        self.zsock
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

#[cfg(test)]
mod tests {
    use czmq_sys;
    use std::sync::{Once, ONCE_INIT};
    use super::*;
    use zmq;

    static INIT_ZSYS: Once = ONCE_INIT;

    #[test]
    fn test_new_pub() {
        zsys_init();

        let zsock = ZSock::new_pub("inproc://test_pub");
        assert!(zsock.is_ok());
    }

    #[test]
    fn test_new_sub() {
        zsys_init();

        let zsock = ZSock::new_sub("inproc://test_sub1", None);
        assert!(zsock.is_ok());
        let zsock = ZSock::new_sub("inproc://test_sub2", Some("moo"));
        assert!(zsock.is_ok());
    }

    #[test]
    fn test_new_req() {
        zsys_init();

        let zsock = ZSock::new_req("inproc://test_req");
        assert!(zsock.is_ok());
    }

    #[test]
    fn test_new_rep() {
        zsys_init();

        let zsock = ZSock::new_rep("inproc://test_rep");
        assert!(zsock.is_ok());
    }

    #[test]
    fn test_new_dealer() {
        zsys_init();

        let zsock = ZSock::new_dealer("inproc://test_dealer");
        assert!(zsock.is_ok());
    }

    #[test]
    fn test_new_router() {
        zsys_init();

        let zsock = ZSock::new_router("inproc://test_router");
        assert!(zsock.is_ok());
    }

    #[test]
    fn test_new_push() {
        zsys_init();

        let zsock = ZSock::new_push("inproc://test_push");
        assert!(zsock.is_ok());
    }

    #[test]
    fn test_new_pull() {
        zsys_init();

        let zsock = ZSock::new_pull("inproc://test_pull");
        assert!(zsock.is_ok());
    }

    #[test]
    fn test_new_xpub() {
        zsys_init();

        let zsock = ZSock::new_xpub("inproc://test_xpub");
        assert!(zsock.is_ok());
    }

    #[test]
    fn test_new_xsub() {
        zsys_init();

        let zsock = ZSock::new_xsub("inproc://test_xsub");
        assert!(zsock.is_ok());
    }

    #[test]
    fn test_new_pair() {
        zsys_init();

        let zsock = ZSock::new_pair("inproc://test_pair");
        assert!(zsock.is_ok());
    }

    #[test]
    fn test_new_stream() {
        zsys_init();

        let zsock = ZSock::new_stream("inproc://test_stream");
        assert!(zsock.is_ok());
    }

    #[test]
    fn test_bind() {
        zsys_init();

        let zsock = ZSock::new(zmq::REQ);
        assert!(zsock.bind("inproc://test_").is_ok());
    }

    #[test]
    fn test_endpoint() {
        zsys_init();

        let zsock = ZSock::new_rep("inproc://test_endpoint").unwrap();
        assert_eq!(zsock.endpoint().unwrap(), "inproc://test_endpoint");
    }

    #[test]
    fn test_unbind() {
        zsys_init();

        let zsock = ZSock::new_rep("inproc://test_unbind").unwrap();
        assert!(zsock.unbind("inproc://test_unbind").is_ok());
    }

    #[test]
    fn test_connect() {
        zsys_init();

        ZSock::new_rep("inproc://test_connect").unwrap();
        let client = ZSock::new(zmq::REQ);
        assert!(client.connect("inproc://test_connect").is_ok());
    }

    #[test]
    fn test_disconnect() {
        zsys_init();

        let zsock = ZSock::new_rep("inproc://test_disconnect").unwrap();
        assert!(zsock.disconnect("inproc://test_disconnect").is_ok());
    }

    #[test]
    fn test_attach() {
        zsys_init();

        let zsock = ZSock::new(zmq::REP);
        let result = zsock.attach(&["inproc://test_attach1", "inproc://test_attach2", "inproc://test_attach3"], true);
        assert!(result.is_ok());
    }

    #[test]
    fn test_type_str() {
        zsys_init();

        let zsock = ZSock::new(zmq::PUB);
        assert_eq!(zsock.type_str().unwrap().unwrap(), "PUB");
    }

    #[test]
    fn test_sendrecv() {
        zsys_init();

        let server = ZSock::new_rep("inproc://test_send").unwrap();
        let client = ZSock::new_req("inproc://test_send").unwrap();

        assert!(client.send("s", "This is a test string.").is_ok());
        // assert_eq!(server.recv("s"));
    }

    // Each new ZSock calls zsys_init(), which is a non-threadsafe
    // fn. To mitigate the race condition, wrap it in a Once struct.
    fn zsys_init() {
        INIT_ZSYS.call_once(|| {
            unsafe { czmq_sys::zsys_init() };
        });
    }
}
