use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
