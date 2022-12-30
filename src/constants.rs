use lazy_static::lazy_static;
use std::time::Duration;

use surf;

pub const MIN_PROFIT: u64 = 500_000;
pub const MIN_PROFIT_PERCENTAGE: usize = 3;
pub const MIN_SIMILAR_AUCS: usize = 3;
pub const MIN_VOLUME: usize = 75;
pub const MAX_BID: u64 = 50_000_000;

pub const BLACKLISTED_ITEMS: [&str; 24] = [
    "Gift the Fish",
    "Crab Hat of Celebration",
    "French Bread",
    "null",
    "Midas Staff",
    "Null",
    "Shrimp the Fish",
    "Corleonite",
    "Midas' Sword",
    "Cake Soul",
    "Aspect of the Void",
    "Revanant Helmet",
    "Revanant Chestplate",
    "Revanant Leggings",
    "Revanant Boots",
    "Tarantula Helmet",
    "Tarantula Chestplate",
    "Tarantula Leggings",
    "Tarantula Boots",
    "Reaper Chestplate",
    "Reaper Leggings",
    "Reaper Boots",
    "Final Destination",
    "Of Divan",
];
// pub const BLACKLISTED_TARGETS: [&str; 17] = [
//     "Of Divan",
//     "Aspect of the Void",
//     "Summoning Ring",
//     "Fire Veil Wand",
//     "Gauntlet",
//     "Vanquished Magma Necklace",
//     "Magma Necklace",
//     "Delirium Necklace",
//     "Lava Shell Necklace",
//     "Molten Necklace",
//     "Thunderbolt Necklace",
//     "Ancient Cloak",
//     "Ghast Cloak",
//     "Molten Cloak",
//     "Scourge Cloak",
//     "Vanquished Ghast Cloak",
//     "Belt",
// ];
// pub const ROMAN_NUMERALS: [&str; 10] = [
//     " I\n", " II\n", " III\n", " IV\n", " V\n", " VI\n", " VII\n", " VIII\n", " IX\n", " X\n",
// ];

pub const IGNORE_AUCTION_MIN: u64 = 3;
pub const BLOCKS: &str = "blocks";
pub const MISC: &str = "misc";
pub const ARMOR: &str = "armor";
pub const WEAPON: &str = "weapon";
pub const ACCESSORIES: &str = "accessories";
pub const CONSUMABLES: &str = "consumables";
// pub const SECONDS_IN_WEEK: u64 = 604_800;
// pub const SECONDS_IN_MONTH: u64 = SECONDS_IN_WEEK * 4;

pub const PATH_TO_CONVERSIONS: &str = "./files/id_conversions.json";
pub const PATH_TO_FLIPS: &str = "./files/flips.json";
// pub const PATH_TO_SALES: &str = "./files/sales.json";
pub const PATH_TO_TARGETS: &str = "./files/targets.json";
pub const PATH_TO_NEXT_UPDATE_FILE: &str = "./files/next_update.txt";
pub const PATH_TO_ENCHANT_RARITIES: &str = "./files/enchant_rarities.json";
// pub const PATH_TO_ESSENCE_TABLE: &str = "./files/essence_table.json";
pub const PATH_TO_VOLUME: &str = "./files/volume.json";
pub const PATH_TO_AVG_LBIN: &str = "./files/avg_lbin.json";

lazy_static! {
    pub static ref HTTP_CLIENT1: surf::Client = surf::Config::new()
        .set_timeout(Some(Duration::from_secs(200)))
        .try_into()
        .unwrap();
}
