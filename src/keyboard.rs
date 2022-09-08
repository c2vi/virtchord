
use evdev::{Device, AttributeSet, EventType, InputEvent, Key};
use evdev::uinput::{VirtualDeviceBuilder, VirtualDevice};
use std::thread::sleep;
use std::time::{Duration, SystemTime};
use std::sync::mpsc;
use stringsort::insertsort;

use std::sync::{Arc, Mutex};
use std::vec;

use crate::config::{Config, ConfigItem};

pub fn create_virt_keyboard(device: &Device) -> VirtualDevice {


    let mut keys = device.supported_keys().expect("Error while getting the supported_keys of the keyboard");
        //AttributeSet::<Key>::new();
    //keys.insert(Key::KEY_A);

    let mut device = VirtualDeviceBuilder::new().unwrap()
        .name("Fake Keyboard")
        .with_keys(&keys).unwrap()
        .build()
        .unwrap();

    return device
}

pub fn key_listen(config: Arc<Mutex<Config>>, tx: mpsc::Sender<u16>){

    let device_path: ConfigItem = ConfigItem::new("keyboard.device-path", &config);
    let max_time: ConfigItem = ConfigItem::new("chord.max-time", &config);

    let mut device = Device::open(device_path.get()).unwrap();

    let mut last_time = SystemTime::now();
    let mut last_key: u16 = 0;
    let mut chord: Vec<u16> = Vec::new();

    loop {
        for event in device.fetch_events().unwrap() {
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

pub fn key_input(config: Arc<Mutex<Config>>, rx: mpsc::Receiver<u16>){

    let device_path: ConfigItem = ConfigItem::new("keyboard.device-path", &config);
    let max_time: ConfigItem = ConfigItem::new("chord.max-time", &config);

    let mut device = Device::open(device_path.get()).unwrap();
    let mut virt_device = create_virt_keyboard(&device);

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
                    } else {
                        println!("Chars: {}", chars);
                    }

                    //do backspace events

                    // do insert events
                }
                break;
            }
        }

    }



                    //if chord.is_empty() {
                        //chord.push(last_key);
                        //chord.push(event.code());
                    //} else {
                        //chord.push(event.code())
                    //}
                    //if !chord.is_empty() {
                    //}


            //let up = evdev::InputEvent::new(EventType::KEY, Key::KEY_A.code(), 1);
            //let down = evdev::InputEvent::new(EventType::KEY, Key::KEY_A.code(), 0);
            //virt_device.emit(&[up, down]);
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



