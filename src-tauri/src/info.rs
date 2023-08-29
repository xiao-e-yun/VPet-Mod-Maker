use std::{collections::BTreeMap, fs::{read, File}, path::PathBuf, io::Write};

use serde::{Deserialize, Serialize};

use crate::config;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Info {
  pub vupmod: String,
  pub icon: String,
  pub author: String,
  pub intro: String,
  #[serde(rename = "foodImage")]
  pub food_image: String,
  pub foods: Vec<Food>,
  pub clicktexts: Vec<ClickText>,
  pub lowtexts: Vec<LowText>,
  pub actions: Vec<Action>,
}

#[tauri::command]
pub fn get_info(name: String) -> Info {
    let info_path = PathBuf::from(config().path.unwrap())
      .join("mod")
      .join(name+".json");
    
    let info_json = read(info_path).unwrap_or(b"{}".to_vec());
    serde_json::from_slice(&info_json).unwrap_or_default()
}

#[tauri::command]
pub fn save_info(name: String, data: Info) {
    let json = serde_json::to_string(&data).unwrap();
    let info_path = PathBuf::from(config().path.unwrap())
      .join("mod")
      .join(name+".json");

    File::create(info_path)
        .unwrap()
        .write_all(json.as_bytes())
        .unwrap();
}

///
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Food {
  pub name: String,
  #[serde(rename = "type")]
  pub ftype: String,
  pub desc: String,
  pub price: f32,
  pub exp: f32,
  pub strength: f32,
  pub food: f32,
  pub drink: f32,
  pub feeling: f32,
  pub health: f32,
  pub likability: f32,
  pub image: String,
}

///
/// https://github.com/LorisYounger/VPet/blob/main/VPet-Simulator.Core/Graph/GraphHelper.cs#L89
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Action {
  pub name: String,
  pub money: (f32,f32),
  #[serde(rename="type")]
  pub atype: ActionType,
  pub graph: String,
  pub food: f32,
  pub drink: f32,
  pub feeling: f32,
  #[serde(rename="levelLimit")]
  pub level_limit: u32,
  #[serde(rename="finishBonus")]
  pub finish_bonus: f32,
  pub time: u32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum ActionType {
  Work,
  Study,
  Play,
}

impl Default for ActionType {
  fn default() -> Self { ActionType::Work }
}

///
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ClickText {
  pub like: (String,String),
  pub state: ClickTextState,
  pub working: String,
  pub text: String,
  pub mode: [bool;4],
  pub daytime: [bool;4]
}

#[derive(Serialize, Deserialize, Debug,PartialEq)]
pub enum ClickTextState {
  Nomal,
  Work,
  Sleep,
}

impl Default for ClickTextState {
  fn default() -> Self { ClickTextState::Nomal }
}

///
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct LowText {
  pub main: bool,
  pub mode: bool,
  pub strength: String,
  pub like: String,
  pub text: String
}
