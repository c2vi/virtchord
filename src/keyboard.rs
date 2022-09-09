
use evdev::{Device, AttributeSet, EventType, InputEvent, Key};
use evdev::uinput::{VirtualDeviceBuilder, VirtualDevice};
use std::collections::HashMap;
use std::thread::sleep;
use std::time::{Duration, SystemTime};
use std::sync::mpsc;
use stringsort::insertsort;
use std::thread;

use std::sync::{Arc, Mutex};
use std::vec;

use crate::config::{Config, ConfigItem};

pub fn key_listen(config: Arc<Mutex<Config>>, tx: mpsc::Sender<u16>, virt_device: Arc<Mutex<VirtualDevice>>){

    let device_path: ConfigItem = ConfigItem::new("keyboard.device-path", &config);
    let max_time: ConfigItem = ConfigItem::new("chord.max-time", &config);

    let mut device = Device::open(device_path.get()).unwrap();

    let mut last_time = SystemTime::now();
    let mut last_key: u16 = 0;
    let mut chord: Vec<u16> = Vec::new();

    loop {
        for event in device.fetch_events().unwrap() {
            let mut virt_device = virt_device.lock().unwrap();
            virt_device.emit(&[event]);
            //if event.event_type() == EventType::SYNCHRONIZATION {
                //println!("SYN: {} - {:?}", event.code(), event.kind());
            //}
            if event.event_type() == EventType::KEY && event.value() == 1 {
                //let dur = event.timestamp().duration_since(last_time).unwrap();

                //let conf_u: u16 = max_time.get().parse().unwrap();
                //let conf_dur = Duration::from_millis(conf_u as u64);

                //if dur < conf_dur {
                tx.send(event.code());
                //}

                // set the "last stuff"
                last_time = event.timestamp();
                last_key = event.code();
            }
        }
    }
}

pub fn key_input(config: Arc<Mutex<Config>>, rx: mpsc::Receiver<u16>, virt_device: Arc<Mutex<VirtualDevice>>){

    let device_path: ConfigItem = ConfigItem::new("keyboard.device-path", &config);
    let max_time: ConfigItem = ConfigItem::new("chord.max-time", &config);

    let mut device = Device::open(device_path.get()).unwrap();

    let timeout = Duration::from_millis(max_time.get().parse().unwrap());
    let mut chord: Vec<u16> = Vec::new();
    chord.push(0);

    loop {
        let key = rx.recv().unwrap();
        chord = [key].to_vec();

        loop {
            if let Ok(key) = rx.recv_timeout(timeout){
                chord.push(key)
            } else {
                if chord.len() > 1 {
                    let config = config.lock().unwrap();
                    let mut chars = String::new();

                    // get chars from codes
                    let map_name = config.config.get("chord.active-key-map")
                        .expect("failed to get \"chord.active-key-map\" from config");
                    let map = config.key_maps.get(&map_name[..])
                        .expect("failed to load the key-map");

                    let chord_map_name = config.config.get("chord.active-profile")
                        .expect("failed to get \"chord.active-key-map\" from config");
                    let chord_map = config.chord_maps.get(&chord_map_name[..])
                        .expect("failed to load the key-map");

                    for key in chord {
                        if let Some(ch) = map.get(&key){
                            chars += ch;
                        } else {
                        }
                    }
                    chars = insertsort(&chars[..]);

                    // get word to insert
                    if let Some(word) = chord_map.get(&chars) {
                        println!("Word: {}", word);
                        send_word(&virt_device, map, chars, word);
                    }

                }
                break;
            }
        }
    }
}

fn send_word(virt_device: &Arc<Mutex<VirtualDevice>>, map: &HashMap<u16, String>, chars: String, word: &String){

    //get reverse key_map
    let mut rev_map: HashMap<char, u16> = HashMap::new();
    for key in map.keys(){
        let key_val = map.get_key_value(key).unwrap();
        rev_map.insert(key_val.1.chars().nth(0).unwrap(), *key_val.0);
    }
    println!("Map: {:?}", rev_map);

    let mut events: Vec<InputEvent> = Vec::new();

    //send delete keys
    let mut i = 0;
    while i < chars.len(){
            //events.push(evdev::InputEvent::new(EventType::MISC, 4, 14 ));
            //events.push(evdev::InputEvent::new(EventType::SYNCHRONIZATION, 0, 0));
            events.push(evdev::InputEvent::new(EventType::KEY, Key::KEY_BACKSPACE.code(), 1));

            //events.push(evdev::InputEvent::new(EventType::MISC, 4, 14 ));
            events.push(evdev::InputEvent::new(EventType::KEY, Key::KEY_BACKSPACE.code(), 0));
            //events.push(evdev::InputEvent::new(EventType::SYNCHRONIZATION, 0, 0));

            i += 1;
    }
    //virt_device.emit(&events);
    //events.clear();
    //println!("Len: {}", events.len());
    //thread::sleep(Duration::from_millis(5));


    for ch in word.chars(){
        let code = *rev_map.get(&ch).unwrap();
        println!("Code: {}", code);



        //events.push(evdev::InputEvent::new(4, 4, code as i32));
        events.push(evdev::InputEvent::new(EventType::KEY, code, 0));
        events.push(evdev::InputEvent::new(EventType::KEY, code, 1));
        //events.push(evdev::InputEvent::new(EventType::SYNCHRONIZATION, 0, 0));

        //events.push(evdev::InputEvent::new(4, 4, code as i32));
        events.push(evdev::InputEvent::new(EventType::KEY, code, 0));
        events.push(evdev::InputEvent::new(EventType::SYNCHRONIZATION, 0, 0));
    }
    events.push(evdev::InputEvent::new(EventType::KEY, 57, 1));
    //events.push(evdev::InputEvent::new(EventType::SYNCHRONIZATION, 0, 0));

    events.push(evdev::InputEvent::new(EventType::KEY, 57, 0));
    //events.push(evdev::InputEvent::new(EventType::SYNCHRONIZATION, 0, 0));

    //events.push(evdev::InputEvent::new(EventType::KEY, Key::KEY_BACKSPACE.code(), 1));
    //events.push(evdev::InputEvent::new(EventType::SYNCHRONIZATION, 0, 0));

    //events.push(evdev::InputEvent::new(EventType::KEY, Key::KEY_BACKSPACE.code(), 0));
    //events.push(evdev::InputEvent::new(EventType::SYNCHRONIZATION, 0, 0));

    //events.push(evdev::InputEvent::new(EventType::KEY, 18, 1));
    //events.push(evdev::InputEvent::new(EventType::SYNCHRONIZATION, 0, 0));

    //events.push(evdev::InputEvent::new(EventType::KEY, 18, 0));
    //events.push(evdev::InputEvent::new(EventType::SYNCHRONIZATION, 0, 0));

    println!("Count: {}", events.len());

    let mut virt_device = virt_device.lock().unwrap();
    virt_device.emit(&events);
            //println!("{:?}", event);
            //virt_device.emit(&[event]);
}

// hopefully temporary as those settings are for me specifically
fn do_reset(){
    std::process::Command::new("setxkbmap")
        .arg("de")
        .output().expect("Failed to run command \"setxkbmap\"");

    std::process::Command::new("xset")
        .arg("r")
        .arg("rate")
        .arg("140")
        .arg("40")
        .output().expect("Failed to run command \"xset\"");
}



