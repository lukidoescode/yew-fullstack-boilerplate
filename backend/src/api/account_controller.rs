use crate::{
    models::{
        response::ResponseBody,
        user::{LoginDTO, User},
    },
    services::account_service,
};
use actix_web::{web, HttpRequest, HttpResponse, Result};
use mongodb::Database;

// POST api/auth/signup
pub async fn signup(user_dto: web::Json<User>, db: web::Data<Database>) -> Result<HttpResponse> {
    match account_service::signup(user_dto.0, &db) {
        Ok(data) => Ok(HttpResponse::Ok().json(ResponseBody::new("SUCCESS", &data))),
        Err(err) => Ok(err.response()),
    }
}

// POST api/auth/login
pub async fn login(
    login_dto: web::Json<LoginDTO>,
    db: web::Data<Database>,
) -> Result<HttpResponse> {
    match account_service::login(&login_dto.0, &db) {
        Ok(token_res) => Ok(HttpResponse::Ok().json(ResponseBody::new("LOGIN_SUCCESS", token_res))),
        Err(err) => Ok(err.response()),
    }
}

// POST api/auth/logout
pub async fn logout(req: HttpRequest, db: web::Data<Database>) -> Result<HttpResponse> {
    if let Some(authen_header) = req.headers().get("Authorization") {
        match account_service::logout(authen_header, &db) {
            Ok(_) => Ok(HttpResponse::Ok().json(ResponseBody::new("LOGOUT_SUCCESS", ""))),
            Err(err) => Ok(err.response()),
        }
    } else {
        Ok(HttpResponse::BadRequest().json(ResponseBody::new("MISSING_TOKEN", "")))
    }
}
