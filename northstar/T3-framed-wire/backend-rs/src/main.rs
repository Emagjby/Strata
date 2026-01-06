use axum::{
    Router,
    body::Body,
    http::{HeaderMap, HeaderValue},
    response::IntoResponse,
    routing::get,
};
use std::collections::BTreeMap;
use std::convert::Infallible as Error;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tower_http::cors::{Any, CorsLayer};

use strata::encode::encode;
use strata::hash::hash_value;
use strata::value::Value;

const MAX_FRAME_BYTES: usize = 16 * 1024 * 1024; // 16 MB

fn write_frame(buf: &mut Vec<u8>, payload: &[u8]) {
    let len = payload.len();

    assert!(len > 0, "zero-length frame");
    assert!(len <= MAX_FRAME_BYTES, "frame too large");

    buf.extend_from_slice(&(len as u32).to_be_bytes());
    buf.extend_from_slice(payload);
}

async fn stream() -> impl IntoResponse {
    let (tx, rx) = mpsc::channel::<Result<bytes::Bytes, Error>>(8);

    tokio::spawn(async move {
        let values = vec![
            Value::Int(42),
            Value::String("hello".into()),
            Value::Map(
                [("answer".into(), Value::Int(42))]
                    .into_iter()
                    .collect::<BTreeMap<_, _>>(),
            ),
        ];

        for value in values {
            let payload = encode(&value).expect("encoding failed");
            let hash = hash_value(&value);
            eprintln!("hash={}", hex::encode(hash));

            let mut framed = Vec::new();
            write_frame(&mut framed, &payload);

            if tx.send(Ok(bytes::Bytes::from(framed))).await.is_err() {
                return;
            }
        }
    });

    let mut headers = HeaderMap::new();
    headers.insert(
        "Content-Type",
        HeaderValue::from_static("application/strata"),
    );

    (headers, Body::from_stream(ReceiverStream::new(rx)))
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new().route("/stream", get(stream)).layer(cors);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
