use std::{
    fs::{self, create_dir, OpenOptions},
    io::Write,
    path::PathBuf,
};

use fs_extra::{copy_items, dir};

use crate::interfaces::{
    info::{Action, ClickText, ClickTextState, Food, Info, LowText},
    pet::{Pet, PetStatus, Rect, Vector},
};
/// Global Var
#[derive(Debug, Default)]
struct Builder {
    path: PathBuf,
    food: PathBuf,
    text: PathBuf,
    pet: PathBuf,
    pet_index: usize,
}

trait Build {
    fn build(&self, builder: &mut Builder) -> Option<String>;
}

#[tauri::command]
pub fn build(name: String) -> Option<String> {
    let info = Info::load(&name);
    let mut builder = Builder::default();
    builder.path = Info::path().join(name);
    info.build(&mut builder)
}

impl Build for Info {
    fn build(&self, builder: &mut Builder) -> Option<String> {
        let path = builder.path.clone();

        if path.exists() {
            let is_error = fs::remove_dir_all(&path).is_err();
            if is_error {
                return Some(String::from("請關閉 VPet 再嘗試"));
            }
        }
        fs::create_dir(&path).unwrap();

        // info
        {
            println!("Write info.lps");
            let text = format!(
                "vupmod#{}:|author#{}:|gamever#100:|ver#100:|\nintro#{}:|",
                self.vupmod, self.author, self.intro
            );
            fs::write(path.join("info.lps"), text).unwrap();

            if !self.icon.is_empty() {
                println!("Copy icon image [{}]", self.icon);
                fs::copy(self.icon.clone(), path.join("icon.png")).unwrap();
            }
        }
        //

        // food
        if !self.foods.is_empty() {
            println!("Create food folder");
            builder.food = path.join("image");
            fs::create_dir(builder.food.clone()).unwrap();

            if !self.food_image.is_empty() {
                println!("Copy food default image");
                fs::copy(self.food_image.clone(), builder.food.join("food.png")).unwrap();
                builder.food = builder.food.join("food");
            }

            println!("Create food image folder");
            fs::create_dir(builder.food.clone()).unwrap();

            println!("Write food/0.lps");
            let path = path.join("food");
            let mut text = String::new();
            for food in self.foods.iter() {
                text += &food.build(builder).unwrap();
            }
            fs::create_dir(&path).unwrap();
            fs::write(path.join("0.lps"), text).unwrap();
        }
        //

        //
        if !self.clicktexts.is_empty() || !self.lowtexts.is_empty() {
            println!("Create text folder");
            builder.text = builder.path.join("text");
            fs::create_dir(&builder.text).unwrap();

            let mut data = String::new();
            for clicktext in self.clicktexts.iter() {
                data += &clicktext.build(builder).unwrap_or_default();
            }

            for lowtext in self.lowtexts.iter() {
                data += &lowtext.build(builder).unwrap_or_default();
            }

            println!("Write text/0.lps");
            fs::write(builder.text.join("0.lps"), data).unwrap();
        }
        //

        if !self.pets.is_empty() || !self.actions.is_empty() {
            builder.pet = path.join("pet");
            fs::create_dir(&builder.pet).unwrap();
        }

        //
        if !self.pets.is_empty() {
            for (index, pet) in self.pets.iter().enumerate() {
                builder.pet_index = index;
                pet.build(builder);
            }
        }
        //

        if !self.actions.is_empty() {
            let pets_name = self.pets.iter().map(|pet| pet.name.clone());
            for (index, name) in pets_name.chain([String::from("萝莉斯")]).enumerate() {
                let mut used = false;
                let mut data = format!("pet#{}:|\n", name);

                for action in self.actions.iter() {
                    if let Some(line) = action.build(builder) {
                        data += &line;
                        used = true;
                    }
                }

                if used {
                    let id = if name == "萝莉斯" {
                        String::from("vup")
                    } else {
                        index.to_string()
                    };
                    fs::write(
                        builder.pet.join(format!("{}.action.lps", id)),
                        data.as_bytes(),
                    )
                    .unwrap();
                }
            }
        }

        None
    }
}

impl Build for Food {
    fn build(&self, _: &mut Builder) -> Option<String> {
        let mut values = String::new();
        push_value(&mut values, "price", self.price);
        push_value(&mut values, "Exp", self.exp);
        push_value(&mut values, "Strength", self.strength);
        push_value(&mut values, "StrengthDrink", self.drink);
        push_value(&mut values, "StrengthFood", self.food);
        push_value(&mut values, "Health", self.health);
        push_value(&mut values, "Feeling", self.feeling);
        push_value(&mut values, "Likability", self.likability);

        fn push_value(values: &mut String, key: &'static str, value: f32) {
            if value.abs() > f32::EPSILON {
                *values += &format!("{}#{}:|", key, value);
            }
        }

        Some(format!(
            "food:|type#{}:|name#{}:|desc#{}:|{}\n",
            self.ftype, self.name, self.desc, values
        ))
    }
}

impl Build for LowText {
    fn build(&self, _: &mut Builder) -> Option<String> {
        let main = if self.main {
            "LowFoodText"
        } else {
            "LowDrinkText"
        };
        let mode = if self.mode { "H" } else { "L" };

        Some(format!(
            "{}:|Mode#{}:|Strength#{}:|Like#{}:|Text#{}:|\n",
            main, mode, self.strength, self.like, self.text
        ))
    }
}

impl Build for ClickText {
    fn build(&self, _: &mut Builder) -> Option<String> {
        let (mut like_min, mut like_max) = self.like.clone();

        let mode = {
            let mut value = 0b0000;
            for boolean in self.mode.iter().cloned().rev() {
                value = (value + boolean as u8) << 1
            }
            value >> 1
        };

        let daytime = {
            let mut value = 0b0000;
            for boolean in self.daytime.iter().cloned().rev() {
                value = (value + boolean as u8) << 1
            }
            value >> 1
        };

        let like = {
            let mut like = String::new();

            like_min = like_min.chars().filter(|c| c.is_digit(10)).collect();
            if !like_min.is_empty() {
                like += &format!("LikeMin#{}:|", like_min)
            };

            like_max = like_max.chars().filter(|c| c.is_digit(10)).collect();
            if !like_max.is_empty() {
                like += &format!("LikeMax#{}:|", like_max)
            };

            like
        };

        let state = {
            let mut state = format!("State#{:?}:|", self.state);
            if self.state == ClickTextState::Work {
                state += &format!("Working#{}:|", self.working);
            }
            state
        };

        Some(format!(
            "clicktext:|Text#{}:|Mode#{}:|daytime#{}:|{}\n",
            self.text,
            mode,
            daytime,
            like + &state,
        ))
    }
}

impl Build for Action {
    fn build(&self, builder: &mut Builder) -> Option<String> {
        self.graph.get(&builder.pet_index).map(|graph|{
        format!("work:|Type#{:?}:|Name#{}:|MoneyBase#{}:|MoneyLevel#{}:|Graph#{}:|StrengthFood#{}:|StrengthDrink#{}:|Feeling#{}:|Time#{}:|FinishBonus#{}:|LevelLimit#{}:|BorderBrush#000000:|Background#413d39:|ButtonBackground#322e2b:|ButtonForeground#FFFFFF:|Foreground#ccbdad:|Left#113:|Top#315:|Width#280:|\n",
        self.atype,self.name,self.money.0,self.money.1,graph,self.food,self.drink,self.feeling,self.time,self.finish_bonus,self.level_limit)
      })
    }
}

///
///
///
impl Build for Pet {
    fn build(&self, builder: &mut Builder) -> Option<String> {
        let path = builder.pet.join(format!("{}.lps", builder.pet_index));
        let index = builder.pet_index;

        println!("Build pet [{}]", self.name);

        // let mut touchhead = String::new();
        // let mut touchraised = String::new();
        // let mut raisepoint = String::new();
        // for (key, val) in ["px", "py", "sw", "sh"].zip(self.touchhead.get()) {
        //     touchhead += &format!("{}#{}:|", key, val);
        // }

        // for ((status, raisepoint_value), touchraised_value) in
        //     self.raisepoint.iter().zip(self.touchraised.get())
        // {
        //     let vector = touchraised_value.clone().unwrap_or_default();
        //     for (key, val) in ["px", "py", "sw", "sh"].zip(vector.get()) {
        //         touchraised += &format!("{}_{}#{}:|", status, key, val)
        //     }
        //     let vector = raisepoint_value.clone().unwrap_or_default();
        //     for (key, val) in ["x", "y"].zip(vector.get()) {
        //         raisepoint += &format!("{}_{}#{}:|", status, key, val)
        //     }
        // }

        let duration = format!(
            "duration:|state#{}:|squat#{}:|boring#{}:|sleep#{}:|",
            self.duration.state, self.duration.squat, self.duration.boring, self.duration.sleep
        );

        //
        let args = String::from("touchhead:|px#159:|py#16:|sw#189:|sh#178:|\n")+
        "touchraised:|happy_px#0:|happy_py#50:|happy_sw#500:|happy_sh#200:|nomal_px#0:|nomal_py#50:|nomal_sw#500:|nomal_sh#200:|poorcondition_px#0:|poorcondition_py#50:|poorcondition_sw#500:|poorcondition_sh#200:|ill_px#0:|ill_py#200:|ill_sw#500:|ill_sh#300:|\n"+
        "raisepoint:|happy_x#290:|happy_y#128:|nomal_x#290:|nomal_y#128:|poorcondition_x#290:|poorcondition_y#128:|ill_x#225:|ill_y#115:|";
        let lps = format!(
            "pet#{}:|intro#{}:|path#{}:|petname#{}:|\n{}\n{}",
            self.name, self.intro, index, self.name, args, duration
        );

        // let lps = format!(
        //     "pet#{}:|intro#{}:|path#{}:|petname#{}:|\ntouchhead:|{}\ntouchraised:|{}\nraisepoint:|{}\n{}",
        //     self.name, self.intro, index, self.name, touchhead, touchraised, raisepoint, duration
        // );

        println!("Write {}.lps", builder.pet_index);
        fs::write(path, lps).unwrap();

        //images
        //images
        //images
        let path = builder.pet.join(format!("{}", builder.pet_index));
        fs::create_dir(&path).unwrap();

        let animations = &self.animations;

        let single = [
            ("default", &animations.default),
            ("startup", &animations.start),
            ("shutdown", &animations.shutdown),
            ("raise/raised_dynamic", &animations.raised_dynamic),
            ("switch_up", &animations.switch_up),
            ("switch_down", &animations.switch_down),
            ("switch_hunger", &animations.switch_hunger),
            ("switch_thirsty", &animations.switch_thirsty),
            ("state_one", &animations.state_one),
            ("state_two", &animations.state_two),
            ("drink/front", &animations.drink_front),
            ("drink/back", &animations.drink_back),
            ("eat/front", &animations.eat_front),
            ("eat/back", &animations.eat_back),
        ];

        for (name, status) in single {
            for (status, animation) in status.iter() {
                if let Some(animation) = animation {
                    let path: PathBuf = path.join(name).join(status);
                    fs::create_dir_all(&path).unwrap();
                    for (index, folder) in animation.0.iter().enumerate() {
                        let output = path.join(format!("{}", index));
                        copy_dir(folder, &output);
                    }
                }
            }
        }

        let loops = [
            ("music", &animations.music),
            ("sleep", &animations.sleep),
            ("raise/raised_static", &animations.raised_static),
            ("touch_body", &animations.touch_body),
            ("touch_head", &animations.touch_head),
            ("say/self", &animations.say_self),
            ("say/serious", &animations.say_serious),
            ("say/shining", &animations.say_shining),
        ];

        for (name, status) in loops {
            for (status, animation) in status.iter() {
                if let Some(animation) = animation {
                    for (mode, vector) in [
                        ("a", &animation.start),
                        ("b", &animation.repeat),
                        ("c", &animation.finish),
                    ] {
                        if let Some(animation) = vector {
                            let path: PathBuf = path.join(name).join(status).join(mode);
                            fs::create_dir_all(&path).unwrap();
                            for (index, folder) in animation.iter().enumerate() {
                                let output = path.join(format!("{}", index));
                                copy_dir(folder, &output);
                            }
                        }
                    }
                }
            }
        }

        let custom = [
            ("move", &animations.moving),
            ("idel", &animations.idel),
            ("work", &animations.work),
        ];

        for (name, animations) in custom {
            for (key, status) in animations.iter() {
                for (status, animation) in status.iter() {
                    if let Some(animation) = animation {
                        for (mode, vector) in [
                            ("a", &animation.start),
                            ("b", &animation.repeat),
                            ("c", &animation.finish),
                        ] {
                            if let Some(animation) = vector {
                                let path: PathBuf =
                                    path.join(name).join(key).join(status).join(mode);
                                fs::create_dir_all(&path).unwrap();
                                for (index, folder) in animation.iter().enumerate() {
                                    let output = path.join(format!("{}", index));
                                    copy_dir(folder, &output);
                                }
                            }
                        }
                    }
                }
            }
        }

        fn copy_dir(from: &String, to: &PathBuf) {
            fs::create_dir(to).unwrap();
            for (index, file) in fs::read_dir(from).unwrap().enumerate() {
                let file = file.unwrap();
                fs::copy(file.path(), to.join(index.to_string() + "_100.png")).unwrap();
            }
        }

        None
    }
}

///
///
///
impl Rect {
    fn get(&self) -> [u32; 4] {
        [self.0, self.1, self.2, self.3]
    }
}
impl Vector {
    fn get(&self) -> [u32; 2] {
        [self.0, self.1]
    }
}

impl<T> PetStatus<T> {
    fn get(&self) -> [&Option<T>; 4] {
        [&self.happy, &self.nomal, &self.poorcondition, &self.ill]
    }
    fn iter(&self) -> [(&str, &Option<T>); 4] {
        ["happy", "nomal", "poorcondition", "ill"].zip(self.get())
    }
}
