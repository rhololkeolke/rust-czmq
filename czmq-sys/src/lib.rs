extern crate libc;

pub use ffi::{
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
    zcert_fprint,
    zcert_test,

    //
    // ZCertstore
    //
    zcertstore_new,
    zcertstore_destroy,
    zcertstore_lookup,
    zcertstore_insert,
    zcertstore_print,
    zcertstore_fprint,
    zcertstore_test,

    //
    // ZList
    //
    zlist_t,
    zlist_new,
    zlist_append,
    zlist_destroy,
    zlist_next,
    zlist_size,
};

#[allow(dead_code, non_camel_case_types, non_snake_case)]
mod ffi {
    include!("ffi.rs");
}
