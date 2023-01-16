mod auction_house;
mod auctions_ended;
mod constants;
mod inputs;
mod pets;
mod structs;
mod utils;

use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;

use constants::EXPIRATION_DATE;

use crate::constants::{
    BLACKLISTED_ITEMS, CONFIRM_PURCHASE_X, CONFIRM_PURCHASE_Y, DEBUG, LORE_ENCHANTS, LORE_TO_KEY,
    MAX_BID, MIN_PROFIT, MIN_PROFIT_PERCENTAGE, MIN_VOLUME, PATH_TO_AVG_LBIN, PATH_TO_VOLUME,
    PURCHASE_X, PURCHASE_Y, TIMEOUT,
};
use crate::utils::{get_unix_timestamp, load_json_file};

use crate::inputs::{init, send};

fn check_for_timeout() {
    let unix_timestamp: i32 = get_unix_timestamp();

    if unix_timestamp > EXPIRATION_DATE {
        println!("Data Expired! Program will not run to prevent bad flips.");
        std::process::exit(0);
    }
}

// fn load_preferences() {
//     let data: String =
//         fs::read_to_string(PATH_TO_PREFERENCES).expect("Unable to read preferences!");
//     let json: HashMap<String, String> = serde_json::from_str(&data).unwrap();
// }

fn print_constants() {
    println!("Debug: {}", DEBUG);
    println!("Request Timeout: {}", TIMEOUT);
    println!("");
    println!("Purchase X: {}px", PURCHASE_X as usize);
    println!("Purchase Y: {}px", PURCHASE_Y as usize);
    println!("Confirm Purchase X: {}px", CONFIRM_PURCHASE_X as usize);
    println!("Confirm Purchase Y: {}px", CONFIRM_PURCHASE_Y as usize);

    println!("");
    println!("Minimum Profit: {}", MIN_PROFIT);
    println!("Minimum Profit %: {}", MIN_PROFIT_PERCENTAGE);
    println!("Minimum Volume: {}", MIN_VOLUME);
    println!("Maximum Bid: {}", MAX_BID);
    println!(
        "Blacklist ({}): {:?}",
        BLACKLISTED_ITEMS.len(),
        BLACKLISTED_ITEMS
    );

    println!("-------------------------------\n");
}
#[no_mangle]
pub async fn flipper() {
    print_constants();
    check_for_timeout();
    println!("Flipper Started!");

    init();
    let holder: i32 = 0;

    let mut auctions_hashed: HashMap<String, Vec<structs::Auction>> = HashMap::new();
    let mut auction_ids: HashMap<String, String> = HashMap::new();
    let mut last_update: i32 = 0;
    let mut prev_last_update = 0;
    let mut avg_lbin_json = load_json_file(PATH_TO_AVG_LBIN);
    let mut volume_json = load_json_file(PATH_TO_VOLUME);
    let mut seen_auction_ids: HashMap<String, String> = HashMap::new();
    // let last_big_update = SystemTime::now()
    //     .duration_since(UNIX_EPOCH)
    //     .unwrap()
    //     .as_secs();

    auction_house::update_all_auctions(&mut auctions_hashed, &mut auction_ids).await;
    let mut next_update = utils::get_next_api_update().await;
    utils::save_next_update(next_update);
    // println!("{:?}", auctions_hashed);
    while holder < 1_000_000 {
        let unix_timestamp: i32 = get_unix_timestamp();

        let mut time_until_update: i32 = next_update - unix_timestamp;

        print!("\r  {}  ", time_until_update);
        // continue;
        if time_until_update == 15 {
            auctions_ended::remove_sold_flips(&mut seen_auction_ids).await;
            avg_lbin_json = load_json_file(PATH_TO_AVG_LBIN);
            volume_json = load_json_file(PATH_TO_VOLUME);
        }
        if time_until_update == 6 {
            send('t'.to_string());
            sleep(Duration::from_secs(1));
        }
        if time_until_update <= 3 {
            // refresh critcal pages
            prev_last_update = last_update;
            auction_house::update_critical_auctions(
                &mut auctions_hashed,
                &mut auction_ids,
                &mut last_update,
            )
            .await;

            auction_house::find_flips(
                &auctions_hashed,
                &avg_lbin_json,
                &volume_json,
                &mut seen_auction_ids,
            );
            // meaning the page updated
            if last_update != prev_last_update {
                time_until_update = -10;
            }
        }

        if time_until_update < -5 {
            next_update = utils::get_next_api_update().await;
            utils::save_next_update(next_update);

            auctions_ended::update_volume(&auctions_hashed).await;
            auctions_ended::update_additional_price_data(&auctions_hashed).await;

            auctions_ended::remove_expired_sold_auctions(&mut auctions_hashed);
            check_for_timeout();

            // auctions_ended::update_auctions_ended().await;
            // if unix_timestamp - last_big_update > 180 {
            //     auctions_hashed.clear();
            //     auction_ids.clear();
            //     auction_house::update_all_auctions(&mut auctions_hashed, &mut auction_ids).await;
            //     auction_house::find_flips(&auctions_hashed, &targets);

            //     last_big_update = SystemTime::now()
            //         .duration_since(UNIX_EPOCH)
            //         .unwrap()
            //         .as_secs();
            // }
        }
    }
}
