use async_std::path::Path;
use async_std::fs::File;

pub async fn open_file(path: &Path) -> std::io::Result<File> {
    File::open(path).await
}