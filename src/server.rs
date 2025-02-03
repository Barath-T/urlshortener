use std::net::{TcpListener, TcpStream};

pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub fn new(addr: &str) -> Server {
        let listener: TcpListener = TcpListener::bind(addr).expect("Unable to bind to the addr");

        Server { listener }
    }
    pub fn accept(&self) -> TcpStream {
        let (stream, _) = self.listener.accept().unwrap();

        return stream;
    }
}
