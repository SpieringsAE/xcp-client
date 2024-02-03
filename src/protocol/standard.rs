use super::commands::XcpResponse;

pub struct XcpConnectResponse {
    pub resource: u8,
    pub mode: u8,
    pub cto_length: u8,
    pub dto_length: u16,
    pub xcp_proto_version: u8,
    pub xcp_transport_version: u8,
}

impl XcpConnectResponse {
    pub fn from_bytes(response: &[u8]) -> Option<Self> {
        if response.len() < 8 {
            return None;
        }
        match XcpResponse(response[0]) {
            XcpResponse::Result => Some(XcpConnectResponse {
                resource: response[1],
                mode: response[2],
                cto_length: response[3],
                dto_length: u16::from_le_bytes(response[4..5].try_into().unwrap()),
                xcp_proto_version: response[6],
                xcp_transport_version: response[7],
            }),
            _ => None,
        }
    }
}
