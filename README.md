## INTRO
Hyperap - Hyper wrapper. A very minimal wrapper for Hyper.rs to create a working webserver. 

## How To Use
~~~rs
extern crate hyperap;
use hyperap::hyper::server::{Response};
use hyperap::hyper::{Method};
use hyperap::server::{HyperApp, RouteFuncParam};
use hyperap::response::{resp};

fn get_static(_req: RouteFuncParam<App>) -> Response {
    hyperap::server::static_file("Cargo.toml")
}
fn hello_world(a: RouteFuncParam<App>) -> Response {
    resp(a.app.hello.as_str())
}
pub struct App {
    pub hello: String,
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
~~~

## TODO
- [ ] more functional on the add_route
- [ ] route is able to specify the GET/POST definition, do validation before go into controller
- [ ] all the routing is able to generate into a swagger file