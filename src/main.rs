use axum::{routing::get, Router};
use state::AppState;
use tower_http::services::{ServeDir, ServeFile};
use web::wallpaper_route;

mod state;
mod web;

// TODO:
// - use a div for wallpaper to update opacity
// - use a html templating engine of some kind
// - handle errors
// - config for collection id

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let state = AppState::new();

    let routes = Router::new()
        .route("/wallpaper", get(wallpaper_route::index))
        .nest_service("/assets", ServeDir::new("assets"))
        .nest_service("/", ServeFile::new("index.html"))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0:3000").await.unwrap();
    axum::serve(listener, routes.into_make_service())
        .await
        .unwrap();
}
