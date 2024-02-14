use std::{
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    sync::mpsc::{Receiver, RecvError, SendError, Sender},
    time::Duration,
};

use serde::Deserialize;

use crate::{protocol::commands::XcpCommand, tcp_client::TcpConnectionParams};

pub enum XcpConnectionType {
    //XCPUSB, cross_usb or nusb can be used, cross_usb seems more high end
    //XCPCAN, this is a difficult one, there is a socketcan package for linux but I can't find anything for windows as of yet
    //XCPSERIAL, unknown
    XcpTcp(TcpConnectionParams),
    //XCPUDP, should be similar to TCP
    //XCPFlexray wut?
}

///Main struct that provides an interface for an XCP connection
pub struct XcpConnection {
    // don't want these to be pub, but I can't initialize them in the client implementation if they aren´t, how fix?
    pub xcp_send: Sender<Box<[u8]>>,
    pub xcp_receive: Receiver<Box<[u8]>>,
    pub timeout: Duration,
    // add fields specifying cto/dto lenght and other connection specific information
    // Or make the xcp connect command a seperate method?
}

impl XcpConnection {
    //Make the XCPCONNECT command a seperate function or just always combine it?
    ///Make a new XCP connection of `XcpConnectionType`
    pub fn connect(xcp_type: XcpConnectionType) -> Option<Self> {
        match xcp_type {
            XcpConnectionType::XcpTcp(params) => Self::tcp_connect(params),
        }
    }

    ///Disconnect the XCP connection, consumes the connection.
    #[inline(always)]
    pub fn disconnect(self) {
        _ = self.send(&[XcpCommand::Disconnect.0]);
    }

    #[inline(always)]
    fn send(&self, bytes: &[u8]) -> Result<(), SendError<Box<[u8]>>> {
        self.xcp_send.send(bytes.into())
    }

    #[inline(always)]
    fn receive(&self) -> Result<Box<[u8]>, RecvError> {
        self.xcp_receive.recv()
    }

    #[inline(always)]
    fn receive_timeout(&self) -> Result<Box<[u8]>, std::sync::mpsc::RecvTimeoutError> {
        self.xcp_receive.recv_timeout(self.timeout)
    }
}

#[derive(Deserialize)]
pub struct XcpServerResponse {
    pub ip: [u8;4],
    pub port: u16,
    pub udp: bool,
    pub tcp: bool,
    pub connected: bool,
}

impl XcpServerResponse {
    // how to specify UDP? enum?
    pub fn connect(self, timeout: Duration) -> Option<XcpConnection> {
        XcpConnection::tcp_connect(TcpConnectionParams {
            addr: SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::try_from(self.ip).unwrap(), self.port)),
            timeout,
        })
    }
}

pub fn discover_ethernet_xcp_servers(timeout: Duration) -> std::io::Result<Vec<XcpServerResponse>> {
    let socket = std::net::UdpSocket::bind("0.0.0.0:5556")?;
    socket.set_broadcast(true)?;
    socket.set_read_timeout(Some(timeout))?;
    let buf = todo!("GET_SLAVE_ID? Vector XCP_Book_V1.5_EN.pdf 1.6.4.1");
    _ = socket.send_to(buf, "239.255.0.0:5556")?;
    let mut result = Vec::new();
    let mut buf = [0u8;512];
    while let Ok(response_size) = socket.recv(&mut buf) {
        if let Ok(xcp_response) = bincode::deserialize::<XcpServerResponse>(&buf[0..response_size]) {
            result.push(xcp_response);
        }
    }
    Ok(result)
}
