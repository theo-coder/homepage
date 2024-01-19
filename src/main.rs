use axum::{routing::get, Router};
use config::AppConfig;
use error::AppResult;
use state::AppState;
use tower_http::services::{ServeDir, ServeFile};
use web::wallpaper_route;

mod config;
mod error;
mod state;
mod web;

// TODO:
// - use a div for wallpaper to update opacity
// - use a html templating engine of some kind

#[tokio::main]
async fn main() -> AppResult<()> {
    dotenv::dotenv().ok();

    let config = AppConfig::read()?;
    let state = AppState::new(config.clone())?;

    let routes = Router::new()
        .route("/wallpaper", get(wallpaper_route::index))
        .nest_service("/assets", ServeDir::new("assets"))
        .nest_service("/", ServeFile::new("index.html"))
        .with_state(state.clone());

    let app_port = config.app_port;

    let listener = tokio::net::TcpListener::bind(format!("0:{}", app_port)).await?;
    axum::serve(listener, routes.into_make_service()).await?;

    Ok(())
}
