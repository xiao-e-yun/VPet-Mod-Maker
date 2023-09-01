use std::{
    fs::{read, File},
    io::Write,
    path::PathBuf,
};

use crate::{config::Config, interfaces::info::Info};

#[tauri::command]
pub fn get_info(name: String) -> Info {
    Info::load(&name)
}

#[tauri::command]
pub fn save_info(name: String, data: Info) {
    data.save(&name)
}

impl Info {
    pub fn path() -> PathBuf {
        PathBuf::from(Config::load().path.unwrap()).join("mod")
    }
    pub fn load(name: &String) -> Self {
        let path = Info::path().join(name.clone() + ".json");

        let json = read(path).unwrap_or(b"{}".to_vec());
        serde_json::from_slice(&json).unwrap_or_default()
    }
    pub fn save(&self, name: &String) {
        let json = serde_json::to_string(&self).unwrap();
        let path = Info::path().join(name.clone() + ".json");

        File::create(path)
            .unwrap()
            .write_all(json.as_bytes())
            .unwrap();
    }
}
