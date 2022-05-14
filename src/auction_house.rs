use std::str;
use std::time::{Instant};
use futures::{stream::FuturesUnordered, StreamExt};
use std::collections::HashMap;
use std::iter::Iterator;
use std::fs;

use crate::structs::*;
use crate::constants;

use std::{result::Result as StdResult};
use std::time::{SystemTime, UNIX_EPOCH};

use cli_clipboard;



extern crate serde_json;
extern crate serde_derive;
extern crate base64;
extern crate json;

// "uuid": "3d487e46a2a54f6d886fd314d2ec7e44",
// "auctioneer": "2c7ae7c461804ccd89f05809d02d333c",
// "profile_id": "a849635dd1e14aa1b591ed045679ef3d",
// "coop": [
//     "14d7762e1edf4faba4bd6cea302fb25f",
//     "2c7ae7c461804ccd89f05809d02d333c"
// ],
// "start": 1649548542912,
//          1650485224000
// "end": 1649721342912,
// "item_name": "Enchanted Hopper",
// "item_lore": "\u00a77This item can be placed inside\n\u00a77any minion. Automatically sells\n\u00a77generated items when the minion\n\u00a77has no space. Items are sold for\n\u00a77\u00a7a90.0% \u00a77of their selling\n\u00a77price.\n\n\u00a79\u00a7lRARE",
// "extra": "Enchanted Hopper Hopper",
// "category": "blocks",
// "tier": "RARE",
// "starting_bid": 1100000,
// "item_bytes": "H4sIAAAAAAAAAC2R3U7bQBCFJ4EW2xdtpT4Aw08vEyJDGsxdBEZBomkUcV+N7Ym9kr1r7a4FeRVeIO+RB0OMo+7V7pnvnBnNRgAhDFQEAIMhDFUB7wP4cm867QcRHHkqQzhmnVfQnyMIF6rgx5pKJ8+PCE4K5dqatkI9G8uBqAGc7Xezl0o5VJ4bzEljxihUzgUq7SQBzgUhvcVGaWX0GOedNw15lVNdb9FxXbsDU7JmS743SpbD14o1+or/G+FCmIocaoOulQZjfDpwZBmdqQvcGAunAu13lEzGk18od7PpI5Q99FG6hFDE1iqxy/zRfpfsd/V6vk4DOF5Sw/BTpFSWQLqfZGHali1E8D1985bm3luVdZ5dcFjgj3R5v5gvX9KHf4u/q1W6lpSuk8JlPJne5nGejTbXyXR0kxez0e31Zjb6HWcZ50WWTIppAKFXDTtPTQvfbq6SqzjG2d00xtUfgCF8faCGSu5/4xN3GRaHvAEAAA==",
//H4sIAAAAAAAAAF1U3W7iRhQ+hM2uQ1eKqkr9VatplUiJUjY2xhDnjgALzgZIAgmBm2gYH4PBP9Qebxbu+gC9702l3vECfQIepQ9S9RiU7aqWpZn5zndmzjfnnMkB7EHGzQFAZgd2XDvzWwZ2q2ESyEwOspKP9+AFBmIC6ZeFvaZr41uPj2Na/pODXHeWeF7nKcBIgR3LhoOi6QhnpGPe5mXM647N86OiY+b1su0YXJw5I1snv+sonGMkXYz3QJH4QSYRxpswFNi9516C8BcuLtXhw0S1Hy49sbBKtO51Va9jTedlK7hfjKpWyfLJ3qyUrhbmJ1xD8r7hDfTLyTC4SUb+vXql33rYvNWEf/e+U/NmrZpQB/0bYzCdGe1lvdheet5g2lq0+5ezYWPot5YDvd0f6K2ppXZqd4WBf1MY0tjpD6edxg1h7Wmr11oOa2LZqlW0YfXSdB7Ud6QgB69sN557fEF3dxVGqBD4BfywXpUbyCPWFYSds/XK1o0iDWdHZdM8hi/JXo1cyaoTHogNQZzoh3DwjNe4z8fPuHm48TwxtTeF0uExHBGridyTk5TAT7SCSaN5dFIwjrdMXdXenJW2zBo6GMS4pZ6V/scsELNQPKaoNTI0ovBJTtg9TX9OiZQ5iUK6YfAf1puEURAzy7LI6bv1qvSWqoJ1UbKLMEjic9adoYcyjOCn9BYiHsh4o0P7+49fWVdGGIzpDLLxwN4opk3J+Puf7BPpqd0JI4bvMVrAtxvWSFNT+Hl/NnM9L63NQwLrXEzY3EWBbLw9kqeCtdQhRhHSUd/TVIShZ4dPAYvQTrbC6B+FAcJnZKZESjcY/0ibfk3e1xg5JJ8ZBUNV2el2hG/WK2O98urXVpXV7tqNeqfNmvWrVr2nwIs29zEN1+i5kgeu+HgbrImej5IqZr/+QUa8ImXkjhKJsQKf20kwxjB4jFNJjxH+Aq+rlV6l2mlddM+1Yha+GvEYu5LLizCMJcUlMKCGxVS9Aoof2q7jYgSv5PZYJe1v2O++q1/Ve53bx218OXid9jddj0/ucRZeyk0y03bPQm7+MdsE7JJ1vCmHdAEkLUloywMslUxhGCLPz2wtXzQNPW/aRZE3SmVhlFXbtG2d3g5Xov8o05DIW4E96foYS+7PYb94qpmnhQIzzzWNXbcAduDlNun06MC/dQ/PvaYEAAA\u003d
// "claimed": false,
// "claimed_bidders": [],
// "highest_bid_amount": 0,
// "last_updated": 1649548542912,
// "bin": true,
// "bids": [],
// "item_uuid": "2058c2cbf3954cd783f762bbecdb90d5"
const BLACKLISTED_ITEMS: [&str; 60] = ["Dirt Bottle", "Gift the Fish", "Spooky Pie", "Spirit Leap", "Beach Ball", "Kat Flower", "Endstone Rose", "Teleport Pad", "Plumber's Sponge", "Block of", "Superboom TNT", "Budget Hopper", "Trap", "Decoy", "Undead Bow", "null", "Training Dummy", "Midas Staff", "Bingo Card", "Null", "Hardened Diamond", "Shiny", "Maxor's", "Emerald Armor", "Ember Rod", "Super Heavy", "Holy Dragon", "Very Wise", "Odd Conjuring", "Stereo Pants", "Miner Helment", "Miner Chestplate", "Miner Leggings", "Miner Boots", "Shrimp the Fish", "Corleonite", "Young Dragon", "Unstable Dragon", "Old Dragon", "Protector Dragon", "Potion", "Skeleton Lord Helmet", "Bedrock", "Midas' Sword", "Repelling Candle", "Cake Soul", "Wild Strawberry Dye", "Tuxedo", "Shrub", "Anvil", "Perfect Helmet", "Perfect Chestplate", "Perfect Leggings", "Perfect Boots", "Rune", "Horse Armor", "Flame Ring", "Cobblestone Wall", "Crab Hat of Celebration", "Map"];
const ROMAN_NUMERALS: [&str; 10] = [" I\n", " II\n", " III\n", " IV\n", " V\n", " VI\n", " VII\n", " VIII\n", " IX\n", " X\n"];
const SECONDS_IN_WEEK: u64 = 604_800;
const IGNORE_AUCTION_MIN: u64 = 7;

const PATH_TO_CONVERSIONS: &str = "./src/files/id_conversions.json";
const PATH_TO_FLIPS: &str = "./src/files/flips.json";
const PATH_TO_SALES: &str = "./src/files/sales.json";


fn to_nbt(item_bytes: ItemBytes) -> Result<PartialNbt, Box<dyn std::error::Error>> {
    let bytes: StdResult<Vec<u8>, _> = item_bytes.into();
    let nbt: PartialNbt = nbt::from_gzip_reader(std::io::Cursor::new(bytes?))?;
    Ok(nbt)
}


fn is_pet(nbt: &PartialNbtElement, name: String) -> Option<Pet> {
    if nbt.tag.extra_attributes.pet_info.is_none() {
        return None;
    }

    // Is a pet
    let res = serde_json::from_str(&nbt.tag.extra_attributes.pet_info.as_ref().unwrap());
    let json: serde_json::Value = res.unwrap();

    let level;
    if name == "" {
        let exp = json["exp"].as_f64().unwrap() as u64;
        level = calc_pet_level(exp, json["tier"].as_str().unwrap().to_string());
    } 
    else {
        // [Lvl 100] Tiger 
        // 100] - > 100
        let split = name.split(" ");
        let vec: Vec<&str> = split.collect();
        level = vec[1].to_string().replace("]", "").parse::<u64>().unwrap();
    }

    let pet = Pet {
        name: json["type"].as_str().unwrap().to_string(),
        level: level,
        tier: json["tier"].as_str().unwrap().to_string(),
        item: if json["heldItem"].as_str() != None {json["heldItem"].as_str().unwrap().to_string()} else {"".to_string()},
        candy: if json["candyUsed"].as_u64() != None {json["candyUsed"].as_u64().unwrap()} else {0},
    };
    return Some(pet);
}


fn calc_pet_level(xp: u64, tier: String) -> u64 {
    let mut offset: HashMap<String, u64> = HashMap::new();
    offset.insert("COMMON".to_string(), 0);
    offset.insert("UNCOMMON".to_string(), 6);
    offset.insert("RARE".to_string(), 11);
    offset.insert("EPIC".to_string(), 16);
    offset.insert("LEGENDARY".to_string(), 20);
    offset.insert("MYTHIC".to_string(), 20);

    let pet_levels: Vec<u64> = vec![100, 110, 120, 130, 145, 160, 175, 190, 210, 230, 250, 275, 300, 330, 360, 400, 440, 490, 540, 600, 660, 730, 800, 880, 960, 1050, 1150, 1260, 1380, 1510, 1650, 1800, 1960, 2130, 2310, 2500, 2700, 2920, 3160, 3420, 3700, 4000, 4350, 4750, 5200, 5700, 6300, 7000, 7800, 8700, 9700, 10800, 12000, 13300, 14700, 16200, 17800, 19500, 21300, 23200, 25200, 27400, 29800, 32400, 35200, 38200, 41400, 44800, 48400, 52200, 56200, 60400, 64800, 69400, 74200, 79200, 84700, 90700, 97200, 104200, 111700, 119700, 128200, 137200, 146700, 156700, 167700, 179700, 192700, 206700, 221700, 237700, 254700, 272700, 291700, 311700, 333700, 357700, 383700, 411700, 441700, 476700, 516700, 561700, 611700, 666700, 726700, 791700, 861700, 936700, 1016700, 1101700, 1191700, 1286700, 1386700, 1496700, 1616700, 1746700, 1886700];
    let offset = offset.get(&tier).unwrap().clone();
    let mut xp = xp;
    let mut level = 1;

    for i in offset..pet_levels.len().try_into().unwrap() {
        let req_xp: u64 = pet_levels[i as usize];
        if xp >= req_xp {
            xp -= req_xp;
            level += 1;
        }
    }
    level

}

fn calc_additional_price(nbt: &PartialNbtElement) -> Option<PriceData> {
    let is_recombed = match nbt.tag.extra_attributes.rarity_upgrades.as_ref() {
        Some(_) => 1,
        None => 0,
    };
    let mut hot_potato_books = 0;
    let mut fuming_potato_books = 0;
    match nbt.tag.extra_attributes.hot_potato_count.as_ref() {
        Some(e) => {
            hot_potato_books = if e <= &10 {*e} else {10};
            fuming_potato_books = if e <= &10 {0} else {e-10};
        }
        None => {}
    };


    let stars = match nbt.tag.extra_attributes.dungeon_item_level.as_ref() {
        Some(e) => *e,
        None => 0,
    };

    let mut ah_lookups = vec![];
    match nbt.tag.extra_attributes.enchantments.as_ref() {
        Some(e) => {
            for (enchant, level) in e {
            let enchant = format!("{}_{}", enchant, level);
            ah_lookups.push(enchant);
            }
        }
        None => {}
    };

    match nbt.tag.extra_attributes.art_of_war_count.as_ref() {
        Some(_) => {ah_lookups.push(String::from("THE_ART_OF_WAR"))},
        None => {}
    };

    match nbt.tag.extra_attributes.ability_scroll.as_ref() {
        Some(e) => {
            for scroll in e {
                ah_lookups.push(scroll.to_string());
            }
        },
        None => {}
    };

    let price_data = PriceData {
        recombed: is_recombed,
        hpb: hot_potato_books.try_into().unwrap(),
        fpb: fuming_potato_books.try_into().unwrap(),
        stars: stars.try_into().unwrap(),
        ah_lookups: ah_lookups,
    };

    Some(price_data)
    
}


fn default_case(filtered_auctions: &mut Vec<Auction>, conversion_table: &mut HashMap<String, String>, auction: &mut Auction, unparsed_auction: &serde_json::Value){
    if conversion_table.contains_key(&auction.name) {
        auction.id = conversion_table.get(&auction.name).unwrap().to_string();
    } 
    else {
        let nbt = &to_nbt(serde_json::from_value(unparsed_auction.get("item_bytes").unwrap().to_owned()).unwrap()).unwrap().i[0];
        auction.id = nbt.tag.extra_attributes.id.as_ref().unwrap().to_string();
        conversion_table.insert(auction.name.clone(), auction.id.clone());
    }
    filtered_auctions.push(auction.clone());
}

fn book_case(filtered_auctions: &mut Vec<Auction>, conversion_table: &mut HashMap<String, String>, auction: &mut Auction, unparsed_auction: &serde_json::Value) {
    // guard condition - removes common and uncommon enchants (default basic enchants)
    if auction.lore.contains("COMMON") || auction.lore.contains("UNCOMMON") {
        return;
    }
    // guard condition - removes any enchanted book with more than 2 enchants
    let total_enchants = ROMAN_NUMERALS.iter().filter(|&num| auction.lore.contains(num) == true).count();
    if total_enchants > 1 {
        return;
    }
    // Only books remaining have 1 enchant and are non-basic/default level enchants

    let g: Vec<&str> = auction.lore.split("\n").collect();
    let enchant = g[0];
    let name = enchant.replace("§9", "").to_lowercase();

    if conversion_table.contains_key(&name) {
        let id = conversion_table.get(&name).unwrap().to_string();
        auction.id = id.clone();
        auction.name = id.clone();
    } 
    else {
        let nbt = &to_nbt(serde_json::from_value(unparsed_auction.get("item_bytes").unwrap().to_owned()).unwrap()).unwrap().i[0];
        let enchants = nbt.tag.extra_attributes.enchantments.as_ref().unwrap();
        for (k, v) in enchants {
            let new_id = format!("{}_{}", k, v);
            auction.id = new_id.clone();
            auction.name = new_id.clone();
        }
        
        conversion_table.insert(name, auction.id.clone());
    }

    
    filtered_auctions.push(auction.clone());
}

fn pet_case(filtered_auctions: &mut Vec<Auction>, auction: &mut Auction) {
    if auction.tier == "COMMON" || auction.tier == "UNCOMMON" || auction.tier == "RARE" {
        return;
    }
    // Candied pet prices fluctuate
    if auction.lore.contains("Pet Candy Used") { 
        return;
    }
    // Guards against skinned pets
    if auction.name.contains("✦") {
        return;
    }

    // println!("{:?}", auction.name);

    let pet_level: u64 = if auction.name.contains("100") {100} else {1};
    let name_split: Vec<&str> = auction.name.split(" ").collect();
    let pet_name = name_split[2].clone().to_uppercase();
    let tier_boosted = if auction.lore.contains("Tier Boost") {true} else {false};

    // let nbt = &to_nbt(serde_json::from_value(unparsed_auction.get("item_bytes").unwrap().to_owned()).unwrap()).unwrap().i[0];
    // auction.pet_data = is_pet(nbt, unparsed_auction.get("item_name").unwrap().as_str().unwrap().to_string());
    // let pet_data = auction.clone().pet_data.unwrap();
    // auction.id = format!("{} {} {} {} {}", pet_data.tier, pet_data.name, pet_data.level, pet_data.item, pet_data.candy);
    auction.id = format!("{} {} {} {}", auction.tier, pet_name, pet_level, tier_boosted);
    filtered_auctions.push(auction.clone());
}

fn process_page(req: serde_json::Value,  conversion_table: &mut HashMap<String, String>) -> Vec<Auction> {
    // let s = Instant::now();
    let unix_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

    let mut filtered_auctions: Vec<Auction> = Vec::new();

    // let mut holder = false;
    // let page_num = req.get("page").unwrap().as_u64().unwrap();

    if req.get("auctions") != None {
        let page = req.get("auctions").unwrap().as_array().unwrap();
        for auc in page.into_iter() {
            let bin: bool = auc.get("bin").unwrap().as_bool().unwrap();
            let claimed: bool = auc.get("claimed").unwrap().as_bool().unwrap();
            
            // Initial checks to prevent wasting time on already ended or wrong type of auction
            if !bin || claimed {
                continue;
            }

            // More in depth checks
            let name: String = auc.get("item_name").unwrap().as_str().unwrap().to_string();
            let lore: String = auc.get("item_lore").unwrap().as_str().unwrap().to_string();
            let category: String = auc.get("category").unwrap().as_str().unwrap().to_string();
            let start: u64 = auc.get("start").unwrap().as_u64().unwrap() / 1000; // convert unix_timestamp from milliseconds to seconds

            if
            name.contains("Skin") ||
            lore.contains("Cake") ||
            lore.contains("Furniture") ||
            lore.contains("Obtained of Floor") ||
            BLACKLISTED_ITEMS.iter().any(|i| name.contains(i)) {
                continue;
            }

            // There are 5 cetegories - 
            // weapons(not stackable), armor(not stackable), accessories (books and dungeon drops), blocks(mostly crap), tools & misc(pets)
            // This way we know what items are stackable and can save time on quantity.

            // Large optimization: Since most time is spend unzipping this nbt data (90%), we will do everything 
            //                     we can to reduce how many times it needs to run. The solution is to only provide
            //                     the price and id field to save time. This shortcut can be performed because of the large 
            //                     database, id_conversion.json, NAME_VARIATION: ID
            // 
            // We only look at lowest bins of all items and only if that item has been up for less than IGNORE_AUCTION_MIN(7)
            // This means any other auctions older than IGNORE_AUCTION_MIN will not be looked at in detail
            // It is these items that we can save compute time on


            // template
            let mut auction = Auction {
                // id: nbt.tag.extra_attributes.id.as_ref().unwrap().to_string(),
                id: "".to_string(),
                seller: auc.get("auctioneer").unwrap().as_str().unwrap().to_string(),
                uuid: auc.get("uuid").unwrap().as_str().unwrap().to_string(),
                tier: auc.get("tier").unwrap().as_str().unwrap().to_string(),
                name: name.clone(),
                lore: lore,
                price: auc.get("starting_bid").unwrap().as_u64().unwrap(),
                start: start,
                // pet_data: is_pet(nbt, auc.get("item_name").unwrap().as_str().unwrap().to_string()),
                pet_data: None,
                additional_price_data: None,
                // last_update: auc.get("last_updated").unwrap().as_u64().unwrap(),
            };



            // blocks, misc, armor, weapon, accessories, consumables 
            // we will skips blocks
            if (unix_timestamp - start) / 60 > IGNORE_AUCTION_MIN { // if this is a possible flip
                if category != "blocks" { // non stackable (known quantity/count)

                    // pets
                    if name.contains("[Lvl 100]") || name.contains("[Lvl 1]") { 
                        pet_case(&mut filtered_auctions, &mut auction);
                        continue;
                    }

                    // Enchanted books
                    else if name.contains("Enchanted Book") {
                        book_case(&mut filtered_auctions, &mut *conversion_table, &mut auction, &auc);
                        continue;
                    }

                    // Default case
                    default_case(&mut filtered_auctions, &mut *conversion_table, &mut auction, &auc);
                    continue;

                }
                continue;
            }
            // anything that makes it to this point will be looked at fore lowest bin flips because they have been up for less than IGNORE_AUCTION_MIN
            
            // A pet
            // Formatted as TIER NAME LEVEL PET_ITEM CANDY_USED
            // Ex: RARE TIGER 78 DWARF_TURTLE_SHELMET 0
            if name.contains("[Lvl 100]") || name.contains("[Lvl 1]") { 
                // if auction.tier == "COMMON" || auction.tier == "UNCOMMOM" || auction.tier == "RARE" {
                //     continue;
                // }
                // let nbt = &to_nbt(serde_json::from_value(auc.get("item_bytes").unwrap().to_owned()).unwrap()).unwrap().i[0];
                // auction.pet_data = is_pet(nbt, auc.get("item_name").unwrap().as_str().unwrap().to_string());

                // let pet_data = auction.clone().pet_data.unwrap();
                // auction.id = format!("{} {} {} {} {}", pet_data.tier, pet_data.name, pet_data.level, pet_data.item, pet_data.candy);
                // filtered_auctions.push(auction);
                pet_case(&mut filtered_auctions, &mut auction);
                continue;
            }
            else if auction.name.contains("Enchanted Book") {
                // if auction.lore.contains("COMMON") || auction.lore.contains("UNCOMMON") {
                //         continue;
                //     }
                //     // guard condition - removes any enchanted book with more than 2 enchants
                //     let total_enchants = ROMAN_NUMERALS.iter().filter(|&num| auction.lore.contains(num) == true).count();
                //     if total_enchants > 1 {
                //         continue;
                //     }

                //     // Only books remaining have 1 enchant and are non-basic/default level enchants
                //     let nbt = &to_nbt(serde_json::from_value(auc.get("item_bytes").unwrap().to_owned()).unwrap()).unwrap().i[0];
                //     let enchants = nbt.tag.extra_attributes.enchantments.as_ref().unwrap();
                //     for (k, v) in enchants {
                //         let new_id = format!("{}_{}", k, v);
                //         auction.id = new_id.clone();
                //         auction.name = new_id.clone();
                //     }
                //     filtered_auctions.push(auction);
                book_case(&mut filtered_auctions, conversion_table, &mut auction, &auc);
                continue;
            }
            else {
                // holder = true;
                let nbt = &to_nbt(serde_json::from_value(auc.get("item_bytes").unwrap().to_owned()).unwrap()).unwrap().i[0];
                auction.id = nbt.tag.extra_attributes.id.as_ref().unwrap().to_string();
                auction.price /= nbt.count; // scales price for stacked items
                auction.additional_price_data = calc_additional_price(nbt);
                filtered_auctions.push(auction);
            }
        }
    }

    else {
        println!("Page failed!");
    }
    // println!("Page {}: {}", page_num, holder);
    // println!("{:?}", s.elapsed());
    filtered_auctions
}

async fn fetch(url: String) -> serde_json::Value{
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


// Find flips that fit criteria and write those to JSON for future use.
fn find_flips(auctions: &HashMap<String, Vec<Auction>>) {
    let min_profit = 500_000;
    let min_profit_percentage = 15;
    let min_volume = 3;
    // let max_bid = 100_000_000;

    let mut possible_flips: Vec<Flip> = Vec::new();
    let mut failed_flips: Vec<Flip> = Vec::new();
    
    let unix_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

    // Load json database for further comparisson 
    let data = fs::read_to_string(PATH_TO_SALES).expect("Unable to read file");
    let mut load_json: HashMap<String, Vec<SaleData>> = HashMap::new();
    if fs::metadata(PATH_TO_SALES).unwrap().len() != 0 {
        load_json = serde_json::from_str(&data).unwrap();
    }
    for (item_name, auc) in auctions.iter() {
        let mut auc = auc.clone();
        auc.sort_by_key(|f| f.price.clone());

        let volume = auc.len();
        if volume < min_volume {
            continue;
        }

        let mut lowest_price: u64 = auc[0].price;
        let next_lowest_price: u64 = auc[1].price;

        // Guard condition
        if lowest_price == 0 {
            lowest_price = 1;
        }

        // let tax = lowest_price * 0.01 + next_lowest * 0.02;
        if next_lowest_price < lowest_price {continue;}
        
        let mut flip_price: u64 = (next_lowest_price - lowest_price).try_into().unwrap();
        let taxes = (flip_price as f64 * 0.02 + lowest_price as f64 * 0.01) as u64;
        flip_price = if taxes < flip_price {flip_price-taxes} else {flip_price};

        let additional_costs = additional_costs(&auctions, &auc[1].additional_price_data);


        // Calculating avg raw price
        let mut avg_raw_price = 0;
        if load_json.contains_key(item_name) {
            let prices = load_json.get(item_name).unwrap();
            if prices.len() != 0 {
                let mut sum = 0;
                for p in prices {
                    sum += p.price;
                }
                avg_raw_price = sum / (prices.len() as u64);
            }
        }

        let mut flip = Flip {
            flip_price: flip_price.try_into().unwrap(),
            profit_percentage: (flip_price * 100 / lowest_price).try_into().unwrap(),
            lowest_bin: auc[0].clone(),
            next_lowest_bin: auc[1].clone(),
            volume: volume,
            last_hour_volume: match load_json.get(&auc[0].id) {
                Some(sales) => sales.iter().filter(|i| unix_timestamp - i.timestamp < 3600).count(),
                None => 0,
            },
            raw_price: if auc[0].price > additional_costs {auc[0].price - additional_costs} else {auc[0].price},
            avg_raw_price: avg_raw_price,
            fail_reason: None,
            // raw_price: 0,
        };

        if flip.raw_price > avg_raw_price {
            continue;
        }

        if (flip.lowest_bin.price + flip.flip_price as u64) as f32 / flip.avg_raw_price as f32 > 1.2 {
            flip.fail_reason = Some("Possibly Overvalued".to_string());
            failed_flips.push(flip);
            continue;
        }

        // If the seller is me
        if flip.lowest_bin.seller == "1d7a1fd3350546efa653389dfa00ea85" {
            continue;
        }

        // && flip.profit_percentage > min_profit_percentage
        if flip.flip_price > min_profit && flip.profit_percentage > min_profit_percentage && flip.lowest_bin.tier == flip.next_lowest_bin.tier {
            if (unix_timestamp - flip.lowest_bin.start) / 60 < IGNORE_AUCTION_MIN {  // if the auction has been up for more than 10 minutes ignore it (good auctions are sniped withing seconds)
                possible_flips.push(flip);
            }
        }
        
    }

    // If the biggest flip is a reasonable amount recommend it
    // Otherwise, check if there is a huge amount of volume on any items
    // If not, recommend the largest flip
    // possible_flips.sort_by_key(|f| f.flip_price.clone());
    if possible_flips.len() == 0 {
        failed_flips.sort_by_key(|f| f.flip_price.clone());
        for flip in &failed_flips[failed_flips.len() - 3..] {
            flip.print();
        }
        return;
    }

    possible_flips.sort_by_key(|f| f.flip_price.clone());
    if possible_flips[possible_flips.len()-1].flip_price < 3_000_000 {
        possible_flips.sort_by_key(|f| f.volume.clone());
        if possible_flips.len() > 0 && possible_flips[possible_flips.len()-1].volume > 70 {
            open_auction(&possible_flips);
        }
        else {
            possible_flips.sort_by_key(|f| f.flip_price.clone());
            open_auction(&possible_flips);

        }
    }
    open_auction(&possible_flips);
    update_flips(&possible_flips);

    for flip in possible_flips.clone() {
        flip.print();
    }
    
    println!("---------------------------------")
}

// pub async fn get_total_pages() -> u64 {
//     let page_0 = fetch(String::from("https://api.hypixel.net/skyblock/auctions?page=0")).await;
//     let total_pages: u64 = page_0.get("totalPages").unwrap().as_u64().unwrap();
//     return total_pages;
// }

pub async fn update_critical_auctions(auctions_hashed: &mut HashMap<String, Vec<Auction>>, seen_ids: &mut Vec<String>) {
    let mut futures = FuturesUnordered::new();

    // Load conversions json
    let file: String = String::from(PATH_TO_CONVERSIONS);
    let data = fs::read_to_string(&file).expect("Unable to read file");
    let mut conversion_tables: HashMap<String, String> = HashMap::new();
    if fs::metadata(&file).unwrap().len() != 0 {
        conversion_tables = serde_json::from_str(&data).unwrap();
    }

    let _now: Instant = Instant::now();

    futures.push(fetch("https://api.hypixel.net/skyblock/auctions?page=0".to_string()));
    futures.push(fetch("https://api.hypixel.net/skyblock/auctions?page=1".to_string()));
    
    while let Some(res) = futures.next().await {
        let page = process_page(res, &mut conversion_tables);
        for auction in page {
            if seen_ids.contains(&auction.uuid) {
                continue;
            }
            seen_ids.push(auction.uuid.clone());
            auctions_hashed.entry(auction.id.clone()).or_default().push(auction);
        }
    }
    find_flips(&auctions_hashed);
    // println!("\rCritical Auctions Returned in: {:?} ({} auctions)", now.elapsed(), auctions_hashed.len());

    let json: String = serde_json::to_string(&conversion_tables).unwrap();
    fs::write(file, &json).expect("Unable to write file");
  

}

pub async fn update_all_auctions(auctions_hashed: &mut HashMap<String, Vec<Auction>>, seen_ids: &mut Vec<String>) {
    let mut futures = FuturesUnordered::new();

    // Load conversions json
    let file: String = String::from(PATH_TO_CONVERSIONS);
    let data = fs::read_to_string(&file).expect("Unable to read file");
    let mut conversion_tables: HashMap<String, String> = HashMap::new();
    if fs::metadata(&file).unwrap().len() != 0 {
        conversion_tables = serde_json::from_str(&data).unwrap();
    }

    // Load previous pages
    // let prev_pages = get_pa
    println!("\rUpdating Non-essential Auctions!");
    // let now: Instant = Instant::now();

    let page_0 = fetch(String::from("https://api.hypixel.net/skyblock/auctions?page=0")).await;
    let total_pages: u64 = page_0.get("totalPages").unwrap().as_u64().unwrap();
    for page_number in 1..total_pages {
        futures.push(fetch(format!("https://api.hypixel.net/skyblock/auctions?page={}", page_number)));
    }
    
    // Process page zero since it was already fetched to get totalPages
    for auction in process_page(page_0, &mut conversion_tables) {
        auctions_hashed.entry(auction.id.clone()).or_default().push(auction);
    }
    while let Some(res) = futures.next().await {
        let page = process_page(res, &mut conversion_tables);
        for auction in page {
            if seen_ids.contains(&auction.uuid) {
                continue;
            }
            seen_ids.push(auction.uuid.clone());
            auctions_hashed.entry(auction.id.clone()).or_default().push(auction);
        }
    }

    find_flips(&auctions_hashed);

    let json: String = serde_json::to_string(&conversion_tables).unwrap();
    fs::write(file, &json).expect("Unable to write file");
  
    // println!("\rNon-essential Auctions Updated in: {:?} ({} auctions)", now.elapsed(), auctions_hashed.len());

    
    // let data = fetch(1).await;
    // let json = data.get("auctions").unwrap().as_array().unwrap();
    // println!("Data: {}", data);
}

pub async fn update_auctions_ended() {
    let req = fetch(String::from("https://api.hypixel.net/skyblock/auctions_ended")).await;
    let auctions = req.get("auctions").unwrap().as_array().unwrap();
    println!("Auctions ended: {}", auctions.len());

    let mut sales: Vec<Sale> = Vec::new();

    for auc in auctions {
        // Guard case for non-BIN auctions
        if !auc.get("bin").unwrap().as_bool().unwrap() {
            continue;
        }
        let nbt = &to_nbt(serde_json::from_value(auc.get("item_bytes").unwrap().to_owned()).unwrap()).unwrap().i[0];
        let id = nbt.tag.extra_attributes.id.as_ref().unwrap();


        // Convert from milliseconds to seconds
        let mut sale = Sale {
            id: id.to_string(),
            data: SaleData {
                price: auc.get("price").unwrap().as_u64().unwrap(),
                timestamp: auc.get("timestamp").unwrap().as_u64().unwrap() / 1000, 
                // buyer: auc.get("buyer").unwrap().as_str().unwrap().to_string(),
            },
        };

        if id == "PET" {
            let pet_data = is_pet(nbt, "".to_string()).unwrap();
            sale.id = format!("{} {} {} {}", pet_data.tier, pet_data.name, pet_data.level, false);
        }

        else if id == "ENCHANTED_BOOK" {
            match nbt.tag.extra_attributes.enchantments.as_ref() {
                Some(e) => {
                    if e.len() == 1 { 
                        for (enchant, level) in e {
                            sale.id = format!("{}_{}", enchant, level);
                        }
                    }
                }
                None => {}
            };
        }
        
        else {
            
        }
        sales.push(sale);

    }

    update_json(sales);

    // {"auction_id":"3fbfc20a9e484d3a87d40a6d308cb459",
    // "bin":true,
    // "buyer":"e5f012df66824a4ea5d7eba2cadbad23",PDcH/6sN7MjfLgl8TyROlsUfWL+GL25vhY+SFzQ7Z7Q3/3gtdOuPv6GEfPiLnGFMsDa69bgifknG2RJStyW/h/j58Q45pabEo8jmpXIf47jtCnxh36Xn0in53v/1K/69Jw8fkHePSZmxmNSnWbDqdNmQXKCuBhmVqxRaVyNi1qhjXyEyhVijha8LkJbMZshW3FJhc1yk9131InkcN8wQ5+TWbcJJVztmLtUxBhTJr7iyft9yClywlIDwgTIIpVYlZflXH1WYm+YLPkdJAnYBShXQwXkqm11L/B7vfwvywFXSPBJ1qtaK7tjpONS+taeo0DOkQd29/Z207moBdCniqlUVhc1VS0Lf/DfLqIH739he2blMTRVnjZ5nSpWlK+eDfGF7DQ/chHUORcWoSs6oh1ZjQnMnaHzdulb67h6z1wGd1Sbi4Ytw2FefW0hvqR9TCr25voklFN5+hZceqrMwhe5kbZMeFUrK56VGSF7nN64rzN1in8Q+CdR6iayq14CVnQhlbc35CnLc3xZOT05Oz8dHFT+z4/PzZrAtbQhVKw/6ff/Rg84wvED4n5AvU123KseZzKtWxUtZAH3ZPfraaH1karaSyaHqwnShjphL2IjeI0nAUOWIQDpyB58ZOIuOh40aCp6FwvZRHPeibJV+VKCeUdG8YhtIbycAJMAicQewnTux5ieOlsfC9aJAELu1Vb6FknuaoYXNFknrNgn74cjo7uRxfHJ2en102d+nD+/UyU2MWSM3pwn1Zr8GladeAatDtwrZtGtm+7KbtNF+m7TSTdYsg82ak2pf+8p9xqQ30XdisKkq/h8MgoPUdOXwQC2eAQ+GMRrHvRD56Pmn3opHXgx2bL9BYvljCB4MD78D3mXcYRuzpjwAbsD1uJoKI4W898rF2rAQAAA==",
    // "price":1700000,
    // "item_bytes":"H4sIAAAAAAAAAF1U3W7jRBQ+adqSZlkqBAgQPzuCrkSFvPVP7MS9a0nTjbS0q2Z/hFarajxzHFt1PNHMeEPfgCuegL1D6nv0UXrHHU+AOLazILhIxj7nO9/55vy4D7ADnbwPAJ0N2Mhlx+nA1g+qKm2nD13L5zuwiaXI1ohu8aaAboOEHnTg3vMy0civeFJgpws7j3OJk4L
    // "seller":"f1678ca543524f65935840f2a6857a7e",
    // "seller_profile":"989723505eda48bd83566c2c7d5c303b",
    // "timestamp":1650575019125}
    //l
    // {TAG_String('bossId'): 6036f586-c454-4109-bd97-06caf5c01fa6, TAG_String('spawnedFor'): 755d18d3-3e33-492b-911b-1f9c2164b30e, TAG_String('modifier'): wise, TAG_String('id'): WISE_DRAGON_BOOTS, TAG_Compound('enchantments'): {5 Entries}, TAG_String('uuid'): e7331058-a49c-4e7c-8892-62e122161681, TAG_String('timestamp'): 4/1/22 1:56 PM}


    
}

fn update_flips(flips: &Vec<Flip>) {
    let file: String = String::from(PATH_TO_FLIPS);
    let json: String = serde_json::to_string(flips).unwrap();
    fs::write(file, &json).expect("Unable to write file");
}

fn get_item_bins(auctions: &HashMap<String, Vec<Auction>>, item_id: String) -> Vec<Auction> {
    match auctions.get(&item_id) {
        Some(item_auctions) => {
            item_auctions.clone().sort_by_key(|f| f.price.clone());
            return item_auctions.clone();
        }
        None => {vec![]}
    }
}

fn additional_costs(auctions: &HashMap<String, Vec<Auction>>, price_data: &Option<PriceData>) -> u64 {
    match price_data {
        Some(price_data) => {
            let mut sum: u64 = 0;

            let recomb_price = 6_500_000;
            let hpb_price = 30_000;
            let fpb_price = 1_300_000;
        
            sum += recomb_price * price_data.recombed;
            sum += hpb_price * price_data.hpb;
            sum += fpb_price * price_data.fpb;
        
            for item in &price_data.ah_lookups {
                let bins = get_item_bins(&auctions, item.to_string());
                if bins.len() > 0 {
                    sum += bins[0].price;
                }
            }
            return sum;
        }
        None => {return 0}
    }
}

fn update_json(new_sales: Vec<Sale>) {
    let file: String = String::from(PATH_TO_SALES);

    let data = fs::read_to_string(&file).expect("Unable to read file");

    let mut load_json: HashMap<String, Vec<SaleData>> = HashMap::new();
    if fs::metadata(&file).unwrap().len() != 0 {
        load_json = serde_json::from_str(&data).unwrap();
    }

    for sale in new_sales {
        load_json.entry(sale.id).or_default().push(sale.data);
    }

    let json: String = serde_json::to_string(&load_json).unwrap();

    fs::write(file, &json).expect("Unable to write file");
}




pub async fn remove_expired_sales() {
    let unix_timestamp: u64 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

    let file: String = String::from(PATH_TO_SALES);
    let data: String = fs::read_to_string(&file).expect("Unable to read file");

    let mut load_json: HashMap<String, Vec<SaleData>> = HashMap::new();
    if fs::metadata(&file).unwrap().len() != 0 {
        load_json = serde_json::from_str(&data).unwrap();
    }
    // Sale is stored where each sale is a HashMap stored in a Vec (Some idiot)
    for key in load_json.clone().keys() {// [ [{}] ]
        load_json.get_mut(key).unwrap().retain(|sale| unix_timestamp - &sale.timestamp < SECONDS_IN_WEEK);
    }
    let json: String = serde_json::to_string(&load_json).unwrap();

    fs::write(file, &json).expect("Unable to write file");
}

fn open_auction(flips: &Vec<Flip>) {
    if flips.len() > 0 {
        let top_auc = &flips[flips.len()-1];
        let cmd = format!("/viewauction {}", top_auc.lowest_bin.uuid);
        // let current_clipboard = cli_clipboard::get_contents().unwrap();

        cli_clipboard::set_contents(cmd.to_owned()).unwrap();
        assert_eq!(cli_clipboard::get_contents().unwrap(), cmd);
    }
}

