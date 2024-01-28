use std::os::unix::net::SocketAddr;

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

pub struct XCPServerResponse {
    addr: SocketAddr,
    udp: bool,
    tcp: bool,
    connected: bool,
}

pub fn discover_xcp_servers() -> std::io::Result<Vec<XCPServerResponse>> {
    let socket = std::net::UdpSocket::bind("0.0.0.0:5555")?;
    socket.set_broadcast(true)?;
    todo!("GET_SLAVE_ID?");
}

pub trait XCP {}
