use axum::{routing::get,     http::StatusCode,
           Json, Router,};

mod api;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/data", get(root));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}