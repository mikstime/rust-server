use std::net::TcpStream;
use std::path::Path;
use std::io::*;


mod open_file;
mod request;
async fn handle_connection(mut stream: std::net::TcpStream) -> std::io::Result<()> {
    let mut buffer = [0; 4096];
    stream.read(&mut buffer);
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    let req = request::Request::new(&buffer).await;
    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write(response.as_bytes())?;
    let mut w = std::io::BufWriter::new(stream);
    let mut r = open_file::open_file(req.path()).await;
    if !r.is_ok() {
        println!("Hello!");
        w.write(b"404 Not Found HTTP/1.1");
        println!("Hello!");
    } else {
        std::io::copy(&mut (r.unwrap()),&mut w);
    }
    Ok(())
}
#[async_std::main]
async fn main () {
    let listener = std::net::TcpListener::bind("127.0.0.1:3000").unwrap();

    let mut incoming = listener.incoming();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        if let Err(e) = handle_connection(stream).await {
            println!("{:?}", e);
        }
    }
    println!("Exit");
}