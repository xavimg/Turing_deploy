use std::lazy::Lazy;
use actix_web::{HttpRequest, http::header::AUTHORIZATION};
use jsonwebtoken::{DecodingKey, decode, Validation, TokenData};
use crate::PlayerTokenLoged;

pub mod route;
pub mod game;

const JWT_SECRET : Lazy<&str> = Lazy::new(|| get_env!("JWT_SECRET"));
const JWT_KEY : Lazy<DecodingKey> = Lazy::new(|| DecodingKey::from_secret(JWT_SECRET.as_ref()));

pub fn get_auth_token (req: &HttpRequest) -> Option<&str> {
    if let Some(auth) = req.headers().get(AUTHORIZATION) {
        if let Ok(auth) = auth.to_str() {
            if auth.starts_with("Bearer") {
                return Some(&auth[7..])
            }
        }
    }

    None
}
pub fn decode_token (req: &HttpRequest) -> Option<Result<TokenData<PlayerTokenLoged>, jsonwebtoken::errors::Error>> {
    let token = get_auth_token(req)?;
    Some(decode::<PlayerTokenLoged>(token, &JWT_KEY, &Validation::default()))
}