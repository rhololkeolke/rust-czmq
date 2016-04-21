//! Module: czmq-zauth

use {czmq_sys, Result, ZActor, ZCertStore, ZMsg};
use error::{Error, ErrorKind};
use std::{error, ptr};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::os::raw::c_void;

pub struct ZAuth {
    zactor: ZActor,
}

unsafe impl Send for ZAuth {}

impl ZAuth {
    pub fn new(certstore: Option<ZCertStore>) -> Result<ZAuth> {
        let ptr = if let Some(cs) = certstore {
            cs.to_raw()
        } else {
            ptr::null_mut()
        };
        let zactor = unsafe { czmq_sys::zactor_new(czmq_sys::zauth, ptr as *mut c_void) };

        if zactor == ptr::null_mut() {
            Err(Error::new(ErrorKind::NullPtr, ZAuthError::Instantiate))
        } else {
            Ok(ZAuth {
                zactor: ZActor::from_raw(zactor),
            })
        }
    }

    pub fn allow(&self, address: &str) -> Result<()> {
        let msg = ZMsg::new();
        try!(msg.addstr("ALLOW"));
        try!(msg.addstr(address));

        try!(self.zactor.send(msg));
        self.zactor.sock().wait()
    }

    pub fn deny(&self, address: &str) -> Result<()> {
        let msg = ZMsg::new();
        try!(msg.addstr("DENY"));
        try!(msg.addstr(address));

        try!(self.zactor.send(msg));
        self.zactor.sock().wait()
    }

    pub fn load_plain(&self, filename: &str) -> Result<()> {
        let msg = ZMsg::new();
        try!(msg.addstr("PLAIN"));
        try!(msg.addstr(filename));

        try!(self.zactor.send(msg));
        self.zactor.sock().wait()
    }

    pub fn load_curve(&self, location: Option<&str>) -> Result<()> {
        let msg = ZMsg::new();
        try!(msg.addstr("CURVE"));

        if let Some(loc) = location {
            try!(msg.addstr(loc));
        } else {
            try!(msg.addstr("*"));
        }

        try!(self.zactor.send(msg));
        self.zactor.sock().wait()
    }

    // XXX This is unimplemented upstream, so it's just a placeholder.
    pub fn load_gssapi(&self) -> Result<()> {
        unimplemented!();
    }

    pub fn verbose(&self) -> Result<()> {
        try!(self.zactor.send_str("VERBOSE"));
        self.zactor.sock().wait()
    }
}

#[derive(Debug)]
pub enum ZAuthError {
    Instantiate,
}

impl Display for ZAuthError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match *self {
            ZAuthError::Instantiate => write!(f, "Could not instantiate new ZAuth struct"),
        }
    }
}

impl error::Error for ZAuthError {
    fn description(&self) -> &str {
        match *self {
            ZAuthError::Instantiate => "Could not instantiate new ZAuth struct",
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;
    use std::thread::sleep;
    use std::time::Duration;
    use super::*;
    use tempdir::TempDir;
    use tempfile::NamedTempFile;
    use {ZCert, ZCertStore, ZCertStoreRaw, ZFrame, ZSock, ZSockType, zsys_init};

    // There can only be one ZAuth instance per context as each ZAuth
    // instance binds to the same inproc endpoint. The simplest way
    // around this limitation is to run all the tests in sequence.
    #[test]
    fn test_zauth() {
        zsys_init();

        test_verbose();
        test_allow_deny();
        test_plain();
        test_curve();
        test_zcertstore();
    }

    fn test_verbose() {
        let zauth = ZAuth::new(None).unwrap();
        assert!(zauth.verbose().is_ok());
    }

    fn test_allow_deny() {
        let server = ZSock::new(ZSockType::PULL);
        server.set_zap_domain("compuglobalhypermega.net");
        server.set_rcvtimeo(100);

        let client = ZSock::new(ZSockType::PUSH);
        client.set_linger(100);
        client.set_sndtimeo(100);

        let zauth = ZAuth::new(None).unwrap();

        assert!(zauth.deny("127.0.0.1").is_ok());
        sleep(Duration::from_millis(100));

        let port = server.bind("tcp://127.0.0.1:*[60000-]").unwrap();

        client.connect(&format!("tcp://127.0.0.1:{}", port)).unwrap();
        sleep(Duration::from_millis(100));

        client.send_str("test").unwrap();
        assert!(server.recv_str().is_err());

        assert!(zauth.allow("127.0.0.1").is_ok());
        sleep(Duration::from_millis(100));

        client.connect(&format!("tcp://127.0.0.1:{}", port)).unwrap();
        sleep(Duration::from_millis(100));

        client.send_str("test").unwrap();
        assert_eq!(server.recv_str().unwrap().unwrap(), "test");
    }

    fn test_plain() {
        let zauth = ZAuth::new(None).unwrap();

        let server = ZSock::new(ZSockType::PULL);
        server.set_zap_domain("sky.net");
        server.set_plain_server(true);
        server.set_rcvtimeo(100);
        let port = server.bind("tcp://127.0.0.1:*[60000-]").unwrap();

        let client = ZSock::new(ZSockType::PUSH);
        client.set_plain_username("moo");
        client.set_plain_password("cow");
        client.set_linger(100);
        client.set_sndtimeo(100);
        client.connect(&format!("tcp://127.0.0.1:{}", port)).unwrap();

        sleep(Duration::from_millis(100));

        client.send_str("test").unwrap();
        assert!(server.recv_str().is_err());

        let mut passwd_file = NamedTempFile::new().unwrap();
        passwd_file.write_all("moo=cow\n".as_bytes()).unwrap();

        zauth.load_plain(passwd_file.path().to_str().unwrap()).unwrap();
        sleep(Duration::from_millis(100));

        client.connect(&format!("tcp://127.0.0.1:{}", port)).unwrap();
        sleep(Duration::from_millis(100));

        client.send_str("test").unwrap();
        assert_eq!(server.recv_str().unwrap().unwrap(), "test");
    }

    fn test_curve() {
        let zauth = ZAuth::new(None).unwrap();

        let server = ZSock::new(ZSockType::PULL);
        let server_cert = ZCert::new().unwrap();
        server_cert.apply(&server);
        server.set_zap_domain("sky.net");
        server.set_curve_server(true);
        server.set_rcvtimeo(100);
        let port = server.bind("tcp://127.0.0.1:*[60000-]").unwrap();

        let endpoint = format!("tcp://127.0.0.1:{}", port);
        let client = ZSock::new_push(&endpoint).unwrap();
        let client_cert = ZCert::new().unwrap();
        client_cert.apply(&client);
        client.set_curve_serverkey(server_cert.public_txt());
        client.set_linger(100);
        client.set_sndtimeo(100);

        sleep(Duration::from_millis(100));

        client.send_str("test").unwrap();
        assert!(server.recv_str().is_err());

        zauth.load_curve(None).unwrap();
        sleep(Duration::from_millis(100));

        client.connect(&endpoint).unwrap();
        sleep(Duration::from_millis(100));

        client.send_str("test").unwrap();
        assert_eq!(server.recv_str().unwrap().unwrap(), "test");

        let auth_client = ZSock::new(ZSockType::PUSH);
        auth_client.set_curve_serverkey(server_cert.public_txt());
        auth_client.set_linger(100);
        auth_client.set_sndtimeo(100);

        let dir = TempDir::new("czmq_test").unwrap();
        let auth_client_cert = ZCert::new().unwrap();
        auth_client_cert.set_meta("moo", "cow");
        auth_client_cert.set_meta("woof", "dog");
        auth_client_cert.apply(&auth_client);
        auth_client_cert.save_public(&format!("{}/testcert.txt", dir.path().to_str().unwrap())).unwrap();

        zauth.load_curve(dir.path().to_str()).unwrap();
        sleep(Duration::from_millis(100));

        auth_client.connect(&endpoint).unwrap();

        auth_client.send_str("test").unwrap();

        let frame = ZFrame::recv(&server).unwrap();
        assert_eq!(frame.data().unwrap().unwrap(), "test");
        assert_eq!(frame.meta("moo").unwrap().unwrap().unwrap(), "cow");
        assert_eq!(frame.meta("woof").unwrap().unwrap().unwrap(), "dog");
    }

    fn test_zcertstore() {
        let certstore = ZCertStore::new(None).unwrap();
        certstore.set_loader(test_loader_fn);

        let _zauth = ZAuth::new(Some(certstore)).unwrap();

        let public_key = [ 105, 76, 150, 58, 214, 191, 218, 65, 50, 172,
                           131, 188, 247, 211, 136, 170, 227, 26, 57, 170,
                           185, 63, 246, 225, 177, 230, 12, 8, 134, 136,
                           105, 106 ];
        let secret_key = [ 245, 217, 172, 73, 106, 28, 195, 17, 218, 132,
                           135, 209, 99, 240, 98, 232, 7, 137, 244, 100,
                           242, 23, 29, 114, 70, 223, 83, 1, 113, 207,
                           132, 149 ];
        let cert = ZCert::from_keys(&public_key, &secret_key);

        let server = ZSock::new(ZSockType::PULL);
        server.set_zap_domain("sky.net");
        server.set_curve_server(true);
        server.set_rcvtimeo(100);
        cert.apply(&server);
        let port = server.bind("tcp://127.0.0.1:*[60000-]").unwrap();

        let client = ZSock::new(ZSockType::PUSH);
        client.set_curve_serverkey(cert.public_txt());
        client.set_linger(100);
        client.set_sndtimeo(100);
        cert.apply(&client);
        client.connect(&format!("tcp://127.0.0.1:{}", port)).unwrap();

        sleep(Duration::from_millis(200));

        client.send_str("test").unwrap();
        assert_eq!(server.recv_str().unwrap().unwrap(), "test");
    }

    unsafe extern "C" fn test_loader_fn(raw: *mut ZCertStoreRaw) {
        let store = ZCertStore::from_raw(raw, false);
        store.empty();
        store.insert(ZCert::new().unwrap());

        let public_key = [ 105, 76, 150, 58, 214, 191, 218, 65, 50, 172,
                           131, 188, 247, 211, 136, 170, 227, 26, 57, 170,
                           185, 63, 246, 225, 177, 230, 12, 8, 134, 136,
                           105, 106 ];
        let secret_key = [ 245, 217, 172, 73, 106, 28, 195, 17, 218, 132,
                           135, 209, 99, 240, 98, 232, 7, 137, 244, 100,
                           242, 23, 29, 114, 70, 223, 83, 1, 113, 207,
                           132, 149 ];
        let cert = ZCert::from_keys(&public_key, &secret_key);
        store.insert(cert);
    }
}
