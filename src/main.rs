use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};

use tokio::sync::Mutex;

use axum::{extract::State, routing::get, Router};
use reqwest::header::AUTHORIZATION;
use tower_http::services::{ServeDir, ServeFile};

// TODO:
// - split to multiple files
// - use a div for wallpaper to update opacity
// - use a html templating engine of some kind
// - handle errors
// - config for collection id

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let app_state = AppState {
        cache: Arc::new(Mutex::new(Cache::default())),
        http_client: reqwest::Client::new(),
    };

    let app = Router::new()
        .route("/wallpaper", get(wallpaper_route))
        .nest_service("/assets", ServeDir::new("assets"))
        .nest_service("/", ServeFile::new("index.html"))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn wallpaper_route(State(state): State<AppState>) -> String {
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

#[derive(Clone, Debug)]
struct AppState {
    cache: Arc<Mutex<Cache>>,
    http_client: reqwest::Client,
}

#[derive(Default, Debug, Clone)]
struct Cache {
    image: Option<String>,
    timestamp: Option<SystemTime>,
}

impl Cache {
    fn get(&self) -> Option<(&String, &SystemTime)> {
        self.image.as_ref().zip(self.timestamp.as_ref())
    }

    fn update(&mut self, image: String) {
        self.image = Some(image);
        self.timestamp = Some(SystemTime::now());
    }
}
