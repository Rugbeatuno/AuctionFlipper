use crate::utils;
use crate::structs::*;
use crate::pets::*;
use crate::constants::*;
use crate::auction_house::to_nbt;

use std::{fs, collections::HashMap};
use std::time::{SystemTime, UNIX_EPOCH};





// Remove auctions in json that are older than the provided time in seconds
fn remove_expired_sales(path: String, max_life: u64) {
    let unix_timestamp: u64 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

    let file: String = String::from(path);
    let data: String = fs::read_to_string(&file).expect("Unable to read file");

    let mut load_json: HashMap<String, Vec<SaleData>> = HashMap::new();
    if fs::metadata(&file).unwrap().len() != 0 {
        load_json = serde_json::from_str(&data).unwrap();
    }
    // Sale is stored where each sale is a HashMap stored in a Vec (Some idiot)
    for key in load_json.clone().keys() {// [ [{}] ]
        load_json.get_mut(key).unwrap().retain(|sale| unix_timestamp - &sale.timestamp < max_life);
    }
    let json: String = serde_json::to_string(&load_json).unwrap();

    fs::write(file, &json).expect("Unable to write file");
}


fn update_json(new_sales: Vec<Sale>, path: String) {
    let file: String = String::from(path);
    let data = fs::read_to_string(&file).expect("Unable to read file");
    let mut load_json: HashMap<String, Vec<SaleData>> = HashMap::new();

    if fs::metadata(&file).unwrap().len() != 0 {
        load_json = serde_json::from_str(&data).unwrap();
    }

    for sale in new_sales {
        load_json.entry(sale.id).or_default().push(sale.data);
    }

    let json: String = serde_json::to_string(&load_json).unwrap();

    fs::write(file, &json).expect("Unable to write file");
}


// Pets are formatted as TIER-NAME-LEVEL-PET_ITEM, and with candy or skins are ignored
// Enchanted books are ENCHANT_LEVEL
// Normal items are just RARITY-ISRECOMBED-REFORGE-NAME-STARS-SPECIALENCHANTS
fn format_sale_for_target(auction: &serde_json::Value) -> Option<Sale> {
    let nbt = &to_nbt(serde_json::from_value(auction.get("item_bytes").unwrap().to_owned()).unwrap()).unwrap().i[0];
    let id = nbt.tag.extra_attributes.id.as_ref().unwrap();

    // Convert from milliseconds to seconds
    let mut sale = Sale {
        id: id.to_string(),
        data: SaleData {
            price: auction.get("price").unwrap().as_u64().unwrap(),
            timestamp: auction.get("timestamp").unwrap().as_u64().unwrap() / 1000, 
        },
    };

    if id == "PET" {
        let pet_data = is_pet(nbt, "".to_string()).unwrap();
        if pet_data.candy > 0 {return None}
        sale.id = format!("{}-{}-{}-{}", pet_data.tier, pet_data.name, pet_data.level, pet_data.item);
    }

    else if id == "ENCHANTED_BOOK" {
        match nbt.tag.extra_attributes.enchantments.as_ref() {
            Some(e) => {
                if e.len() == 1 { 
                    for (enchant, level) in e {
                        sale.id = format!("{}_{}", enchant, level);
                    }
                }
            }
            None => {return None}
        };
    }

    // Normal items are just ISRECOMBED-REFORGE-NAME-STARS
    else {
        let is_recombed: u8 = nbt.tag.extra_attributes.rarity_upgrades?;
        let reforge: &String = nbt.tag.extra_attributes.modifier.as_ref()?;
        let id: String = sale.id;
        let stars: u8 = nbt.tag.extra_attributes.dungeon_item_level?;
        sale.id = format!("{}-{}-{}-{}", is_recombed, reforge, id, stars);
    }
    Some(sale)
}

fn format_sale_for_sales(auction: &serde_json::Value) -> Sale {
    let nbt = &to_nbt(serde_json::from_value(auction.get("item_bytes").unwrap().to_owned()).unwrap()).unwrap().i[0];
    let id = nbt.tag.extra_attributes.id.as_ref().unwrap();


    // Convert from milliseconds to seconds
    let mut sale = Sale {
        id: id.to_string(),
        data: SaleData {
            price: auction.get("price").unwrap().as_u64().unwrap(),
            timestamp: auction.get("timestamp").unwrap().as_u64().unwrap() / 1000, 
            // buyer: auc.get("buyer").unwrap().as_str().unwrap().to_string(),
        },
    };

    if id == "PET" {
        let pet_data = is_pet(nbt, "".to_string()).unwrap();
        sale.id = format!("{} {} {} {}", pet_data.tier, pet_data.name, pet_data.level, false);
    }

    else if id == "ENCHANTED_BOOK" {
        match nbt.tag.extra_attributes.enchantments.as_ref() {
            Some(e) => {
                if e.len() == 1 { 
                    for (enchant, level) in e {
                        sale.id = format!("{}_{}", enchant, level);
                    }
                }
            }
            None => {}
        };
    }

    sale
}


// This function accomplishes to things
// 1) we update sales.json which is much less detailed than targets.json
// 2) update targets.json with detailed keys
pub async fn update_auctions_ended() {
    let req = utils::fetch(String::from("https://api.hypixel.net/skyblock/auctions_ended")).await;
    let auctions = req.get("auctions").unwrap().as_array().unwrap();
    println!("Auctions ended: {}", auctions.len());

    let mut sales: Vec<Sale> = Vec::new();
    let mut targets: Vec<Sale> = Vec::new();

    for auc in auctions {
        // Guard case for non-BIN auctions
        if !auc.get("bin").unwrap().as_bool().unwrap() {
            continue;
        }

        sales.push(format_sale_for_sales(auc));
        match format_sale_for_target(auc) {
            Some(sale) => {targets.push(sale)},
            None => {}
        }
    }

    update_json(sales, PATH_TO_SALES.to_string());
    update_json(targets, PATH_TO_TARGETS.to_string());

    remove_expired_sales(PATH_TO_SALES.to_string(), SECONDS_IN_WEEK);
    remove_expired_sales(PATH_TO_TARGETS.to_string(), SECONDS_IN_MONTH);

    // {"auction_id":"3fbfc20a9e484d3a87d40a6d308cb459",
    // "bin":true,
    // "buyer":"e5f012df66824a4ea5d7eba2cadbad23",PDcH/6sN7MjfLgl8TyROlsUfWL+GL25vhY+SFzQ7Z7Q3/3gtdOuPv6GEfPiLnGFMsDa69bgifknG2RJStyW/h/j58Q45pabEo8jmpXIf47jtCnxh36Xn0in53v/1K/69Jw8fkHePSZmxmNSnWbDqdNmQXKCuBhmVqxRaVyNi1qhjXyEyhVijha8LkJbMZshW3FJhc1yk9131InkcN8wQ5+TWbcJJVztmLtUxBhTJr7iyft9yClywlIDwgTIIpVYlZflXH1WYm+YLPkdJAnYBShXQwXkqm11L/B7vfwvywFXSPBJ1qtaK7tjpONS+taeo0DOkQd29/Z207moBdCniqlUVhc1VS0Lf/DfLqIH739he2blMTRVnjZ5nSpWlK+eDfGF7DQ/chHUORcWoSs6oh1ZjQnMnaHzdulb67h6z1wGd1Sbi4Ytw2FefW0hvqR9TCr25voklFN5+hZceqrMwhe5kbZMeFUrK56VGSF7nN64rzN1in8Q+CdR6iayq14CVnQhlbc35CnLc3xZOT05Oz8dHFT+z4/PzZrAtbQhVKw/6ff/Rg84wvED4n5AvU123KseZzKtWxUtZAH3ZPfraaH1karaSyaHqwnShjphL2IjeI0nAUOWIQDpyB58ZOIuOh40aCp6FwvZRHPeibJV+VKCeUdG8YhtIbycAJMAicQewnTux5ieOlsfC9aJAELu1Vb6FknuaoYXNFknrNgn74cjo7uRxfHJ2en102d+nD+/UyU2MWSM3pwn1Zr8GladeAatDtwrZtGtm+7KbtNF+m7TSTdYsg82ak2pf+8p9xqQ30XdisKkq/h8MgoPUdOXwQC2eAQ+GMRrHvRD56Pmn3opHXgx2bL9BYvljCB4MD78D3mXcYRuzpjwAbsD1uJoKI4W898rF2rAQAAA==",
    // "price":1700000,
    // "item_bytes":"H4sIAAAAAAAAAF1U3W7jRBQ+adqSZlkqBAgQPzuCrkSFvPVP7MS9a0nTjbS0q2Z/hFarajxzHFt1PNHMeEPfgCuegL1D6nv0UXrHHU+AOLazILhIxj7nO9/55vy4D7ADnbwPAJ0N2Mhlx+nA1g+qKm2nD13L5zuwiaXI1ohu8aaAboOEHnTg3vMy0civeFJgpws7j3OJk4L
    // "seller":"f1678ca543524f65935840f2a6857a7e",
    // "seller_profile":"989723505eda48bd83566c2c7d5c303b",
    // "timestamp":1650575019125}
    //l
    // {TAG_String('bossId'): 6036f586-c454-4109-bd97-06caf5c01fa6, TAG_String('spawnedFor'): 755d18d3-3e33-492b-911b-1f9c2164b30e, TAG_String('modifier'): wise, TAG_String('id'): WISE_DRAGON_BOOTS, TAG_Compound('enchantments'): {5 Entries}, TAG_String('uuid'): e7331058-a49c-4e7c-8892-62e122161681, TAG_String('timestamp'): 4/1/22 1:56 PM}


    
}