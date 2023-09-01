use crate::interfaces::pet::Pet;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    pub vupmod: String,
    pub icon: String,
    pub author: String,
    pub intro: String,
    pub food_image: String,
    pub foods: Vec<Food>,
    pub clicktexts: Vec<ClickText>,
    pub lowtexts: Vec<LowText>,
    pub actions: Vec<Action>,
    pub pets: Vec<Pet>,
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
#[serde(rename_all = "camelCase")]
pub struct Action {
    pub name: String,
    pub money: (f32, f32),
    #[serde(rename = "type")]
    pub atype: ActionType,
    pub graph: String,
    pub food: f32,
    pub drink: f32,
    pub feeling: f32,
    pub level_limit: u32,
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
    fn default() -> Self {
        ActionType::Work
    }
}

///
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ClickText {
    pub like: (String, String),
    pub state: ClickTextState,
    pub working: String,
    pub text: String,
    pub mode: [bool; 4],
    pub daytime: [bool; 4],
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum ClickTextState {
    Nomal,
    Work,
    Sleep,
}

impl Default for ClickTextState {
    fn default() -> Self {
        ClickTextState::Nomal
    }
}

///
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct LowText {
    pub main: bool,
    pub mode: bool,
    pub strength: String,
    pub like: String,
    pub text: String,
}
