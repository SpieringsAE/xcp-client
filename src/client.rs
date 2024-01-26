use crate::tcp_client::TCPClient;

pub enum XCPConnectionType {
	XCPUSB,
	XCPCAN,
	XCPSERIAL,
	XCPTCP,
	XCPUDP,
}

pub struct XCPClient;

impl XCPClient {
	pub fn new(xcp_type: XCPConnectionType) -> impl XCP {
		match xcp_type {
			XCPConnectionType::XCPTCP => TCPClient {},
			_ => TCPClient {},
		}
	}
}

pub trait XCP {

}