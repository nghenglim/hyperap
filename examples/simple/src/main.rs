extern crate hyperap;
extern crate swagger_spec;
use hyperap::hyper::server::{Response, Request};
use hyperap::hyper::{self, Method, StatusCode};
use hyperap::server::{Hyperap, HyperapCore};
use hyperap::response::{resp};
use hyperap::futures;
use hyperap::futures::future::Future;  
use swagger_spec::{Parameter, PathItem, ParameterIn, Schema, DataType};
use std::sync::Arc;

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
#[derive(Clone, Debug)]
pub struct RouteDefinition {
    pub swagger: PathItem,
}
impl RouteDefinition {
    pub fn new() -> RouteDefinition {
        RouteDefinition {
            swagger: PathItem::new(),
        }
    }
    pub fn add_parameter(&mut self, p: Parameter) -> &mut Self {
        self.swagger.add_parameter(p);
        self
    }
}
impl HyperapCore for App {
    type M = MiddlewareResult;
    type R = RouteDefinition;
    type Resp = Response;
    fn default_route(_a: Self::M) -> Self::Resp {
        Response::new().with_status(StatusCode::NotFound)
    }
    fn middleware(&self, req: Request, func: Arc<Fn(Self::M) -> Self::Resp>, _route_definition: Arc<Option<Self::R>>) -> Box<Future<Item = Response, Error = hyper::Error>> {
        let m = Self::M {
            path: req.path().to_owned(),
            hello: self.hello.clone(),
        };
        let resp = (func)(m);
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
    app.add_route(Method::Get, "/", hello_world, RouteDefinition::new()
        .add_parameter(
            Parameter::new()
            .schema(
                Schema::new()
                .type_(DataType::Integer)
                .clone()
            )
            .name("limit")
            .in_(ParameterIn::Query)
            .clone()
        )
        .clone()
    );
    app.run();
}