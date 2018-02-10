extern crate hyperap;
extern crate futures;
use hyperap::hyper::server::{Response};
use hyperap::hyper::{self, Method, StatusCode};
use hyperap::server::{Hyperap, HyperapCore, MiddlewareParam};
use hyperap::response::{resp};
use futures::future::Future;  

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
    hello: String,
}
#[derive(Debug)]
pub struct RouteDefinition {
    parameters: Vec<RouteDefinitionParameters>
}
#[derive(Debug)]
pub struct RouteDefinitionParameters {
    in_: String,
    name: String,
}
impl HyperapCore for App {
    type M = MiddlewareResult;
    type R = RouteDefinition;
    type Resp = Response;
    fn default_route(_a: Self::M) -> Self::Resp {
        Response::new().with_status(StatusCode::NotFound)
    }
    fn middleware(&self, p: MiddlewareParam<Self::M, Self::R, Self::Resp>) -> Box<Future<Item = Response, Error = hyper::Error>> {
        let m = Self::M {
            path: p.req.path().to_owned(),
            hello: self.hello.clone(),
        };
        let resp = (p.func)(m);
        Box::new(futures::future::ok(resp))
    }
}
fn main() {
    let the_app = App {
        hello: "Hello World".to_owned(),
    };
    let mut app = Hyperap::new(the_app);
    app.open_browser(true);
    app.add_pure_route(Method::Get, "/static", get_static);
    app.add_route(Method::Get, "/", hello_world, RouteDefinition {
        parameters: vec![RouteDefinitionParameters {
            in_: "query".to_owned(),
            name: "offset".to_owned(),
        }]
    });
    app.port(3000);
    app.run();
}