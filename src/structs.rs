use crate::utils;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

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
    pub price: u64,
    pub start: u64,
    pub end: u64,
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
        let is_pet = self.id.contains("-1-") || self.id.contains("-100-");
        return is_pet;
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

    // pub fn get_stars_id(&self, stars: Option<u64>) -> String {
    //     match stars {
    //         Some(s) => return self.get_volume_id() + &format!("_stars={}", s),
    //         None => {
    //             return self.get_volume_id()
    //                 + &format!(
    //                     "_stars={}",
    //                     self.additional_price_data
    //                         .as_ref()
    //                         .unwrap()
    //                         .stars
    //                         .unwrap()
    //                         .to_string()
    //                 );
    //         }
    //     }
    // }

    pub fn get_extra_data_id(&self, recombed: Option<bool>, stars: Option<i32>) -> String {
        let mut base_id = self.get_volume_id();
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
            None => return false,
        }
    }

    fn count_stars(&self) -> i32 {
        let stars = self.name.matches("✪").count();
        if stars < 5 {
            return stars as i32;
        }
        if self.name.contains("➊") {
            return 6;
        }
        if self.name.contains("➋") {
            return 7;
        }
        if self.name.contains("➌") {
            return 8;
        }
        if self.name.contains("➍") {
            return 9;
        }
        if self.name.contains("➎") {
            return 10;
        }
        return 5;
    }
    pub fn is_dungeonized(&self) -> bool {
        match self.additional_price_data.as_ref() {
            Some(e) => match e.stars {
                Some(_) => return true,
                None => return false,
            },
            None => {
                if self.name.contains("✪") {
                    return true;
                }
                return false;
            }
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PriceData {
    pub recombed: Option<u64>,
    pub hpb: u64,
    pub fpb: u64,
    pub stars: Option<u64>,
    pub ah_lookups: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
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
    pub flip_price: usize,
    pub profit_percentage: usize, // not in decimal x100
    pub lowest_bin: Auction,
    pub next_lowest_bin: Auction,
    pub similar_auctions: usize,
    pub volume_last_week: usize,
    pub estimated_target_price: usize,
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
            utils::prettify_large_nums(self.next_lowest_bin.price as usize).bright_black()
        );
        println!(
            "Est. Target: {}",
            utils::prettify_large_nums(self.estimated_target_price.try_into().unwrap()).yellow()
        );
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
        println!("Cmd: /viewauction {}", self.lowest_bin.uuid);
        println!("")
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Pet {
    pub name: String,
    pub level: u64,
    pub tier: String,
    pub item: String,
    pub candy: u64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Sale {
    pub id: String,
    pub data: SaleData,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SaleData {
    pub price: u64,
    pub timestamp: u64,
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
    pub count: u64,
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
    pub modifier: Option<String>,
    pub item_tier: Option<u8>,       // Obtained on floor
    pub dungeon_item: Option<u8>,    // Recombed
    pub rarity_upgrades: Option<u8>, // Recombed
    pub hot_potato_count: Option<u8>,
    pub dungeon_item_level: Option<u8>, // Stars
    pub upgrade_level: Option<u8>,      // Stars
    pub art_of_war_count: Option<u8>,
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
