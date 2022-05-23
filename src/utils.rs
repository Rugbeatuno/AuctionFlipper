use std::{collections::HashMap, vec, fs};
// use std::fs;
use crate::{constants, structs};
use std::time::{SystemTime, UNIX_EPOCH};
use colored::*;

// const PATH_TO_PAGES: &str = "./src/files/pages.txt";

pub fn prettify_large_nums(num: usize) -> String {
    let mut conversions: HashMap<String, char> = HashMap::new();
    conversions.insert("1".to_string(), 'k');
    conversions.insert("2".to_string(), 'M');
    conversions.insert("3".to_string(), 'B');
    conversions.insert("4".to_string(), 'T');

    if num < 1000 {
        return num.to_string();
    }

    let mut divides = 0;
    let mut num = num as f64;
    while num >= 1000.0 {
        num /= 1000.0;
        divides += 1;
    }

    let float_val = format!("{:.1$}", num, 1);
    return format!("{}{}", float_val, conversions.get(&divides.to_string()).unwrap().to_string());

}

pub fn rarity_to_color(flip: &structs::Flip) -> String {
    match flip.lowest_bin.tier.as_str() {
        "COMMON" => {return flip.lowest_bin.name.truecolor(125, 125, 125).to_string()},
        "UNCOMMON" => {return flip.lowest_bin.name.truecolor(78, 237, 81).to_string()},
        "RARE" => {return flip.lowest_bin.name.truecolor(60, 120, 239).to_string()},
        "EPIC" => {return flip.lowest_bin.name.as_str().magenta().to_string()},
        "LEGENDARY" => {return flip.lowest_bin.name.truecolor(207, 138, 35).to_string()},
        "MYTHIC" => {return flip.lowest_bin.name.truecolor(232, 74, 231).to_string()},
        "SUPREME" => {return flip.lowest_bin.name.truecolor(213, 24, 29).to_string()},
        "SPECIAL" => {return flip.lowest_bin.name.red().to_string()},
        _ => {}
    };
    "".to_string()
}


// pub async fn get_duration_until_api_update() -> u64 {
//     let req = surf::get("https://api.hypixel.net/skyblock/auctions");
//     let res = constants::HTTP_CLIENT1.send(req).await;

//     let json = res.unwrap().body_json().await;
//     let json: serde_json::Value = json.unwrap();

//     let mut last_updated = json.get("lastUpdated").unwrap().as_u64().unwrap() / 1000;
//     let unix_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

//     while last_updated < unix_timestamp {
//         last_updated += 60;
//     }
//     return last_updated;

// }


// Uses cloudflare age headers
pub async fn get_next_api_update() -> u64 {
    let res = constants::HTTP_CLIENT1
        .get("https://api.hypixel.net/skyblock/auctions?page=0")
        .send()
        .await;

    let unix_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    match res {
        Ok(res_unwrap) => match res_unwrap.header("age") {
            Some(age_header) => {
                let age = age_header.as_str().parse::<u64>().unwrap();
                println!("Page age: {}", age);
                // cloudfare doesn't return an exact time in ms, so the +2 accounts for that
                let age = age % 60;
                let time = 60 - age + 2;
                return unix_timestamp + time;
            } 
            None => {
                return unix_timestamp + 60;

            }

            }
        Err(_) => {
            return unix_timestamp + 60;
        }

    }
}


// pub async fn get_update_times() -> Vec<u64> {
//     let unix_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
//     let next_update = get_next_api_update().await;
//     let time_until_update = next_update - unix_timestamp;

//     let mut times: Vec<u64> = vec![];

//     // If the time until next update is 20s or 40s update entire auction house
//     // non-essential auctions
//     times.push(next_update - 20);
//     times.push(next_update - 40);

//     // from 8s prior to update to 3s after update
//     // essential auctions
//     for i in next_update-8..next_update+4 {
//         times.push(i);
//     }

//     times
// }


pub async fn fetch(url: String) -> serde_json::Value{
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



pub fn save_next_update(next_update: u64) {
    fs::write(constants::PATH_TO_NEXT_UPDATE_FILE, next_update.to_string()).expect("Unable to write file");
}