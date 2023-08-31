export interface Info {
  vupmod: string,
  icon: string,
  intro: string,
  author: string,
  foodImage: string,
  foods: Food[],
  clicktexts: ClickText[],
  lowtexts: LowText[],
  actions: Action[],
  pets: Pet[],
}

export interface Food {
  name: string,
  type: string,
  desc: string,
  price: number,
  exp: number,
  strength: number,
  food: number,
  drink: number,
  feeling: number,
  health: number,
  likability: number,
  image: string,
}

export interface ClickText {
  like: [string, string],
  state: "Nomal" | "Work" | "Sleep",
  working: string,
  text: string,
  mode: [boolean, boolean, boolean, boolean],
  daytime: [boolean, boolean, boolean, boolean]
}

export interface LowText {
  main: boolean,
  mode: boolean,
  strength: string,
  like: string,
  text: string
}

export interface Action {
  name: string,
  money: [number, number],
  type: "Work" | "Study" | "Play",
  graph: string,
  food: number,
  drink: number,
  feeling: number,
  levelLimit: number,
  finishBonus: number,
  time: number,
}


export interface Pet {
  name: string,
  intro: string,
  touchhead: Rect,
  touchraised: PetStatus<Rect>,
  raisepoint: PetStatus<Vector>,
  animations: PetAnimations,
  duration: PetDurations,
}

export interface PetDurations {
  state: number,
  squat: number,
  boring: number,
  sleep: number,
}

///
export interface PetAnimations {
  default: PetStatus<PetAnimationSingle>,
  start: PetStatus<PetAnimationSingle>,
  shutdown: PetStatus<PetAnimationSingle>,
  raisedStatic: PetStatus<PetAnimationSingle>,

  music: PetStatus<PetAnimationLoop>,
  sleep: PetStatus<PetAnimationLoop>,
  raisedDynamic: PetStatus<PetAnimationLoop>,

  switchUp: PetStatus<PetAnimationSingle>,
  switchDown: PetStatus<PetAnimationSingle>,
  switchHunger: PetStatus<PetAnimationSingle>,
  switchThirsty: PetStatus<PetAnimationSingle>,

  touchHead: PetStatus<PetAnimationLoop>,
  touchBody: PetStatus<PetAnimationLoop>,

  saySelf: PetStatus<PetAnimationLoop>,
  saySerious: PetStatus<PetAnimationLoop>,
  sayShining: PetStatus<PetAnimationLoop>,

  stateOne: PetStatus<PetAnimationSingle>,
  stateTwo: PetStatus<PetAnimationSingle>,

  drinkFront: PetStatus<PetAnimationSingle>,
  drinkBack: PetStatus<PetAnimationSingle>,

  eatFront: PetStatus<PetAnimationSingle>,
  eatBack: PetStatus<PetAnimationSingle>,
  
  move: Record<string, PetStatus<PetAnimationLoop>>,
  idel: Record<string, PetStatus<PetAnimationLoop>>,
  work: Record<string, PetStatus<PetAnimationLoop>>,
}


export interface PetStatus<T> {
  happy?: T,
  nomal?: T,
  poorcondition?: T,
  ill?: T,
}

///
export interface PetAnimationLoop {
  start?: string[],
  repeat?: string[],
  finish?: string[],
}

export type PetAnimationSingle = string[]



//utils
type Vector = [number, number]
type Rect = [number, number, number, number]