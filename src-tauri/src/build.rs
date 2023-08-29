use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use crate::{
    config,
    info::{get_info, ClickText, ClickTextState, Food, Info, LowText, Action},
};

#[tauri::command]
pub fn build_mod(name: String) -> Option<String> {
    let data = get_info(name.clone());

    let path = PathBuf::from(config().path.unwrap()).join("mod").join(name);
    if path.exists() {
      let is_error = fs::remove_dir_all(&path).is_err();
      if is_error {
        return Some(String::from("請關閉 VPet 再嘗試"));
      }
    }
    fs::create_dir(&path).unwrap();

    //build info.lps
    let mut info_file = File::create(path.join("info.lps")).unwrap();
    info_file.write_all(build_info(&data).as_bytes()).unwrap();

    if !data.icon.is_empty() {
        fs::copy(data.icon, path.join("icon.png")).unwrap();
    };

    if data.foods.len() > 0 {
        let food_path = path.join("food");
        let mut image_path = path.join("image");
        fs::create_dir(&food_path).unwrap();
        fs::create_dir(&image_path).unwrap();

        if !data.food_image.is_empty() {
            fs::copy(&data.food_image, image_path.join("food.png")).unwrap();
            //copy other files to .../image/food/
            image_path = image_path.join("food");
            fs::create_dir(&image_path).unwrap();
        }

        let mut foods_info = String::new();
        for food in data.foods.iter() {
            foods_info += &build_food(food);
            // copy image to /image/{name}.png
            if !food.image.is_empty() {
                fs::copy(&food.image, image_path.join(food.name.clone() + ".png")).unwrap();
            };
        }

        File::create(food_path.join("0.lps"))
            .unwrap()
            .write_all(foods_info.as_bytes())
            .unwrap();
    }

    if data.clicktexts.len() + data.lowtexts.len() > 0 {
        let text_path = path.join("text");
        fs::create_dir(&text_path).unwrap();

        let mut text_info = String::new();
        for text in data.clicktexts.iter() {
            text_info += &build_clicktext(text);
        }

        for text in data.lowtexts.iter() {
            text_info += &build_lowtext(text);
        }
        File::create(text_path.join("0.lps"))
            .unwrap()
            .write_all(text_info.as_bytes())
            .unwrap();
    }

    if data.actions.len() > 0 {
      let action_path = path.join("pet");
      fs::create_dir(&action_path).unwrap();

  let mut action_info = String::from("pet#默认虚拟桌宠:|intor#虚拟主播模拟器默认人物形象:|path#vup:|petname#萝莉斯:|\n");
  for action in data.actions.iter() {
    action_info += &build_action(action)
  };


      fs::create_dir(action_path.join("vup")).unwrap();
      File::create(action_path.join("vup.actions.lps"))
      .unwrap()
      .write_all(action_info.as_bytes())
      .unwrap();
    }

    None
}

fn build_food(food: &Food) -> String {
    let mut values = String::new();
    push_value(&mut values, "price", food.price);
    push_value(&mut values, "Exp", food.exp);
    push_value(&mut values, "Strength", food.strength);
    push_value(&mut values, "StrengthDrink", food.drink);
    push_value(&mut values, "StrengthFood", food.food);
    push_value(&mut values, "Health", food.health);
    push_value(&mut values, "Feeling", food.feeling);
    push_value(&mut values, "Likability", food.likability);

    
    fn push_value(values: &mut String, key: &'static str, value: f32) {
        if value.abs() > f32::EPSILON {
            *values += &format!("{}#{}:|", key, value);
        }
    }

    format!(
        "food:|type#{}:|name#{}:|desc#{}:|{}\n",
        food.ftype, food.name, food.desc, values
    )
}

fn build_clicktext(clicktext: &ClickText) -> String {
    let (mut like_min, mut like_max) = clicktext.like.clone();

    let mode = {
        let mut value = 0b0000;
        for boolean in clicktext.mode.iter().cloned().rev() {
            value = (value + boolean as u8) << 1
        }
        value >> 1
    };

    let daytime = {
        let mut value = 0b0000;
        for boolean in clicktext.daytime.iter().cloned().rev() {
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
        let mut state = format!("State#{:?}:|", clicktext.state);
        if clicktext.state == ClickTextState::Work {
            state += &format!("Working#{}:|", clicktext.working);
        }
        state
    };

    format!(
        "clicktext:|Text#{}:|Mode#{}:|daytime#{}:|{}\n",
        clicktext.text,
        mode,
        daytime,
        like + &state,
    )
}

fn build_lowtext(lowtext: &LowText) -> String {
    let main = if lowtext.main {
        "LowFoodText"
    } else {
        "LowDrinkText"
    };
    let mode = if lowtext.mode { "H" } else { "L" };

    format!(
        "{}:|Mode#{}:|Strength#{}:|Like#{}:|Text#{}:|\n",
        main, mode, lowtext.strength, lowtext.like, lowtext.text
    )
}

fn build_action(action: &Action) -> String {
  format!("work:|Type#{:?}:|Name#{}:|MoneyBase#{}:|MoneyLevel#{}:|Graph#{}:|StrengthFood#{}:|StrengthDrink#{}:|Feeling#{}:|Time#{}:|FinishBonus#{}:|LevelLimit#{}:|BorderBrush#000000:|Background#413d39:|ButtonBackground#322e2b:|ButtonForeground#FFFFFF:|Foreground#ccbdad:|Left#113:|Top#315:|Width#280:|\n",
  action.atype,action.name,action.money.0,action.money.1,action.graph,action.food,action.drink,action.feeling,action.time,action.finish_bonus,action.level_limit)
}

fn build_info(info: &Info) -> String {
    format!(
        "vupmod#{}:|author#Skip:|gamever#100:|ver#100:|\n",
        info.vupmod
    ) + &format!("intro#{}:|", info.intro)
}