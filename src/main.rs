use crate::routes::create_routes::create_routers;

mod api;
mod services;
mod errors;
mod routes;

#[tokio::main]
async fn main() {
    let routers = create_routers();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, routers).await.unwrap();
}

