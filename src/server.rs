use std::io::{BufReader, Read};
use std::net::{TcpListener, TcpStream};

use crate::request::Request;

pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub fn new(addr: &str) -> Server {
        let listener: TcpListener = TcpListener::bind(addr).unwrap();

        Server { listener }
    }
    pub fn accept(&self) -> TcpStream {
        let (stream, _) = self.listener.accept().unwrap();

        return stream;
    }
}
