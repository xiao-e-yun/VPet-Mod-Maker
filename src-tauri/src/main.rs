// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![feature(array_zip)]

mod build;
mod config;
mod info;

pub mod interfaces;

use std::{
    fs::{create_dir, read_dir, self},
    path::PathBuf,
};

use config::Config;

use crate::{
    build::build,
    info::{get_info, save_info},
    interfaces::info::Info,
};

#[tauri::command]
fn load_mods() -> Option<Vec<String>> {
    let path = Config::load().path?;
    let mods = PathBuf::from(path).join("mod");
    let dirs = read_dir(mods).ok()?;
    Some(
        dirs.filter_map(|res| {
            let file = res.unwrap().path().to_str().unwrap().to_string();
            let path = PathBuf::from(file.clone());
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
    if !PathBuf::from(path.clone()).join("mod").is_dir() {
        return Some("不正確路徑");
    }

    let mut config = Config::load();
    config.path = Some(path);
    config.save();
    None
}

#[tauri::command]
fn create_mod(name: String) {
    let dir = Info::path();
    if !dir.exists() {
        create_dir(&dir).unwrap()
    }
    fs::write(dir.join(name+".json"), "{}").unwrap();
}

fn main() {
    println!("Start VPet Maker");
    println!("Power by xiaoeyun");
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            load_mods, set_path, create_mod, get_info, save_info, build
        ])
        .run(tauri::generate_context!())
        .expect("Error while running tauri Application");
}
