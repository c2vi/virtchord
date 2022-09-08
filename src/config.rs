
use std::collections::HashMap;

use std::hash::Hash;
use std::sync::{Arc, Mutex};
use std::fs::File;
use std::io::Read;
use stringsort::insertsort;

//config reading currently is a f**ing mess

pub fn default_config() -> Config {
    Config{
        config: read_main(),

        chord_maps: HashMap::from([
            (String::from("en"), read_chord_map("./chords-en.conf")),
        ]),

        key_maps: HashMap::from([
            (String::from("my"), read_key_map("./key-map.conf")),
        ]),
    }
}

pub struct Config{
    pub config: HashMap<String, String>,
    pub chord_maps: HashMap<String, HashMap<String, String>>,
    pub key_maps: HashMap<String, HashMap<u16, String>>,
}

pub struct ConfigItem<'a>{
    key: String,
    config: &'a Arc<Mutex<Config>>,
}

impl ConfigItem<'_>{
    pub fn new<'a>(key: &'a str, config: &'a Arc<Mutex<Config>>) -> ConfigItem<'a>{
        ConfigItem{
            key:String::from(key),
            config: &config,
        }
    }

    pub fn get(&self) -> String {
        // sorry
        self.config.lock().unwrap()
            .config.get(&self.key).unwrap().to_string()
    }
}

fn read_key_map(file_name: &str) -> HashMap<u16, String>{
    let mut file = File::open(String::from("/home/sebastian/.config/virtchord/") + file_name)
        .expect("could not open key_map config file");
    let mut st = String::new();
    let mut map: HashMap<u16, String> = HashMap::new();

    file.read_to_string(&mut st).expect("Failed to read key-map config file");

    let stvec: Vec<&str> = st.split("\n").collect();

    for line in stvec {
        if line.len() > 2 && &line[..1] != "#" {
            let key: u16 = line.split("=").collect::<Vec<&str>>()[0].parse().unwrap();
            let val: &str = line.split("=").collect::<Vec<&str>>()[1];
            map.insert(key, String::from(val));
        }
    }

    return map;
}

fn read_chord_map(file_name: &str) -> HashMap<String, String>{
    let mut file = File::open(String::from("/home/sebastian/.config/virtchord/") + file_name)
        .expect("could not open chord_map config file");
    let mut st = String::new();
    let mut map: HashMap<String, String> = HashMap::new();

    file.read_to_string(&mut st).expect("Failed to read chord_map config file");

    let stvec: Vec<&str> = st.split("\n").collect();

    for line in stvec {
        if line.len() > 2 && &line[..1] != "#" {
            let key: &str = line.split("=").collect::<Vec<&str>>()[0];
            let val: &str = line.split("=").collect::<Vec<&str>>()[1];
            map.insert(insertsort(key), String::from(val));
        }
    }
    return map;
}

fn read_main() -> HashMap<String, String>{
    let mut file = File::open("/home/sebastian/.config/virtchord/main.conf")
        .expect("could not open main config file");
    let mut st = String::new();
    let mut map: HashMap<String, String> = HashMap::new();

    file.read_to_string(&mut st).expect("Failed to read main config file");

    let st_vec: Vec<&str> = st.split("\n").collect();

    for line in st_vec {
        if line.len() > 2 && &line[..1] != "#" {
            let ve: Vec<&str> = line.split("=").collect();
            map.insert(String::from(ve[0]), String::from(ve[1]));
        };
    }
    return map;
}




