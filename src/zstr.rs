pub fn zstr_recv(source: *mut ::std::os::raw::c_void)
 -> *mut ::std::os::raw::c_char;
pub fn zstr_recvx(source: *mut ::std::os::raw::c_void,
                  string_p: *mut *mut ::std::os::raw::c_char, ...)
 -> ::std::os::raw::c_int;
pub fn zstr_send(dest: *mut ::std::os::raw::c_void,
                 string: *const ::std::os::raw::c_char)
 -> ::std::os::raw::c_int;
pub fn zstr_sendm(dest: *mut ::std::os::raw::c_void,
                  string: *const ::std::os::raw::c_char)
 -> ::std::os::raw::c_int;
pub fn zstr_sendf(dest: *mut ::std::os::raw::c_void,
                  format: *const ::std::os::raw::c_char, ...)
 -> ::std::os::raw::c_int;
pub fn zstr_sendfm(dest: *mut ::std::os::raw::c_void,
                   format: *const ::std::os::raw::c_char, ...)
 -> ::std::os::raw::c_int;
pub fn zstr_sendx(dest: *mut ::std::os::raw::c_void,
                  string: *const ::std::os::raw::c_char, ...)
 -> ::std::os::raw::c_int;
pub fn zstr_free(string_p: *mut *mut ::std::os::raw::c_char);
pub fn zstr_test(verbose: u8);
pub fn zstr_recv_nowait(source: *mut ::std::os::raw::c_void)
 -> *mut ::std::os::raw::c_char;
