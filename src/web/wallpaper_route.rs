use std::time::Duration;

use axum::extract::State;
use reqwest::header::AUTHORIZATION;

use crate::{error::AppResult, state::AppState};

pub async fn index(State(state): State<AppState>) -> AppResult<String> {
    let mut cache = state.cache.lock().await;

    if let Some((wallpaper, timestamp)) = cache.get() {
        println!("cache hit");
        let elapsed = timestamp
            .elapsed()
            .unwrap_or_else(|_| Duration::from_secs(0));

        if elapsed < Duration::from_secs(3600) {
            return Ok(wallpaper.to_string());
        }
    } else {
        println!("new wallpaper requested");
    }

    let collection_id = state.config.wallpaper.collection_id.unwrap_or(1053828);

    let new_wallpaper = &state
        .http_client
        .get(format!(
            "https://api.unsplash.com/photos/random?collections={}",
            collection_id
        ))
        .header(
            AUTHORIZATION,
            format!(
                "Client-ID {}",
                std::env::var("UNSPLASH_API_KEY").expect("unable to read env")
            ),
        )
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?["urls"]["full"];

    cache.update(new_wallpaper.to_string().clone());

    Ok(new_wallpaper.to_string())
}
