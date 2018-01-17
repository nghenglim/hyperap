extern crate fawkes;
use fawkes::hyper::server::{Response};
use fawkes::hyper::{Method};
use fawkes::server::{FawkesApp, RouteFuncParam};

fn get_static(_req: RouteFuncParam) -> Response {
    fawkes::server::static_file("Cargo.toml")
}
fn main() {
    let mut app = FawkesApp::new();
    app.open_browser(true);
    app.add_route(&Method::Get, "/", get_static);
    app.port(3000);
    app.run();
}