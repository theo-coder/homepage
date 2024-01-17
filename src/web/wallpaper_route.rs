use std::time::Duration;

use axum::extract::State;
use reqwest::header::AUTHORIZATION;

use crate::state::AppState;

pub async fn index(State(state): State<AppState>) -> String {
    let mut cache = state.cache.lock().await;

    if let Some((wallpaper, timestamp)) = cache.get() {
        println!("cache hit");
        let elapsed = timestamp
            .elapsed()
            .unwrap_or_else(|_| Duration::from_secs(0));

        if elapsed < Duration::from_secs(3600) {
            return wallpaper.to_string();
        }
    } else {
        println!("new wallpaper requested");
    }

    let new_wallpaper = &state
        .http_client
        .get("https://api.unsplash.com/photos/random?collections=1053828")
        .header(
            AUTHORIZATION,
            format!(
                "Client-ID {}",
                std::env::var("UNSPLASH_API_KEY").expect("unable to read env")
            ),
        )
        .send()
        .await
        .unwrap()
        .json::<serde_json::Value>()
        .await
        .unwrap()["urls"]["full"];

    cache.update(new_wallpaper.to_string().clone());

    new_wallpaper.to_string()
}
