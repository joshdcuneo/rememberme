mod app;
mod db;
mod entry;
mod error;

use actix_web::middleware::{Logger, self, TrailingSlash};
use actix_web::{App, HttpServer};
use dotenvy::dotenv;
use env_logger::Env;
use paperclip::actix::{web, OpenApiExt};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().expect(".env file not found");
    env_logger::init_from_env(Env::default().default_filter_or("debug"));
    let pool = db::init().await.expect("Failed to create pool");

    HttpServer::new(move || app!(pool))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
