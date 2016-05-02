use std::io::prelude::*;
use std::net::TcpStream;
use regex::Regex;
use header::Header as Header;

#[allow(dead_code)]
pub struct Request {
    hostname: String,
    path: String,
    headers: Vec<Header>
}

static RES_FOR_BROWSER: &'static str = "HTTP/1.0 200 OK\r\nContent-Type: text/html\r\n\r\nOK";

// Read the request and return a Request object.
// The information that we need is hostname, path and some
// headers.
fn read_request(stream: &mut TcpStream) -> Option<Request> {
    let mut buf: [u8; 1024] = [0; 1024];
    
    match stream.read(&mut buf) {
        Err(_) => {
            return None;
        },
        Ok(_) => {}
    }

    let header_buf = String::from_utf8_lossy(&mut buf);
    let mut iterator = header_buf.split("\r\n");
    let request_line: &str = iterator.next().unwrap();
    let request_tokens: Vec<&str> = request_line.split(" ").collect();
    let url: &str = request_tokens[1];

    let re = Regex::new(r"(\w*?)://(.*?)/(.*)").unwrap();

    let caps = match re.captures(url) {
        None => { return None },
        Some(value) => { value }
    };

    let mut request = Request {
        hostname: caps.at(3).unwrap().to_string(),
        path: caps.at(3).unwrap().to_string(),
        headers: Vec::new()
    };

    for header in iterator {
        if header.len() > 0 {
            let tokens: Vec<&str> = header.splitn(2, ": ").collect();
            if tokens.len() == 2 {
                request.headers.push(Header {
                    key: tokens[0].to_string(),
                    value: tokens[1].to_string()
                });
            }
        }
    }

    return Some(request);
}


pub fn handle_request(mut stream: TcpStream) {
    let request = read_request(&mut stream);
    
    match request {
        None => {
            stream.write(RES_FOR_BROWSER.as_bytes()).unwrap();
            return;
        },
        Some(_) => {},
    }

    println!("{:?}", request.unwrap().hostname);

    // send response
    stream.write(RES_FOR_BROWSER.as_bytes()).unwrap();
}
