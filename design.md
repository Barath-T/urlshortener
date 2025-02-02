

# db
  - get_record(short_url)
  - insert(shorturl, originalurl, expirrationdate, max_uses)
  - delete(shorturl)

  - get_last_id(range);
    - increment and return
# api
  - get
    - check uses
    - original link
  - post
    - body {expiration_date, max_uses, original_link}
    - get_last_id
    - convert
    - insert

# server
  - start sever
  - maintain threads
  - parse requests
  - response

# server - api
```

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
    pub fn response<T>(&mut self, status_code: u16, body: &T) -> Result<String, RequestError>;
    where
        T: ?Sized + Serialize
}
```

# Todo
> replace todos with our api calls
```
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
```

