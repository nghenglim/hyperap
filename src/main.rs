extern crate fawkes;
use fawkes::hyper::server::{Response};
use fawkes::hyper::{Method};
use fawkes::server::{FawkesApp, RouteFuncParam};
use fawkes::response::{resp};

fn get_static(_req: RouteFuncParam<App>) -> Response {
    fawkes::server::static_file("Cargo.toml")
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
    let mut app = FawkesApp::new(the_app);
    app.open_browser(true);
    app.add_route(&Method::Get, "/static", get_static);
    app.add_route(&Method::Get, "/", hello_world);
    app.port(3000);
    app.run();
}