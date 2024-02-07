use std::{
    net::SocketAddr,
    sync::mpsc::{Receiver, RecvError, SendError, Sender},
    time::Duration,
};

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
    // maybe a better datatype than Vec<u8>, needs to be implement Send to go the connection handling thread.
    pub xcp_send: Sender<Vec<u8>>,
    pub xcp_receive: Receiver<Vec<u8>>,
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
        _ = self.send(vec![XcpCommand::Disconnect.0]);
    }

    #[inline(always)]
    fn send(&self, bytes: Vec<u8>) -> Result<(), SendError<Vec<u8>>> {
        self.xcp_send.send(bytes)
    }

    #[inline(always)]
    fn receive(&self) -> Result<Vec<u8>, RecvError> {
        self.xcp_receive.recv()
    }
}

pub struct XcpServerResponse {
    pub addr: SocketAddr,
    pub udp: bool,
    pub tcp: bool,
    pub connected: bool,
}

impl XcpServerResponse {
    // how to specify UDP? enum?
    pub fn connect(self, timeout: Duration) -> Option<XcpConnection> {
        XcpConnection::tcp_connect(TcpConnectionParams {
            addr: self.addr,
            timeout,
        })
    }
    //how to decode?
    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        todo!("what does this response look like in bytes? Check for echo due to broadcast?")
    }
}

pub fn discover_ethernet_xcp_servers(timeout: Duration) -> std::io::Result<Vec<XcpServerResponse>> {
    let socket = std::net::UdpSocket::bind("0.0.0.0:5556")?;
    socket.set_broadcast(true)?;
    socket.set_read_timeout(Some(timeout))?;
    let buf = todo!("GET_SLAVE_ID? Vector XCP_Book_V1.5_EN.pdf 1.6.4.1");
    _ = socket.send_to(buf, "239.255.0.0:5556")?;
    let mut result = Vec::new();
    let mut buf = Vec::new();
    while let Ok(response) = socket.recv(&mut buf) {
        if let Some(xcp_response) = XcpServerResponse::from_bytes(buf.as_slice()) {
            result.push(xcp_response);
        }
    }
    Ok(result)
}
