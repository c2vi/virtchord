
#![ allow( warnings ) ]

mod test {
    pub fn hi(){
        crate::config::default_config();
    }
}

pub fn testing(){
    println!("hi");
}


mod keyboard;
mod config;

// the part of the program that is supposed to run as a daemon
mod daemon;


fn main() {

    // some argument parsing
    
    // for now (sth like --run arg by default and) just run the daemon part
    crate::daemon::main();

}

