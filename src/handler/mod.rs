mod request;
mod response;
mod open_file;
mod mime;

use mime::extract_mime;

async fn get_403(req: request::Request, mut res: response::Response) -> bool {
    if req.path().to_string_lossy().contains("../") {
        res.set_status(403);
        res.send().await;
        return true;
    }
    if req.path().to_string_lossy().ends_with("index.html") & !(req.path().exists().await) {
        res.set_status(403);
        res.send().await;
        return true;
    }
    return false;
}

async fn get_404(req: request::Request, mut res: response::Response) -> bool {
    if !req.path().exists().await {
        res.set_status(404);
        res.send().await;
        return true;
    }
    return false;
}

async fn get_200(req: request::Request, mut res: response::Response) -> bool {
    res.set_content_type(extract_mime(req.path().extension().unwrap().to_str().unwrap()).to_string());
    res.set_content_length(async_std::fs::metadata(req.path()).await.unwrap().len().to_string());
    let r = open_file::open_file(req.path()).await.unwrap();
    res.send_content(r).await;
    return true;
}

async fn head_200(req: request::Request, mut res: response::Response) -> bool {
    res.set_content_type(extract_mime(req.path().extension().unwrap().to_str().unwrap()).to_string());
    res.set_content_length(async_std::fs::metadata(req.path()).await.unwrap().len().to_string());
    res.send().await;
    return true;
}

async fn handle_get(req: request::Request, res: response::Response) {
    if get_403(req.clone(), res.clone()).await {
        return;
    }
    if get_404(req.clone(), res.clone()).await {
        return;
    }
    if get_200(req.clone(), res.clone()).await {
        return;
    }
}

async fn handle_head(req: request::Request, res: response::Response) {
    if get_403(req.clone(), res.clone()).await {
        return;
    }
    if get_404(req.clone(), res.clone()).await {
        return;
    }
    if head_200(req.clone(), res.clone()).await {
        return;
    }
}

async fn handle_405(req: request::Request, mut res: response::Response) {
    res.set_status(405);
    res.send().await;
}

pub async fn handle_connection(stream: async_std::net::TcpStream) -> std::io::Result<()> {
    let req = request::Request::new(stream.clone()).await;
    let res = response::Response::new(stream.clone()).await;
    match req.method() {
        "GET" => handle_get(req.clone(), res.clone()).await,
        "HEAD" => handle_head(req.clone(), res.clone()).await,
        _ => handle_405(req.clone(), res.clone()).await,
    };
    Ok(())
}