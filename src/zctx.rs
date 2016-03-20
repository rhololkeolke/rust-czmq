//! Module: czmq-zctx

use czmq_sys;
use std::{ptr, result};
use std::os::raw::c_void;

// Generic error code "-1" doesn't map to an error message, so just
// return an empty tuple.
pub type Result<T> = result::Result<T, ()>;

pub struct ZCtx {
    zctx: *mut czmq_sys::zctx_t,
}

impl Drop for ZCtx {
    fn drop(&mut self) {
        unsafe { czmq_sys::zctx_destroy(&mut self.zctx) };
    }
}

impl ZCtx {
    pub fn new() -> ZCtx {
        ZCtx {
            zctx: unsafe { czmq_sys::zctx_new() },
        }
    }

    pub fn shadow(&self) -> Result<ZCtx> {
        let shadow_ctx = unsafe { czmq_sys::zctx_shadow(self.zctx) };

        if shadow_ctx == ptr::null_mut() {
            return Err(());
        }

        Ok(ZCtx {
            zctx: shadow_ctx,
        })
    }

    pub fn set_iothreads(&self, iothreads: i32) {
        unsafe { czmq_sys::zctx_set_iothreads(self.zctx, iothreads) };
    }

    pub fn set_linger(&self, linger: i32) {
        unsafe { czmq_sys::zctx_set_linger(self.zctx, linger) };
    }

    pub fn set_pipehwm(&self, hwm: i32) {
        unsafe { czmq_sys::zctx_set_pipehwm(self.zctx, hwm) };
    }

    pub fn set_sndhwm(&self, hwm: i32) {
        unsafe { czmq_sys::zctx_set_sndhwm(self.zctx, hwm) };
    }

    pub fn set_rcvhwm(&self, hwm: i32) {
        unsafe { czmq_sys::zctx_set_rcvhwm(self.zctx, hwm) };
    }

    pub fn underlying(&self) -> *mut c_void {
        unsafe { czmq_sys::zctx_underlying(self.zctx) }
    }

    pub fn borrow_raw(&self) -> *mut czmq_sys::zctx_t {
        self.zctx
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        // Just test for panics
        ZCtx::new();
    }

    #[test]
    fn test_shadow() {
        assert!(ZCtx::shadow(&ZCtx::new()).is_ok());
    }

    #[test]
    fn test_set_iothreads() {
        let ctx = ZCtx::new();
        // Just test for panics
        ctx.set_iothreads(5);
    }

    #[test]
    fn test_set_linger() {
        let ctx = ZCtx::new();
        // Just test for panics
        ctx.set_linger(5);
    }

    #[test]
    fn test_set_pipehwm() {
        let ctx = ZCtx::new();
        // Just test for panics
        ctx.set_pipehwm(5);
    }

    #[test]
    fn test_set_sndhwm() {
        let ctx = ZCtx::new();
        // Just test for panics
        ctx.set_sndhwm(5);
    }

    #[test]
    fn test_set_rcvhwm() {
        let ctx = ZCtx::new();
        // Just test for panics
        ctx.set_rcvhwm(5);
    }

    #[test]
    fn test_underlying() {
        let ctx = ZCtx::new();
        // Just test for panics
        ctx.underlying();
    }
}
