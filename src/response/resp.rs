extern crate hyper;
use self::hyper::server::{Response};
use self::hyper::header::ContentLength;

pub fn resp<S: Into<String>>(a: S) -> Response {
    let s = a.into();
    Response::new()
        .with_header(ContentLength(s.len() as u64))
        .with_body(s)
}