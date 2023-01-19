use rdev::{listen, simulate, Button, EventType, Key};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{fs, thread, time, vec};

use crate::constants::{
    CLOSE_CHAT, CONFIRM_PURCHASE_X, CONFIRM_PURCHASE_Y, DEBUG, PATH_TO_FLIPS,
    PATH_TO_NEXT_UPDATE_FILE, PURCHASE_X, PURCHASE_Y,
};
use crate::structs::{Auction, Flip, FlipType};
use crate::utils::get_unix_timestamp;

fn load_flips() -> Vec<Flip> {
    let data = fs::read_to_string(PATH_TO_FLIPS).expect("Unable to read file");
    let mut flips: Vec<Flip> = vec![];
    if fs::metadata(PATH_TO_FLIPS).unwrap().len() != 0 {
        flips = serde_json::from_str(&data).unwrap();
    }
    flips
}

fn are_different(flips1: &Vec<Flip>, flips2: &Vec<Flip>) -> bool {
    if flips1.len() != flips2.len() {
        return true;
    }
    if flips1.len() == 0 && flips2.len() == 0 {
        return false;
    }
    if flips1[0].id == flips2[0].id {
        return false;
    } else {
        return true;
    }
}

pub fn file_modified_time_in_seconds(path: &str) -> i32 {
    fs::metadata(path)
        .unwrap()
        .modified()
        .unwrap()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i32
}

pub fn init() {
    thread::spawn(|| {
        let mut prev_flips: Vec<Flip> = load_flips();
        prev_flips.sort_by_key(|a| a.flip_price);
        prev_flips.reverse();

        if prev_flips.len() == 0 {
            let random_auc = Auction {
                id: "".to_string(),
                seller: "".to_string(),
                uuid: "".to_string(),
                tier: "".to_string(),
                name: "".to_string(),
                lore: "".to_string(),
                extra: "".to_string(),
                price: 0,
                start: 0,
                end: 0,
                pet_data: None,
                additional_price_data: None,
                item_bytes: serde_json::Value::Null,
                category: "".to_string(),
            };

            prev_flips.push(Flip {
                id: "".to_string(),
                internal_id: "".to_string(),
                flip_type: FlipType::LBin,
                flip_price: 0,
                profit_percentage: 0,
                lowest_bin: random_auc.clone(),
                next_lowest_bin: random_auc.clone(),
                similar_auctions: 0,
                volume_last_week: 0,
                estimated_target_price: 0,
                target_name: "".to_string(),
            })
        }

        let mut counter = 0;
        let mut blocked_until = 0;

        listen(move |event| {
            let now: i32 = get_unix_timestamp();

            match event.name {
                Some(string) => {
                    // println!("{}", string);
                    let mut new_flips = load_flips();
                    new_flips.sort_by_key(|a| a.flip_price);
                    new_flips.reverse();

                    if new_flips.len() == 0 {
                        let last_mod = file_modified_time_in_seconds(PATH_TO_NEXT_UPDATE_FILE);
                        let time_since: i32 = now - last_mod;
                        if now >= last_mod && time_since < 3 {
                            blocked_until = now + 1;
                        }
                    }

                    if string == "\u{1f}".to_string() && now > blocked_until {
                        menu_routine();
                    }
                    // see whether the flips are new
                    // we can take advantage of the fact that we will never see the same auction twice
                    // there compare the auction ids of the first auctions

                    if are_different(&prev_flips, &new_flips) {
                        counter = 0;
                        prev_flips = new_flips.clone();
                    }
                    if new_flips.len() > 0 {
                        if string == "\u{1d}".to_string() {
                            if counter < new_flips.len() - 1 {
                                counter += 1;
                            }

                            send("t".to_string());
                            std::thread::sleep(std::time::Duration::from_millis(50));
                            send(format!("/viewauction {}*", &new_flips[counter].id));
                            println!("Opening auction {}/{}", counter + 1, new_flips.len());
                        }
                        if string == "\u{1c}".to_string() {
                            if counter > 0 {
                                counter -= 1;
                            }

                            send("t".to_string());
                            std::thread::sleep(std::time::Duration::from_millis(50));
                            send(format!("/viewauction {}*", &new_flips[counter].id));
                            println!("Opening auction {}/{}", counter + 1, new_flips.len());
                        }
                    }
                }
                None => (),
            }
        })
    });
}

pub fn send(text: String) {
    if DEBUG {
        return;
    }
    if text == CLOSE_CHAT {
        simulate(&EventType::KeyPress(Key::Escape)).unwrap();
        return;
    }
    let mut conversions: HashMap<String, Key> = HashMap::new();
    conversions.insert("a".to_string(), Key::KeyA);
    conversions.insert("b".to_string(), Key::KeyB);
    conversions.insert("c".to_string(), Key::KeyC);
    conversions.insert("d".to_string(), Key::KeyD);
    conversions.insert("e".to_string(), Key::KeyE);
    conversions.insert("f".to_string(), Key::KeyF);
    conversions.insert("g".to_string(), Key::KeyG);
    conversions.insert("h".to_string(), Key::KeyH);
    conversions.insert("i".to_string(), Key::KeyI);
    conversions.insert("j".to_string(), Key::KeyJ);
    conversions.insert("k".to_string(), Key::KeyK);
    conversions.insert("l".to_string(), Key::KeyL);
    conversions.insert("m".to_string(), Key::KeyM);
    conversions.insert("n".to_string(), Key::KeyN);
    conversions.insert("o".to_string(), Key::KeyO);
    conversions.insert("p".to_string(), Key::KeyP);
    conversions.insert("q".to_string(), Key::KeyQ);
    conversions.insert("r".to_string(), Key::KeyR);
    conversions.insert("s".to_string(), Key::KeyS);
    conversions.insert("t".to_string(), Key::KeyT);
    conversions.insert("u".to_string(), Key::KeyU);
    conversions.insert("v".to_string(), Key::KeyV);
    conversions.insert("w".to_string(), Key::KeyW);
    conversions.insert("x".to_string(), Key::KeyX);
    conversions.insert("y".to_string(), Key::KeyY);
    conversions.insert("z".to_string(), Key::KeyZ);
    conversions.insert("1".to_string(), Key::Num1);
    conversions.insert("2".to_string(), Key::Num2);
    conversions.insert("3".to_string(), Key::Num3);
    conversions.insert("4".to_string(), Key::Num4);
    conversions.insert("5".to_string(), Key::Num5);
    conversions.insert("6".to_string(), Key::Num6);
    conversions.insert("7".to_string(), Key::Num7);
    conversions.insert("8".to_string(), Key::Num8);
    conversions.insert("9".to_string(), Key::Num9);
    conversions.insert("0".to_string(), Key::Num0);
    conversions.insert("/".to_string(), Key::Slash);
    conversions.insert(" ".to_string(), Key::Space);
    conversions.insert("*".to_string(), Key::Return);

    // simulate(&EventType::KeyPress(Key::Dot)).unwrap();
    // simulate(&EventType::KeyRelease(Key::Dot)).unwrap();

    for char in text.chars().into_iter() {
        let char = char.to_string();
        let key = conversions.get(&char).unwrap().to_owned();

        simulate(&EventType::KeyPress(key)).unwrap();
        simulate(&EventType::KeyRelease(key)).unwrap();
        simulate(&EventType::KeyRelease(key)).unwrap();
    }
}

fn menu_routine() {
    let delay = time::Duration::from_millis(20);

    simulate(&EventType::MouseMove {
        x: PURCHASE_X,
        y: PURCHASE_Y,
    })
    .unwrap();
    thread::sleep(delay);
    click();
    simulate(&EventType::MouseMove {
        x: CONFIRM_PURCHASE_X,
        y: CONFIRM_PURCHASE_Y,
    })
    .unwrap();
    thread::sleep(delay);

    click();
}

fn click() {
    simulate(&EventType::ButtonPress(Button::Right));
    simulate(&EventType::ButtonRelease(Button::Right));
}
