use std::io::prelude::*;
use hyper;
use hyper::server::Request;
use hyper::server::Response;


pub fn handle(req: Request, res: Response) {
    
}

// url is the remote request url
fn request_remote(url: String) -> String {
    let client = hyper::Client::new();
    let mut res = client.get(&url)
        .header(hyper::header::Connection::close())
        .send().unwrap();

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    return body;
}

// url is the resource path url
fn request_local(url: String) -> String {
    return String::new();
}
