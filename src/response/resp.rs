extern crate hyper;
use self::hyper::server::{Response};
use self::hyper::header::ContentLength;

pub fn resp(a: &str) -> Response {
    Response::new()
        .with_header(ContentLength(a.len() as u64))
        .with_body(a.to_owned())
}