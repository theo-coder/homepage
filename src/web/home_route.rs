use askama::Template;
use axum::{extract::State, response::IntoResponse};

use crate::state::AppState;

pub async fn index(State(state): State<AppState>) -> impl IntoResponse {
    let bg_opacity = state.config.wallpaper.opacity.unwrap_or(0.8);
    let bg_blur = state.config.wallpaper.blur.unwrap_or(true);

    HomeTemplate {
        bg_opacity,
        bg_blur,
    }
}

#[derive(Template)]
#[template(path = "index.html")]
struct HomeTemplate {
    bg_opacity: f32,
    bg_blur: bool,
}
