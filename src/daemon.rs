
use std::io::Read;
use std::{hash::Hasher, collections::HashMap};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::fs::File;
use evdev::uinput::{VirtualDeviceBuilder, VirtualDevice};
use evdev::{Device, AttributeSet, EventType, InputEvent, Key};

use signal_hook::{consts::SIGWINCH, consts::SIGINT, iterator::Signals};

use crate::keyboard;
use crate::config::{Config, ConfigItem, default_config};

pub fn main(){

    // load config
    // into Arc<Mutex<>> so that gui/config and keyboard threads can acces at same time
    // settings that update in real time
    
    let config_dir = "/home/sebastian/.config/virtchord";
    
    let mut config: Arc<Mutex<Config>> = Arc::new(Mutex::new(default_config()));
    let key_listen_config = config.clone();
    let key_input_config = config.clone();

    let (tx, rx) = mpsc::channel();

    let device_path: ConfigItem = ConfigItem::new("keyboard.device-path", &config);

    let mut device = Device::open(device_path.get()).unwrap();
    let mut virt_device_hi = create_virt_keyboard(&device);
    let mut virt_device: Arc<Mutex<VirtualDevice>> = Arc::new(Mutex::new(virt_device_hi));

    let mut virt_device_listen = virt_device.clone();
    let mut virt_device_input = virt_device.clone();

    // start the keyboard event listening thread
    thread::spawn(move || {
        keyboard::key_listen(key_listen_config, tx, virt_device_listen);
    });

    // the thread that actually inputs things with the virtual device

    thread::spawn(move || {
        keyboard::key_input(key_input_config, rx, virt_device_input);
    });

    //the main thread listens for commands from the pipe that an unpriveleged user can write to

    let mut cmd = String::new();

    loop {
        let mut pipe = File::open(String::from(config_dir) + "/pipe1")
            .expect("couldn't open the pipe that is used for ipc. \"~/.config/virtchord/pipe1\"");
        cmd.clear();
        pipe.read_to_string(&mut cmd).unwrap();
        println!("Read: '{}'", cmd);
    }
}

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



