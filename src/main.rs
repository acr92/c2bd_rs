use std::net::TcpListener;
use webserver::run;

const DEFAULT_HOST: &str = "127.0.0.1";
const DEFAULT_PORT: u16 = 8080;
const DEFAULT_TARGET_DATE: &str = "2023-05-31T17:00:00+02:00";

#[actix_web::main]
async fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let bind_address = std::env::var("BIND_ADDRESS").unwrap_or_else(|_| DEFAULT_HOST.to_string());
    let bind_port =
        std::env::var("PORT").map_or(DEFAULT_PORT, |a| a.parse::<u16>().unwrap_or(DEFAULT_PORT));
    let target_date = std::env::var("TARGET_DATE")
        .unwrap_or_else(|_| DEFAULT_TARGET_DATE.to_string())
        .parse::<chrono::DateTime<chrono::FixedOffset>>()
        .unwrap();

    let tcp = TcpListener::bind((bind_address, bind_port)).unwrap();
    log::info!("Listening on {}", tcp.local_addr().unwrap());

    run(tcp, target_date).await.unwrap().await.unwrap();
}
