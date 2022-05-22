use lazy_static::lazy_static;
use std::time::Duration;

extern crate surf;

pub const BLACKLISTED_ITEMS: [&str; 72] = ["Pig Mask", "French Bread", "Bonzo Head", "Scarf Head", "Professor Head", "Thorn Head", "Livid Head", "Large Storage", "Magma Boots", "Magma Leggings", "Magma Chestplate", "Magma Helmet", "Dirt Bottle", "Gift the Fish", "Spooky Pie", "Spirit Leap", "Beach Ball", "Kat Flower", "Endstone Rose", "Teleport Pad", "Plumber's Sponge", "Block of", "Superboom TNT", "Budget Hopper", "Trap", "Decoy", "Undead Bow", "null", "Training Dummy", "Midas Staff", "Bingo Card", "Null", "Hardened Diamond", "Shiny", "Maxor's", "Emerald Armor", "Ember Rod", "Super Heavy", "Holy Dragon", "Very Wise", "Odd Conjuring", "Stereo Pants", "Miner Helment", "Miner Chestplate", "Miner Leggings", "Miner Boots", "Shrimp the Fish", "Corleonite", "Young Dragon", "Unstable Dragon", "Old Dragon", "Protector Dragon", "Potion", "Skeleton Lord Helmet", "Bedrock", "Midas' Sword", "Repelling Candle", "Cake Soul", "Wild Strawberry Dye", "Tuxedo", "Shrub", "Anvil", "Perfect Helmet", "Perfect Chestplate", "Perfect Leggings", "Perfect Boots", "Rune", "Horse Armor", "Flame Ring", "Cobblestone Wall", "Crab Hat of Celebration", "Map"];
pub const ROMAN_NUMERALS: [&str; 10] = [" I\n", " II\n", " III\n", " IV\n", " V\n", " VI\n", " VII\n", " VIII\n", " IX\n", " X\n"];

pub const IGNORE_AUCTION_MIN: u64 = 7;
pub const SECONDS_IN_WEEK: u64 = 604_800;
pub const SECONDS_IN_MONTH: u64 = SECONDS_IN_WEEK * 4;

pub const PATH_TO_CONVERSIONS: &str = "./files/id_conversions.json";
pub const PATH_TO_FLIPS: &str = "./files/flips.json";
pub const PATH_TO_SALES: &str = "./files/sales.json";
pub const PATH_TO_TARGETS: &str = "./files/targets.json";

lazy_static! {
    pub static ref HTTP_CLIENT1: surf::Client = surf::Config::new()
    .set_timeout(Some(Duration::from_secs(15)))
    .try_into()
    .unwrap();
    
    
}