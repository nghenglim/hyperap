extern crate hyperap;
use hyperap::hyper::server::{Response};
use hyperap::hyper::{Method};
use hyperap::server::{HyperApp, Middleware, MiddlewareParam};
use hyperap::response::{resp};

fn get_static(_a: MiddlewareResult) -> Response {
    hyperap::server::static_file("Cargo.toml")
}
fn hello_world(a: MiddlewareResult) -> Response {
    resp(a.hello.clone() + " at path " + &a.path)
}
fn not_found_route(a: MiddlewareResult) -> Response {
    resp("not found route at path ".to_owned() + &a.path)
}
pub struct App {
    pub hello: String,
}
pub struct MiddlewareResult {
    path: String,
    pub hello: String,
}
#[derive(Clone)]
pub struct RouteDefinition {
    parameters: Vec<RouteDefinitionParameters>
}
#[derive(Clone)]
pub struct RouteDefinitionParameters {
    _in: String,
    _name: String,
}
impl Middleware for App {
    type M = MiddlewareResult;
    type R = RouteDefinition;
    fn middleware(&self, p: MiddlewareParam<MiddlewareResult, Self::R>) -> Response {
        let m = MiddlewareResult {
            path: p.req.path().to_owned(),
            hello: self.hello.clone(),
        };
        (p.func)(m)
    }
}
fn main() {
    let the_app = App {
        hello: "Hello World".to_owned(),
    };
    let mut app = HyperApp::new(the_app);
    app.open_browser(true);
    app.set_default_route(not_found_route);
    app.add_route(Method::Get, "/static", get_static, vec![RouteDefinition {
        parameters: vec![RouteDefinitionParameters {
            _in: "query".to_owned(),
            _name: "offset".to_owned(),
        }]
    }]);
    app.add_pure_route(Method::Get, "/", hello_world);
    app.port(3000);
    app.run();
}