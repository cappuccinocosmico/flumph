//! Application configuration system

use serde::{Serialize, Deserialize};
use anyhow::Result;
use std::fs;
use tracing::{info, warn};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub server_ip: String,
    pub server_port: u16,
    pub camera_index: usize,
    pub camera_resolution_width: u32,
    pub camera_resolution_height: u32,
    pub camera_framerate: u32,
    pub encoder_bitrate_kbps: u32,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            server_ip: "127.0.0.1".to_string(),
            server_port: 8080,
            camera_index: 0,
            camera_resolution_width: 640,
            camera_resolution_height: 480,
            camera_framerate: 30,
            encoder_bitrate_kbps: 500,
        }
    }
}

impl AppConfig {
    pub fn load(path: &str) -> Result<Self> {
        info!("Attempting to load configuration from {}", path);
        if fs::metadata(path).is_ok() {
            let config_str = fs::read_to_string(path)?;
            let config: AppConfig = serde_json::from_str(&config_str)?;
            info!("Configuration loaded successfully.");
            Ok(config)
        } else {
            warn!("Configuration file not found at {}. Using default configuration.", path);
            let default_config = AppConfig::default();
            default_config.save(path)?;
            Ok(default_config)
        }
    }

    pub fn save(&self, path: &str) -> Result<()> {
        info!("Saving configuration to {}", path);
        let config_str = serde_json::to_string_pretty(self)?;
        fs::write(path, config_str)?;
        info!("Configuration saved successfully.");
        Ok(())
    }
}
