
use std::io::Read;
use std::{hash::Hasher, collections::HashMap};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::fs::File;

use signal_hook::{consts::SIGWINCH, consts::SIGINT, iterator::Signals};

use crate::keyboard;
use crate::config::{Config, default_config};

pub fn main(){

    // load config
    // into Arc<Mutex<>> so that gui/config and keyboard threads can acces at same time
    // settings that update in real time
    
    let config_dir = "/home/sebastian/.config/virtchord";
    
    let mut config: Arc<Mutex<Config>> = Arc::new(Mutex::new(default_config()));
    let key_listen_config = config.clone();
    let key_input_config = config.clone();

    let (tx, rx) = mpsc::channel();

    // start the keyboard event listening thread
    thread::spawn(move || {
        keyboard::key_listen(key_listen_config, tx);
    });

    // the thread that actually inputs things with the virtual device

    thread::spawn(move || {
        keyboard::key_input(key_input_config, rx);
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
    // use custom signals to tell the daemon to do stuff
    // - 34: reload config
    // - 35: open quick GUI
    // - 36: open proper GUI

    // DOES NOT WORK, bcs signals yould have to be sent as root, which is annoying.
    //let mut signals = Signals::new(&[34]).unwrap();

    //for sig in signals.forever() {
        //if sig == 34 {
            //println!("reloading config, does not work yet at all");
            //let temp = config.clone();
            //temp.lock().unwrap().config
                //.entry("chord-max-time")
                //.and_modify(|val| *val = "20ms");
        //}
    //}



