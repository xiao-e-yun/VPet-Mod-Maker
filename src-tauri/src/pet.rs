use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Pet {
    name: String,
    intro: String,
    touchhead: Rect,
    touchraised: PetStatus<Rect>,
    raisepoint: PetStatus<Vector>,
    animations: PetAnimations,
    duration: PetDurations,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PetDurations {
    state: u32,
    squat: u32,
    boring: u32,
    sleep: u32,
}

///
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct PetAnimations {
    pub default: PetStatus<PetAnimationSingle>,
    pub start: PetStatus<PetAnimationSingle>,
    pub shutdown: PetStatus<PetAnimationSingle>,
    pub raised_static: PetStatus<PetAnimationSingle>,

    pub music: PetStatus<PetAnimationLoop>,
    pub sleep: PetStatus<PetAnimationLoop>,
    pub raised_dynamic: PetStatus<PetAnimationLoop>,

    pub switch_up: PetStatus<PetAnimationSingle>,
    pub switch_down: PetStatus<PetAnimationSingle>,
    pub switch_hunger: PetStatus<PetAnimationSingle>,
    pub switch_thirsty: PetStatus<PetAnimationSingle>,

    pub touch_head: PetStatus<PetAnimationLoop>,
    pub touch_body: PetStatus<PetAnimationLoop>,

    #[serde(rename="saySelf")]
    pub say_this: PetStatus<PetAnimationLoop>,
    pub say_serious: PetStatus<PetAnimationLoop>,
    pub say_shining: PetStatus<PetAnimationLoop>,
    
    pub state_one: PetStatus<PetAnimationSingle>,
    pub state_two: PetStatus<PetAnimationSingle>,

    pub drink_front: PetStatus<PetAnimationSingle>,
    pub drink_back: PetStatus<PetAnimationSingle>,

    pub eat_front: PetStatus<PetAnimationSingle>,
    pub eat_back: PetStatus<PetAnimationSingle>,
    //allow add custom
    #[serde(rename = "move")]
    pub moving: HashMap<String, PetStatus<PetAnimationLoop>>,
    pub idel: HashMap<String, PetStatus<PetAnimationLoop>>,
    pub work: HashMap<String, PetStatus<PetAnimationLoop>>,
}

///
#[derive(Serialize, Deserialize, Debug)]
pub struct PetStatus<T> {
    happy: Option<T>,
    nomal: Option<T>,
    poorcondition: Option<T>,
    ill: Option<T>,
}

impl<T> Default for PetStatus<T> {
    fn default() -> Self {
        Self {
            happy: None,
            poorcondition: None,
            ill: None,
            nomal: None,
        }
    }
}

///
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct  PetAnimationSingle(Vec<String>);
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct  PetAnimationLoop{
  start: Option<Vec<String>>,
  repeat: Option<Vec<String>>,
  finish: Option<Vec<String>>,
}

//utils
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Rect(u32, u32, u32, u32);

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Vector(u32, u32);
