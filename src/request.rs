use serde_json;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;

#[derive(Debug)]
pub enum RequestType {
    GET,
    POST,
}
#[derive(Debug)]
pub struct Request {
    stream: TcpStream,
    pub req_type: RequestType,
    pub path: String,
    pub headers: String,
    pub body: HashMap<String, String>,
}

impl Request {
    pub fn new(request_stream: TcpStream) -> Self {
        let (req_type, path, headers, body) = Self::parse(&request_stream);

        Self {
            stream: request_stream,
            path,
            req_type,
            headers,
            body,
        }
    }
    pub fn response(&mut self, status_code: u16, body: HashMap<&str, &str>) -> Result<(), String> {
        let status_line: String = format!("HTTP/1.1 {} {}", status_code, "OK");
        let serialized_body: String = serde_json::to_string(&body).unwrap();

        let response: String = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status_line,
            serialized_body.len(),
            serialized_body
        );
        self.stream.write(response.as_bytes()).unwrap();
        self.stream.flush().unwrap();
        Ok(())
    }
    fn parse(req: &TcpStream) -> (RequestType, String, String, HashMap<String, String>) {
        let mut bufreader: BufReader<TcpStream> = BufReader::new(req.try_clone().unwrap());

        let mut request_line: String = String::new();
        let mut headers: String = String::new();
        let mut content_length: u32 = 0;

        bufreader.read_line(&mut request_line).unwrap();

        let mut split = request_line.split_whitespace();
        let req_type: RequestType = match split.next() {
            Some("GET") => RequestType::GET,
            Some("POST") => RequestType::POST,
            _ => todo!(),
        };

        let path = match split.next() {
            Some(s) => s,
            None => "/",
        };

        let mut request_line: String = String::new();

        while bufreader.read_line(&mut request_line).unwrap() > 0 {
            headers.push_str(&request_line);
            let mut split = request_line.split_whitespace();

            if split.next() == Some("Content-Length:") {
                content_length = split.next().unwrap().trim().parse::<u32>().unwrap();
            }
            if request_line == "\r\n" {
                break;
            }

            request_line.clear();
        }

        let mut body: HashMap<String, String> = HashMap::new();
        let mut buf: Vec<u8> = vec![0; content_length as usize];
        bufreader.read_exact(&mut buf);

        if content_length > 0 {
            body = serde_json::from_str(&String::from_utf8_lossy(&buf)).unwrap();
        }

        (req_type, path.to_string(), headers, body)
    }
}
