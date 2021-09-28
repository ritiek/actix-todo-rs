use todo_actix::Index;
use todo_actix::static_handler;

use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::resource("/")
                .route(web::get().to(Index::root))
                .route(web::post().to(Index::save_note)),
        )
        .service(static_handler::serve_css)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
