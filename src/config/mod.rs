mod types;

use serde::Deserialize;
use std::fs;
pub use types::*;

use crate::config::types::Config;

use anyhow::Error;

fn load_config() -> Result<Config, Error> {
    let config_content = fs::read_to_string("config.toml");
    let config: Config = toml::from_str(&config_content)?;
    Ok(config)
}

