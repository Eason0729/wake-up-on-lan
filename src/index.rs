use actix_files ;
use actix_web::{ Error, HttpRequest};

pub async fn index_handler(_req: HttpRequest) -> Result<actix_files::NamedFile, Error> {
    let path = std::path::PathBuf::from("./public/index.html");
    let file = actix_files::NamedFile::open(path)?;
    Ok(file
        .use_last_modified(true))
}
