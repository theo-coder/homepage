use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{config::AppConfig, error::AppResult};

use self::cache::Cache;

mod cache;

#[derive(Clone, Debug)]
pub struct AppState {
    pub cache: Arc<Mutex<Cache>>,
    pub http_client: reqwest::Client,
    pub config: AppConfig,
}

impl AppState {
    pub fn new(config: AppConfig) -> AppResult<Self> {
        Ok(Self {
            cache: Arc::new(Mutex::new(Cache::default())),
            http_client: reqwest::Client::new(),
            config,
        })
    }
}
