use todo_actix::Index;

use actix_web::{get, post, web, App, Error, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::resource("/")
                    .route(web::get().to(Index::get_notes))
                    .route(web::post().to(Index::save_note)),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
