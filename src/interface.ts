export interface ModInfo {
  vupmod: string,
  icon: string,
  intro: string,
  author: string,
  foodImage: string,
  foods: Food[],
  clicktexts: ClickText[],
  lowtexts: LowText[],
  actions: Action[],
  pet: Pet,
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
  like: [string,string],
  state: "Nomal" | "Work" | "Sleep",
  working: string,
  text: string,
  mode: [boolean,boolean,boolean,boolean],
  daytime: [boolean,boolean,boolean,boolean]
}

export interface LowText {
  main: boolean,
  mode: boolean,
  strength: string,
  like: string,
  text: string
}

export interface Pet {
  name: string,
  intor: string,
  petname: string,
}

export interface Action {
  name: string,
  money: [number,number],
  type: "Work" | "Study" | "Play",
  graph: string,
  food: number,
  drink: number,
  feeling: number,
  levelLimit: number,
  finishBonus: number,
  time: number,
}