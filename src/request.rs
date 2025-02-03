use serde::Serialize;
use serde_json;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;

#[derive(Debug)]
pub enum RequestError {
    SerializeError(String),
    DeserializeError(String),
    StreamError(String),
    BufferError(String),
    ParseError(String),
    NotSupportedError(String),
}
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
    pub body: Option<HashMap<String, String>>,
}

impl Request {
    pub fn new(request_stream: TcpStream) -> Result<Self, RequestError> {
        let (req_type, path, headers, body) = Self::parse(&request_stream)?;

        Ok(Self {
            stream: request_stream,
            path,
            req_type,
            headers,
            body,
        })
    }
    pub fn response<T>(&mut self, status_code: u16, body: &T) -> Result<String, RequestError>
    where
        T: ?Sized + Serialize,
    {
        let status_line: String = format!("HTTP/1.1 {} {}", status_code, "OK");
        let serialized_body: String = match serde_json::to_string(&body) {
            Ok(s) => s,
            Err(err) => return Err(RequestError::SerializeError(err.to_string())),
        };

        let response: String = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status_line,
            serialized_body.len(),
            serialized_body
        );
        match self.stream.write(response.as_bytes()) {
            Err(err) => return Err(RequestError::StreamError(err.to_string())),
            _ => (),
        };
        match self.stream.flush() {
            Err(err) => return Err(RequestError::StreamError(err.to_string())),
            _ => (),
        }

        Ok(response)
    }
    fn parse(
        req: &TcpStream,
    ) -> Result<(RequestType, String, String, Option<HashMap<String, String>>), RequestError> {
        let mut bufreader: BufReader<TcpStream> = BufReader::new(match req.try_clone() {
            Ok(stream) => stream,
            Err(err) => return Err(RequestError::StreamError(err.to_string())),
        });

        let mut request_line: String = String::new();
        let mut headers: String = String::new();
        let mut content_length: u32 = 0;

        match bufreader.read_line(&mut request_line) {
            Err(err) => return Err(RequestError::BufferError(err.to_string())),
            _ => (),
        }

        let mut split = request_line.split_whitespace();
        let req_type: RequestType = match split.next() {
            Some("GET") => RequestType::GET,
            Some("POST") => RequestType::POST,
            _ => {
                return Err(RequestError::NotSupportedError(
                    "This type of request is not supported".to_string(),
                ))
            }
        };

        let path = match split.next() {
            Some(s) => s,
            None => {
                return Err(RequestError::ParseError(
                    "Couldn't find path in header of the request".to_string(),
                ))
            }
        };

        let mut request_line: String = String::new();

        while match bufreader.read_line(&mut request_line) {
            Ok(n) => n,
            Err(err) => return Err(RequestError::BufferError(err.to_string())),
        } > 0
        {
            headers.push_str(&request_line);
            let mut split = request_line.split_whitespace();

            if split.next() == Some("Content-Length:") {
                content_length = match match split.next() {
                    Some(s) => s,
                    None => {
                        return Err(RequestError::ParseError(
                            "Unable to parse content length".to_string(),
                        ))
                    }
                }
                .trim()
                .parse::<u32>()
                {
                    Ok(n) => n,
                    Err(err) => return Err(RequestError::ParseError(err.to_string())),
                };
            }
            if request_line == "\r\n" {
                break;
            }

            request_line.clear();
        }

        let mut body: Option<HashMap<String, String>> = None;
        let mut buf: Vec<u8> = vec![0; content_length as usize];
        match bufreader.read_exact(&mut buf) {
            Err(err) => return Err(RequestError::BufferError(err.to_string())),
            _ => (),
        };

        if content_length > 0 {
            body = Some(match serde_json::from_str(&String::from_utf8_lossy(&buf)) {
                Ok(map) => map,
                Err(err) => return Err(RequestError::DeserializeError(err.to_string())),
            });
        }

        Ok((req_type, path.to_string(), headers, body))
    }
}
