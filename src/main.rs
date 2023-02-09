mod api;
mod error;

use axum::extract::{Query as AxumQuery, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{response::Html, routing::post, Router};
use civilization::init_service;
use redis::aio::Connection;
use redis::{AsyncCommands, Client};
use redis::{ErrorKind, RedisError};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

type Query<KEY, VAL> = AxumQuery<HashMap<KEY, VAL>>;

#[derive(Clone)]
struct AppState {
    redis: Client,
}

enum BeijingError {
    RedisError(RedisError),
}

impl IntoResponse for BeijingError {
    fn into_response(self) -> Response {
        match self {
            BeijingError::RedisError(e) => {
                tokio::spawn(async move { println!("{e}") });
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}

impl From<RedisError> for BeijingError {
    fn from(value: RedisError) -> Self {
        BeijingError::RedisError(value)
    }
}

#[tokio::main]
async fn main() {
    init_service();
    let redis =
        Client::open("redis+unix:///run/redis/redis.sock").expect("Connection to redis sock");

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let app_state = Arc::new(AppState { redis });

    let app = Router::new()
        .route("/", post(get_user))
        .with_state(app_state);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_user(app_state: State<Arc<AppState>>) -> Result<&'static str, BeijingError> {
    let redis_con = app_state.redis.get_async_connection().await?;

    Ok("sf")
}

async fn authorize(
    client_id: Query<String, String>,
    redirect_uri: Query<String, String>,
    response_type: Query<String, String>,
    code_challenge: Query<String, String>,
    state: Query<String, String>,
    nonce: Query<String, String>
) {

}
