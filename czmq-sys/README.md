# czmq-sys

FFI bindings to CZMQ library. Bindings for CZMQ version 4.0 are
bundled. If you are on a different architecture/OS you may want to
enable the "buildtime_bindgen" feature. This will run bindgen on
build, rather than using the bundled files. Note that this will likely
increase your build times.

## Generating new bindings

The command used to generate the bundled bindings is:

``` shell
bindgen -l czmq -o bindgen-bindings/bindgen-draft.rs --builtins bindgen.h --no-unstable-rust --opaque-type zmq_msg_t --blacklist-type IPPORT_RESERVED --blacklist-type max_align_t -- -DCZMQ_BUILD_DRAFT_API=1
bindgen -l czmq -o bindgen-bindings/bindgen.rs --builtins bindgen.h --no-unstable-rust --opaque-type zmq_msg_t --blacklist-type IPPORT_RESERVED --blacklist-type max_align_t
```
