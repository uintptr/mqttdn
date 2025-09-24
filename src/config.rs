use std::{
    fs,
    path::{Path, PathBuf},
};

use log::info;
use rstaples::staples::find_file;
use serde::Deserialize;

use crate::error::{Error, Result};

#[derive(Deserialize)]
pub struct MQTTServer {
    pub host: String,
}

#[derive(Debug, Deserialize)]
pub struct MQTTTopic {
    pub topic: String,
    pub payload: String,
    pub osd: Option<String>,
    pub command: Option<String>,
}

#[derive(Deserialize)]
pub struct Config {
    pub server: MQTTServer,
    pub topics: Vec<MQTTTopic>,
}

fn find_config() -> Result<PathBuf> {
    let rel_path = Path::new("config").join("config.toml");

    if let Ok(path) = find_file(rel_path) {
        return Ok(path);
    }

    let home_dir = home::home_dir().ok_or(Error::HomeNotFound)?;

    let home_config = home_dir.join(".config").join("mqttdn").join("config.toml");

    if home_config.exists() {
        return Ok(home_config);
    }

    Err(Error::ConfigFileNotFound)
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_file = find_config()?;

        info!("config file: {}", config_file.display());
        let config_data = fs::read_to_string(config_file)?;

        let config: Config = toml::from_str(&config_data)?;

        Ok(config)
    }
}
