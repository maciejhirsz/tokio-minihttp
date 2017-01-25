use std::fmt::{self, Write};

use headers::Headers;

pub struct Response {
    headers: Headers,
    response: Vec<u8>,
    status_message: StatusMessage,
}

enum StatusMessage {
    Ok,
    Custom(u32, String)
}

impl Response {
    pub fn new() -> Response {
        Response {
            headers: Headers::new(),
            response: Vec::new(),
            status_message: StatusMessage::Ok,
        }
    }

    pub fn status_code(&mut self, code: u32, message: &str) -> &mut Response {
        self.status_message = StatusMessage::Custom(code, message.to_string());
        self
    }

    pub fn headers_mut(&mut self) -> &mut Headers {
        &mut self.headers
    }

    pub fn header(&mut self, name: &str, val: &str) -> &mut Response {
        self.headers.add(name, val);
        self
    }

    pub fn body(&mut self, s: &str) -> &mut Response {
        self.response = s.to_string().into_bytes();
        self
    }
}

impl From<String> for Response {
    fn from(res: String) -> Response {
        let mut headers = Headers::new();

        headers.add("Content-Type", "text/plain; charset=utf-8");

        Response {
            headers: headers,
            response: res.into_bytes(),
            status_message: StatusMessage::Ok,
        }
    }
}

impl<'a> From<&'a str> for Response {
    #[inline]
    fn from(res: &str) -> Response {
        res.to_string().into()
    }
}

pub fn encode(msg: Response, buf: &mut Vec<u8>) {
    let length = msg.response.len();
    let now = ::date::now();

    write!(FastWrite(buf), "\
        HTTP/1.1 {}\r\n\
        Content-Length: {}\r\n\
        Date: {}\r\n\
    ", msg.status_message, length, now).unwrap();

    for (k, v) in msg.headers.iter() {
        buf.extend_from_slice(k.as_bytes());
        buf.extend_from_slice(b": ");
        buf.extend_from_slice(v.as_bytes());
        buf.extend_from_slice(b"\r\n");
    }

    buf.extend_from_slice(b"\r\n");
    buf.extend_from_slice(&msg.response);
}

// TODO: impl fmt::Write for Vec<u8>
//
// Right now `write!` on `Vec<u8>` goes through io::Write and is not super
// speedy, so inline a less-crufty implementation here which doesn't go through
// io::Error.
struct FastWrite<'a>(&'a mut Vec<u8>);

impl<'a> fmt::Write for FastWrite<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.0.extend_from_slice(s.as_bytes());
        Ok(())
    }

    fn write_fmt(&mut self, args: fmt::Arguments) -> fmt::Result {
        fmt::write(self, args)
    }
}

impl fmt::Display for StatusMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StatusMessage::Ok => f.pad("200 OK"),
            StatusMessage::Custom(c, ref s) => write!(f, "{} {}", c, s),
        }
    }
}
