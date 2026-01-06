use axum::{routing::get, Json, Router};
use base64::engine::general_purpose::STANDARD as base64;
use base64::Engine;
use serde::Serialize;
use std::collections::BTreeMap;
use tower_http::cors::{Any, CorsLayer};

use strata::encode::encode;
use strata::hash::hash_value;
use strata::value::Value;

#[derive(Serialize)]
struct Payload {
    bytes_base64: String,
    hash_hex: String,
}

async fn payload() -> Json<Payload> {
    let value = Value::Map(
        [("answer".to_string(), Value::Int(42))]
            .into_iter()
            .collect::<BTreeMap<_, _>>(),
    );

    let bytes = encode(&value).unwrap();
    let hash = hash_value(&value);

    Json(Payload {
        bytes_base64: base64.encode(&bytes),
        hash_hex: hex::encode(hash),
    })
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    let app = Router::new().route("/payload", get(payload)).layer(cors);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
