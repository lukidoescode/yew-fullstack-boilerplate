use crate::config::server::ServerConfig;
use actix_web::{http::StatusCode, web, Error, HttpRequest, HttpResponse};
use mime_guess;
use std::fs;
use std::path::Path;

pub async fn serve_static(
    req: HttpRequest,
    _: web::Bytes,
    config: web::Data<ServerConfig>,
) -> Result<HttpResponse, Error> {
    let mut req_path = req.uri().path();
    if req_path == "" || req_path.ends_with('/') {
        req_path = "/index.html";
    }

    // TODO: add etag capabilities

    let mut loc = format!("{}{}", config.http_serve_static, req_path);

    let path_loc = loc.clone();
    let path = Path::new(path_loc.as_str());

    // default to index.html
    if !path.is_file() {
        loc = format!("{}/index.html", config.http_serve_static);
    }

    match fs::read(loc.clone()) {
        Ok(content) => Ok(HttpResponse::build(StatusCode::OK)
            .content_type(
                mime_guess::from_path(loc)
                    .first_or_octet_stream()
                    .essence_str(),
            )
            .body(content)),
        Err(error) => Err(Error::from(error)),
    }
}
