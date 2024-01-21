use std::{fs::File, io::Write, path::Path};

use serde::Deserialize;

use crate::error::AppResult;

use self::wallpaper::WallpaperConfig;

mod wallpaper;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub app_port: u16,
    pub wallpaper: WallpaperConfig,
}

impl AppConfig {
    pub fn read() -> AppResult<Self> {
        let home_directory = std::env::var("HOME")?;
        let config_path = Path::new(&home_directory).join(".config/homepage");

        std::fs::create_dir_all(&config_path)?;

        let config_file = config_path.join("config.toml");

        if !config_file.exists() {
            let mut file =
                File::create(&config_file).expect("Failed to create default config file");

            let config_template = include_bytes!("../../settings/base.toml");

            file.write_all(config_template)
                .expect("Failed to write to default config file");
        }

        let config_dir = project_root::get_project_root()
            .or_else(|_| std::env::current_dir())?
            .join("settings");

        let config = config::Config::builder()
            .add_source(config::File::from(config_dir.join("base.toml")))
            .add_source(config::File::from(config_file))
            .build()?;

        wallpaper::validate(&config)?;

        Ok(config.try_deserialize()?)
    }
}
