use std::io::prelude::*;
use std::net::TcpStream;
use regex::Regex;
use chalk::Chalk;
use chalk::colors::Colors;

#[derive(Debug)]
pub struct Header {
    pub key: String,
    pub value: String
}

// no scheme

#[allow(dead_code)]
pub struct Request {
    hostname: String,
    path: String,
    headers: Vec<Header>
}

#[allow(dead_code)]
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
    let mut request_tokens: Vec<&str> = request_line.split(" ").collect();
    let url: &str = request_tokens[1];

    let host_line: &str = iterator.next().unwrap();
    request_tokens = host_line.split(" ").collect();
    let host: &str = request_tokens[1];

    let re = Regex::new(r"(\w*?)://(.*?)/(.*)").unwrap();

    // If it is not an absolute-url, then use the host and path from 
    // the request line.
    let caps = re.captures(url);

    let mut request = Request {
        hostname: String::new(),
        path: String::new(),
        headers: Vec::new()
    };

    if caps.is_none() {
        request.hostname = host.to_string();
        request.path = url.to_string();
    } else {
        let caps_unwrapped = caps.unwrap();
        request.hostname = caps_unwrapped.at(2).unwrap().to_string();
        request.path = caps_unwrapped.at(3).unwrap().to_string();
    }

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

    // debug 
    request.headers.push(Header {
        key: String::from("Host"),
        value: String::from("static.moseeker.com")
    });

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

    let uw_request = request.unwrap();

    log_request(&uw_request);

    println!("{:?}", &uw_request.headers);

    // debug 
    let address_string = format!("{}:{}", "static.moseeker.com", 80);
    let mut server_stream = TcpStream::connect(&*address_string).unwrap();
    send_request(&uw_request, &mut server_stream);

    // search and fetch the assets, if assets not found,
    // redirect the request to remote
    // let mut content = Assets::get_asset(&uwRequest.path);
    // if content.is_none() {
    //     content = send_request(&stream);
    // }

    let mut content_buf: Vec<u8> = Vec::new();
    server_stream.read_to_end(&mut content_buf).unwrap();
  
    // println!("{}", String::from_utf8_lossy(&content_buf));
    // send response
    // stream.write(RES_FOR_BROWSER.as_bytes()).unwrap();
    stream.write(&content_buf).unwrap();
}

fn send_request(request: &Request, stream: &mut TcpStream) {
    let request_line = b"GET /hr/app.74843587.css HTTP/1.1";
    // request.hostname = String::from("static.moseeker.com");
    stream.write(request_line).unwrap();

    for header in request.headers.iter() {
        let outbound_header = format!("{}: {}\r\n", header.key, header.value);
        stream.write(&outbound_header.into_bytes()).unwrap();
    }

    stream.write(b"Connection: close\r\n").unwrap();
    stream.write(b"\r\n").unwrap();
}

fn log_request(request: &Request) {
    let mut message = String::from("GET: ");
    message.push_str(&request.path);

    let green_message = Chalk::new(Colors::Green, &message).color();
    println!("{}", green_message);
}
