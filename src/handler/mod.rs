use async_std::prelude::*;
mod request;
mod open_file;
//@TODO head request
async fn handle_403(req: request::Request) -> bool {
    if req.path().to_string_lossy().contains("../") {
        req.stream().write(b"HTTP/1.1 403 Forbidden\r\n\r\n").await;
        return true;
    }
    return false;
}
async fn handle_404(req: request::Request) -> bool {
    if !req.path().exists().await {
        req.stream().write(b"HTTP/1.1 404 Not Found\r\n\r\n").await;
        return true;
    }
    return false;
}
async fn handle_200(req: request::Request) -> bool {
    req.stream().write(b"HTTP/1.1 200 OK\r\n\r\n").await; //?
    let r = open_file::open_file(req.path()).await;
    let mut w = async_std::io::BufWriter::new(req.stream());
    async_std::io::copy(&mut (r.unwrap()), &mut w).await; //?
    w.flush().await; //?
    return true;
}
pub async fn handle_connection(stream: async_std::net::TcpStream) -> std::io::Result<()> {

    let req = request::Request::new(stream).await;

    if handle_403(req.clone()).await {
        return Ok(());
    }
    if handle_404(req.clone()).await {
        return Ok(());
    }
    if handle_200(req.clone()).await {
        return Ok(());
    }
    Ok(())
}