use std::collections::HashMap;
use std::fs;
use crate::constants;
use std::time::{SystemTime, UNIX_EPOCH};

const PATH_TO_PAGES: &str = "./src/files/pages.txt";

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
    let req = surf::get("https://api.hypixel.net/skyblock/auctions");
    let res = constants::HTTP_CLIENT1.send(req).await;

    let json = res.unwrap().body_json().await;
    let json: serde_json::Value = json.unwrap();
    let mut last_updated = json.get("lastUpdated").unwrap().as_u64().unwrap();
    let unix_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    last_updated /= 1000;
    while last_updated <= unix_timestamp {
        last_updated += 60;
    }
    return last_updated;

}

pub async fn update_pages_file(p: Option<u64>) {
    let mut pages: u64 = p.unwrap_or(0);
    let file: String = String::from(PATH_TO_PAGES);
    
    if pages == 0 {
        let req = surf::get("https://api.hypixel.net/skyblock/auctions");
        let res = constants::HTTP_CLIENT1.send(req).await;
        let json = res.unwrap().body_json().await;
        let json: serde_json::Value = json.unwrap();
    
        pages = match json.get("totalPages") {
            Some(p) => p.as_u64().unwrap(),
            None => 0
        };
    }
    
    let json: String = serde_json::to_string(&pages).unwrap();
    fs::write(file, &json).expect("Unable to write file");

}

// pub fn get_page_file() -> u64 {
//     let file: String = String::from(PATH_TO_PAGES);
//     let data = fs::read_to_string(&file).expect("Unable to read file");
//     let mut pages: u64 = 0;
//     if fs::metadata(&file).unwrap().len() != 0 {
//         pages = data.parse().unwrap();
//     }
//     return pages;

// }