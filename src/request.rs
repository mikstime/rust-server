use std::path::{Path, PathBuf};

pub struct Request {
    _path: PathBuf,
    _protocol: String,
    _method: String,
}

impl Request {
    pub async fn new(buffer: &[u8; 4096]) -> Request {
        let strings = String::from_utf8_lossy(buffer);
        // Split request by rows
        let split_strings = strings.split("\r\n").collect::<Vec<&str>>();
        // First line contains Method, Path, Protocol
        let first_line_split = split_strings[0].split(" ").collect::<Vec<&str>>();
        // Extract them
        let method = first_line_split[0].trim();
        let path = PathBuf::from(first_line_split[1].trim());
        let protocol = first_line_split[2].trim();
        Request {
            _path: path,
            _method: method.to_string(),
            _protocol: protocol.to_string(),
        }
    }
    pub fn method(&self) -> &str {
        self._method.as_ref()
    }
    pub fn protocol(&self) -> &str {
        self._protocol.as_ref()
    }
    pub fn path(&self) -> &Path {
        self._path.as_path()
    }
}