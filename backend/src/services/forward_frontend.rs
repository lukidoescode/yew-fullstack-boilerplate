use actix_web::{client::Client, http::StatusCode, web, Error, HttpRequest, HttpResponse};
use std::{env, time::Duration};
use url::Url;

const X_FORWARDED_FOR: &str = "x-forwarded-for";
const FORWARD_URL: &str = "http://localhost:8080";

pub async fn forward(
    req: HttpRequest,
    body: web::Bytes,
    client: web::Data<Client>,
) -> Result<HttpResponse, Error> {
    let query = match req.uri().query() {
        Some(query) => format!("?{}", query),
        None => String::new(),
    };
    // TODO: this block can also be handled outside of this function
    let forward_url_str =
        env::var("YEW_FULLSTACK_FORWARD_FRONTEND_URL").unwrap_or(String::from(FORWARD_URL));
    let forward_url = match Url::parse(forward_url_str.as_str()) {
        Ok(url) => url,
        Err(err) => {
            warn!("An error occured while parsing the forward URI: {}", err);
            return Ok(HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                .body("Internal server error"));
        }
    };

    debug!("forward_url is: {}", forward_url_str);
    let forward_uri = format!("{}{}{}", forward_url_str, req.uri().path(), query);

    let mut forward_req = client
        .request_from(forward_uri.clone(), req.head())
        .timeout(Duration::new(120, 0)) // 120 seconds
        .no_decompress();
    forward_req.headers_mut().remove("host");
    let forward_req = forward_req.header(
        "Host",
        String::from(forward_url.host_str().unwrap_or("localhost")),
    );

    let forward_req = if let Some(addr) = req.head().peer_addr {
        forward_req.header(X_FORWARDED_FOR, format!("{}", addr.ip()))
    } else {
        forward_req
    };

    debug!("forwarded request {:?}", forward_req);

    let mut res = forward_req
        .send_body(body.clone())
        .await
        .map_err(Error::from)?;

    // default to
    if res.status() == StatusCode::NOT_FOUND {
        info!("Alternatively requesting index.html because of 404");
        let forward_uri = format!("{}{}{}", forward_url_str, "/index.html", query);

        let mut forward_req = client
            .request_from(forward_uri.clone(), req.head())
            .timeout(Duration::new(120, 0)) // 120 seconds
            .no_decompress();
        forward_req.headers_mut().remove("host");
        let forward_req = forward_req.header(
            "Host",
            String::from(forward_url.host_str().unwrap_or("localhost")),
        );

        let forward_req = if let Some(addr) = req.head().peer_addr {
            forward_req.header(X_FORWARDED_FOR, format!("{}", addr.ip()))
        } else {
            forward_req
        };

        res = forward_req.send_body(body).await.map_err(Error::from)?;
    }

    trace!(
        "forwarding response from {:?}: {:?}",
        forward_uri.clone(),
        res
    );

    let mut client_resp = HttpResponse::build(res.status());
    // Remove `Connection` as per
    // https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Connection#Directives
    for (header_name, header_value) in res.headers().iter().filter(|(h, _)| *h != "connection") {
        client_resp.header(header_name.clone(), header_value.clone());
    }

    trace!("client response builder is {:?}", client_resp);

    let body = res.body().limit(104_857_600).await?;

    Ok(client_resp.body(body))
}
