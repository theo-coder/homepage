use std::sync::Arc;

use tokio::sync::Mutex;

use self::cache::Cache;

mod cache;

#[derive(Clone, Debug)]
pub struct AppState {
    pub cache: Arc<Mutex<Cache>>,
    pub http_client: reqwest::Client,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            cache: Arc::new(Mutex::new(Cache::default())),
            http_client: reqwest::Client::new(),
        }
    }
}
