mod auction_house;
mod constants;
mod structs;
mod utils;

// use chrono::*;
use std::time::{SystemTime, UNIX_EPOCH};
use std::fs;
use std::collections::HashMap;

use warp::{Filter};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _web = tokio::spawn(async {
        let get_items = warp::get()
        .and(warp::path("flips"))
        .and(warp::path("get"))
        .and(warp::path::end())
        .and_then(get_flips);
    
        let routes = get_items;
        warp::serve(routes).run(([127, 0, 0, 1], 5000)).await;
    });

    // update page count
    utils::update_pages_file(None).await;

    let mut next_update: u64 = 0;
    let holder: u64 = 0;

    let mut auctions_hashed: HashMap<String, Vec<structs::Auction>> = HashMap::new();
    let mut auction_ids: Vec<String> = vec![];

    while holder < 1_000_000 {
        let unix_timestamp: u64 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        // let seconds = Local::now().second();

        if next_update == 0 {
            next_update = utils::get_duration_until_api_update2().await;
        }

        let time_until_update: i32 = next_update as i32 - unix_timestamp as i32;

        if next_update == 0 {
            continue;
        }
        print!("\r  {}  ", time_until_update);

            
        if time_until_update == 20 || time_until_update == 40 { // refresh non-essential pages
            auctions_hashed.clear();
            auction_ids.clear();
            auction_house::update_all_auctions(&mut auctions_hashed, &mut auction_ids).await;
        }
        else if time_until_update % 10 == 0 {
            auction_house::update_critical_auctions(&mut auctions_hashed, &mut auction_ids).await;
        }

        if time_until_update <= 1 { // refresh critcal pages
            auction_house::update_critical_auctions(&mut auctions_hashed, &mut auction_ids).await;
        }

        if time_until_update < -10 {
            auction_house::update_auctions_ended().await;
            auction_house::remove_expired_sales().await;
            next_update = utils::get_duration_until_api_update2().await;
            auctions_hashed.clear();
            auction_ids.clear();
            auction_house::update_all_auctions(&mut auctions_hashed, &mut auction_ids).await;
        }

    }
    Ok(())
}

async fn get_flips() -> Result<impl warp::Reply, warp::Rejection> {
    let file: String = String::from("./src/files/flips.json");

    let data = fs::read_to_string(&file).expect("Unable to read file");

    let load_json: serde_json::Value = serde_json::from_str(&data).unwrap();
    Ok(warp::reply::json(&load_json))
}