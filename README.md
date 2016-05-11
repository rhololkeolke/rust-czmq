# rust-czmq [![Build Status](https://travis-ci.org/petehayes102/rust-czmq.svg?branch=master)](https://travis-ci.org/petehayes102/rust-czmq)

Rust binding for [CZMQ](http://czmq.zeromq.org). Find the docs at [https://petehayes102.github.io/rust-czmq]().

This library is laid out in the same style and uses the same dual licence as the [Rust ZMQ binding](https://github.com/erickt/rust-zmq).

Note that much of this library is incomplete. For the modules that do exist, you can find comments like this one:

_from src/zmsg.rs_
```rust
// pub fn zmsg_sendm(self_p: *mut *mut zmsg_t,
//                   dest: *mut ::std::os::raw::c_void)
//  -> ::std::os::raw::c_int;
```

This is the FFI wrapper for the C library, which I'm using as a placeholder for a proper Rust fn. I'll keep plugging away, though CZMQ is a massive library. Suffice to say, PRs are most welcome! :D

## CZMQ

CZMQ is a high level C binding for ZeroMQ that provides useful abstractions for complex tasks, as well as much of the boilerplate that you would otherwise have to write yourself.

More information can be found at [https://czmq.zeromq.org](https://czmq.zeromq.org).

## Multithreading

CZMQ's ZSock struct is particularly useful for multithreading as CZMQ maintains an internal global state for sharing a single context object. This means that you can instantiate new ZSocks across threads without having to explicitly share a context.

### Caveats

Note that I am **not** suggesting that you share a **single** ZSock across threads. This is profoundly un-thread-safe :)

Also, be warned that each call to ZSock precipitates a call to the C fn `zsys_init()`, which **is not thread-safe**! After the first call to `zsys_init()` it is safe to call from child threads, but prior to this, avoid creating race conditions between multiple ZSock instantiations unless you like panics.

The easiest way to mitigate this problem is to create a ZSock in your master thread (which you'll probably do anyway) before you start creating child threads. This will call `zsys_init()` uncontested and initialise the global ZSys state safely.

Another way around it for situations where you can't control the master thread (e.g. tests), you can follow the example I have used in this library for running safe tests:

_from src/lib.rs_
```rust
#[cfg(test)]
use std::sync::{Once, ONCE_INIT};

#[cfg(test)]
static INIT_ZSYS: Once = ONCE_INIT;

#[cfg(test)]
fn zsys_init() {
    INIT_ZSYS.call_once(|| {
        unsafe { czmq_sys::zsys_init() };
    });
}
```

Then in your tests (or anywhere you want to protect calls to ZSock::new()):

```rust
#[test]
fn test_my_thing() {
    zsys_init();

    // Now go nuts...
}
```

## Bindgen

To generate the FFI code (_czmq-sys/src/ffi.rs_), you'll need [rust-bindgen](https://github.com/crabtw/rust-bindgen). Clone and build this repo, then use it to generate the binding.

Here's a sample command:

```bash
/path/to/bindgen -l czmq -o /path/to/rust-czmq/czmq-sys/src/ffi.rs -builtins /path/to/rust-czmq/czmq-sys/bindgen.h
```

For OS X, we will probably also need to help Clang find some missing headers. If you see an error like this...

```
/usr/local/include/czmq_prelude.h:177:13: fatal error: 'TargetConditionals.h' file not found
```

...then you'll need to manually specify the include directory (or directories) where the missing headers reside.

Note: You'll need Xcode installed to get the missing headers.

Here's a sample command with the missing headers' include path:

```bash
/path/to/bindgen -l czmq -o /path/to/rust-czmq/czmq-sys/src/ffi.rs -builtins -I/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX10.11.sdk/usr/include/ /path/to/rust-czmq/czmq-sys/bindgen.h
```

If the path above doesn't exist (it probably doesn't), or you're still getting errors about missing headers (.h files), use `find` to locate the headers on your machine. When you've found them, manually specify their paths using one or more `-I` flags as in the example above.

If you're still struggling, feel free to raise a ticket and someone might be able to point you in the right direction.

## Roadmap

1. Finish existing modules by replacing the FFI stubs with Rust fns
2. Solve the variadic fn issue with ZSock::send/recv - marrying Rust's typesystem with C's void pointers is difficult
3. Reproduce CZMQ examples in Rust
4. Documentation for the library
5. Continue the soul crushing pursuit of full library support

## Thanks

Cheers to [Andrew Hobden](https://github.com/hoverbear) for the [tutorial](http://hoverbear.org/2015/03/07/rust-travis-github-pages/) about auto-publishing Rust docs to GitHub Pages!
