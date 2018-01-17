extern crate hyper;
extern crate futures;

use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::ffi::OsStr;
use self::hyper::server::{Response};
use self::hyper::{StatusCode};

pub fn static_file(path: &str) -> Response {
    let mut f = match File::open(path) {
        Ok(f) => f,
        Err(_) => {
            return Response::new().with_status(StatusCode::NotFound)
        },
    };

    let mut data = Vec::new();
    let _content = f.read_to_end(&mut data);

    let mut headers = hyper::header::Headers::new();
    
    const UNKNOWN_CONTENT_TYPE: &str = "text/plain";
    let content_type = match Path::new(path).extension().and_then(OsStr::to_str) {
        Some(ext) => match ext {
            "html" => "text/html",
            "css" => "text/css",
            "js" => "application/javascript",
            _ => UNKNOWN_CONTENT_TYPE,
        },
        None => UNKNOWN_CONTENT_TYPE,
    };
    headers.set_raw("Content-Type", content_type);

    Response::new()
        .with_status(StatusCode::Ok)
        .with_headers(headers)
        .with_body(data)
}