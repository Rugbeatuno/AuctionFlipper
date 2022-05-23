extern crate serde_json;
extern crate serde_derive;
extern crate base64;
extern crate json;
use futures::{stream::FuturesUnordered, StreamExt};
use std::{str, fs, iter::Iterator, collections::HashMap};
use std::{result::Result as StdResult};
use std::time::{Instant, SystemTime, UNIX_EPOCH};
// use cli_clipboard;

use crate::structs::*;
use crate::constants::*;
use crate::pets::*;


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
                pet_case(&mut filtered_auctions, &mut auction);
                continue;
            }
            else if auction.name.contains("Enchanted Book") {
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

    let res = HTTP_CLIENT1.send(req).await;
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
    let min_profit = 650_000;
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
        if (unix_timestamp - flip.lowest_bin.start) / 60 > IGNORE_AUCTION_MIN {
            continue;
        }  // if the auction has been up for more than 7 minutes ignore it (good auctions are sniped withing seconds)

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
            possible_flips.push(flip);
        }
        
    }

    // If the biggest flip is a reasonable amount recommend it
    // Otherwise, check if there is a huge amount of volume on any items
    // If not, recommend the largest flip
    // possible_flips.sort_by_key(|f| f.flip_price.clone());
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
    if possible_flips[possible_flips.len()-1].flip_price < 3_000_000 {
        possible_flips.sort_by_key(|f| f.volume.clone());
        if possible_flips.len() > 0 && possible_flips[possible_flips.len()-1].volume > 70 {
            // open_auction(&possible_flips);
        }
        else {
            possible_flips.sort_by_key(|f| f.flip_price.clone());
            // open_auction(&possible_flips);

        }
    }
    // open_auction(&possible_flips);
    for flip in possible_flips.clone() {
        flip.print();
    }
    
    update_flips(&possible_flips);

    
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





// fn open_auction(flips: &Vec<Flip>) {
    // if flips.len() > 0 {
    //     let top_auc = &flips[flips.len()-1];
    //     let cmd = format!("/viewauction {}", top_auc.lowest_bin.uuid);
    //     // let current_clipboard = cli_clipboard::get_contents().unwrap();

    //     cli_clipboard::set_contents(cmd.to_owned()).unwrap();
    //     assert_eq!(cli_clipboard::get_contents().unwrap(), cmd);
    // }
// }

