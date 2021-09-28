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
    pub async fn save_note(mut payload: Multipart) -> HttpResponse {
        while let Ok(Some(mut field)) = payload.try_next().await {
            let content_type = field.content_disposition().unwrap();
            let filename = content_type.get_filename().unwrap();
            let filepath = format!("./files/{}", filename);

            let mut output_file = web::block(|| std::fs::File::create(filepath))
                .await
                .unwrap();

            while let Ok(Some(chunk)) = field.try_next().await {
                output_file = web::block(move || output_file.write_all(&chunk).map(|_| output_file))
                    .await
                    .unwrap();
            }
        }
        HttpResponse::Ok()
            .content_type("text/html")
            .body("yay your upload is done")
    }

    // #[get("/")]
    pub async fn get_notes() -> HttpResponse {
        let response = r#"
            <html>
              <body>
                <form target="/" method="post" enctype="multipart/form-data">
                    <!-- <input type="file" method="post" name="uploaded-file" enctype="multipart/form"/> -->
                    <!-- <input type="text" name="username"/> -->
                    <input type="file" name="uploaded-file"/>
                    <button type="submit">Submit</button>
                </form>
              </body>
            </html>"#;
        HttpResponse::Ok().content_type("text/html").body(response)
    }
}
