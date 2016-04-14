//! Module: czmq-zauth

use {czmq_sys, Result, ZActor, ZMsg};

pub struct ZAuth {
    zactor: ZActor,
}

unsafe impl Send for ZAuth {}

impl ZAuth {
    pub fn new() -> Result<ZAuth> {
        Ok(ZAuth {
            zactor: try!(ZActor::new(czmq_sys::zauth)),
        })
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

#[cfg(test)]
mod tests {
    use std::io::Write;
    use std::thread::sleep;
    use std::time::Duration;
    use super::*;
    use tempdir::TempDir;
    use tempfile::NamedTempFile;
    use {ZCert, ZFrame, ZSock, ZSockType, zsys_init};

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
    }

    fn test_verbose() {
        let zauth = ZAuth::new().unwrap();
        assert!(zauth.verbose().is_ok());
    }

    fn test_allow_deny() {
        let server = ZSock::new(ZSockType::PULL);
        server.set_zap_domain("compuglobalhypermega.net");
        server.set_rcvtimeo(100);

        let client = ZSock::new(ZSockType::PUSH);
        client.set_linger(100);
        client.set_sndtimeo(100);

        let zauth = ZAuth::new().unwrap();

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
        let zauth = ZAuth::new().unwrap();

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
        let zauth = ZAuth::new().unwrap();

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
}
