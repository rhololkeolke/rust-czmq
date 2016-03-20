//! Module: czmq-zsock

use czmq_sys;
use std::result;
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

    // pub fn zsock_new_pub(endpoint: *const ::std::os::raw::c_char)
    //  -> *mut zsock_t;
    // pub fn zsock_new_sub(endpoint: *const ::std::os::raw::c_char,
    //                      subscribe: *const ::std::os::raw::c_char)
    //  -> *mut zsock_t;
    // pub fn zsock_new_req(endpoint: *const ::std::os::raw::c_char)
    //  -> *mut zsock_t;
    // pub fn zsock_new_rep(endpoint: *const ::std::os::raw::c_char)
    //  -> *mut zsock_t;
    // pub fn zsock_new_dealer(endpoint: *const ::std::os::raw::c_char)
    //  -> *mut zsock_t;
    // pub fn zsock_new_router(endpoint: *const ::std::os::raw::c_char)
    //  -> *mut zsock_t;
    // pub fn zsock_new_push(endpoint: *const ::std::os::raw::c_char)
    //  -> *mut zsock_t;
    // pub fn zsock_new_pull(endpoint: *const ::std::os::raw::c_char)
    //  -> *mut zsock_t;
    // pub fn zsock_new_xpub(endpoint: *const ::std::os::raw::c_char)
    //  -> *mut zsock_t;
    // pub fn zsock_new_xsub(endpoint: *const ::std::os::raw::c_char)
    //  -> *mut zsock_t;
    // pub fn zsock_new_pair(endpoint: *const ::std::os::raw::c_char)
    //  -> *mut zsock_t;
    // pub fn zsock_new_stream(endpoint: *const ::std::os::raw::c_char)
    //  -> *mut zsock_t;
    // pub fn zsock_destroy(self_p: *mut *mut zsock_t);
    // pub fn zsock_bind(_self: *mut zsock_t,
    //                   format: *const ::std::os::raw::c_char, ...)
    //  -> ::std::os::raw::c_int;
    // pub fn zsock_endpoint(_self: *mut zsock_t)
    //  -> *const ::std::os::raw::c_char;
    // pub fn zsock_unbind(_self: *mut zsock_t,
    //                     format: *const ::std::os::raw::c_char, ...)
    //  -> ::std::os::raw::c_int;
    // pub fn zsock_connect(_self: *mut zsock_t,
    //                      format: *const ::std::os::raw::c_char, ...)
    //  -> ::std::os::raw::c_int;
    // pub fn zsock_disconnect(_self: *mut zsock_t,
    //                         format: *const ::std::os::raw::c_char, ...)
    //  -> ::std::os::raw::c_int;
    // pub fn zsock_attach(_self: *mut zsock_t,
    //                     endpoints: *const ::std::os::raw::c_char,
    //                     serverish: u8) -> ::std::os::raw::c_int;
    // pub fn zsock_type_str(_self: *mut zsock_t)
    //  -> *const ::std::os::raw::c_char;
    // pub fn zsock_send(_self: *mut ::std::os::raw::c_void,
    //                   picture: *const ::std::os::raw::c_char, ...)
    //  -> ::std::os::raw::c_int;
    // pub fn zsock_vsend(_self: *mut ::std::os::raw::c_void,
    //                    picture: *const ::std::os::raw::c_char,
    //                    argptr: va_list) -> ::std::os::raw::c_int;
    // pub fn zsock_recv(_self: *mut ::std::os::raw::c_void,
    //                   picture: *const ::std::os::raw::c_char, ...)
    //  -> ::std::os::raw::c_int;
    // pub fn zsock_vrecv(_self: *mut ::std::os::raw::c_void,
    //                    picture: *const ::std::os::raw::c_char,
    //                    argptr: va_list) -> ::std::os::raw::c_int;
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
    // pub fn zsock_test(verbose: u8);
    // pub fn zsock_new_checked(_type: ::std::os::raw::c_int,
    //                          filename: *const ::std::os::raw::c_char,
    //                          line_nbr: size_t) -> *mut zsock_t;
    // pub fn zsock_destroy_checked(self_p: *mut *mut zsock_t,
    //                              filename: *const ::std::os::raw::c_char,
    //                              line_nbr: size_t);
    // pub fn zsock_new_pub_checked(endpoint: *const ::std::os::raw::c_char,
    //                              filename: *const ::std::os::raw::c_char,
    //                              line_nbr: size_t) -> *mut zsock_t;
    // pub fn zsock_new_sub_checked(endpoint: *const ::std::os::raw::c_char,
    //                              subscribe: *const ::std::os::raw::c_char,
    //                              filename: *const ::std::os::raw::c_char,
    //                              line_nbr: size_t) -> *mut zsock_t;
    // pub fn zsock_new_req_checked(endpoint: *const ::std::os::raw::c_char,
    //                              filename: *const ::std::os::raw::c_char,
    //                              line_nbr: size_t) -> *mut zsock_t;
    // pub fn zsock_new_rep_checked(endpoint: *const ::std::os::raw::c_char,
    //                              filename: *const ::std::os::raw::c_char,
    //                              line_nbr: size_t) -> *mut zsock_t;
    // pub fn zsock_new_dealer_checked(endpoint: *const ::std::os::raw::c_char,
    //                                 filename: *const ::std::os::raw::c_char,
    //                                 line_nbr: size_t) -> *mut zsock_t;
    // pub fn zsock_new_router_checked(endpoint: *const ::std::os::raw::c_char,
    //                                 filename: *const ::std::os::raw::c_char,
    //                                 line_nbr: size_t) -> *mut zsock_t;
    // pub fn zsock_new_push_checked(endpoint: *const ::std::os::raw::c_char,
    //                               filename: *const ::std::os::raw::c_char,
    //                               line_nbr: size_t) -> *mut zsock_t;
    // pub fn zsock_new_pull_checked(endpoint: *const ::std::os::raw::c_char,
    //                               filename: *const ::std::os::raw::c_char,
    //                               line_nbr: size_t) -> *mut zsock_t;
    // pub fn zsock_new_xpub_checked(endpoint: *const ::std::os::raw::c_char,
    //                               filename: *const ::std::os::raw::c_char,
    //                               line_nbr: size_t) -> *mut zsock_t;
    // pub fn zsock_new_xsub_checked(endpoint: *const ::std::os::raw::c_char,
    //                               filename: *const ::std::os::raw::c_char,
    //                               line_nbr: size_t) -> *mut zsock_t;
    // pub fn zsock_new_pair_checked(endpoint: *const ::std::os::raw::c_char,
    //                               filename: *const ::std::os::raw::c_char,
    //                               line_nbr: size_t) -> *mut zsock_t;
    // pub fn zsock_new_stream_checked(endpoint: *const ::std::os::raw::c_char,
    //                                 filename: *const ::std::os::raw::c_char,
    //                                 line_nbr: size_t) -> *mut zsock_t;
    // pub fn zsock_new_server_checked(endpoint: *const ::std::os::raw::c_char,
    //                                 filename: *const ::std::os::raw::c_char,
    //                                 line_nbr: size_t) -> *mut zsock_t;
    // pub fn zsock_new_client_checked(endpoint: *const ::std::os::raw::c_char,
    //                                 filename: *const ::std::os::raw::c_char,
    //                                 line_nbr: size_t) -> *mut zsock_t;
    // pub fn zsock_new_radio_checked(endpoint: *const ::std::os::raw::c_char,
    //                                filename: *const ::std::os::raw::c_char,
    //                                line_nbr: size_t) -> *mut zsock_t;
    // pub fn zsock_new_dish_checked(endpoint: *const ::std::os::raw::c_char,
    //                               filename: *const ::std::os::raw::c_char,
    //                               line_nbr: size_t) -> *mut zsock_t;

    pub fn borrow_raw(&self) -> *mut czmq_sys::zsock_t {
        self.zsock
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use zmq;
// }
