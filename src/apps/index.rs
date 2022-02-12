use rocket::fs::relative;
use rocket::fs::NamedFile;
use std::path::{Path, PathBuf};

// pub fn index() -> Value {
//     let sample: Value = json!({
//         "firstname": "Hayden",
//         "lastname": "Rose"
//     });
//     sample
// }

pub async fn index() -> Option<NamedFile> {
    let path = Path::new(relative!("static/index/index.html"));

    NamedFile::open(path).await.ok()
}
