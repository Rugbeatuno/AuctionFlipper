use colored::Colorize;
use futures::stream::FuturesUnordered;
use futures::StreamExt;

use crate::constants::*;
use crate::structs::*;
use crate::utils;
use crate::utils::get_unix_timestamp;
use crate::utils::{fetch, load_json_file};
use std::hash::Hash;
use std::thread::sleep;
use std::time::Duration;
use std::{collections::HashMap, fs};

// Remove auctions in json that are older than the provided time in seconds
// fn remove_expired_sales(path: String, max_life: u64) {
//     let unix_timestamp: u64 = SystemTime::now()
//         .duration_since(UNIX_EPOCH)
//         .unwrap()
//         .as_secs();

//     let file: String = String::from(path);
//     let data: String = fs::read_to_string(&file).expect("Unable to read file");

//     let mut load_json: HashMap<String, Vec<SaleData>> = HashMap::new();
//     if fs::metadata(&file).unwrap().len() != 0 {
//         load_json = serde_json::from_str(&data).unwrap();
//     }
//     // Sale is stored where each sale is a HashMap stored in a Vec (Some idiot)
//     for key in load_json.clone().keys() {
//         // [ [{}] ]
//         load_json
//             .get_mut(key)
//             .unwrap()
//             .retain(|sale| unix_timestamp - &sale.timestamp < max_life);
//     }
//     let json: String = serde_json::to_string(&load_json).unwrap();

//     fs::write(file, &json).expect("Unable to write file");
// }

// fn update_json(new_sales: Vec<Sale>, path: String) {
//     let file: String = String::from(path);
//     let data = fs::read_to_string(&file).expect("Unable to read file");
//     let mut load_json: HashMap<String, Vec<SaleData>> = HashMap::new();

//     if fs::metadata(&file).unwrap().len() != 0 {
//         load_json = serde_json::from_str(&data).unwrap();
//     }

//     for sale in new_sales {
//         load_json.entry(sale.id).or_default().push(sale.data);
//     }

//     let json: String = serde_json::to_string(&load_json).unwrap();

//     fs::write(file, &json).expect("Unable to write file");
// }

// // This function accomplishes to things
// // 1) we update sales.json which is much less detailed than targets.json
// // 2) update targets.json with detailed keys
// pub async fn update_auctions_ended() {
//     let req: serde_json::Value = utils::fetch(String::from(
//         "https://api.hypixel.net/skyblock/auctions_ended",
//     ))
//     .await;
//     let auctions = match req.get("auctions") {
//         Some(e) => e.as_array().unwrap().to_owned(),
//         None => vec![],
//     };
//     println!("Auctions ended: {}", auctions.len());

//     let mut sales: Vec<Sale> = Vec::new();
//     let mut targets: Vec<Sale> = Vec::new();

//     for auc in auctions {
//         // Guard case for non-BIN auctions
//         if !auc.get("bin").unwrap().as_bool().unwrap() {
//             continue;
//         }

//         let nbt =
//             &to_nbt(serde_json::from_value(auc.get("item_bytes").unwrap().to_owned()).unwrap())
//                 .unwrap()
//                 .i[0];

//         let mut sale = Sale {
//             id: utils::format_name_for_sales(nbt),
//             data: SaleData {
//                 price: auc.get("price").unwrap().as_u64().unwrap(),
//                 timestamp: auc.get("timestamp").unwrap().as_u64().unwrap() / 1000,
//             },
//         };
//         sales.push(sale.clone());
//         match utils::format_name_for_target(nbt) {
//             Some(id) => {
//                 sale.id = id;
//                 targets.push(sale);
//             }
//             None => {}
//         }
//     }

//     update_json(sales, PATH_TO_SALES.to_string());
//     update_json(targets, PATH_TO_TARGETS.to_string());

//     remove_expired_sales(PATH_TO_SALES.to_string(), SECONDS_IN_WEEK);
//     remove_expired_sales(PATH_TO_TARGETS.to_string(), SECONDS_IN_MONTH);

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
// }

async fn future(url: String, id: String) -> (serde_json::Value, String) {
    let res = fetch(url.to_string()).await;
    (res, id)
}

// assumming there is no backup, this function is called once a minute
// and the rate of the api is 100/min
// so we just have to limit to the first 100 reqs
async fn call_api(urls: Vec<(String, String)>) -> Vec<(serde_json::Value, String)> {
    let mut urls = urls;
    if urls.len() > 100 {
        urls = urls[0..100].to_owned();
    }
    let mut futures = FuturesUnordered::new();
    let mut returns: Vec<(serde_json::Value, String)> = vec![];

    let mut counter = 0;
    let max = urls.len();

    while counter < max {
        for key in &urls[counter..] {
            counter += 1;

            futures.push(future(key.0.to_string(), key.1.to_string()));

            if counter % 20 == 0 {
                break;
            }
        }
        while let Some(mut res) = futures.next().await {
            res.0 = res.0;
            match res.0 {
                serde_json::Value::Null => {
                    println!("FAILED: {}", res.1);
                    continue;
                }
                _ => {
                    if res.0.as_array().is_none() {
                        println!("FAILED {}", res.1);
                        continue;
                    }
                    returns.push(res);
                }
            }
        }
        sleep(Duration::from_secs(2));
    }
    returns
}

fn find_min_avg(val: &serde_json::Value) -> i32 {
    let mut prices: Vec<i32> = vec![];
    for i in val.as_array().unwrap() {
        prices.push(i.get("min").unwrap().as_f64().unwrap() as i32);
    }

    if prices.len() == 0 {
        return 0;
    }
    // remove the top 15% of sales
    // and the bottom 5% of sales
    prices.sort();
    let mut start_idx = 0;
    let mut end_idx = prices.len();
    if prices.len() > 30 {
        start_idx = (prices.len() as f32 * 0.05) as usize;
        end_idx = (prices.len() as f32 * 0.85) as usize;
    }
    let prices = &prices[start_idx..end_idx];
    let sum: i32 = prices.iter().sum();
    let avg = sum / prices.len() as i32;
    return avg;
}

fn find_volume(val: &serde_json::Value) -> i32 {
    let mut vol = 0;
    for i in val.as_array().unwrap() {
        vol += i.get("volume").unwrap().as_f64().unwrap() as i32;
    }
    return vol;
}

pub async fn update_additional_price_data(auctions_hashed: &HashMap<String, Vec<Auction>>) {
    // open file
    let mut lbin_json = load_json_file(PATH_TO_AVG_LBIN);
    let mut volume_json = load_json_file(PATH_TO_VOLUME);
    let copy_of_volume_json = load_json_file(PATH_TO_VOLUME_COPY);

    // "https://sky.coflnet.com/api/item/price/{}/history/week?Stars={}",
    let mut recomb_keys: Vec<(String, String)> = vec![];
    for i in auctions_hashed.keys() {
        let auc = &auctions_hashed.get(i).unwrap()[0];

        // pets
        if auc.is_pet() {
            for k in [1, 100] {
                for r in [EPIC, LEGENDARY] {
                    let id = auc.get_extra_data_id(None, None, Some((r, k)));
                    let copy_volume = match copy_of_volume_json.get(&id) {
                        Some(e) => e.parse().unwrap(),
                        None => 0,
                    };
                    if !volume_json.contains_key(&id) && copy_volume > 0 {
                        recomb_keys.push((
                            format!(
                                "https://sky.coflnet.com/api/item/price/{}/history/week?PetLevel={}&Rarity={}",
                                auc.get_volume_id(),
                                k,
                                r
                            ),
                            id.to_string(),
                        ));
                    }
                }
            }
        }
        // all items can be recombed, but we only care about weapons, armor, and accessories
        if auc.category == WEAPON || auc.category == ARMOR || auc.category == ACCESSORIES {
            // we will have to find the id for the base item just recombed
            // then 1-5 stars without recomb
            // and 1-5 stars with recomb

            // all items that can be recombed
            let id = auc.get_extra_data_id(Some(true), None, None);
            let copy_volume = match copy_of_volume_json.get(&id) {
                Some(e) => e.parse().unwrap(),
                None => 0,
            };
            if !volume_json.contains_key(&id) && copy_volume > 0 {
                recomb_keys.push((
                    format!(
                        "https://sky.coflnet.com/api/item/price/{}/history/week?Recombobulated=true",
                        auc.get_volume_id(),
                    ),
                    id.to_string(),
                ));
            }

            // just armor and weapons
            if auc.category != ACCESSORIES {
                for k in 1..=5 {
                    let id = auc.get_extra_data_id(Some(true), Some(k), None);
                    let copy_volume = match copy_of_volume_json.get(&id) {
                        Some(e) => e.parse().unwrap(),
                        None => 0,
                    };
                    if !volume_json.contains_key(&id) && copy_volume > 0 {
                        recomb_keys.push((
                            format!(
                                "https://sky.coflnet.com/api/item/price/{}/history/week?Recombobulated=true&Stars={}",
                                auc.get_volume_id(),
                                k
                            ),
                            id.to_string(),
                        ));
                    }
                    let id = auc.get_extra_data_id(Some(false), Some(k), None);
                    let copy_volume = match copy_of_volume_json.get(&id) {
                        Some(e) => e.parse().unwrap(),
                        None => 0,
                    };
                    if !volume_json.contains_key(&id) && copy_volume > 0 {
                        recomb_keys.push((
                            format!(
                                "https://sky.coflnet.com/api/item/price/{}/history/week?Stars={}",
                                auc.get_volume_id(),
                                k
                            ),
                            id.to_string(),
                        ));
                    }
                }
            }
        }
    }
    println!("Indexing prices for {} keys.", recomb_keys.len());

    for res in call_api(recomb_keys).await {
        let avg = find_min_avg(&res.0);
        let volume = find_volume(&res.0);
        lbin_json.insert(res.1.to_string(), avg.to_string());
        volume_json.insert(res.1.to_string(), volume.to_string());
    }

    let json: String = serde_json::to_string(&lbin_json).unwrap();
    fs::write(PATH_TO_AVG_LBIN, &json).expect("Unable to write file");

    let json: String = serde_json::to_string(&volume_json).unwrap();
    fs::write(PATH_TO_VOLUME, &json).expect("Unable to write file");
}

pub async fn update_volume(auctions_hashed: &HashMap<String, Vec<Auction>>) {
    // todo ?Stars=1-7
    // todo &Recombobulated=true
    // todo HotPotatoCount=1-9
    // todo HotPotatoCount=10
    // todo HotPotatoCount=15
    // todo ?Stars=1-7
    // todo ?Stars=1-7

    // Load volume json
    let mut volume_json = load_json_file(PATH_TO_VOLUME);
    let mut lbin_json = load_json_file(PATH_TO_AVG_LBIN);
    let copy_of_volume_json = load_json_file(PATH_TO_VOLUME_COPY);

    let mut keys: Vec<(String, String)> = vec![];
    for i in auctions_hashed.keys() {
        let id = auctions_hashed.get(i).unwrap()[0].get_volume_id();
        let copy_volume = match copy_of_volume_json.get(&id) {
            Some(e) => e.parse().unwrap(),
            None => 0,
        };
        if !volume_json.contains_key(&id) && copy_volume > 0 {
            keys.push((
                format!("https://sky.coflnet.com/api/item/price/{}/history/week", id),
                id.to_string(),
            ));
        }
    }
    println!("Indexing volume for {} keys.", keys.len());
    for res in call_api(keys).await {
        let avg = find_min_avg(&res.0);
        let volume = find_volume(&res.0);
        volume_json.insert(res.1.to_string(), volume.to_string());
        lbin_json.insert(res.1.to_string(), avg.to_string());
    }
    let json: String = serde_json::to_string(&volume_json).unwrap();
    fs::write(PATH_TO_VOLUME, &json).expect("Unable to write file");

    let j2: String = serde_json::to_string(&lbin_json).unwrap();
    fs::write(PATH_TO_AVG_LBIN, &j2).expect("Unable to write file");
}

pub fn remove_expired_sold_auctions(hashed_auctions: &mut HashMap<String, Vec<Auction>>) {
    let unix_timestamp: i32 = get_unix_timestamp();

    // remove auctions that have expired
    for key in hashed_auctions.keys() {
        for (idx, auc) in hashed_auctions.get(key).unwrap().iter().enumerate() {
            if auc.end < unix_timestamp {
                let mut auctions = hashed_auctions.get(key).unwrap().to_owned();
                auctions.remove(idx);
            }
        }
    }

    // remove auctions that have been sold
}

pub async fn remove_sold_flips(seen_flips: &mut HashMap<String, String>) {
    let known_flippers = HashMap::from([
        ("a629701e17f04efeaaaa3ec0c9acb8db", "JasperDFH"),
        ("d496178a9dd54ab18065f0ca05bc074c", "Froste"),
        ("2d25ddacc1ea4280bef7b1fd4ea4440a", "F_r_o_z_t"),
        ("c30a0ca2166d4494aefed3adcaa20551", "Officialobama_69"),
        ("c9c491ff95e14c11927635d426ef4244", "HeroOfRedemption"),
        ("99ab148c61b146ed9b5d07df7b46984c", "Coflnet"),
        ("6fa66d4d3a5049b1b7ed90e3209619df", "NotEinfacheEnte"),
        ("9f53bd6952b84b11b4745c135e082e07", "Anthqny"),
        ("51b3a8d4e5f44111af0c23267ae49d80", "Jaybot_"),
        ("1d7a1fd3350546efa653389dfa00ea85", "Mobuls"),
        ("02a8617e9eaa4662a1e3b87153354159", "Cqrrots"),
        ("b28039aea41a4f02a30dfdb371696c48", "Lunarv4"),
        ("f2b62c85fb3647669ece58639f17d8a8", "Bee_bot"),
        ("80ecc4e30b88471dbf25bccc665ff9f2", "xPando"),
        ("fb394d1dfb684a73aa9dbeaf13176a70", "Azstarr"),
        ("37b415acf7ec4af59651bd7b9bf340d7", "Snuppi69"),
        ("c93ad22c28234e518e55bda8f688df26", "Graytechwarrior"),
    ]);
    let data = fs::read_to_string(PATH_TO_FLIPS).expect("Unable to read file");
    let mut flips: Vec<Flip> = vec![];
    if fs::metadata(PATH_TO_FLIPS).unwrap().len() != 0 {
        flips = serde_json::from_str(&data).unwrap();
    }
    let req: serde_json::Value = utils::fetch(String::from(
        "https://api.hypixel.net/skyblock/auctions_ended",
    ))
    .await;
    for (_, flip) in flips.to_owned().iter().enumerate() {
        let auctions = match req.get("auctions") {
            Some(e) => e.as_array().unwrap().to_owned(),
            None => vec![],
        };
        for auc in auctions {
            // track flippers
            let buyer = auc.get("buyer").unwrap().as_str().unwrap();
            let auc_id = auc.get("auction_id").unwrap().as_str().unwrap().to_string();

            if known_flippers.contains_key(&buyer) {
                println!(
                    "{}",
                    format!(
                        "{}",
                        format!(
                            "{} purchased: {}",
                            known_flippers.get(&buyer).unwrap(),
                            auc_id
                        )
                        .green()
                    )
                );
            }

            if flip.id == auc_id.to_string() {
                let index_element = flips.iter().position(|x| x.id == flip.id).unwrap();
                seen_flips.insert(flip.id.clone(), "".to_string());
                flips.remove(index_element);
            }
        }
    }
    let json: String = serde_json::to_string(&flips).unwrap();
    fs::write(PATH_TO_FLIPS, &json).expect("Unable to write file");
}
