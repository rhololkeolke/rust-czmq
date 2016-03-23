//! Module: czmq-zauth

use {czmq_sys, ZActor};
use std::result;

// Generic error code "-1" doesn't map to an error message, so just
// return an empty tuple.
pub type Result<T> = result::Result<T, ()>;

pub struct ZAuth {
    zactor: ZActor,
}

impl ZAuth {
    pub fn new() -> Result<ZAuth> {
        Ok(ZAuth {
            zactor: try!(ZActor::new(czmq_sys::zauth)),
        })
    }

    pub fn verbose() {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert!(ZAuth::new().is_ok());
    }

    // #[test]
    // fn test_() {
    //     let zauth = create_zauth();
    //     zauth.allow("127.0.0.1");
    // }
    //
    // fn create_zauth() -> ZAuth {
    //     let ctx = ZCtx::new();
    //     ZAuth::new(&ctx).unwrap()
    // }
}
