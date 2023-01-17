use lazy_static::lazy_static;
use std::collections::HashMap;
use std::time::Duration;
use surf;

pub const DEBUG: bool = false;
pub const TIMEOUT: i32 = 200;
// // pub const EXPIRATION_DATE: i32 = 1673239600;
pub const EXPIRATION_DATE: i32 = 1673946161;

// pub const DEBUG: bool = JSON.get("DEBUG").unwrap().parse().unwrap();
// pub const TIMEOUT: i32 = JSON.get("TIMEOUT").unwrap().to_owned().parse().unwrap();
// pub const MIN_PROFIT: u64 = JSON.get("MIN_PROFIT").unwrap().parse().unwrap();
// pub const MIN_PROFIT_PERCENTAGE: usize =
//     JSON.get("MIN_PROFIT_PERCENTAGE").unwrap().parse().unwrap();
// pub const MIN_VOLUME: usize = JSON.get("MIN_VOLUME").unwrap().parse().unwrap();
// pub const MIN_SIMILAR_AUCS: usize = JSON.get("MIN_SIMILAR_AUCS").unwrap().parse().unwrap();
// pub const MAX_BID: u64 = JSON.get("MAX_BID").unwrap().parse().unwrap();
// pub const PURCHASE_X: f64 = JSON.get("PURCHASE_X").unwrap().parse().unwrap();
// pub const PURCHASE_Y: f64 = JSON.get("PURCHASE_Y").unwrap().parse().unwrap();
// pub const CONFIRM_PURCHASE_X: f64 = JSON.get("CONFIRM_PURCHASE_X").unwrap().parse().unwrap();
// pub const CONFIRM_PURCHASE_Y: f64 = JSON.get("CONFIRM_PURCHASE_Y").unwrap().parse().unwrap();
pub const MIN_PROFIT: i32 = 750_000;
// pub const MIN_PROFIT: u64 = 750_000;
pub const MIN_PROFIT_PERCENTAGE: i32 = 3;
pub const MIN_SIMILAR_AUCS: i32 = 3;
pub const MIN_VOLUME: i32 = 300;
pub const MAX_BID: i32 = 50_000_000;

pub const CLOSE_CHAT: &str = "ESC";

// rugbeatuno
pub const PURCHASE_X: f64 = 721.0;
pub const PURCHASE_Y: f64 = 387.0;
pub const CONFIRM_PURCHASE_X: f64 = 650.0;
pub const CONFIRM_PURCHASE_Y: f64 = 373.0;

// pub mut PURCHASE_X: f64 = 0.0;
// pub let mut PURCHASE_Y: f64 = 0.0;
// pub let mut CONFIRM_PURCHASE_X: f64 = 0.0;
// pub let mut CONFIRM_PURCHASE_Y: f64 = 0.0;

// lunarv4
// pub const PURCHASE_X: f64 = 1280.0;
// pub const PURCHASE_Y: f64 = 640.0;
// pub const CONFIRM_PURCHASE_X: f64 = 1180.0;
// pub const CONFIRM_PURCHASE_Y: f64 = 615.0;

// cqrrots
// pub const PURCHASE_X: f64 = 445.0;
// pub const PURCHASE_Y: f64 = 226.0;
// pub const CONFIRM_PURCHASE_X: f64 = 480.0;
// pub const CONFIRM_PURCHASE_Y: f64 = 235.0;
// pub const PURCHASE_X: f64 = 890.0;
// pub const PURCHASE_Y: f64 = 452.0;
// pub const CONFIRM_PURCHASE_X: f64 = 960.0;
// pub const CONFIRM_PURCHASE_Y: f64 = 470.0;

pub const BLACKLISTED_ITEMS: [&str; 39] = [
    "Frozen Blaze",
    "Of Divan",
    "'s Helmet",
    "Mastiff",
    "Reaper Mask",
    "Necromancer Lord Helmet",
    "Shadow Assassin Helmet",
    "Bonemerang",
    "Diver's",
    "Flaming Fist",
    "Delirium Necklace",
    "Aspect of the Void",
    "Ember Rod",
    "Reaper Scyth",
    "Golden Bonzo",
    "Golden Scarf",
    "Wither Helmet",
    "Soul Whip",
    "Final Destination",
    "Bonzo's Mask",
    "Perfect Helmet",
    "Perfect Chestplate",
    "Perfect Leggings",
    "Perfect Boots",
    "Corleonite",
    "Midas",
    "Cake Soul",
    "Revenant",
    "Tarantula Helmet",
    "Tarantula Chestplate",
    "Tarantula Leggings",
    "Tarantula Boots",
    "Reaper Chestplate",
    "Reaper Leggings",
    "Reaper Boots",
    "Horseman's Candle",
    "Nutcracker",
    "null",
    "Null",
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
pub const ROMAN_NUMERALS_ORDERED: [&str; 10] =
    ["VIII", "VII", "VI", "IV", "V", "III", "II", "IX", "I", "X"];

pub const IGNORE_AUCTION_MIN: i32 = 3;

pub const BLOCKS: &str = "blocks";
pub const MISC: &str = "misc";
pub const ARMOR: &str = "armor";
pub const WEAPON: &str = "weapon";
pub const ACCESSORIES: &str = "accessories";
pub const CONSUMABLES: &str = "consumables";

pub const COMMON: &str = "COMMON";
pub const UNCOMMON: &str = "UNCOMMON";
pub const RARE: &str = "RARE";
pub const EPIC: &str = "EPIC";
pub const LEGENDARY: &str = "LEGENDARY";
pub const SPECIAL: &str = "SPECIAL";
pub const VERY_SPECIAL: &str = "VERY_SPECIAL";
pub const MYTHIC: &str = "MYTHIC";
pub const SUPREME: &str = "SUPREME";
pub const DIVINE: &str = "DIVINE";
// pub const SECONDS_IN_WEEK: u64 = 604_800;
// pub const SECONDS_IN_MONTH: u64 = SECONDS_IN_WEEK * 4;

pub const PATH_TO_CONVERSIONS: &str = "./files/id_conversions.json";
pub const PATH_TO_FLIPS: &str = "./files/flips.json";
// pub const PATH_TO_SALES: &str = "./files/sales.json";
// pub const PATH_TO_TARGETS: &str = "./files/targets.json";
pub const PATH_TO_NEXT_UPDATE_FILE: &str = "./files/next_update.txt";
pub const PATH_TO_ENCHANT_RARITIES: &str = "./files/enchant_rarities.json";
// pub const PATH_TO_ESSENCE_TABLE: &str = "./files/essence_table.json";
pub const PATH_TO_VOLUME: &str = "./files/volume.json";
pub const PATH_TO_VOLUME_COPY: &str = "./files/volume copy.json";
pub const PATH_TO_AVG_LBIN: &str = "./files/avg_lbin.json";
// pub const PATH_TO_PREFERENCES: &str = "./files/preferences.json";

pub const FUMING_POTATO_BOOK: i32 = 750000;
lazy_static! {
    pub static ref HTTP_CLIENT1: surf::Client = surf::Config::new()
        .set_timeout(Some(Duration::from_secs(TIMEOUT as u64)))
        .try_into()
        .unwrap();
    pub static ref ENCHANT_PRICES: HashMap<String, Vec<i32>> = HashMap::from([
        (
            "ultimate_legion".to_string(),
            vec![0, 1000000, 2000000, 4000000, 8000000, 16000000],
        ),
        (
            "ultimate_wisdom".to_string(),
            vec![0, 150000, 300000, 600000, 1200000, 2400000],
        ),
        (
            "ultimate_last_stand".to_string(),
            vec![0, 150000, 300000, 600000, 1200000, 2400000],
        ),
        (
            "ultimate_wise".to_string(),
            vec![0, 125000, 250000, 500000, 1000000, 2000000],
        ),
        (
            "ultimate_soul_eater".to_string(),
            vec![0, 1000000, 2000000, 4000000, 8000000, 16000000],
        ),
        (
            "ultimate_one_for_all".to_string(),
            vec![0, 3000000, 3000000, 3000000, 3000000, 3000000],
        ),
        (
            "growth".to_string(),
            vec![0, 0, 0, 0, 0, 0, 1000000, 70000000]
        ),
        (
            "protection".to_string(),
            vec![0, 0, 0, 0, 0, 0, 1000000, 15000000]
        ),
        ("sugar_rush".to_string(), vec![0, 0, 500000, 1000000]),
        (
            "hecatomb".to_string(),
            vec![
                0, 1000000, 1000000, 1000000, 1000000, 1000000, 1250000, 1500000, 2000000, 2500000,
                3000000
            ]
        ),
        (
            "champion".to_string(),
            vec![
                0, 1000000, 1000000, 1000000, 1000000, 1000000, 1000000, 1000000, 1000000, 1000000,
                1000000
            ]
        ),
    ]);
    pub static ref ROMAN_NUMERALS: HashMap<String, i32> = HashMap::from([
        ("I".to_string(), 1),
        ("II".to_string(), 2),
        ("III".to_string(), 3),
        ("IV".to_string(), 4),
        ("V".to_string(), 5),
        ("VI".to_string(), 6),
        ("VII".to_string(), 7),
        ("VIII".to_string(), 8),
        ("IX".to_string(), 9),
        ("X".to_string(), 10),
    ]);
    pub static ref LORE_ENCHANTS: [String; 11] = [
        "Ultimate Wise".to_string(),
        "Legion".to_string(),
        "lWisdom".to_string(),
        "Last Stand".to_string(),
        "Soul Eater".to_string(),
        "One For All".to_string(),
        "Champion".to_string(),
        "Growth".to_string(),
        "Hecatomb".to_string(),
        "Protection".to_string(),
        "Sugar Rush".to_string(),
    ];
    pub static ref LORE_TO_KEY: HashMap<String, String> = HashMap::from([
        (LORE_ENCHANTS[0].clone(), "ultimate_wise".to_string()),
        (LORE_ENCHANTS[1].clone(), "ultimate_legion".to_string()),
        (LORE_ENCHANTS[2].clone(), "ultimate_wisdom".to_string()),
        (LORE_ENCHANTS[3].clone(), "ultimate_last_stand".to_string()),
        (LORE_ENCHANTS[4].clone(), "ultimate_soul_eater".to_string()),
        (LORE_ENCHANTS[5].clone(), "ultimate_one_for_all".to_string()),
        (LORE_ENCHANTS[6].clone(), "champion".to_string()),
        (LORE_ENCHANTS[7].clone(), "growth".to_string()),
        (LORE_ENCHANTS[8].clone(), "hecatomb".to_string()),
        (LORE_ENCHANTS[9].clone(), "protection".to_string()),
        (LORE_ENCHANTS[10].clone(), "sugar_rush".to_string()),
    ]);
}
