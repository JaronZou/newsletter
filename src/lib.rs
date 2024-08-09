use actix_web::{dev::Server, web, App, HttpResponse, HttpServer};

/// Create an asynchronous server bind to the given `TcpListener`.
pub fn run(listener: std::net::TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().route("/subscriptions", web::post().to(subscribe)))
        .listen(listener)?
        .run();

    Ok(server)
}

#[allow(dead_code)]
#[derive(serde::Deserialize)]
struct FormData {
    name: String,
    email: String,
}

// The parameter we expected is a FormData, the `actix_web` will deserialize
// it by invoking the functions of the `Deserialize` trait.
// 
// Once the `Deserialize` is passed, the `actix_web` invokes the route's 
// handler. Otherwise, it responds with 400.
async fn subscribe(_from: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
