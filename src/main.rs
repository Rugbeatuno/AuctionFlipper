use auctionhouse::flipper;

use warp::{Filter};
use std::{fs};


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

    flipper().await;
    Ok(())
}


async fn get_flips() -> Result<impl warp::Reply, warp::Rejection> {
    let file: String = String::from("./src/files/flips.json");
    let data = fs::read_to_string(&file).expect("Unable to read file");
    let load_json: serde_json::Value = serde_json::from_str(&data).unwrap();
    Ok(warp::reply::json(&load_json))
}
