use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct WallpaperConfig {
    pub collection_id: Option<u32>,
    pub opacity: Option<f32>,
    pub blur: Option<bool>,
}
