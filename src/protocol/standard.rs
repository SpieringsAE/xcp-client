use serde::Deserialize;

#[derive(Deserialize)]
pub struct XcpConnectResponse {
    pub response_code: u8,
    pub resource: u8,
    pub mode: u8,
    pub cto_length: u8,
    pub dto_length: u16,
    pub xcp_proto_version: u8,
    pub xcp_transport_version: u8,
}
