// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod info;
mod build;

use std::{
    fs::{create_dir, read, read_dir, File},
    io::Write,
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

use crate::{info::{get_info,save_info}, build::build_mod};

fn config() -> Config {
    let data = read("./config.json").unwrap_or(b"{}".to_vec());
    serde_json::from_slice(&data).unwrap()
}

fn save_config(config: &Config) {
    let config = serde_json::to_string(&config).unwrap_or("{}".to_string());
    File::create("./config.json")
      .unwrap()
      .write_all(config.as_bytes())
      .unwrap();
}

#[tauri::command]
fn load_mods() -> Option<Vec<String>> {
    let mods = PathBuf::from(config().path?).join("mod");
    let dirs = read_dir(mods).ok()?;
    Some(
        dirs.filter_map(|res| {
            let file = res.unwrap().path().to_str().unwrap().to_string();
            let path = PathBuf::from(file.clone());
            if path.file_name().unwrap() == "0000_core" {
                return None;
            }
            if path.is_file() && path.extension().unwrap_or_default() == "json" {
                Some(path.file_stem().unwrap().to_str().unwrap().to_string())
            } else {
                None
            }
        })
        .collect(),
    )
}

#[tauri::command]
fn set_path(path: String) -> Option<&'static str> {
    if !PathBuf::from(path.clone()).join("mod").is_dir() { return Some("不正確路徑") }

    let mut config = config();
    config.path = Some(path);
    save_config(&config);
    None
}

#[tauri::command]
fn create_mod(name: String) {
  let path = PathBuf::from(config().path.unwrap()).join("mod").join(format!("{}.json",name));
  File::create(path).unwrap();
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
  path: Option<String>,
}

fn main() {
    tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![load_mods, set_path, create_mod,get_info,save_info,build_mod])
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}