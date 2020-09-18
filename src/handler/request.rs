use async_std::path::{Path, PathBuf};
use async_std::io::*;

#[derive(Clone)]
pub struct Request {
    _stream: async_std::net::TcpStream,
    _path: PathBuf,
    _protocol: String,
    _method: String,
}

const BASE_DIR: &str = "./public/";

fn prepare_path(path: PathBuf) -> PathBuf {
    let path = path.as_path();
    let _ = match path.file_name() {
        None => "",
        _ => path.file_name().unwrap().to_str().unwrap(),
    };
    let file_ext = path.extension();
    let mut path_to_use = path.to_str().unwrap().to_string();
    if file_ext == None {
        path_to_use = path_to_use + "/index.html"
    }

    if path_to_use.starts_with("/") {
        path_to_use = path_to_use[1..].parse().unwrap()
    }

    let path_combined: PathBuf = [BASE_DIR, path_to_use.as_str()].iter().collect();
    return path_combined;
}

impl Request {
    pub async fn new(mut stream: async_std::net::TcpStream) -> Request {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).await;
        let strings = String::from_utf8_lossy(&buffer);
        // Split request by rows
        let split_strings = strings.split("\r\n").collect::<Vec<&str>>();
        // First line contains Method, Path, Protocol
        let mut first_line_split = split_strings[0].split(" ").collect::<Vec<&str>>();
        // Extract them
        let method = first_line_split[0].trim();
        if first_line_split.len() < 3 {
            while first_line_split.len() < 3 {
                first_line_split.push("");
            }
        }
        let url = percent_encoding::percent_decode(first_line_split[1].trim().as_bytes()).decode_utf8_lossy().to_string();
        let split_url: Vec<&str> = url.split("?").collect();
        let path = prepare_path(PathBuf::from(split_url[0]));
        let protocol = first_line_split[2].trim();
        Request {
            _stream: stream,
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
    pub fn stream(&self) -> &async_std::net::TcpStream {
        &self._stream
    }
}