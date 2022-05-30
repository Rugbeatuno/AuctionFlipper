mod auction_house;
mod constants;
mod structs;
mod utils;
mod pets;
mod auctions_ended;

use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;

use crate::structs::{Sale, SaleData};

fn update_targets() -> Option<HashMap<String, Vec<SaleData>>> {
    let file: String = String::from(constants::PATH_TO_TARGETS);
    let data = fs::read_to_string(&file).expect("Unable to read file");
    let mut targets: HashMap<String, Vec<SaleData>> = HashMap::new();
    if fs::metadata(&file).unwrap().len() != 0 {
        targets = serde_json::from_str(&data).unwrap();
        Some(targets);
    }
    None
}

#[no_mangle]
pub async fn flipper() {
    println!("Flipper Started!");
    let holder: u64 = 0;

    let mut auctions_hashed: HashMap<String, Vec<structs::Auction>> = HashMap::new();
    let mut auction_ids: Vec<String> = vec![];

    let mut targets: HashMap<String, Vec<SaleData>> = HashMap::new();
    match update_targets() {
        Some(t) => {targets = t;}
        None => {}
    }
   

    auction_house::update_all_auctions(&mut auctions_hashed, &mut auction_ids).await;
    let mut next_update = utils::get_next_api_update().await;
    utils::save_next_update(next_update);

    while holder < 1_000_000 {
        let unix_timestamp: u64 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let time_until_update: i32 = next_update as i32 - unix_timestamp as i32;

        print!("\r  {}  ", time_until_update);
            
        if time_until_update == 20 || time_until_update == 40 { // refresh non-essential pages
            auctions_hashed.clear();
            auction_ids.clear();
            auction_house::update_all_auctions(&mut auctions_hashed, &mut auction_ids).await;
            auction_house::find_flips(&auctions_hashed, &targets)
        }

        if time_until_update <= 10 { // refresh critcal pages
            auction_house::update_critical_auctions(&mut auctions_hashed, &mut auction_ids).await;
            auction_house::find_flips(&auctions_hashed, &targets)
        }

        if time_until_update < -5 {
            next_update = utils::get_next_api_update().await;
            utils::save_next_update(next_update);
            match update_targets() {
                Some(t) => {targets = t;}
                None => {}
            }
            auctions_ended::update_auctions_ended().await;
            auctions_hashed.clear();
            auction_ids.clear();
            auction_house::update_all_auctions(&mut auctions_hashed, &mut auction_ids).await;
            auction_house::find_flips(&auctions_hashed, &targets);
        }
    }
}