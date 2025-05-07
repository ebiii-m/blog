use actix_web::{web, App, Error, HttpResponse, HttpServer, Responder, Result};
use serde::Deserialize;
use serde_json::json;
use configs::{blog_config, config_server};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(blog_config::config)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}