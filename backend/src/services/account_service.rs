use crate::{
    error::ServiceError,
    models::user::{LoginDTO, PublicUserDTO, User},
    models::user_token::UserToken,
    utils::token as token_utils,
};
use actix_web::{
    http::{header::HeaderValue, StatusCode},
    web,
};
use mongodb::Database;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TokenBodyResponse {
    pub token: String,
    pub token_type: String,
    pub user: PublicUserDTO,
}

pub fn signup(user: User, db: &web::Data<Database>) -> Result<String, ServiceError> {
    match User::signup(user, db) {
        Ok(id) => Ok(id.to_hex()),
        Err(err) => match err.as_str() {
            "USER_ALREADY_EXISTS" => Err(ServiceError::new(StatusCode::BAD_REQUEST, err)),
            _ => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, err)),
        },
    }
}

pub fn login(
    login: &LoginDTO,
    db: &web::Data<Database>,
) -> Result<TokenBodyResponse, ServiceError> {
    let logged_user = User::login(&login, db)?;
    let public_user: PublicUserDTO = logged_user.clone().into();
    match serde_json::from_value(
        json!({ "token": UserToken::generate_token(logged_user), "token_type": "bearer", "user": public_user }),
    ) {
        Ok(token_res) => Ok(token_res),
        Err(_) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "INTERNAL_SERVER_ERROR".to_string(),
        )),
    }
}

pub fn logout(authen_header: &HeaderValue, db: &web::Data<Database>) -> Result<(), ServiceError> {
    if let Ok(authen_str) = authen_header.to_str() {
        if authen_str.starts_with("bearer") {
            let token = authen_str[6..authen_str.len()].trim();
            if let Ok(token_data) = token_utils::decode_token(token.to_string()) {
                if let Ok(username) = token_utils::verify_token(&token_data, &db) {
                    if let Some(user) = User::find_by_email_or_username(&username, &db) {
                        User::logout(user.id.unwrap(), db);
                        return Ok(());
                    }
                }
            }
        }
    }

    Err(ServiceError::new(
        StatusCode::INTERNAL_SERVER_ERROR,
        "TOKEN_PROCESSING_ERROR".to_string(),
    ))
}
