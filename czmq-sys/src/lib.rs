extern crate libc;

pub use ffi::{
    //
    // ZActor
    //
    zactor_t,
    zactor_fn,
    zactor_new,
    zactor_destroy,
    zactor_send,
    zactor_recv,
    zactor_is,
    zactor_resolve,
    zactor_sock,

    //
    // ZAuth
    //
    zauth,

    //
    // ZCert
    //
    zcert_t,
    zcert_new,
    zcert_new_from,
    zcert_load,
    zcert_destroy,
    zcert_public_key,
    zcert_secret_key,
    zcert_public_txt,
    zcert_secret_txt,
    zcert_set_meta,
    zcert_meta,
    zcert_meta_keys,
    zcert_save,
    zcert_save_public,
    zcert_save_secret,
    zcert_apply,
    zcert_dup,
    zcert_eq,
    zcert_print,

    //
    // ZCertstore
    //
    zcertstore_t,
    zcertstore_new,
    zcertstore_destroy,
    zcertstore_lookup,
    zcertstore_insert,
    zcertstore_print,
    zcertstore_test,

    //
    // ZFrame
    //
    zframe_t,
    zframe_new,
    zframe_new_empty,
    zframe_from,
    zframe_recv,
    zframe_destroy,
    zframe_send,
    zframe_size,
    zframe_data,
    zframe_dup,
    zframe_strhex,
    zframe_strdup,
    zframe_streq,
    zframe_more,
    zframe_set_more,
    zframe_eq,
    zframe_reset,
    zframe_print,

    //
    // ZHashX
    //
    zhashx_t,
    zhashx_new,
    zhashx_unpack,
    zhashx_destroy,
    zhashx_insert,
    zhashx_update,
    zhashx_delete,
    zhashx_purge,
    zhashx_lookup,
    zhashx_rename,
    zhashx_freefn,
    zhashx_size,
    zhashx_keys,
    zhashx_values,
    zhashx_first,
    zhashx_next,
    zhashx_cursor,
    zhashx_comment,
    zhashx_save,
    zhashx_load,
    zhashx_refresh,
    zhashx_pack,
    zhashx_dup,
    zhashx_set_destructor,
    zhashx_set_duplicator,
    zhashx_set_key_destructor,
    zhashx_set_key_duplicator,
    zhashx_set_key_comparator,
    zhashx_set_key_hasher,

    //
    // ZList
    //
    zlist_t,
    zlist_new,
    zlist_append,
    zlist_destroy,
    zlist_next,
    zlist_size,

    //
    // ZMonitor
    //
    zmonitor,

    //
    // ZMsg
    //
    zmsg_t,
    zmsg_new,
    zmsg_recv,
    zmsg_load,
    zmsg_decode,
    zmsg_new_signal,
    zmsg_destroy,
    zmsg_send,
    zmsg_sendm,
    zmsg_size,
    zmsg_content_size,
    zmsg_prepend,
    zmsg_append,
    zmsg_pop,
    zmsg_pushmem,
    zmsg_addmem,
    zmsg_pushstr,
    zmsg_addstr,
    zmsg_pushstrf,
    zmsg_addstrf,
    zmsg_popstr,
    zmsg_addmsg,
    zmsg_popmsg,
    zmsg_remove,
    zmsg_first,
    zmsg_next,
    zmsg_last,
    zmsg_save,
    zmsg_encode,
    zmsg_dup,
    zmsg_print,
    zmsg_eq,
    zmsg_signal,
    zmsg_is,
    zmsg_test,
    zmsg_unwrap,
    zmsg_recv_nowait,
    zmsg_wrap,
    zmsg_push,
    zmsg_add,
    zmsg_fprint,

    //
    // ZPoller
    //
    zpoller_t,
    zpoller_new,
    zpoller_destroy,
    zpoller_add,
    zpoller_remove,
    zpoller_wait,
    zpoller_expired,
    zpoller_terminated,

    //
    // ZSock
    //
    zsock_t,
    zsock_new,
    zsock_new_pub,
    zsock_new_sub,
    zsock_new_req,
    zsock_new_rep,
    zsock_new_dealer,
    zsock_new_router,
    zsock_new_push,
    zsock_new_pull,
    zsock_new_xpub,
    zsock_new_xsub,
    zsock_new_pair,
    zsock_new_stream,
    zsock_destroy,
    zsock_bind,
    zsock_endpoint,
    zsock_unbind,
    zsock_connect,
    zsock_disconnect,
    zsock_attach,
    zsock_type_str,
    zsock_send,
    zsock_recv,
    zsock_bsend,
    zsock_brecv,
    zsock_set_unbounded,
    zsock_signal,
    zsock_wait,
    zsock_flush,
    zsock_is,
    zsock_resolve,
    zsock_tos,
    zsock_set_tos,
    zsock_set_router_handover,
    zsock_set_router_mandatory,
    zsock_set_probe_router,
    zsock_set_req_relaxed,
    zsock_set_req_correlate,
    zsock_set_conflate,
    zsock_zap_domain,
    zsock_set_zap_domain,
    zsock_mechanism,
    zsock_plain_server,
    zsock_set_plain_server,
    zsock_plain_username,
    zsock_set_plain_username,
    zsock_plain_password,
    zsock_set_plain_password,
    zsock_curve_server,
    zsock_set_curve_server,
    zsock_curve_publickey,
    zsock_set_curve_publickey,
    zsock_set_curve_publickey_bin,
    zsock_curve_secretkey,
    zsock_set_curve_secretkey,
    zsock_set_curve_secretkey_bin,
    zsock_curve_serverkey,
    zsock_set_curve_serverkey,
    zsock_set_curve_serverkey_bin,
    zsock_gssapi_server,
    zsock_set_gssapi_server,
    zsock_gssapi_plaintext,
    zsock_set_gssapi_plaintext,
    zsock_gssapi_principal,
    zsock_set_gssapi_principal,
    zsock_gssapi_service_principal,
    zsock_set_gssapi_service_principal,
    zsock_ipv6,
    zsock_set_ipv6,
    zsock_immediate,
    zsock_set_immediate,
    zsock_set_router_raw,
    zsock_ipv4only,
    zsock_set_ipv4only,
    zsock_set_delay_attach_on_connect,
    zsock_type,
    zsock_sndhwm,
    zsock_set_sndhwm,
    zsock_rcvhwm,
    zsock_set_rcvhwm,
    zsock_affinity,
    zsock_set_affinity,
    zsock_set_subscribe,
    zsock_set_unsubscribe,
    zsock_identity,
    zsock_set_identity,
    zsock_rate,
    zsock_set_rate,
    zsock_recovery_ivl,
    zsock_set_recovery_ivl,
    zsock_sndbuf,
    zsock_set_sndbuf,
    zsock_rcvbuf,
    zsock_set_rcvbuf,
    zsock_linger,
    zsock_set_linger,
    zsock_reconnect_ivl,
    zsock_set_reconnect_ivl,
    zsock_reconnect_ivl_max,
    zsock_set_reconnect_ivl_max,
    zsock_backlog,
    zsock_set_backlog,
    zsock_maxmsgsize,
    zsock_set_maxmsgsize,
    zsock_multicast_hops,
    zsock_set_multicast_hops,
    zsock_rcvtimeo,
    zsock_set_rcvtimeo,
    zsock_sndtimeo,
    zsock_set_sndtimeo,
    zsock_set_xpub_verbose,
    zsock_tcp_keepalive,
    zsock_set_tcp_keepalive,
    zsock_tcp_keepalive_idle,
    zsock_set_tcp_keepalive_idle,
    zsock_tcp_keepalive_cnt,
    zsock_set_tcp_keepalive_cnt,
    zsock_tcp_keepalive_intvl,
    zsock_set_tcp_keepalive_intvl,
    zsock_tcp_accept_filter,
    zsock_set_tcp_accept_filter,
    zsock_rcvmore,
    zsock_fd,
    zsock_events,
    zsock_last_endpoint,

    //
    // ZStr
    //
    zstr_t,
    zstr_recv,
    zstr_recvx,
    zstr_send,
    zstr_sendm,
    zstr_sendf,
    zstr_sendfm,
    zstr_sendx,
    zstr_free,
    zstr_recv_nowait,

    //
    // ZSys
    //
    zsys_init,
    zsys_create_pipe,
    zsys_interrupted,
};

#[cfg(feature = "draft")]
pub use ffi::{
    //
    // ZCertstore
    //
    zcertstore_loader,
    zcertstore_destructor,
    zcertstore_set_loader,
    zcertstore_empty,

    //
    // ZFrame
    //
    zframe_meta,

    //
    // ZPoller
    //
    zpoller_set_nonstop,
};

#[allow(dead_code, non_camel_case_types, non_snake_case, non_upper_case_globals)]
mod ffi {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
