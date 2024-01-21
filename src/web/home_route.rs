use askama::Template;
use axum::{extract::State, response::IntoResponse};

use crate::{constants, state::AppState};

pub async fn index(State(state): State<AppState>) -> impl IntoResponse {
    let wallpaper_config = state.config.wallpaper;

    HomeTemplate {
        bg_opacity: wallpaper_config
            .opacity
            .unwrap_or(constants::WALLPAPER_OPACITY),
        bg_blur: wallpaper_config.blur.unwrap_or(constants::WALLPAPER_BLUR),
    }
}

#[derive(Template)]
#[template(path = "index.html")]
struct HomeTemplate {
    bg_opacity: f32,
    bg_blur: bool,
}
