use std::{
    io::{Read, Write},
    net::SocketAddr,
    sync::mpsc::{channel, Receiver, Sender},
    thread,
    time::Duration,
};

use crate::client::XCP;

pub struct TCPConnectionParams {
    addr: SocketAddr,
    timeout: Duration,
}

pub struct TCPClient {
    xcp_send: Sender<Vec<u8>>,
    xcp_receive: Receiver<Vec<u8>>,
}

impl TCPClient {
    pub fn connect(params: TCPConnectionParams) -> Self {
        let (xcp_send, writer) = channel::<Vec<u8>>();
        let (reader, xcp_receive) = channel::<Vec<u8>>();
        let xcp_send = xcp_send.clone();
        thread::spawn(move || {
            let mut readstream =
                std::net::TcpStream::connect_timeout(&params.addr, params.timeout).unwrap();
            let mut writestream = readstream.try_clone().unwrap();
            thread::spawn(move || loop {
                let recv = writer.recv().unwrap();
                writestream.write(recv.as_slice());
            });
            loop {
                let mut recv_buf = Vec::new();
                readstream.read_to_end(&mut recv_buf).unwrap();
                reader.send(recv_buf).unwrap();
            }
        });
        TCPClient {
            xcp_send,
            xcp_receive,
        }
    }
}

impl XCP for TCPClient {}
