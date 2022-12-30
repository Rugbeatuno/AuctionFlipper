use serde_json;

use futures::{stream::FuturesUnordered, StreamExt};
use serde_derive::{Deserialize, Serialize};
use std::result::Result as StdResult;
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use std::{collections::HashMap, fs, str};

// use cli_clipboard;

use crate::constants::*;
use crate::inputs::send;
use crate::pets::*;
use crate::structs::*;
use crate::utils::load_json_file;

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

// const PATH_TO_TARGETS: &str = "./src/files/targets.json";

pub fn to_nbt(item_bytes: ItemBytes) -> Result<PartialNbt, Box<dyn std::error::Error>> {
    let bytes: StdResult<Vec<u8>, _> = item_bytes.into();
    let nbt: PartialNbt = nbt::from_gzip_reader(std::io::Cursor::new(bytes?))?;
    Ok(nbt)
}

fn calc_additional_price(nbt: &PartialNbtElement) -> Option<PriceData> {
    // println!("{:?}", nbt.tag.extra_attributes);
    // println!();
    let is_recombed = match &nbt.tag.extra_attributes.rarity_upgrades {
        Some(_) => Some(1),
        None => None,
    };
    // let mut hot_potato_books = 0;
    // let mut fuming_potato_books = 0;
    // match nbt.tag.extra_attributes.hot_potato_count.as_ref() {
    //     Some(e) => {
    //         hot_potato_books = if e <= &10 { *e } else { 10 };
    //         fuming_potato_books = if e <= &10 { 0 } else { e - 10 };
    //     }
    //     None => {}
    // };

    let stars = match &nbt.tag.extra_attributes.dungeon_item_level {
        Some(e) => Some(*e as u64),
        None => match &nbt.tag.extra_attributes.upgrade_level {
            Some(k) => Some(*k as u64),
            None => None,
        },
    };

    let ah_lookups = vec![];
    // match nbt.tag.extra_attributes.enchantments.as_ref() {
    //     Some(e) => {
    //         for (enchant, level) in e {
    //             let enchant = format!("{}_{}", enchant, level);
    //             ah_lookups.push(enchant);
    //         }
    //     }
    //     None => {}
    // };

    // match nbt.tag.extra_attributes.art_of_war_count.as_ref() {
    //     Some(_) => ah_lookups.push(String::from("THE_ART_OF_WAR")),
    //     None => {}
    // };

    // match nbt.tag.extra_attributes.ability_scroll.as_ref() {
    //     Some(e) => {
    //         for scroll in e {
    //             ah_lookups.push(scroll.to_string());
    //         }
    //     }
    //     None => {}
    // };

    let price_data = PriceData {
        recombed: is_recombed,
        hpb: 0,
        fpb: 0,
        stars,
        ah_lookups,
    };

    Some(price_data)
}

fn default_case(
    filtered_auctions: &mut Vec<Auction>,
    conversion_table: &mut HashMap<String, String>,
    auction: &mut Auction,
    unparsed_auction: &serde_json::Value,
) {
    if conversion_table.contains_key(&auction.name) {
        auction.id = conversion_table.get(&auction.name).unwrap().to_string();
    } else {
        let nbt = &to_nbt(
            serde_json::from_value(unparsed_auction.get("item_bytes").unwrap().to_owned()).unwrap(),
        )
        .unwrap()
        .i[0];
        auction.id = nbt.tag.extra_attributes.id.as_ref().unwrap().to_string();
        conversion_table.insert(auction.name.to_string(), auction.id.to_string());
    }
    filtered_auctions.push(auction.to_owned());
}

// fn book_case(
//     filtered_auctions: &mut Vec<Auction>,
//     conversion_table: &mut HashMap<String, String>,
//     rarity_table: &mut HashMap<String, String>,
//     auction: &mut Auction,
//     unparsed_auction: &serde_json::Value,
// ) {
//     // guard condition - removes common and uncommon enchants (default basic enchants)
//     if auction.lore.contains("COMMON") || auction.lore.contains("UNCOMMON") {
//         return;
//     }
//     // guard condition - removes any enchanted book with more than 2 enchants
//     let total_enchants = ROMAN_NUMERALS
//         .iter()
//         .filter(|&num| auction.lore.contains(num) == true)
//         .count();
//     if total_enchants > 1 {
//         return;
//     }
//     // Only books remaining have 1 enchant and are non-basic/default level enchants

//     let g: Vec<&str> = auction.lore.split("\n").collect();
//     let enchant = g[0];
//     let name = enchant.replace("§9", "").to_lowercase();

//     if conversion_table.contains_key(&name) {
//         let id = conversion_table.get(&name).unwrap().to_string();
//         auction.id = id.clone();
//         auction.name = id.clone();
//     } else {
//         let nbt = &to_nbt(
//             serde_json::from_value(unparsed_auction.get("item_bytes").unwrap().to_owned()).unwrap(),
//         )
//         .unwrap()
//         .i[0];
//         let enchants = nbt.tag.extra_attributes.enchantments.as_ref().unwrap();
//         for (k, v) in enchants {
//             let new_id = format!("{}_{}", k, v);
//             auction.id = new_id.clone();
//             auction.name = new_id.clone();
//         }

//         conversion_table.insert(name.clone(), auction.id.clone());
//     }

//     // log enchants that are not basic into rarity_table - using hashmap for speed
//     if !rarity_table.contains_key(&name) {
//         rarity_table.insert(auction.name.clone(), "".to_string());
//     }

//     filtered_auctions.push(auction.clone());
// }

fn process_page(
    req: serde_json::Value,
    conversion_table: &mut HashMap<String, String>,
    avg_lbin_json: &HashMap<String, String>,
    volume_json: &HashMap<String, String>,
) -> Vec<Auction> {
    // let s = Instant::now();
    let unix_timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

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
            let rarity: String = auc.get("tier").unwrap().as_str().unwrap().to_string();
            let category: String = auc.get("category").unwrap().as_str().unwrap().to_string();
            let start: u64 = auc.get("start").unwrap().as_u64().unwrap() / 1000; // convert unix_timestamp from milliseconds to seconds
            let end: u64 = auc.get("end").unwrap().as_u64().unwrap() / 1000; // convert unix_timestamp from milliseconds to seconds

            if category == BLOCKS || category == CONSUMABLES {
                continue;
            }
            // println!("Check 1 {:?}", now.elapsed());
            if (rarity == "COMMON" || rarity == "UNCOMMON") && category != "accessories" {
                continue;
            }

            if category == ARMOR || category == WEAPON {
                if lore.contains("Obtained of Floor") {
                    continue;
                }
            }

            if category == MISC {
                if rarity == "RARE"
                    || name.contains("Skin")
                    || name.contains("Rune")
                    || lore.contains("Furniture")
                {
                    continue;
                }
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
                id: "".to_string(),
                seller: auc.get("auctioneer").unwrap().as_str().unwrap().to_string(),
                uuid: auc.get("uuid").unwrap().as_str().unwrap().to_string(),
                tier: rarity,
                name: name.to_string(),
                lore,
                price: auc.get("starting_bid").unwrap().as_u64().unwrap(),
                start,
                end,
                pet_data: None,
                additional_price_data: None,
                item_bytes: auc.get("item_bytes").unwrap().to_owned(),
                category,
            };

            // blocks, misc, armor, weapon, accessories, consumables
            // we will skips blocks
            if (unix_timestamp - start) / 60 > IGNORE_AUCTION_MIN {
                // if this is a possible flip
                // non stackable (known quantity/count)

                // pets
                if name.contains("[Lvl 100]") || name.contains("[Lvl 1]") {
                    pet_case(&mut filtered_auctions, &mut auction);
                    continue;
                }
                // Enchanted books
                // else if name.contains("Enchanted Book") {
                //     book_case(
                //         &mut filtered_auctions,
                //         &mut *conversion_table,
                //         rarity_table,
                //         &mut auction,
                //         &auc,
                //     );
                //     continue;
                // }

                // Default case
                default_case(
                    &mut filtered_auctions,
                    &mut *conversion_table,
                    &mut auction,
                    &auc,
                );
                continue;
            }
            // anything that makes it to this point will be looked at fore lowest bin flips because they have been up for less than IGNORE_AUCTION_MIN

            // A pet
            // Formatted as TIER NAME LEVEL PET_ITEM CANDY_USED
            // Ex: RARE TIGER 78 DWARF_TURTLE_SHELMET 0
            if name.contains("[Lvl 100]") || name.contains("[Lvl 1]") {
                pet_case(&mut filtered_auctions, &mut auction);
                continue;
            }
            // unzipping nbt is the highest computation task we perform
            // therefore we should do as few as possible
            // we check if the item was recombed or was starred, would it be be a possible flip
            // if so then we unzip it
            else {
                match conversion_table.get(&auction.name) {
                    Some(e) => {
                        auction.id = e.to_string();
                        let mut unzip = false;
                        for i in 0..=5 {
                            let id1 = auction.get_extra_data_id(Some(true), Some(i));
                            let id2 = auction.get_extra_data_id(Some(false), Some(i));
                            for k in [id1, id2] {
                                let mut meets_profit_reqs = false;
                                let mut meets_volume_reqs = false;

                                match avg_lbin_json.get(&k) {
                                    Some(e) => {
                                        if e.parse::<i32>().unwrap() - auction.price as i32
                                            > MIN_PROFIT as i32
                                        {
                                            meets_profit_reqs = true;
                                        }
                                    }
                                    // no need to unzip because we wont even be able to compare
                                    // not stored in the database
                                    None => {}
                                }
                                match volume_json.get(&k) {
                                    Some(e) => {
                                        if e.parse::<i32>().unwrap() > MIN_VOLUME as i32 {
                                            meets_volume_reqs = true;
                                            if meets_profit_reqs && meets_volume_reqs {
                                                unzip = true;
                                            }
                                            break;
                                        }
                                    }
                                    // no need to unzip because we wont even be able to compare
                                    // not stored in the database
                                    None => {}
                                }
                            }
                            break;
                        }
                        if unzip {
                            // holder = true;
                            let nbt = &to_nbt(
                                serde_json::from_value(auc.get("item_bytes").unwrap().to_owned())
                                    .unwrap(),
                            )
                            .unwrap()
                            .i[0];
                            auction.id = nbt.tag.extra_attributes.id.as_ref().unwrap().to_string();
                            auction.price /= nbt.count; // scales price for stacked items
                                                        // println!("{:?}", auction);
                            auction.additional_price_data = calc_additional_price(nbt);
                        }
                    }
                    None => {
                        let nbt = &to_nbt(
                            serde_json::from_value(auc.get("item_bytes").unwrap().to_owned())
                                .unwrap(),
                        )
                        .unwrap()
                        .i[0];
                        let new_id = nbt.tag.extra_attributes.id.as_ref().unwrap().to_string();
                        conversion_table.insert(auction.name.to_string(), new_id.to_string());
                        auction.id = new_id;
                        auction.price /= nbt.count; // scales price for stacked items
                                                    // println!("{:?}", auction);
                        auction.additional_price_data = calc_additional_price(nbt);
                    }
                }

                filtered_auctions.push(auction);
            }
        }
    } else {
        println!("Page failed!");
        // std::process::exit(0);
    }

    // println!("Page {}: {}", page_num, holder);
    // println!("{:?}", s.elapsed());
    filtered_auctions
}

async fn fetch(url: String) -> serde_json::Value {
    let req = surf::get(url);

    let res = HTTP_CLIENT1.send(req).await;
    if res.is_ok() {
        let json = res.unwrap().body_json().await;
        if json.is_ok() {
            return json.unwrap();
        }
    }

    serde_json::Value::Null
}

// fn calculate_enchant_costs(all_auctions: &HashMap<String, Vec<Auction>>, auction: &Auction) -> u64 {
//     // Load enchant-rarity-table
//     let enchant_file: String = String::from(PATH_TO_ENCHANT_RARITIES);
//     let data2 = fs::read_to_string(&enchant_file).expect("Unable to read file");
//     let mut rarity_table: HashMap<String, String> = HashMap::new();
//     if fs::metadata(&enchant_file).unwrap().len() != 0 {
//         rarity_table = serde_json::from_str(&data2).unwrap();
//     }

//     let mut enchant_costs = 0;
//     let nbt = &to_nbt(serde_json::from_value(auction.item_bytes.clone()).unwrap())
//         .unwrap()
//         .i[0];
//     match nbt.tag.extra_attributes.enchantments.as_ref() {
//         Some(enchants) => {
//             for (k, v) in enchants {
//                 let enchant_name = format!("{}_{}", k, v);
//                 // if enchant is not a basic enchant
//                 if rarity_table.contains_key(&enchant_name) {
//                     if !all_auctions.contains_key(&enchant_name) {
//                         continue;
//                     }

//                     let mut enchant_auctions = all_auctions.get(&enchant_name).unwrap().clone();
//                     enchant_auctions.sort_by_key(|a| a.price);
//                     enchant_costs += enchant_auctions[0].price;
//                 }
//             }
//         }
//         None => {
//             return 0;
//         }
//     }

//     enchant_costs
// }

#[derive(Deserialize, Serialize, Debug)]
enum EssenceTable {
    String(String),
    Vec(u64),
}
// fn calculate_essence_costs(auction: &Auction) -> u64 {
//     let mut essence_cost: HashMap<String, u64> = HashMap::new();
//     essence_cost.insert("wither".to_string(), 3000);

//     if !auction.name.contains("✪") {
//         return 0;
//     }

//     let nbt = &to_nbt(serde_json::from_value(auction.item_bytes.clone()).unwrap())
//         .unwrap()
//         .i[0];
//     let stars: u8 = match nbt.tag.extra_attributes.dungeon_item_level {
//         Some(s) => s,
//         None => 0,
//     };

//     if stars > 0 && stars <= 5 {
//         let data = fs::read_to_string(PATH_TO_ESSENCE_TABLE).expect("Unable to read file");
//         let table: HashMap<String, HashMap<String, Vec<u64>>> =
//             serde_json::from_str(&data).unwrap();
//         // "wither": {"NECRON_BLADE HYPERION SCYLLA ASTRAEA VALKYRIE DARK_CLAYMORE": [1, 2, 3, 4, 5],}
//         for essence_type in table.keys() {
//             for key in table.get(essence_type).unwrap().keys() {
//                 if key.contains(&auction.id) {
//                     let upgrade_amounts: &Vec<u64> =
//                         table.get(essence_type).unwrap().get(key).unwrap();

//                     let total_essence: u64 = upgrade_amounts[..stars as usize].iter().sum();
//                     return total_essence * essence_cost.get(essence_type).unwrap();
//                 }
//             }
//         }
//     }

//     0
// }

// Find flips that fit criteria and write those to JSON for future use.
pub fn find_flips(
    auctions: &HashMap<String, Vec<Auction>>,
    avg_lbin_json: &HashMap<String, String>,
    volume_json: &HashMap<String, String>,
) {
    let s = Instant::now();
    // Code block to measure.

    // let pet_splits =  3;

    let mut possible_flips: Vec<Flip> = Vec::new();
    let mut seen_flips: Vec<String> = vec![];

    let mut failed_flips: Vec<Flip> = Vec::new();

    let unix_timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // store pointers to all new auctions
    let mut new_auctions: Vec<&Auction> = vec![];
    for (_, auction) in auctions.iter() {
        for auc in auction {
            if (unix_timestamp - auc.start) / 60 < IGNORE_AUCTION_MIN {
                new_auctions.push(auc);
            }
        }
    }

    // 1) iterate through all of the new auctions
    // 2) find the average target price for each auction
    // 3) find the lowest bin profit
    // 4) check if there is profit for target value or lowest bin
    // 5) create flip object
    for auction in &new_auctions {
        // remove pets that are not level 100 or 1
        if auction.name.contains("Lvl") {
            if auction.id.contains("-1-") || auction.id.contains("-100-") {
            } else {
                continue;
            }
        }
        if auction.price > MAX_BID {
            continue;
        }

        // lbin - only if this auction is actually the lowest bin

        // sorting takes way too much time, so we will manually find the 2 lowest items
        let similar_auctions = auctions.get(&auction.id).unwrap();
        let num_of_similar_auctions = similar_auctions.len();
        if num_of_similar_auctions < MIN_SIMILAR_AUCS {
            continue;
        }

        let mut lowest_bin: &Auction = &similar_auctions[0];
        let mut next_lowest_bin: &Auction = &similar_auctions[1];
        for auc in similar_auctions {
            if auc.price < next_lowest_bin.price {
                next_lowest_bin = auc;
            }
            if next_lowest_bin.price < lowest_bin.price {
                let temp = lowest_bin;
                lowest_bin = next_lowest_bin;
                next_lowest_bin = temp;
            }
        }

        // Guard condition - delete later
        if lowest_bin.price == 0 {
            auction.print_failed("lowest_bin.price == 0");
            continue;
        }

        // this would mean that the lowest bin is also more than avg_target_price
        // if next_lowest_bin.price > avg_target_price {
        //     auction.print_failed("lowest_bin.price > avg_target_price - market manipulation");
        //     continue;
        // }

        let auc_avg_lbin: u64 = match avg_lbin_json.get(&auction.get_volume_id()) {
            Some(x) => x.parse().unwrap(),
            None => 1000000000,
        };

        if lowest_bin.price >= auc_avg_lbin {
            // lowest_bin.print_failed(&format!("{} > {}", lowest_bin.price, auc_avg_lbin));
            continue;
        }

        // we already know that sales are dropped after 1 week, we can just count all sales to get last_week_volume
        // item ids are formatted different internally (more information) - we must extract the actual game id
        // pets are formatted the same for both sales and targets - so are books
        let volume_last_week = match volume_json.get(&auction.get_volume_id()) {
            Some(x) => x.parse().unwrap(),
            None => 0,
        };

        // final checks - guard cases
        if lowest_bin.seller == "1d7a1fd3350546efa653389dfa00ea85"  || // If is my auction
            auction.seller == "1d7a1fd3350546efa653389dfa00ea85"  || // If is my auction
            volume_last_week < MIN_VOLUME
        {
            // auction.print_failed(format!("volume {} {}", sale_name, volume_last_week).as_ref());
            continue;
        }

        // first we check for bin profit and
        if lowest_bin.uuid == auction.uuid {
            // if this auction really is the lowest bin and different rarity guard condition

            let mut flip_price: u64 = (next_lowest_bin.price - lowest_bin.price)
                .try_into()
                .unwrap();
            let taxes = (flip_price as f64 * 0.02 + lowest_bin.price as f64 * 0.01) as u64;
            flip_price = if taxes < flip_price {
                flip_price - taxes
            } else {
                flip_price
            };
            let profit_percentage = (flip_price * 100 / lowest_bin.price) as usize;

            if flip_price > MIN_PROFIT && profit_percentage > MIN_PROFIT_PERCENTAGE {
                // compare agaisnt blacklist
                let mut remove = false;
                for i in BLACKLISTED_ITEMS {
                    if auction.name.contains(i) {
                        remove = true;
                        break;
                    }
                }
                if remove {
                    continue;
                }

                let estimated_target_price =
                    match avg_lbin_json.get(&auction.get_extra_data_id(None, None)) {
                        Some(e) => e.parse().unwrap(),
                        None => 0,
                    };

                let flip = Flip {
                    id: lowest_bin.uuid.to_owned(),
                    internal_id: auction.get_volume_id(),
                    flip_type: FlipType::LBin,
                    flip_price: flip_price as usize,
                    profit_percentage,
                    lowest_bin: lowest_bin.to_owned(),
                    next_lowest_bin: next_lowest_bin.to_owned(),
                    similar_auctions: num_of_similar_auctions,
                    volume_last_week,
                    estimated_target_price,
                    target_name: "".to_string(),
                };
                if !seen_flips.contains(&flip.id) {
                    seen_flips.push(flip.id.to_string());
                    possible_flips.push(flip);
                    continue;
                }
                // } else {
                //     // auction.print_failed("didnt meet min profit lbin");
                // }
            }
        } else {
            // auction.print_failed("didnt meet lbin reqs");
        }
    }

    // make flip
    for auction in &new_auctions {
        if seen_flips.contains(&auction.uuid) {
            continue;
        }

        if auction.price > MAX_BID {
            continue;
        }

        let id = &auction.get_extra_data_id(None, None);
        if id == &auction.get_volume_id() {
            continue;
        }
        if auction.name.contains("✪") && !auction.is_dungeonized() {
            println!("{:?}", auction);
            println!("{:?}", auction.additional_price_data);
            println!("{}", id);
            println!("");
        }

        let predicted_value = match avg_lbin_json.get(id) {
            Some(e) => e.parse().unwrap(),
            None => 0,
        };
        // avg the volume between the recombed version and starred version
        let volume = match volume_json.get(id) {
            Some(e) => e.parse().unwrap(),
            None => 0,
        };
        if volume < MIN_VOLUME as i32 {
            continue;
        }
        let flip_price = predicted_value - auction.price as i64;
        if flip_price < MIN_PROFIT as i64 {
            continue;
        }

        // check if any other auctions are cheaper and have the same id
        let mut delete = false;
        for i in auctions.get(&auction.get_volume_id()).unwrap() {
            if &i.get_extra_data_id(None, None) == id && i.price < auction.price {
                delete = true;
                break;
            }
        }
        if delete {
            continue;
        }

        // compare agaisnt blacklist
        let mut remove = false;
        for i in BLACKLISTED_ITEMS {
            if auction.name.contains(i) {
                remove = true;
                break;
            }
        }
        if remove {
            continue;
        }

        let lowest_bin = auction.clone().to_owned();
        let mut next_lowest_bin = auction.clone().to_owned();
        next_lowest_bin.price = predicted_value as u64;

        // find lbin price
        let mut cheapest: &Auction = &lowest_bin.clone();
        for auc in auctions.get(&auction.get_volume_id()).unwrap() {
            if auc.price < lowest_bin.price {
                cheapest = auc;
            }
        }

        possible_flips.push(Flip {
            id: auction.uuid.to_owned(),
            internal_id: id.to_string(),
            flip_type: FlipType::Target,
            flip_price: flip_price as usize,
            profit_percentage: (flip_price * 100 / auction.price as i64) as usize,
            lowest_bin,
            next_lowest_bin,
            similar_auctions: 0,
            volume_last_week: volume as usize,
            estimated_target_price: cheapest.price as usize,
            target_name: "".to_string(),
        });

        seen_flips.push(auction.uuid.to_string());
    }
    println!("Flips found in: {:?}", s.elapsed());

    update_flips(&possible_flips);
    if possible_flips.len() == 0 {
        failed_flips.sort_by_key(|f| f.flip_price.clone());
        if failed_flips.len() > 1 {
            for flip in &failed_flips[failed_flips.len() - 1..] {
                flip.print();
            }
        }
        return;
    }

    possible_flips.sort_by_key(|f| f.flip_price.clone());
    // if possible_flips[possible_flips.len() - 1].flip_price < 3_000_000 {
    //     possible_flips.sort_by_key(|f| f.similar_auctions.clone());
    //     if possible_flips.len() > 0
    //         && possible_flips[possible_flips.len() - 1].similar_auctions > 70
    //     {
    //         // open_auction(&possible_flips);
    //     } else {
    //         possible_flips.sort_by_key(|f| f.flip_price.clone());
    //         // open_auction(&possible_flips);
    //     }
    // }
    // open_auction(&possible_flips);
    send(format!(
        "/viewauction {}",
        possible_flips[possible_flips.len() - 1].id
    ));
    for flip in possible_flips {
        flip.print();
    }
    println!("---------------------------------");
}

// pub async fn get_total_pages() -> u64 {
//     let page_0 = fetch(String::from("https://api.hypixel.net/skyblock/auctions?page=0")).await;
//     let total_pages: u64 = page_0.get("totalPages").unwrap().as_u64().unwrap();
//     return total_pages;
// }

pub async fn update_critical_auctions(
    auctions_hashed: &mut HashMap<String, Vec<Auction>>,
    seen_ids: &mut HashMap<String, String>,
    last_update: &mut i32,
) {
    // let mut futures = FuturesUnordered::new();

    // Load conversions json
    // Load enchant-rarity-table
    let mut conversion_tables = load_json_file(PATH_TO_CONVERSIONS);
    let rarity_table = load_json_file(PATH_TO_ENCHANT_RARITIES);
    let avg_lbins = load_json_file(PATH_TO_AVG_LBIN);
    let volume_json = load_json_file(PATH_TO_VOLUME);

    // run head calls synchronously and check the page age, once the age is around 60 or 0 then switch to get requests
    loop {
        let req = surf::head("https://api.hypixel.net/skyblock/auctions?page=0".to_string());
        let res = HTTP_CLIENT1.send(req).await;
        match res {
            Ok(unwrapped) => match unwrapped.header("age") {
                Some(age) => {
                    let age: i32 = age.as_str().parse().unwrap();
                    if age == 0 || (age > 0 && age < 50) {
                        break;
                    }
                }
                None => continue,
            },
            Err(e) => {
                println!("Error in api calls {}", e);
                return;
            }
        }
    }

    let now = Instant::now();
    let res = fetch("https://api.hypixel.net/skyblock/auctions?page=0".to_string()).await;
    *last_update = res.get("lastUpdated").unwrap().as_i64().unwrap() as i32;
    let now2 = Instant::now();
    let page = process_page(res, &mut conversion_tables, &avg_lbins, &volume_json);

    for auction in page {
        if seen_ids.contains_key(&auction.uuid) {
            continue;
        }
        seen_ids.insert(auction.uuid.to_string(), "".to_string());
        auctions_hashed
            .entry(auction.id.to_string())
            .or_default()
            .push(auction);
    }
    println!(
        "\rCritical Auctions Returned in: {:?} ({} auctions) - processed in ({:?})",
        now.elapsed(),
        auctions_hashed.len(),
        now2.elapsed(),
    );

    // save new conversion
    let json: String = serde_json::to_string(&conversion_tables).unwrap();
    fs::write(PATH_TO_CONVERSIONS, &json).expect("Unable to write file");

    // save new enchants
    let json: String = serde_json::to_string(&rarity_table).unwrap();
    fs::write(PATH_TO_ENCHANT_RARITIES, &json).expect("Unable to write file");
}

pub async fn update_all_auctions(
    auctions_hashed: &mut HashMap<String, Vec<Auction>>,
    seen_ids: &mut HashMap<String, String>,
) {
    let mut futures = FuturesUnordered::new();

    let mut conversion_tables = load_json_file(PATH_TO_CONVERSIONS);
    let rarity_table = load_json_file(PATH_TO_ENCHANT_RARITIES);
    let avg_lbins = load_json_file(PATH_TO_AVG_LBIN);
    let volume_json = load_json_file(PATH_TO_VOLUME);

    // Load previous pages
    // let prev_pages = get_pa
    // println!("\rUpdating Non-essential Auctions!");
    // let now: Instant = Instant::now();

    let page_0 = fetch(String::from(
        "https://api.hypixel.net/skyblock/auctions?page=0",
    ))
    .await;

    let total_pages: u64 = match page_0.get("totalPages") {
        Some(pages) => pages.as_u64().unwrap(),
        None => 0,
    };

    for page_number in 1..total_pages {
        futures.push(fetch(format!(
            "https://api.hypixel.net/skyblock/auctions?page={}",
            page_number
        )));
    }

    // Process page zero since it was already fetched to get totalPages
    for auction in process_page(page_0, &mut conversion_tables, &avg_lbins, &volume_json) {
        auctions_hashed
            .entry(auction.id.to_string())
            .or_default()
            .push(auction);
    }
    while let Some(res) = futures.next().await {
        let page = process_page(res, &mut conversion_tables, &avg_lbins, &volume_json);
        for auction in page {
            if seen_ids.contains_key(&auction.uuid) {
                continue;
            }
            seen_ids.insert(auction.uuid.to_string(), "".to_string());
            auctions_hashed
                .entry(auction.id.to_string())
                .or_default()
                .push(auction);
        }
    }

    let json: String = serde_json::to_string(&conversion_tables).unwrap();
    fs::write(PATH_TO_CONVERSIONS, &json).expect("Unable to write file");

    // save new enchants
    let json: String = serde_json::to_string(&rarity_table).unwrap();
    fs::write(PATH_TO_ENCHANT_RARITIES, &json).expect("Unable to write file");
    // println!("\rNon-essential Auctions Updated in: {:?} ({} auctions)", now.elapsed(), auctions_hashed.len());

    // let data = fetch(1).await;
    // let json = data.get("auctions").unwrap().as_array().unwrap();
    // println!("Data: {}", data);
}

fn update_flips(flips: &Vec<Flip>) {
    let file: String = String::from(PATH_TO_FLIPS);
    let json: String = serde_json::to_string(flips).unwrap();
    fs::write(file, &json).expect("Unable to write file");
}
