use rocket::fs::relative;
use rocket::fs::NamedFile;
use std::path::{Path, PathBuf};

pub async fn project1() -> Option<NamedFile> {
    let path = Path::new(relative!("static/project1/project1.html"));

    NamedFile::open(path).await.ok()
}
