//! Module: czmq-zmonitor

use {czmq_sys, Error, ErrorKind, RawInterface, Result, Sockish, ZActor, ZMsg};
use std::{error, ptr, result};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::os::raw::c_void;

#[derive(Debug, PartialEq)]
pub enum ZMonitorEvents {
    Connected,
    ConnectDelayed,
    ConnectRetried,
    Listening,
    BindFailed,
    Accepted,
    AcceptFailed,
    Closed,
    CloseFailed,
    Disconnected,
    MonitorStopped,
    All,
    Unknown,
}

impl ZMonitorEvents {
    pub fn to_str<'a>(&'a self) -> &'a str {
        match self {
            &ZMonitorEvents::Connected      => "CONNECTED",
            &ZMonitorEvents::ConnectDelayed => "CONNECT_DELAYED",
            &ZMonitorEvents::ConnectRetried => "CONNECT_RETRIED",
            &ZMonitorEvents::Listening      => "LISTENING",
            &ZMonitorEvents::BindFailed     => "BIND_FAILED",
            &ZMonitorEvents::Accepted       => "ACCEPTED",
            &ZMonitorEvents::AcceptFailed   => "ACCEPT_FAILED",
            &ZMonitorEvents::Closed         => "CLOSED",
            &ZMonitorEvents::CloseFailed    => "CLOSE_FAILED",
            &ZMonitorEvents::Disconnected   => "DISCONNECTED",
            &ZMonitorEvents::MonitorStopped => "MONITOR_STOPPED",
            &ZMonitorEvents::All            => "ALL",
            &ZMonitorEvents::Unknown        => "UNKNOWN",
        }
    }

    pub fn from_str(event: &str) -> ZMonitorEvents {
        match event {
            "CONNECTED"       => ZMonitorEvents::Connected,
            "CONNECT_DELAYED" => ZMonitorEvents::ConnectDelayed,
            "CONNECT_RETRIED" => ZMonitorEvents::ConnectRetried,
            "LISTENING"       => ZMonitorEvents::Listening,
            "BIND_FAILED"     => ZMonitorEvents::BindFailed,
            "ACCEPTED"        => ZMonitorEvents::Accepted,
            "ACCEPT_FAILED"   => ZMonitorEvents::AcceptFailed,
            "CLOSED"          => ZMonitorEvents::Closed,
            "CLOSE_FAILED"    => ZMonitorEvents::CloseFailed,
            "DISCONNECTED"    => ZMonitorEvents::Disconnected,
            "MONITOR_STOPPED" => ZMonitorEvents::MonitorStopped,
            "ALL"             => ZMonitorEvents::All,
            _                 => ZMonitorEvents::Unknown,
        }
    }
}

impl Display for ZMonitorEvents {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "ZMonitorEvent: {}", self.to_str())
    }
}

pub struct ZMonitor {
    zactor: ZActor,
}

unsafe impl Send for ZMonitor {}

impl ZMonitor {
    pub fn new<S: Sockish>(zsock: &S) -> Result<ZMonitor> {
        let zactor = unsafe { czmq_sys::zactor_new(czmq_sys::zmonitor, zsock.borrow_raw()) };

        if zactor == ptr::null_mut() {
            Err(Error::new(ErrorKind::NullPtr, ZMonitorError::Instantiate))
        } else {
            Ok(ZMonitor {
                zactor: ZActor::from_raw(zactor as *mut c_void, true),
            })
        }
    }

    pub fn set_attrs(&self, attrs: &[ZMonitorEvents]) -> Result<()> {
        let msg = ZMsg::new();
        try!(msg.addstr("LISTEN"));
        for a in attrs {
            try!(msg.addstr(a.to_str()));
        }

        self.zactor.send(msg)
    }

    pub fn get_attr(&self) -> Result<result::Result<ZMonitorEvents, Vec<u8>>> {
        let msg = try!(ZMsg::recv(&self.zactor));

        match msg.popstr() {
            Some(result) => match result {
                Ok(s) => Ok(Ok(ZMonitorEvents::from_str(&s))),
                Err(v) => Ok(Err(v)),
            },
            None => Err(Error::new(ErrorKind::MissingFrame, ZMonitorError::MissingAttr))
        }
    }

    pub fn start(&self) -> Result<()> {
        try!(self.zactor.send_str("START"));
        self.zactor.sock().wait()
    }

    pub fn verbose(&self) -> Result<()> {
        self.zactor.send_str("VERBOSE")
    }
}

#[derive(Debug)]
pub enum ZMonitorError {
    Instantiate,
    MissingAttr,
}

impl Display for ZMonitorError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match *self {
            ZMonitorError::Instantiate => write!(f, "Could not instantiate new ZMonitor struct"),
            ZMonitorError::MissingAttr => write!(f, "ZMonitor didn't reply with attribute"),
        }
    }
}

impl error::Error for ZMonitorError {
    fn description(&self) -> &str {
        match *self {
            ZMonitorError::Instantiate => "Could not instantiate new ZMonitor struct",
            ZMonitorError::MissingAttr => "ZMonitor didn't reply with attribute",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use {ZSock, ZSockType, ZSys};

    #[test]
    fn test_attrs() {
        ZSys::init();

        let server = ZSock::new(ZSockType::PULL);
        let server_mon = ZMonitor::new(&server).unwrap();
        server_mon.set_attrs(&[ZMonitorEvents::All]).unwrap();
        server_mon.start().unwrap();

        let client = ZSock::new(ZSockType::PUSH);
        let client_mon = ZMonitor::new(&client).unwrap();
        client_mon.set_attrs(&[ZMonitorEvents::All]).unwrap();
        client_mon.start().unwrap();

        server.bind("ipc://zmonitor_test").unwrap();
        assert_eq!(server_mon.get_attr().unwrap().unwrap(), ZMonitorEvents::Listening);

        client.connect("ipc://zmonitor_test").unwrap();
        assert_eq!(client_mon.get_attr().unwrap().unwrap(), ZMonitorEvents::Connected);
    }

    #[test]
    fn test_verbose() {
        ZSys::init();

        let zsock = ZSock::new(ZSockType::REP);
        let zmonitor = ZMonitor::new(&zsock).unwrap();
        assert!(zmonitor.verbose().is_ok());
    }
}
