mod request;
mod server;

use request::{Request, RequestType};
use server::Server;

use std::collections::HashMap;
use std::net::TcpStream;

fn main() {
    let server: Server = Server::new("127.0.0.1:8080");
    loop {
        let stream: TcpStream = server.accept();
        let mut request = match Request::new(stream) {
            Ok(req) => req,
            Err(err) => {
                eprintln!("{:?}", err);
                continue;
            }
        };
        println!("{:?}", request);

        match request.path.as_str() {
            "/" => match request.req_type {
                RequestType::POST => todo!(),
                RequestType::GET => todo!(),
            },
            _ => match request.response(404, &HashMap::from([("message", "not found")])) {
                Err(err) => {
                    eprintln!("{:?}", err);
                    continue;
                },
                _ => ()
            },
        }
    }
}
