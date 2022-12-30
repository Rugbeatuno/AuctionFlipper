use rdev::{simulate, Button, EventType, Key, SimulateError};
use std::collections::HashMap;
use std::{thread, time};

use rdev::{listen, Event};

fn callback(event: Event) {
    match event.name {
        Some(string) => {
            // down arrow
            println!("{}", string);
            if string == "\u{1f}".to_string() {
                menu_routine();
            }
        }
        None => (),
    }
}

pub fn init() {
    thread::spawn(|| {
        if let Err(error) = listen(callback) {
            println!("Error: {:?}", error)
        }
    });
}

pub fn send(text: String) {
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

    simulate(&EventType::KeyPress(Key::KeyT)).unwrap();
    thread::sleep(time::Duration::from_millis(45));
    simulate(&EventType::KeyRelease(Key::KeyT)).unwrap();

    simulate(&EventType::KeyPress(Key::Dot)).unwrap();
    simulate(&EventType::KeyRelease(Key::Dot)).unwrap();

    for char in text.chars().into_iter() {
        let char = char.to_string();
        let key = conversions.get(&char).unwrap().to_owned();

        simulate(&EventType::KeyPress(key)).unwrap();
        simulate(&EventType::KeyRelease(key)).unwrap();
        simulate(&EventType::KeyRelease(key)).unwrap();
    }
    simulate(&EventType::KeyPress(Key::Return)).unwrap();
    simulate(&EventType::KeyRelease(Key::Return)).unwrap();
}

fn menu_routine() {
    let purchase_x: f64 = 721.0;
    let purchase_y: f64 = 387.0;
    let confirm_purchase_x: f64 = 650.0;
    let confirm_purchase_y: f64 = 373.0;

    let delay = time::Duration::from_millis(20);

    simulate(&EventType::MouseMove {
        x: purchase_x,
        y: purchase_y,
    })
    .unwrap();
    thread::sleep(delay);
    click();
    simulate(&EventType::MouseMove {
        x: confirm_purchase_x,
        y: confirm_purchase_y,
    })
    .unwrap();
    thread::sleep(delay);

    click();
}

fn click() {
    simulate(&EventType::ButtonPress(Button::Left)).unwrap();
    simulate(&EventType::ButtonRelease(Button::Left)).unwrap();
}
