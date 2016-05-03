// Copyright 2016 Towry Wang. All rights reserved.
//
// This is a simple proxy server that when receive a 
// request, do not forward it first but check if there
// is an available static file in local, if it does, 
// send the local static file instead use the the one 
// from remote server.

extern crate flight_proxy;

use std::net::{TcpListener};
use std::thread;
use std::env;

use flight_proxy::*;

static PRG_NAME: &'static str = "flight-proxy";

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3456").unwrap();

    // init the config
    let mut p = env::current_dir().unwrap();
    p.push("proxy");
    p.set_extension("yml");

    let mut _config = config::Config::new();
    _config.load_from_path(&p);

    // running the server
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
