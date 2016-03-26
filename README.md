# rust-czmq [![Build Status](https://travis-ci.org/petehayes102/rust-czmq.svg?branch=master)](https://travis-ci.org/petehayes102/rust-czmq)

Rust binding for [CZMQ](http://czmq.zeromq.org).

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

## Roadmap

1. Finish existing modules by replacing the FFI stubs with Rust fns
2. Solve the variadic fn issue with ZSock::send/recv - marrying Rust's typesystem with C's void pointers is difficult
3. Reproduce CZMQ examples in Rust
4. Documentation for the library
5. Publish docs to GitHub pages
6. Continue the soul crushing pursuit of full library support
