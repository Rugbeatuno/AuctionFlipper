mod auction_house;
mod constants;
mod structs;
mod utils;

use chrono::*;
use std::time::{SystemTime, UNIX_EPOCH};
use std::fs;


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



    let mut next_update: u64 = 0;
    let mut holder: u64 = 0;
    while holder < 1_000_000 {
        let unix_timestamp: u64 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let dt = Local::now();
        print!("\r  {}  ", dt.second());

        if next_update == 0 && dt.second() == 59 || next_update == unix_timestamp {
            // utils::get_duration_until_api_update().await;
            auction_house::update_auctions().await;
            auction_house::update_auctions_ended().await;
            auction_house::remove_expired_sales().await;
            next_update = utils::get_duration_until_api_update().await;
            holder += 1;

        }
    }
    Ok(())
}

async fn get_flips() -> Result<impl warp::Reply, warp::Rejection> {
    let file: String = String::from("./src/flips.json");

    let data = fs::read_to_string(&file).expect("Unable to read file");

    let load_json: serde_json::Value = serde_json::from_str(&data).unwrap();
    Ok(warp::reply::json(&load_json))
}