use std::{
    fs,
    path::{Path, PathBuf},
};

use log::info;
use rstaples::staples::find_file;
use serde::Deserialize;

use crate::error::{Error, Result};

const CONFIG_FILE_NAME: &str = "mqttdn.toml";

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
    //
    // SxS
    //
    if let Ok(path) = find_file(CONFIG_FILE_NAME) {
        return Ok(path);
    }

    //
    // config/config.toml
    //
    let rel_path = Path::new("config").join(CONFIG_FILE_NAME);

    if let Ok(path) = find_file(rel_path) {
        return Ok(path);
    }

    Err(Error::ConfigFileNotFound)
}

impl Config {
    pub fn from_file<P>(file_path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        // so we can print the absolute path
        let file_path = file_path.as_ref().canonicalize()?;

        info!("config file: {}", file_path.display());

        let config_data = fs::read_to_string(file_path)?;

        let config: Config = toml::from_str(&config_data)?;

        Ok(config)
    }

    pub fn from_default() -> Result<Self> {
        let config_file = find_config()?;

        Config::from_file(config_file)
    }
}
