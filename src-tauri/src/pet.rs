
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Pet {
  name: String,
  intro: String,
  petname: String,
  touchhead: Rect,
  touchraised: (Rect,Rect,Rect,Rect),
  raisepoint: (Vector,Vector,Vector,Vector),
  
  duration: PetDuration,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub enum PetStatus {
  Happy,
  Poorcondition,
  Ill,
  Nomal,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Rect(u32,u32,u32,u32);

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Vector(u32,u32);

/// seconds
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PetDuration {
  state: f32,
  squat: f32,
  boring: f32,
  sleep: f32,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Work {
  #[rename="type"]
  rtype: String,
  name: String,
  images: Vec<String>
  moneybase: f32,
  moneylevel: f32,
  graph: String,
  strengthFood: f32,
  strengthDrink: f32,
  feeling: f32,
  time: f32,
}