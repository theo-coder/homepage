use config::{Config, ConfigError};
use serde::Deserialize;

use crate::error::{AppError, AppResult};

#[derive(Debug, Deserialize, Clone)]
pub struct WallpaperConfig {
    pub collection_id: Option<u32>,
    pub opacity: Option<f32>,
    pub blur: Option<bool>,
}

pub fn validate(config: &Config) -> AppResult<()> {
    if let Ok(Some(opacity)) = config.get::<Option<f32>>("wallpaper.opacity") {
        if !(0. ..=1.).contains(&opacity) {
            return Err(AppError::Config(ConfigError::Message(
                "wallpaper opacity must be between 0 and 1".to_string(),
            )));
        }
    }

    Ok(())
}
