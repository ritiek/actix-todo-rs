use actix_multipart::Multipart;
use actix_web::{get, post, web, App, Error, HttpResponse, HttpServer, Responder};
use futures::{StreamExt, TryStreamExt};
use serde::Deserialize;
use std::io::Write;

#[derive(Debug, Deserialize)]
struct FormData {
    username: String,
}

pub struct Index;

impl Index {
    // #[get("/")]
    pub async fn root() -> HttpResponse {
        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(include_str!("../views/index.html"))
    }

    pub async fn save_note(mut payload: Multipart) -> HttpResponse {
        while let Ok(Some(mut field)) = payload.try_next().await {
            let content_type = field.content_disposition().unwrap();
            let filename = content_type.get_filename().unwrap();
            let filepath = format!("./files/{}", filename);

            let mut output_file = web::block(|| std::fs::File::create(filepath))
                .await
                .unwrap();

            while let Ok(Some(chunk)) = field.try_next().await {
                output_file =
                    web::block(move || output_file.write_all(&chunk).map(|_| output_file))
                        .await
                        .unwrap();
            }
        }
        HttpResponse::Ok()
            .content_type("text/html")
            .body("yay your upload is done")
    }
}
