use axum::{
    Router,
    body::Body,
    http::{Response, StatusCode, header},
    routing::get,
};
use std::collections::BTreeMap;
use tower_http::cors::{Any, CorsLayer};

use strata::encode::encode;
use strata::value::Value;

async fn payload() -> Response<Body> {
    let value = Value::Map(
        [("answer".to_string(), Value::Int(42))]
            .into_iter()
            .collect::<BTreeMap<_, _>>(),
    );

    let encoded = match encode(&value) {
        Ok(bytes) => bytes,
        Err(_) => {
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("Encoding error"))
                .unwrap();
        }
    };

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/strata")
        .body(Body::from(encoded))
        .unwrap()
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
