mod business_days;
mod health;

use crate::business_days::calculate_business_days;
use crate::health::health_check;
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use chrono::{DateTime, FixedOffset};
use std::net::TcpListener;

pub async fn run(
    tcp: TcpListener,
    target_date: DateTime<FixedOffset>,
) -> Result<Server, actix_web::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(calculate_business_days)
            .service(health_check)
            .app_data(Data::new(target_date))
    })
    .listen(tcp)?
    .run();
    Ok(server)
}
