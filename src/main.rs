use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get("Hello World!"));

    let listener = tokio::net::TcpListener::bind("0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
