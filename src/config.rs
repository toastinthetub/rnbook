use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub entries_file: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        // e.g. ~/.config/rnbook/entries.json on Unix-likes
        // or %AppData%\\rnbook\\entries.json on Windows
        let mut default_path = dirs::config_dir().expect("Couldn't find system config directory");
        default_path.push("rnbook");
        default_path.push("entries.json");

        Self {
            entries_file: default_path,
        }
    }
}

impl Config {
    /// Loads the config. If it doesn't exist, this will create
    /// a default config file and return that.
    pub fn load() -> Result<Self, std::io::Error> {
        let mut config_path = dirs::config_dir()
            .ok_or_else(|| Error::new(ErrorKind::NotFound, "Couldn't find config directory"))?;
        config_path.push("rnbook");
        config_path.push("config.json");

        if config_path.exists() {
            let contents = fs::read_to_string(&config_path)?;
            let cfg: Config = serde_json::from_str(&contents)?;
            Ok(cfg)
        } else {
            // Config file not found: create a new one with defaults
            let cfg = Config::default();
            cfg.save()?;
            Ok(cfg)
        }
    }

    /// Saves the current config to the config directory.
    pub fn save(&self) -> Result<(), std::io::Error> {
        let mut config_dir = dirs::config_dir()
            .ok_or_else(|| Error::new(ErrorKind::NotFound, "Couldn't find config directory"))?;
        config_dir.push("rnbook");
        fs::create_dir_all(&config_dir)?;

        let mut config_path = config_dir.clone();
        config_path.push("config.json");

        let serialized = serde_json::to_string_pretty(self)?;
        fs::write(config_path, serialized)?;
        Ok(())
    }
}
