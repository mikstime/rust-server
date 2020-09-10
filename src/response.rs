use std::ops::Add;

pub fn parse (req: &std::net::TcpStream) -> String {
    let mut response = String::new();
    response.push_str("HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body>Hello world!!!!</body></html>\r\n");
//    println!("{:?}",response.as_bytes().clone());
//    b"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body>Hello world!!!!</body></html>\r\n"
    response
}