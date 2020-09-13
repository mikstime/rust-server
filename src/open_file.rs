use std::path::{Path, PathBuf};
use async_std::fs::File;//std::fs::File;
//@TODO refactor
// If no file name mentioned serve index.html
const BASE_DIR : &str = "./public/httptest/";

pub async fn open_file(path: &Path) -> std::io::Result<File> {

    let file_name = match path.file_name() {
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
    File::open(path_combined.to_str().unwrap()).await
}