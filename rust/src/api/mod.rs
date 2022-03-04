use std::{lazy::Lazy, fmt::Display};
use actix_web::{HttpRequest, http::header::AUTHORIZATION, ResponseError};
use chrono::{DateTime, Utc};
use jsonwebtoken::{DecodingKey, decode, Validation, TokenData, encode, EncodingKey, Header};
use crate::PlayerTokenLoged;

pub mod route;
pub mod game;
pub mod ws;

const JWT_SECRET : Lazy<String> = Lazy::new(|| get_env!("JWT_SECRET"));
const JWT_KEY : Lazy<DecodingKey> = Lazy::new(|| DecodingKey::from_secret(JWT_SECRET.as_ref()));

pub(super) fn test_token (id: u64) -> (PlayerTokenLoged, String) {
    let body = PlayerTokenLoged::default_for(id);
    let token = encode(&Header::default(), &body, &EncodingKey::from_secret(JWT_SECRET.as_ref())).unwrap();
    (body, token)
} 

#[inline]
pub fn is_loopback (req: &HttpRequest) -> bool {
    if let Some(ip) = req.peer_addr() {
        return ip.ip().is_loopback()
    }

    true
}

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

pub fn decode_token (req: &HttpRequest) -> Result<(&str, TokenData<PlayerTokenLoged>), TokenError> {
    if let Some(string) = get_auth_token(req) {
        let token = decode::<PlayerTokenLoged>(&string, &JWT_KEY, &Validation::default()).map_err(|e| TokenError::JWT(e))?;
        let now = Utc::now();

        if token.claims.issued_at >= token.claims.expiration_date {
            return Err(TokenError::TokenExpiredBeforeIssued { issued: token.claims.issued_at, expired: token.claims.expiration_date })
        } else if token.claims.issued_at > now {
            return Err(TokenError::TokenInFuture(token.claims.issued_at))
        } else if token.claims.expiration_date <= now {
            return Err(TokenError::TokenExpired(token.claims.expiration_date))
        }

        return Ok((string, token))
    }

    Err(TokenError::TokenNotFound)
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenError {
    JWT(jsonwebtoken::errors::Error),
    TokenExpiredBeforeIssued { issued: DateTime<Utc>, expired: DateTime<Utc> },
    TokenInFuture(DateTime<Utc>),
    TokenExpired(DateTime<Utc>),
    TokenNotFound
}

impl std::error::Error for TokenError {}
impl ResponseError for TokenError {}

impl Display for TokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::JWT(x) => x.fmt(f),
            Self::TokenExpiredBeforeIssued { issued, expired } => f.write_fmt(format_args!("Token issued in {issued} and expired in {expired}")),
            Self::TokenInFuture(date) => f.write_fmt(format_args!("Token issued in the future ({date})")),
            Self::TokenExpired(date) => f.write_fmt(format_args!("Token expired in {date}")),
            Self::TokenNotFound => f.write_str("Token not found")
        }
    }
}