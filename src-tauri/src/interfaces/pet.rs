use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Pet {
    pub name: String,
    pub intro: String,
    pub touchhead: Rect,
    pub touchraised: PetStatus<Rect>,
    pub raisepoint: PetStatus<Vector>,
    pub animations: PetAnimations,
    pub duration: PetDurations,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PetDurations {
    pub state: u32,
    pub squat: u32,
    pub boring: u32,
    pub sleep: u32,
}

///
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PetAnimations {
    pub default: PetStatus<PetAnimationSingle>,
    pub start: PetStatus<PetAnimationSingle>,
    pub shutdown: PetStatus<PetAnimationSingle>,
    pub raised_static: PetStatus<PetAnimationLoop>,

    pub music: PetStatus<PetAnimationLoop>,
    pub sleep: PetStatus<PetAnimationLoop>,
    pub raised_dynamic: PetStatus<PetAnimationSingle>,

    pub switch_up: PetStatus<PetAnimationSingle>,
    pub switch_down: PetStatus<PetAnimationSingle>,
    pub switch_hunger: PetStatus<PetAnimationSingle>,
    pub switch_thirsty: PetStatus<PetAnimationSingle>,

    pub touch_head: PetStatus<PetAnimationLoop>,
    pub touch_body: PetStatus<PetAnimationLoop>,

    pub say_self: PetStatus<PetAnimationLoop>,
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
    pub moving: Vec<(String, PetStatus<PetAnimationLoop>)>,
    pub idel: Vec<(String, PetStatus<PetAnimationLoop>)>,
    pub work: Vec<(String, PetStatus<PetAnimationLoop>)>,
}

///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PetStatus<T> {
  #[serde(skip_serializing_if = "Option::is_none")]
   pub happy: Option<T>,
  #[serde(skip_serializing_if = "Option::is_none")]
   pub nomal: Option<T>,
  #[serde(skip_serializing_if = "Option::is_none")]
   pub poorcondition: Option<T>,
  #[serde(skip_serializing_if = "Option::is_none")]
   pub ill: Option<T>,
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
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PetAnimationSingle(pub Vec<String>);
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PetAnimationLoop {
  #[serde(skip_serializing_if = "Option::is_none")]
   pub  start: Option<Vec<String>>,
  #[serde(skip_serializing_if = "Option::is_none")]
   pub  repeat: Option<Vec<String>>,
  #[serde(skip_serializing_if = "Option::is_none")]
   pub  finish: Option<Vec<String>>,
}

//utils
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Rect(pub u32, pub u32, pub u32, pub u32);

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Vector(pub u32, pub u32);
