
use std::process;
use std::path::PathBuf;
use std::io::prelude::*;
use std::fs::File;
use std::error::Error;
use yaml_rust::YamlLoader;

pub struct Config {
    pub remote_url: String,
    // maybe a slice of String
    pub search_path: String 
}

// still confuse with the lifetime modifier.

impl Config {
    pub fn new() -> Config {
        let config = Config {
            remote_url: "".to_owned(),
            search_path: "".to_owned()
        };

        return config;
    }

    pub fn load_from_path(&mut self, p: &PathBuf) {
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
        let doc = &docs[0];
        // static remote_url: &'static str = doc["remote_url"].as_str().unwrap();
        // static search_path: &'static str = doc["search_path"].as_str().unwrap();
        
        // what the ...
        // ther must be a better way.
        self.remote_url = doc["remote_url"].as_str().unwrap().to_string();
        self.search_path = doc["search_path"].as_str().unwrap().to_string();
    }
}
