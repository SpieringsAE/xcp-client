use crate::tcp_client::{TCPClient, TCPConnectionParams};

pub enum XCPConnectionType {
    XCPUSB,
    XCPCAN,
    XCPSERIAL,
    XCPTCP(TCPConnectionParams),
    XCPUDP,
}

pub struct XCPClient;

impl XCPClient {
    pub fn new(xcp_type: XCPConnectionType) -> impl XCP {
        match xcp_type {
            XCPConnectionType::XCPTCP(params) => TCPClient::connect(params),
            _ => unreachable!(),
        }
    }
}

pub trait XCP {}
