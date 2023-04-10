use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use std::net::TcpListener;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

pub async fn run(tcp: TcpListener) -> Result<Server, actix_web::Error> {
    let server = HttpServer::new(move || App::new().wrap(Logger::default()).service(hello))
        .listen(tcp)?
        .run();
    Ok(server)
}
