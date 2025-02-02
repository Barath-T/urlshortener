mod request;
mod server;

use request::Request;
use server::Server;

use std::collections::HashMap;
use std::net::TcpStream;

fn main() {
    let server: Server = Server::new("localhost:8080");
    loop {
        let stream: TcpStream = server.accept();
        let mut request_str = Request::new(stream);
        println!("{:?}", request_str);
        request_str
            .response(404, HashMap::from([("helo", "world"), ("nothing", "here")]))
            .unwrap();
    }
}
