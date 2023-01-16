use crate::{
    constants::{
        ARMOR, ENCHANT_PRICES, LORE_ENCHANTS, LORE_TO_KEY, ROMAN_NUMERALS, ROMAN_NUMERALS_ORDERED,
        WEAPON,
    },
    utils::{self},
};
use serde_derive::{Deserialize, Serialize};
use std::{collections::HashMap, time::Instant};

use colored::*;

// Auction structs
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Auction {
    pub id: String,
    pub seller: String,
    pub uuid: String,
    pub tier: String,
    pub name: String,
    pub lore: String,
    pub extra: String,
    pub price: i32,
    pub start: i32,
    pub end: i32,
    pub pet_data: Option<Pet>,
    pub additional_price_data: Option<PriceData>,
    pub item_bytes: serde_json::Value,
    pub category: String,
    // last_update: u64,
}

impl Auction {
    pub fn print_failed(&self, reason: &str) {
        println!("{}", format!("This auction failed ({}):", reason).red());
        println!("Seller: {}", self.seller);
        println!("Item: {}", self.name);
        println!("Price: {}", self.price);
        println!("---------------------");
    }

    pub fn is_pet(&self) -> bool {
        let is_pet = self.name.contains("[Lvl");
        return is_pet;
    }

    // [Lvl 1] Snail
    // 1]
    // 1
    pub fn get_pet_level(&self) -> i32 {
        let vec: Vec<&str> = self.name.split(" ").collect();
        let num = vec.get(1).unwrap().replace("]", "").parse().unwrap();
        return num;
    }

    pub fn get_volume_id(&self) -> String {
        if self.is_pet() {
            let pet_id = "PET_".to_owned()
                + &self.name.split("] ").collect::<Vec<&str>>()[1]
                    .to_uppercase()
                    .replace(" ", "_")
                    .replace("_EGG", "");
            return pet_id;
        }
        return self.id.clone();
    }

    pub fn get_extra_data_id(
        &self,
        recombed: Option<bool>,
        stars: Option<i32>,
        pet_data: Option<(&str, i32)>,
    ) -> String {
        let mut base_id = self.get_volume_id();

        if self.is_pet() {
            match pet_data {
                Some(data) => {
                    base_id.push_str(&format!("_rarity={}_level={}", data.0, data.1));
                }
                None => {
                    base_id.push_str(&format!(
                        "_rarity={}_level={}",
                        self.tier,
                        self.get_pet_level()
                    ));
                }
            }
            return base_id;
        }

        match recombed {
            Some(e) => {
                if e {
                    base_id.push_str("_recomb")
                }
            }
            None => {
                if self.is_recombed() {
                    base_id.push_str("_recomb")
                }
            }
        }

        match stars {
            Some(e) => {
                if e > 0 {
                    base_id.push_str(&format!("_stars={}", e));
                }
            }
            None => {
                if self.is_dungeonized() {
                    base_id.push_str(&format!("_stars={}", self.count_stars()));
                }
            }
        }

        base_id
    }

    // pub fn can_be_dungeonized(&self) -> bool {
    //     return self.category == WEAPON || self.category == ARMOR;
    // }

    // pub fn can_be_recombed(&self) -> bool {
    //     return self.category == WEAPON || self.category == ARMOR || self.category == ACCESSORIES;
    // }

    pub fn is_recombed(&self) -> bool {
        match self.additional_price_data.as_ref() {
            Some(e) => match e.recombed {
                Some(_) => return true,
                None => return false,
            },
            None => {
                // let lore_length = self.lore.len();
                // if lore_length > 50 {
                //     let lore_section = self.lore.split("\n").last().unwrap();
                match self.lore.rfind("§d§l§ka§r §d§l§d§l") {
                    // return true;
                    Some(_) => return true,
                    None => return false,
                }
                // }
                // if self.lore.rfind("§d§l§ka§r §d§l§d§l") > 0 {
                //     return true;
                // }
                // return false;
            }
        }
    }

    pub fn count_stars(&self) -> i32 {
        let stars = self.name.matches("✪").count();
        if stars == 0 {
            return 0;
        }
        if stars <= 5 {
            return stars as i32;
        }
        // if self.name.contains("➊") {
        //     return 6;
        // }
        // if self.name.contains("➋") {
        //     return 7;
        // }
        // if self.name.contains("➌") {
        //     return 8;
        // }
        // if self.name.contains("➍") {
        //     return 9;
        // }
        // if self.name.contains("➎") {
        //     return 10;
        // }
        return 0;
    }
    pub fn is_dungeonized(&self) -> bool {
        match self.additional_price_data.as_ref() {
            Some(e) => match e.stars {
                Some(_) => return true,
                None => return false,
            },
            None => {
                if self.count_stars() > 0 {
                    return true;
                }
                return false;
            }
        }
    }

    fn find_ultimate_enchant(&self) -> i32 {
        let mut total = 0;
        if self.has_ultimate_enchant() {
            for key in LORE_ENCHANTS.iter() {
                if self.extra.contains(key) {
                    let start = self.lore.find(key).unwrap() + key.len();
                    let lore = &self.lore[start + 1..];
                    let end = lore.find(" ").unwrap();
                    let new_lore = &lore[..end];

                    for roman_num in ROMAN_NUMERALS_ORDERED {
                        match new_lore.find(roman_num) {
                            Some(idx) => {
                                // println!("{} - {}", roman_num, idx);
                                if idx > 5 {
                                    continue;
                                }
                                let enchant_level =
                                    ROMAN_NUMERALS.get(roman_num).unwrap().clone() as usize;
                                total += ENCHANT_PRICES.get(LORE_TO_KEY.get(key).unwrap()).unwrap()
                                    [enchant_level];
                                break;
                            }
                            None => {}
                        }
                    }
                }
            }
        }
        total
    }
    // loop {
    //     if !["I", "V"]
    //         .contains(&self.lore.chars().nth(end).unwrap().to_string().as_str())
    //     {
    //         end -= 1;
    //     } else {
    //         break;
    //     }
    // }
    // let enchant = &self.lore[start..end];
    // println!("HERE({})", new_lore);
    // self.parse_ultimate_enchant(enchant.to_string());

    pub fn add_enchant_price(&self, num: i32) -> i32 {
        match self.additional_price_data.as_ref() {
            Some(e) => return e.ultimate_enchant_price + num,
            None => {
                // go out and attempt to find enchant and its price
                return num + self.find_ultimate_enchant();
            }
        }
    }
    // ✖
    // \n\n§b and a roman numberal
    // pub fn has_attributes(&self) -> bool {
    //     if self.category != ARMOR || self.category != WEAPON || self.category != MISC {
    //         return false;
    //     }
    //     match self.additional_price_data.as_ref() {
    //         Some(e) => return e.ultimate_enchant_price != 0,
    //         None => {
    //             for key in LORE_TO_KEY.keys() {
    //                 if self.lore.contains(key) {
    //                     return true;
    //                 }
    //             }
    //             return false;
    //         }
    //     }
    // }

    pub fn has_ultimate_enchant(&self) -> bool {
        if ![ARMOR, WEAPON].contains(&&self.category.as_str()) {
            return false;
        }
        match self.additional_price_data.as_ref() {
            Some(e) => {
                return e.ultimate_enchant_price != 0;
            }
            None => {
                for key in LORE_ENCHANTS.iter() {
                    if self.extra.contains(key) {
                        return true;
                    }
                }
                return false;
            }
        }
    }

    pub fn print(&self) {
        println!("------------------------------------------");
        println!("Name: {}", self.name.on_bright_magenta());
        println!("Id: {}", self.id.cyan());
        println!("Price: {}", self.price.to_string().green());
        println!("Category: {}", self.category);
        println!("Auction Id: {}", self.uuid.yellow());
        println!("Seller: {}", self.seller.yellow());
        println!("Bytes: {}", self.item_bytes.to_string().bright_black());
        println!("Lore: {}", self.lore.bright_black());
        println!(
            "Enchant Price: {}",
            self.add_enchant_price(0).to_string().yellow()
        );
        println!("Has Ultimate Enchant {}", self.has_ultimate_enchant());
        println!("Find Ultimate Enchant {}", self.find_ultimate_enchant());
        println!("------------------------------------------\n");
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PriceData {
    pub recombed: Option<i32>,
    pub fpb_price: i32,
    pub stars: Option<i32>,
    pub ultimate_enchant_price: i32,
    pub attribute_price: i32,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum FlipType {
    LBin,
    Target,
}
impl FlipType {
    pub fn to_string(&self) -> String {
        return match self.clone() {
            FlipType::LBin => "LBin".to_string(),
            FlipType::Target => "Target".to_string(),
        };
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Flip {
    pub id: String,
    pub internal_id: String,
    pub flip_type: FlipType,
    pub flip_price: i32,
    pub profit_percentage: i32, // not in decimal x100
    pub lowest_bin: Auction,
    pub next_lowest_bin: Auction,
    pub similar_auctions: i32,
    pub volume_last_week: i32,
    pub estimated_target_price: i32,
    pub target_name: String,
    // pub raw_price: u64,
    // pub avg_raw_price: u64,
    // pub fail_reason: Option<String>,
}

impl Flip {
    pub fn print(&self) {
        // let raw_price = utils::prettify_large_nums(self.raw_price.try_into().unwrap());
        // let est_raw_price = utils::prettify_large_nums(self.avg_raw_price.try_into().unwrap());
        // // This means that this flip would cause you to flip the item then more than it is normally worth
        // // Could mean over bidding - indicated by red
        // if self.lowest_bin.price + self.flip_price as u64 > self.avg_raw_price {
        // if self.fail_reason != None {
        //     println!("{} - {}", self.lowest_bin.name.underline().red(), self.fail_reason.as_ref().unwrap().red());
        // }
        // else {
        let colored_name = utils::rarity_to_color(self);
        println!("{} ({})", colored_name, self.flip_type.to_string());
        // }
        println!(
            "Estimated Profit: {}",
            format!(
                "+{} ({}%)",
                utils::prettify_large_nums(self.flip_price),
                self.profit_percentage
            )
            .green()
        );
        // println!("Profit: {}%", self.profit_percentage.to_string().green());
        println!(
            "Price: {} -> {}",
            utils::prettify_large_nums(self.lowest_bin.price.try_into().unwrap()).yellow(),
            utils::prettify_large_nums(self.next_lowest_bin.price).bright_black()
        );
        if self.flip_type == FlipType::LBin {
            println!(
                "Est. Value: {}",
                utils::prettify_large_nums(self.estimated_target_price.try_into().unwrap())
                    .yellow()
            );
        } else {
            println!(
                "Lowest BIN: {}",
                utils::prettify_large_nums(self.estimated_target_price.try_into().unwrap())
                    .yellow()
            );
        }
        // println!("Est. Raw Price: {} < {}", raw_price.green(), est_raw_price.green());
        println!(
            "Similar Auctions: {}",
            self.similar_auctions.to_string().cyan()
        );
        println!(
            "Volume (week): {}",
            self.volume_last_week.to_string().cyan()
        );
        println!("Internal Id: {}", self.internal_id);
        println!(
            "Cmd: {} {}",
            "/viewauction".italic().bright_black(),
            self.lowest_bin.uuid.italic().bright_black()
        );
        println!("")
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Pet {
    pub name: String,
    pub level: i32,
    pub tier: String,
    pub item: String,
    pub candy: i32,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Sale {
    pub id: String,
    pub data: SaleData,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SaleData {
    pub price: i32,
    pub timestamp: i32,
    // pub buyer: String,
}

// NBT structs
#[derive(Deserialize, Debug)]
pub struct PartialNbt {
    pub i: Vec<PartialNbtElement>,
}

#[derive(Deserialize, Debug)]
pub struct PartialNbtElement {
    #[serde(rename = "Count")]
    pub count: i32,
    pub tag: PartialTag,
}

#[derive(Deserialize, Debug)]
pub struct PartialTag {
    #[serde(rename = "ExtraAttributes")]
    pub extra_attributes: PartialExtraAttr,
    pub display: DisplayInfo,
}

// {TAG_String('petInfo'): {"type":"SILVERFISH","active":false,"exp":7979388.534995773,"tier":"LEGENDARY","hideInfo":false,"candyUsed":0,"skin":"SILVERFISH","uuid":"19be0d69-3ccd-450e-81da-779e96f8b9c6"}, TAG_String('id'): PET, TAG_String('uuid'): 19be0d69-3ccd-450e-81da-779e96f8b9c6, TAG_String('timestamp'): 4/19/22 1:34 PM}
#[derive(Deserialize, Debug)]
pub struct PartialExtraAttr {
    #[serde(rename = "petInfo")]
    pub pet_info: Option<String>,
    pub id: Option<String>,
    pub enchantments: Option<HashMap<String, i32>>,
    pub attributes: Option<HashMap<String, i32>>,
    pub modifier: Option<String>,
    pub item_tier: Option<i32>,       // Obtained on floor
    pub dungeon_item: Option<i32>,    // Recombed
    pub rarity_upgrades: Option<i32>, // Recombed
    pub hot_potato_count: Option<i32>,
    pub dungeon_item_level: Option<i32>, // Stars
    pub upgrade_level: Option<i32>,      // Stars
    pub art_of_war_count: Option<i32>,
    pub ability_scroll: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
pub struct DisplayInfo {
    #[serde(rename = "Name")]
    pub name: String,
    // #[serde(rename = "Lore")]
    // pub lore: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(untagged)]
pub enum ItemBytes {
    T0(ItemBytesT0),
    Data(String),
}

impl Into<String> for ItemBytes {
    fn into(self) -> String {
        match self {
            Self::T0(ibt0) => {
                let ItemBytesT0::Data(x) = ibt0;
                x
            }
            Self::Data(x) => x,
        }
    }
}

impl Into<Result<Vec<u8>, Box<dyn std::error::Error>>> for ItemBytes {
    fn into(self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let b64: String = self.into();
        Ok(base64::decode(&b64)?)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(tag = "type", content = "data")]
pub enum ItemBytesT0 {
    #[serde(rename = "0")]
    Data(String),
}
