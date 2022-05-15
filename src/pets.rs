use crate::structs::*;

use std::collections::HashMap;

pub fn is_pet(nbt: &PartialNbtElement, name: String) -> Option<Pet> {
    if nbt.tag.extra_attributes.pet_info.is_none() {
        return None;
    }

    // Is a pet
    let res = serde_json::from_str(&nbt.tag.extra_attributes.pet_info.as_ref().unwrap());
    let json: serde_json::Value = res.unwrap();

    let level;
    if name == "" {
        let exp = json["exp"].as_f64().unwrap() as u64;
        level = calc_pet_level(exp, json["tier"].as_str().unwrap().to_string());
    } 
    else {
        // [Lvl 100] Tiger 
        // 100] - > 100
        let split = name.split(" ");
        let vec: Vec<&str> = split.collect();
        level = vec[1].to_string().replace("]", "").parse::<u64>().unwrap();
    }

    let pet = Pet {
        name: json["type"].as_str().unwrap().to_string(),
        level: level,
        tier: json["tier"].as_str().unwrap().to_string(),
        item: if json["heldItem"].as_str() != None {json["heldItem"].as_str().unwrap().to_string()} else {"".to_string()},
        candy: if json["candyUsed"].as_u64() != None {json["candyUsed"].as_u64().unwrap()} else {0},
    };
    return Some(pet);
}


fn calc_pet_level(xp: u64, tier: String) -> u64 {
    let mut offset: HashMap<String, u64> = HashMap::new();
    offset.insert("COMMON".to_string(), 0);
    offset.insert("UNCOMMON".to_string(), 6);
    offset.insert("RARE".to_string(), 11);
    offset.insert("EPIC".to_string(), 16);
    offset.insert("LEGENDARY".to_string(), 20);
    offset.insert("MYTHIC".to_string(), 20);

    let pet_levels: Vec<u64> = vec![100, 110, 120, 130, 145, 160, 175, 190, 210, 230, 250, 275, 300, 330, 360, 400, 440, 490, 540, 600, 660, 730, 800, 880, 960, 1050, 1150, 1260, 1380, 1510, 1650, 1800, 1960, 2130, 2310, 2500, 2700, 2920, 3160, 3420, 3700, 4000, 4350, 4750, 5200, 5700, 6300, 7000, 7800, 8700, 9700, 10800, 12000, 13300, 14700, 16200, 17800, 19500, 21300, 23200, 25200, 27400, 29800, 32400, 35200, 38200, 41400, 44800, 48400, 52200, 56200, 60400, 64800, 69400, 74200, 79200, 84700, 90700, 97200, 104200, 111700, 119700, 128200, 137200, 146700, 156700, 167700, 179700, 192700, 206700, 221700, 237700, 254700, 272700, 291700, 311700, 333700, 357700, 383700, 411700, 441700, 476700, 516700, 561700, 611700, 666700, 726700, 791700, 861700, 936700, 1016700, 1101700, 1191700, 1286700, 1386700, 1496700, 1616700, 1746700, 1886700];
    let offset = offset.get(&tier).unwrap().clone();
    let mut xp = xp;
    let mut level = 1;

    for i in offset..pet_levels.len().try_into().unwrap() {
        let req_xp: u64 = pet_levels[i as usize];
        if xp >= req_xp {
            xp -= req_xp;
            level += 1;
        }
    }
    level

}

pub fn pet_case(filtered_auctions: &mut Vec<Auction>, auction: &mut Auction) {
    if auction.tier == "COMMON" || auction.tier == "UNCOMMON" || auction.tier == "RARE" {
        return;
    }
    // Candied pet prices fluctuate
    if auction.lore.contains("Pet Candy Used") { 
        return;
    }
    // Guards against skinned pets
    if auction.name.contains("✦") {
        return;
    }

    // println!("{:?}", auction.name);

    let pet_level: u64 = if auction.name.contains("100") {100} else {1};
    let name_split: Vec<&str> = auction.name.split(" ").collect();
    let pet_name = name_split[2].clone().to_uppercase();
    let tier_boosted = if auction.lore.contains("Tier Boost") {true} else {false};

    // let nbt = &to_nbt(serde_json::from_value(unparsed_auction.get("item_bytes").unwrap().to_owned()).unwrap()).unwrap().i[0];
    // auction.pet_data = is_pet(nbt, unparsed_auction.get("item_name").unwrap().as_str().unwrap().to_string());
    // let pet_data = auction.clone().pet_data.unwrap();
    // auction.id = format!("{} {} {} {} {}", pet_data.tier, pet_data.name, pet_data.level, pet_data.item, pet_data.candy);
    auction.id = format!("{} {} {} {}", auction.tier, pet_name, pet_level, tier_boosted);
    filtered_auctions.push(auction.clone());
}
