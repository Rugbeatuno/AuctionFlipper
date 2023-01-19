use std::{collections::HashMap, fs};
// use std::fs;
use crate::{
    constants::{
        self, COMMON, DIVINE, EPIC, LEGENDARY, MYTHIC, RARE, SPECIAL, SUPREME, UNCOMMON,
        VERY_SPECIAL,
    },
    structs,
};

use colored::*;
use std::time::{SystemTime, UNIX_EPOCH};

// const PATH_TO_PAGES: &str = "./src/files/pages.txt";

pub fn prettify_large_nums(num: i32) -> String {
    let mut conversions: HashMap<String, char> = HashMap::new();
    conversions.insert("1".to_string(), 'k');
    conversions.insert("2".to_string(), 'M');
    conversions.insert("3".to_string(), 'B');
    conversions.insert("4".to_string(), 'T');

    if num < 1000 {
        return num.to_string();
    }

    let mut divides = 0;
    let mut num = num as f64;
    while num >= 1000.0 {
        num /= 1000.0;
        divides += 1;
    }

    let float_val = format!("{:.1$}", num, 1);
    return format!(
        "{}{}",
        float_val,
        conversions.get(&divides.to_string()).unwrap().to_string()
    );
}

pub fn rarity_to_color(flip: &structs::Flip) -> String {
    match flip.lowest_bin.tier.as_str() {
        COMMON => return flip.lowest_bin.name.truecolor(125, 125, 125).to_string(),
        UNCOMMON => return flip.lowest_bin.name.truecolor(78, 237, 81).to_string(),
        RARE => return flip.lowest_bin.name.truecolor(60, 120, 239).to_string(),
        EPIC => return flip.lowest_bin.name.as_str().magenta().to_string(),
        LEGENDARY => return flip.lowest_bin.name.truecolor(207, 138, 35).to_string(),
        MYTHIC => return flip.lowest_bin.name.truecolor(232, 74, 231).to_string(),
        SUPREME => return flip.lowest_bin.name.truecolor(213, 24, 29).to_string(),
        SPECIAL => return flip.lowest_bin.name.red().to_string(),
        VERY_SPECIAL => return flip.lowest_bin.name.red().to_string(),
        DIVINE => return flip.lowest_bin.name.red().to_string(),
        _ => {}
    };
    println!("No Match {}", flip.lowest_bin.tier);
    "".to_string()
}

// pub async fn get_duration_until_api_update() -> u64 {
//     let req = surf::get("https://api.hypixel.net/skyblock/auctions");
//     let res = constants::HTTP_CLIENT1.send(req).await;

//     let json = res.unwrap().body_json().await;
//     let json: serde_json::Value = json.unwrap();

//     let mut last_updated = json.get("lastUpdated").unwrap().as_u64().unwrap() / 1000;
//     let unix_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

//     while last_updated < unix_timestamp {
//         last_updated += 60;
//     }
//     return last_updated;

// }

pub fn get_unix_timestamp() -> i32 {
    return SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i32;
}

pub fn load_json_file(path: &str) -> HashMap<String, String> {
    let data: String = fs::read_to_string(path).expect("Unable to read file");
    let mut json: HashMap<String, String> = HashMap::new();
    if fs::metadata(&path).unwrap().len() != 0 {
        json = serde_json::from_str(&data).unwrap();
    }
    return json;
}

// Uses cloudflare age headers
pub async fn get_next_api_update() -> i32 {
    let res = constants::HTTP_CLIENT1
        .get("https://api.hypixel.net/skyblock/auctions?page=0")
        .send()
        .await;

    let unix_timestamp: i32 = get_unix_timestamp();

    match res {
        Ok(res_unwrap) => match res_unwrap.header("age") {
            Some(age_header) => {
                let age = age_header.as_str().parse::<i32>().unwrap();
                println!("Page age: {}", age);
                // cloudfare doesn't return an exact time in ms, so the +2 accounts for that
                let age = age % 60;
                let time = 60 - age;
                return unix_timestamp + time;
            }
            None => {
                return unix_timestamp + 60;
            }
        },
        Err(_) => {
            return unix_timestamp + 60;
        }
    }
}

pub async fn fetch(url: String) -> serde_json::Value {
    let req = surf::get(url);

    let res = constants::HTTP_CLIENT1.send(req).await;
    if res.is_ok() {
        let json = res.unwrap().body_json().await;
        if json.is_ok() {
            return json.unwrap();
        }
    }

    serde_json::Value::Null
}

pub fn save_next_update(next_update: i32) {
    fs::write(constants::PATH_TO_NEXT_UPDATE_FILE, next_update.to_string())
        .expect("Unable to write file");
}

// Pets are formatted as TIER-NAME-LEVEL-PET_ITEM, and with candy or skins are ignored
// Enchanted books are ENCHANT_LEVEL
// Normal items are just RARITY-ISRECOMBED-REFORGE-NAME-STARS-SPECIALENCHANTS
// pub fn format_name_for_target(nbt: &PartialNbtElement) -> Option<String> {
//     let id = nbt.tag.extra_attributes.id.as_ref().unwrap();
//     let mut name: String = String::from("");

//     if id == "PET" {
//         let pet_data = is_pet(&nbt, "".to_string()).unwrap();
//         if pet_data.candy > 0 {
//             return None;
//         }
//         name = format!(
//             "{}-{}-{}-{}",
//             pet_data.tier, pet_data.name, pet_data.level, pet_data.item
//         );
//     } else if id == "ENCHANTED_BOOK" {
//         match nbt.tag.extra_attributes.enchantments.as_ref() {
//             Some(e) => {
//                 if e.len() == 1 {
//                     for (enchant, level) in e {
//                         name = format!("{}_{}", enchant, level);
//                     }
//                 }
//             }
//             None => return None,
//         };
//     }
//     // Normal items are just ISRECOMBED-REFORGE-NAME-STARS
//     else {
//         let is_recombed: bool = match nbt.tag.extra_attributes.rarity_upgrades {
//             Some(_) => true,
//             None => false,
//         };
//         let reforge: String = match nbt.tag.extra_attributes.modifier.as_ref() {
//             Some(r) => r.to_string(),
//             None => "none".to_string(),
//         };
//         let stars: u8 = match nbt.tag.extra_attributes.dungeon_item_level {
//             Some(s) => s,
//             None => 0,
//         };
//         let hpb: u8 = match nbt.tag.extra_attributes.hot_potato_count {
//             Some(s) => s,
//             None => 0,
//         };
//         let fpb: u8 = if hpb > 10 { hpb - 10 } else { 0 };

//         let mut ultimate_enchant = "none".to_string();
//         match nbt.tag.extra_attributes.enchantments.as_ref() {
//             Some(e) => {
//                 for (enchant, level) in e {
//                     if enchant.contains("ultimate") {
//                         ultimate_enchant = format!("{}_{}", enchant, level).to_owned();
//                     }
//                 }
//             }
//             None => {}
//         };

//         name = format!(
//             "{}-{}-{}-{}-{}-{}-{}",
//             is_recombed, reforge, id, stars, hpb, fpb, ultimate_enchant
//         );
//     }
//     Some(name)
// }

// pub fn format_name_for_sales(nbt: &PartialNbtElement) -> String {
//     let mut name: String = nbt.tag.extra_attributes.id.as_ref().unwrap().to_string();

//     if name == "PET" {
//         let pet_data = is_pet(nbt, "".to_string()).unwrap();
//         name = format!("{}-{}-{}", pet_data.tier, pet_data.name, pet_data.level);
//     } else if name == "ENCHANTED_BOOK" {
//         match nbt.tag.extra_attributes.enchantments.as_ref() {
//             Some(e) => {
//                 if e.len() == 1 {
//                     for (enchant, level) in e {
//                         name = format!("{}_{}", enchant, level);
//                     }
//                 }
//             }
//             None => {}
//         };
//     }

//     name
// }

// we will used a trimmed mean to remove outliers
// meaning, ignoring the top 10% and bottom 10% of the list
// pub fn find_avg_target_price(
//     targets: &HashMap<String, Vec<SaleData>>,
//     formatted_target_name: String,
// ) -> u64 {
//     match targets.get(&formatted_target_name) {
//         Some(flips) => {
//             // if flips.len() < 100 {
//             // return 0;
//             // }

//             let mut flips = flips.clone();
//             flips.sort_by_key(|f| f.price.());

//             let ten_percent = flips.len() / 5;
//             let trimmed_flips = &flips[ten_percent..flips.len() - ten_percent];
//             if trimmed_flips.len() == 0 {
//                 return 0;
//             }
//             return trimmed_flips.iter().map(|e| e.price).sum::<u64>() / trimmed_flips.len() as u64;
//         }
//         None => return 0,
//     }
// }
