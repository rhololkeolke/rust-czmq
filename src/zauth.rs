//! Module: czmq-zauth

use {czmq_sys, ZActor};

pub struct ZAuth {
    zactor: ZActor,
}

impl ZAuth {
    pub fn new() -> ZAuth {
        ZAuth {
            zactor: ZActor::new(czmq_sys::zauth),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ZCtx;

    #[test]
    fn test_new() {
        ZAuth::new();
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
