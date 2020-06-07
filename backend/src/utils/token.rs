use crate::models::{
    user::User,
    user_token::{UserToken, KEY},
};
use actix_web::web;
use jsonwebtoken::{DecodingKey, TokenData, Validation};
use mongodb::Database;

pub fn decode_token(token: String) -> jsonwebtoken::errors::Result<TokenData<UserToken>> {
    jsonwebtoken::decode::<UserToken>(
        &token,
        &DecodingKey::from_secret(&KEY),
        &Validation::default(),
    )
}

pub fn verify_token(
    token_data: &TokenData<UserToken>,
    db: &web::Data<Database>,
) -> Result<String, String> {
    if User::is_valid_login_session(&token_data.claims, &db) {
        Ok(token_data.claims.user.to_string())
    } else {
        Err("Invalid token".to_string())
    }
}
