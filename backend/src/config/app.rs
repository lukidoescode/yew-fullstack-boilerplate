use crate::api::account_controller;
#[cfg(feature = "forward-frontend")]
use crate::services::forward_frontend::forward;
#[cfg(not(feature = "forward-frontend"))]
use crate::services::serve_frontend::serve_static;
use actix_web::web;

pub fn config_services(cfg: &mut web::ServiceConfig) {
    info!("Configurating routes...");
    cfg.service(
        web::scope("/api").service(
            web::scope("/auth")
                .service(web::resource("/signup").route(web::post().to(account_controller::signup)))
                .service(web::resource("/login").route(web::post().to(account_controller::login)))
                .service(
                    web::resource("/logout").route(web::post().to(account_controller::logout)),
                ),
        ),
    );
    config_frontend(cfg);
}

#[cfg(feature = "forward-frontend")]
fn config_frontend(cfg: &mut web::ServiceConfig) {
    info!("Configurating frontend reverse proxy...");
    cfg.service(web::scope("/").default_service(web::route().to(forward)));
}

#[cfg(not(feature = "forward-frontend"))]
fn config_frontend(cfg: &mut web::ServiceConfig) {
    info!("Configurating static frontend...");
    cfg.service(web::scope("/").default_service(web::route().to(serve_static)));
}
