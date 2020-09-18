use async_std::prelude::*;
use std::net::Shutdown;
#[derive(Clone, Copy)]
enum Status {
    NOT_FOUND,
    SUCCESS,
    METHOD_NOT_ALLOWED,
    FORBIDDEN
}
impl Status {
    pub fn extract(s: Status) -> (i32, String) {
        match s {
            Status::SUCCESS => (200, String::from("Success")),
            Status::FORBIDDEN => (403, String::from("Forbidden")),
            Status::NOT_FOUND => (404, String::from("Not Found")),
            Status::METHOD_NOT_ALLOWED => (405, String::from("Method Not Allowed")),
        }
    }
}
#[derive(Clone)]
pub struct Response {
    _stream: async_std::net::TcpStream,
    _status: Status,
    _content_type: String,
    _content_length: String,
    _server: String,
    _connection: String,
    _date: std::time::SystemTime,
}
impl Response {
    pub fn set_stream(&mut self, stream : async_std::net::TcpStream) {
        self._stream = stream;
    }
    pub fn set_status(&mut self, status: i32) {
        self._status = match status {
            403 => Status::FORBIDDEN,
            404 => Status::NOT_FOUND,
            405 => Status::METHOD_NOT_ALLOWED,
            _ => Status::SUCCESS,
        };
    }
    pub fn set_content_type(&mut self, content_type: String) {
        self._content_type = content_type;
    }
    pub fn set_content_length(&mut self, content_length: String) {
        self._content_length = content_length;
    }
    pub fn set_date(&mut self, date: std::time::SystemTime) {
        self._date = date
    }
    pub fn set_server(&mut self, server: String) {
        self._server = server
    }
    pub fn set_connection(&mut self, connection: String) {
        self._connection = connection
    }
}
impl Response {
    pub fn stream(&mut self) -> &async_std::net::TcpStream {
        &self._stream
    }
    pub fn status(&mut self) -> &Status {
        &self._status
    }
    pub fn content_type(&mut self) -> &String {
        &self._content_type
    }
    pub fn content_length(&mut self) -> &String {
        &self._content_length
    }
    pub fn date(&mut self) -> &std::time::SystemTime {
        &self._date
    }
    pub fn server(&mut self) -> &String {
        &self._server
    }
    pub fn connection(&mut self) -> &String {
        &self._connection
    }
}
impl Response {
    fn prepare(&self) -> String {
        let (status_code, status_name) = Status::extract(self._status);
        format!(
            "HTTP/1.1 {} {}\r\nDate: {}\r\n\
            Server: {}\r\nContent-Length: {}\r\nContent-Type: {}\r\nConnection: {}\r\n\r\n",
            status_code, status_name, httpdate::fmt_http_date(self._date),
            self._server, self._content_length, self._content_type, self._connection)
    }
    pub async fn new(stream: async_std::net::TcpStream) -> Response {
        Response {
            _stream: stream,
            _status: Status::SUCCESS,
            _content_type: "application/octet-stream".to_string(),
            _content_length: "0".to_string(),
            _server: "Rusty".to_string(),
            _date: std::time::SystemTime::now(),
            _connection: "closer".to_string(),
        }
    }
    pub async fn send(&mut self) -> std::io::Result<()>{
        let (status_code, status_name) = Status::extract(self._status);
        let res = self.prepare();
        self.stream().write(res.as_bytes()).await?;
        self.stream().shutdown(Shutdown::Both)?;
        Ok(())
    }
    pub async fn send_content(&mut self, mut content: async_std::fs::File) -> std::io::Result<()>
    {
        let res = self.prepare();

        self.stream().write(res.as_bytes()).await?;
        async_std::io::copy(&mut content, &mut self.stream()).await?;
        self.stream().shutdown(Shutdown::Both)?;
        Ok(())
    }
}