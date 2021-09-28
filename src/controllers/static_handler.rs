use actix_web::{get, post, web::{self, Path}, App, Error, HttpResponse, HttpServer, Responder};
use std::{fs, path, env};

#[get("/css/{filename}")]
pub async fn serve_css(filename: Path<String>) -> HttpResponse {
    let filename = filename.to_string();
    let filepath = path::Path::new(&filename);
    if filepath.exists() {
        let content = fs::read_to_string(filepath).unwrap();
        HttpResponse::Ok()
            .content_type("text/css")
            .body(content)
    } else {
        HttpResponse::NotFound().finish()
    }
}
