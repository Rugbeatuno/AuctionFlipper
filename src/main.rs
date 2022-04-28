mod auction_house;
mod constants;
mod structs;
mod utils;

use chrono::*;
use std::time::{SystemTime, UNIX_EPOCH};
// use std::fs::File;


// #[macro_use]
// extern crate diesel;
// #[macro_use]
// extern crate diesel_codegen;
// extern crate dotenv;

// use dotenv::dotenv;
// use std::env;

// fn main() {
//     println!("Hello, World!");
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut next_update: u64 = 0;
    loop {
        let unix_timestamp: u64 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let dt = Local::now();
        print!("\r  {}  ", dt.second());

        if next_update == 0 && dt.second() == 59 || next_update == unix_timestamp {
            // utils::get_duration_until_api_update().await;
            auction_house::update_auctions().await;
            auction_house::update_auctions_ended().await;
            next_update = utils::get_duration_until_api_update().await;
        }
    }
    Ok(())
}