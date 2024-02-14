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

///Parameters necessary to set up a XCP connection over the TCP IP Protocol
pub struct TcpConnectionParams {
    ///IP address of the server you wish to connect to
    pub addr: SocketAddr,
    ///Acceptable response delay from the server before the connection is deemed terminated
    pub timeout: Duration,
}

impl XcpConnection {
    ///Try to connect to a TCP based XCP server
    /// * params
    pub fn tcp_connect(params: TcpConnectionParams) -> Option<XcpConnection> {
        let (xcp_send, writer) = channel::<Box<[u8]>>();
        let (reader, xcp_receive) = channel::<Box<[u8]>>();
        let xcp_send = xcp_send.clone();

        thread::spawn(move || {
            //receive thread
            let mut readstream =
                std::net::TcpStream::connect_timeout(&params.addr, params.timeout).unwrap();
            let mut writestream = readstream.try_clone().unwrap(); // thread is allowed to panic and die if it can't make a readstream

            thread::spawn(move || {
                //send thread
                loop {
                    let recv = writer.recv().unwrap(); // thread is allowed to panic and die when the channel is broken
                    if recv[0] == XcpCommand::Disconnect.0 {
                        _ = writestream
                            .write_all(&recv);
                        _=writestream.flush();
                        break;
                    }
                    if let Err(_) =writestream
                        .write_all(&recv){
                            eprintln!("write stream is terminating");
                            break;
                    }
                }
            });
            let mut recv_buf = [0u8;2048];
            loop {
                //receive loop
                let num_bytes = readstream.read(&mut recv_buf).unwrap();
                if let Err(_) = reader.send(recv_buf[0..num_bytes].into()) {
                    eprintln!("receive stream is terminating");
                    break;
                }
            }
        });
        // Send Connect request, fails if the TcpStream fails to connect
        let Ok(()) = xcp_send.send([XcpCommand::Connect.0, 0].into()) else {
            return None;
        };
        // Receive response, probably shouldn't fail if the sending failed, unless the connection died inbetween those times.
        let Ok(response) = xcp_receive.recv_timeout(params.timeout) else {
            return None;
        };
        // Validate response and save information in the Connection struct
        let Ok(connect_response) = bincode::deserialize::<XcpConnectResponse>(&response) else {
            return None;
        };
        Some(XcpConnection {
            xcp_send,
            xcp_receive,
            timeout: params.timeout,
        })
    }
}
