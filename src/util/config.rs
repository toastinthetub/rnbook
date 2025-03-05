/*
 * src/util/config.rs
 *
 * This file is part of rnbook.
 *
 * rnbook is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * rnbook is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with rnbook. If not, see <https://www.gnu.org/licenses/>.
 */

use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{Error, ErrorKind},
    path::PathBuf,
};

/// on Unix-like systems the config file is stored at ~/.config/rnbook/config.json.
/// on Windows it is stored at %APPDATA%/rnbook/config.json.
/// the configuration also contains the path to the entries directory,
/// which the user may change. If that directory doesn’t exist, it will be created.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    /// the path where entry files are stored.
    /// (the master index file “rnbook_master_list.json” will be inside this directory)
    pub entries_path: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        // use dirs::config_dir() to get the os-appropriate config directory
        let base_config = dirs::config_dir().expect("couldn't find system config directory");
        let mut config_dir = base_config.clone();
        config_dir.push("rnbook");
        // default entries directory: <config_dir>/entries
        let mut entries_path = config_dir.clone();
        entries_path.push("entries");
        Self { entries_path }
    }
}

impl Config {
    /// load the config from the OS-specific config directory
    /// if the file does not exist or is invalid, the default configuration is used and written to disk
    pub fn load() -> Result<Self, std::io::Error> {
        let mut config_path = dirs::config_dir()
            .ok_or_else(|| Error::new(ErrorKind::NotFound, "couldn't find config directory"))?;
        config_path.push("rnbook");
        config_path.push("config.json");

        if config_path.exists() {
            let contents = fs::read_to_string(&config_path)?;
            let cfg: Config = serde_json::from_str(&contents)?;
            Ok(cfg)
        } else {
            let cfg = Config::default();
            cfg.save()?;
            Ok(cfg)
        }
    }

    /// save the configuration to the appropriate config directory
    pub fn save(&self) -> Result<(), std::io::Error> {
        let mut config_dir = dirs::config_dir()
            .ok_or_else(|| Error::new(ErrorKind::NotFound, "couldn't find config directory"))?;
        config_dir.push("rnbook");
        fs::create_dir_all(&config_dir)?;
        let mut config_file = config_dir.clone();
        config_file.push("config.json");
        let serialized = serde_json::to_string_pretty(self)?;
        fs::write(config_file, serialized)?;
        Ok(())
    }

    /// returns the path to the master index file, which is always inside the entries directory
    pub fn master_index_path(&self) -> PathBuf {
        let mut p = self.entries_path.clone();
        p.push("rnbook_master_list.json");
        p
    }
}
