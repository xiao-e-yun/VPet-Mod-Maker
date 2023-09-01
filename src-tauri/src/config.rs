use std::{
    cell::Cell,
    fs::{read, File},
    io::Write,
};

use serde::{Deserialize, Serialize};

static mut CONFIG_CACHE: Cell<Option<Config>> = Cell::new(None);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
  pub path: Option<String>,
}

impl Config {
    pub fn load() -> Self {
        let cache = unsafe { CONFIG_CACHE.get_mut().clone() };

        match cache {
            Some(config) => config,
            None => {
                let data = read("./config.json").unwrap_or(b"{}".to_vec());
                let config: Config = serde_json::from_slice(&data).unwrap();
                unsafe { CONFIG_CACHE.set(Some(config.clone())) }
                config
            }
        }
    }
    pub fn save(&self) {
        unsafe { CONFIG_CACHE.set(Some(self.clone())) }

        let config = serde_json::to_string(&self).unwrap_or("{}".to_string());
        File::create("./config.json")
            .unwrap()
            .write(config.as_bytes())
            .unwrap();
    }
}
