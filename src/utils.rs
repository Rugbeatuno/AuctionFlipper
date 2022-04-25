use std::collections::HashMap;
use crate::constants;

use std::time::{SystemTime, UNIX_EPOCH};
use chrono::*;


// pub fn capitalize(s: &str) -> String {
//     s[0..1].to_uppercase() + &s[1..]
// }

pub fn prettify_large_nums(num: usize) -> String {
    let mut conversions: HashMap<String, char> = HashMap::new();
    conversions.insert("1".to_string(), 'K');
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

pub async fn get_duration_until_api_update() -> u64 {
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
                let time = 60 - age + 2;
                return unix_timestamp + time;
            } 
            None => {
                let res = 60 - Local::now().second();
                let res = res as u64;
                return unix_timestamp + res;
                
            }

            }
        Err(_) => {
            let res = 60 - Local::now().second();
            let res = res as u64;
            return unix_timestamp + res;
        }

    }
}