mod api;
mod error;

#[macro_use]
extern crate serde_json;

use axum::response::{IntoResponse, Response};
use axum::{routing::post, Json, Router};
use civilization::init_service;
use jsonwebtoken::{DecodingKey, EncodingKey};
use once_cell::sync::Lazy;
use serde::Serialize;
use std::net::SocketAddr;
use std::str::FromStr;

use error::AuthError;

static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET").expect("Set JWT_SECRET env var");
    Keys::new(secret.as_bytes())
});

struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

#[derive(Serialize)]
struct AuthResponse {
    token_type: String,
    access_token: String,
}

impl AuthResponse {
    fn new(access_token: String) -> Self {
        AuthResponse {
            token_type: "Bearer".into(),
            access_token,
        }
    }
}

async fn token() -> Result<Json<AuthResponse>, AuthError> {
    Ok(Json(AuthResponse::new("Shish".into())))
}

#[tokio::main]
async fn main() {
    init_service();

    let app = Router::new().route("/token", post(token));

    let addr = SocketAddr::from_str("127.0.0.1:8081").unwrap();

    tracing::debug!("Listening on {addr:?}");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
