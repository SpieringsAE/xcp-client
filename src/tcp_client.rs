use std::{
    io::{Read, Write},
    net::SocketAddr,
    sync::mpsc::channel,
    thread,
    time::Duration,
};

use crate::{
    connection::XcpConnection, protocol::commands::XcpCommand,
    protocol::standard::XcpConnectResponse,
};

pub struct TcpConnectionParams {
    pub addr: SocketAddr,
    pub timeout: Duration,
}

impl XcpConnection {
    ///Try to connect to a TCP based XCP server
    pub fn tcp_connect(params: TcpConnectionParams) -> Option<XcpConnection> {
        let (xcp_send, writer) = channel::<Vec<u8>>();
        let (reader, xcp_receive) = channel::<Vec<u8>>();
        let xcp_send = xcp_send.clone();

        thread::spawn(move || {
            //receive thread
            let mut readstream =
                std::net::TcpStream::connect_timeout(&params.addr, params.timeout).unwrap();
            let mut writestream = readstream.try_clone().unwrap();

            thread::spawn(move || {
                //send thread
                loop {
                    let recv = writer.recv().unwrap();
                    if recv[0] == XcpCommand::Disconnect.0 {
                        writestream
                            .write(recv.as_slice())
                            .expect("Could not send response to XCP server");
                        break;
                    }
                    writestream
                        .write(recv.as_slice())
                        .expect("Could not send response to XCP server");
                }
            });

            loop {
                //receive loop
                let mut recv_buf = Vec::new();
                readstream.read_to_end(&mut recv_buf).unwrap();
                reader.send(recv_buf).unwrap();
            }
        });
        // Send Connect request, fails if the TcpStream fails to connect
        let Ok(()) = xcp_send.send(vec![XcpCommand::Connect.0, 0]) else {
            return None;
        };
        // Receive response, probably shouldn't fail if the sending failed, unless the connection died inbetween those times.
        let Ok(response) = xcp_receive.recv() else {
            return None;
        };
        // Validate response and save information in the Client struct
        let Some(connect_response) = XcpConnectResponse::from_bytes(response.as_slice()) else {
            return None;
        };
        Some(XcpConnection {
            xcp_send,
            xcp_receive,
        })
    }
}
