use std::{fs, path::Path};

use directories::ProjectDirs;
use log::info;
use serde::Deserialize;

use crate::error::Result;

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

impl Config {
    pub fn from_file<P>(file_path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        // so we can print the absolute path
        let config_data = fs::read_to_string(&file_path)?;

        info!("config file: {}", file_path.as_ref().display());

        let config: Config = toml::from_str(&config_data)?;
        Ok(config)
    }

    pub fn from_default(dirs: &ProjectDirs) -> Result<Self> {
        let config_file = dirs.config_dir().join(CONFIG_FILE_NAME);
        Config::from_file(config_file)
    }
}
