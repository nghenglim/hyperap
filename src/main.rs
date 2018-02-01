extern crate hyperap;
use hyperap::hyper::server::{Response, Request};
use hyperap::hyper::{Method};
use hyperap::server::{HyperApp, Middleware};
use hyperap::response::{resp};

fn get_static(_a: MiddlewareResult) -> Response {
    hyperap::server::static_file("Cargo.toml")
}
fn hello_world(a: MiddlewareResult) -> Response {
    resp(a.hello.clone() + " at path " + &a.path)
}
pub struct App {
    pub hello: String,
}
pub struct MiddlewareResult {
    path: String,
    pub hello: String,
}
impl Middleware for App {
    type M = MiddlewareResult;
    fn middleware(&self, req: Request) -> Self::M {
        MiddlewareResult {
            path: req.path().to_owned(),
            hello: self.hello.clone(),
        }
    }
}
fn main() {
    let the_app = App {
        hello: "Hello World".to_owned(),
    };
    let mut app = HyperApp::new(the_app);
    app.open_browser(true);
    app.add_route(&Method::Get, "/static", get_static);
    app.add_route(&Method::Get, "/", hello_world);
    app.port(3000);
    app.run();
}