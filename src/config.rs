
use std::process;
use std::sync;
use std::path::PathBuf;
use std::io::prelude::*;
use std::fs::File;
use std::error::Error;
use yaml_rust::YamlLoader;

pub struct Config<'a> {
    remote_url: &'a str,
    // maybe a slice of String
    search_path: &'a str 
}

// still confuse with the lifetime modifier.

static INIT: sync::Once = sync::ONCE_INIT;
static mut CONFIG: Config<'static> = Config {
    remote_url: "",
    search_path: ""
};


impl<'a> Config<'a> {
    pub fn init(remote_url: &'static str, search_path: &'static str) {
        unsafe {
            INIT.call_once(|| {
                CONFIG.remote_url = remote_url;
                CONFIG.search_path = search_path;
            });

            // at exit
        }
    }

    pub fn get() -> &'a Config<'a> {
        unsafe {
            return &CONFIG;
        }
    }

    pub fn load_from_path(p: &PathBuf) {
        // we should borrow the p instead of move.
        // because the p will be used later.
        let mut file = match File::open(p) {
            Err(_) => {
                println!("Configure file not exist in current directory");
                process::exit(1);
            },
            Ok(file) => file,
        };

        let mut buffer = String::new();

        match file.read_to_string(&mut buffer) {
            Err(e) => { 
                println!("could not read {}: {}", p.display(), 
                        Error::description(&e)); 
                process::exit(1); 
            },
            Ok(_) => {},
        }

        let docs = YamlLoader::load_from_str(&buffer).unwrap();
        let doc = docs[0];
        static remote_url: &'static str = doc["remote_url"].as_str().unwrap();
        static search_path: &'static str = doc["search_path"].as_str().unwrap();
        
        Config::init(remote_url, search_path);
    }
}
