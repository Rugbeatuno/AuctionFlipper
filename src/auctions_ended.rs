use futures::stream::FuturesUnordered;
use futures::StreamExt;

use crate::constants::*;
use crate::structs::*;
use crate::utils;
use crate::utils::{fetch, load_json_file};
use std::iter::Sum;
use std::thread::sleep;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};
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

async fn get_remaining_fetch() -> i32 {
    let res = HTTP_CLIENT1
        .head("https://sky.coflnet.com/api/item/price/POWER_WITHER_BOOTS/history/week")
        .send()
        .await;

    match res {
        Ok(e) => match e.header("x-rate-limit-remaining") {
            Some(val) => {
                return val[0].to_string().parse().unwrap();
            }
            None => 0,
        },
        Err(_) => {
            return 0;
        }
    };
    0
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
                        println!("PAIN{}", res.0);
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

fn find_min_avg(val: &serde_json::Value) -> u64 {
    let mut prices = vec![];
    for i in val.as_array().unwrap() {
        prices.push(i.get("min").unwrap().as_f64().unwrap() as u64);
    }

    if prices.len() == 0 {
        return 0;
    }
    // remove the top 20% of sales
    prices.sort();
    let mut length = prices.len();
    if prices.len() > 10 {
        length = (prices.len() as f32 * 0.8) as usize;
    }
    let prices = &prices[0..length];
    let sum: u64 = prices.iter().sum();
    let avg = sum / length as u64;
    return avg;
}

fn find_volume(val: &serde_json::Value) -> u64 {
    let mut vol = 0;
    for i in val.as_array().unwrap() {
        vol += i.get("volume").unwrap().as_f64().unwrap() as u64;
    }
    return vol;
}

pub async fn update_additional_price_data(auctions_hashed: &HashMap<String, Vec<Auction>>) {
    // open file
    let mut lbin_json = load_json_file(PATH_TO_AVG_LBIN);
    let mut volume_json = load_json_file(PATH_TO_VOLUME);

    // "https://sky.coflnet.com/api/item/price/{}/history/week?Stars={}",
    let mut recomb_keys: Vec<(String, String)> = vec![];
    for i in auctions_hashed.keys() {
        let auc = &auctions_hashed.get(i).unwrap()[0];
        // all items can be recombed, but we only care about weapons, armor, and accessories
        if auc.category == WEAPON || auc.category == ARMOR || auc.category == ACCESSORIES {
            // we will have to find the id for the base item just recombed
            // then 1-5 stars without recomb
            // and 1-5 stars with recomb

            let id = auc.get_extra_data_id(Some(true), None);
            if !lbin_json.contains_key(&id) {
                recomb_keys.push((
                    format!(
                        "https://sky.coflnet.com/api/item/price/{}/history/week?Recombobulated=true",
                        auc.get_volume_id(),
                    ),
                    id.to_string(),
                ));
            }

            if auc.category != ACCESSORIES {
                for k in 1..=5 {
                    let id = auc.get_extra_data_id(Some(true), Some(k));
                    if !lbin_json.contains_key(&id) {
                        recomb_keys.push((
                            format!(
                                "https://sky.coflnet.com/api/item/price/{}/history/week?Recombobulated=true&Stars={}",
                                auc.get_volume_id(),
                                k
                            ),
                            id.to_string(),
                        ));
                    }
                    let id = auc.get_extra_data_id(Some(false), Some(k));
                    if !lbin_json.contains_key(&id) {
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

    let mut keys: Vec<(String, String)> = vec![];
    for i in auctions_hashed.keys() {
        let id = auctions_hashed.get(i).unwrap()[0].get_volume_id();
        if !volume_json.contains_key(&id) {
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
    let unix_timestamp: u64 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

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

pub async fn remove_sold_flips() {
    let data = fs::read_to_string(PATH_TO_FLIPS).expect("Unable to read file");
    let mut flips: Vec<Flip> = vec![];
    if fs::metadata(PATH_TO_FLIPS).unwrap().len() != 0 {
        flips = serde_json::from_str(&data).unwrap();
    }
    let req: serde_json::Value = utils::fetch(String::from(
        "https://api.hypixel.net/skyblock/auctions_ended",
    ))
    .await;
    for (idx, flip) in flips.to_owned().iter().enumerate() {
        let auctions = match req.get("auctions") {
            Some(e) => e.as_array().unwrap().to_owned(),
            None => vec![],
        };
        for auc in auctions {
            let auc_id = auc.get("auction_id").unwrap();
            if flip.id == auc_id.to_string() {
                flips.remove(idx);
            }
        }
    }
    let json: String = serde_json::to_string(&flips).unwrap();
    fs::write(PATH_TO_FLIPS, &json).expect("Unable to write file");
}
