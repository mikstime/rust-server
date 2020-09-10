pub fn extract_mime(ext:&str) -> &str {
    match ext {
        "html" => "text/html",
        "css" => "text/css",
        "js" => "application/javascript",
        "jpg" => "image/jpeg",
        "jpeg" => "image/jpeg",
        "png" => "image/png",
        "gif" => "image/gif",
        "swf" => "application/x-shockwave-flash",
        _ => "application/octet-stream",
    }
}