// Copyright 2016 Towry Wang. All rights reserved.
//
// This is a simple proxy server that when receive a 
// request, do not forward it first but check if there
// is an available static file in local, if it does, 
// send the local static file instead use the the one 
// from remote server.

extern crate yaml_rust;
extern crate flight_proxy;

use std::io::prelude::*;
use std::fs::File;
use std::net::{TcpListener};
use std::thread;
use std::env;
use std::error::Error;

// use yaml_rust::{YamlLoader};
use flight_proxy::*;

static PRG_NAME: &'static str = "flight-proxy";

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3456").unwrap();


    let mut p = env::current_dir().unwrap();
    p.push("proxy");
    p.set_extension("yml");
    
    // we should borrow the p instead of move.
    // because the p will be used later.
    let mut file = match File::open(&p) {
        Err(_) => {
            println!("Configure file not exist in current directory");
            std::process::exit(1);
        },
        Ok(file) => file,
    };

    let mut buffer = String::new();

    match file.read_to_string(&mut buffer) {
        Err(e) => panic!("could not read {}: {}", p.display(), 
                    Error::description(&e)),
        Ok(_) => {},
    }

    // let docs = YamlLoader::load_from_str(&buffer).unwrap();
    // let doc = &docs[0];

    // println!("{}", doc["remote_static"].as_str().unwrap());

    println!("{} is running at port: {}", PRG_NAME, 3456);

    for stream in listener.incoming() {
        match stream {
            Err(_) => {}
            Ok(stream) => {
                thread::spawn(move || {
                    request::handle_request(stream);
                });
            }
        }
    }

    drop(listener);
}
