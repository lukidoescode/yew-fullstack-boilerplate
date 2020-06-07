#[macro_use()]
extern crate actix_rt;
extern crate actix_cors;
extern crate actix_web;
#[macro_use]
extern crate log;
extern crate bson;
extern crate chrono;
extern crate env_logger;
extern crate jsonwebtoken;
#[cfg(not(feature = "forward-frontend"))]
extern crate mime;
#[cfg(not(feature = "forward-frontend"))]
extern crate mime_guess;
extern crate mongodb;
#[macro_use()]
extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate url;

pub mod api;
pub mod config;
pub mod error;
pub mod models;
pub mod services;
pub mod utils;

use actix_cors::Cors;
use actix_web::{client::Client, http, App, HttpResponse, HttpServer, Responder};
use config::{db::config_db, server::ServerConfig};
use std::{env, io};

#[allow(dead_code)]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env_logger::init();

    let serve_static = env::var("YEW_FULLSTACK_STATIC")
        .unwrap_or(env::var("PWD").unwrap_or(String::from("/usr/local/share/yew-fullstack/www")));
    let default_http_host = String::from("127.0.0.1");
    let default_http_port = String::from("3000");
    let http_host = env::var("YEW_FULLSTACK_HOST").unwrap_or(default_http_host);
    let http_port = env::var("YEW_FULLSTACK_PORT").unwrap_or(default_http_port);
    let http_host_port = format!("{}:{}", http_host, http_port);
    // let http_host_port_cors = http_host_port.clone();

    let db_name = env::var("YEW_FULLSTACK_DB_NAME").unwrap_or(String::from("yew-fullstack"));
    let db_client = config_db();

    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .wrap(
                Cors::new()
                    // .allowed_origin(http_host_port_cors.as_str())
                    .send_wildcard()
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600)
                    .finish(),
            )
            .data(Client::new())
            .data(ServerConfig {
                http_serve_static: serve_static.clone(),
            })
            .data(db_client.database(db_name.as_str()))
            // .route("/", web::get().to(index))
            .configure(config::app::config_services)
    })
    .bind(http_host_port.clone())?
    .run()
    .await
}
