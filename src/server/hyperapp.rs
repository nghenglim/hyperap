extern crate hyper;
extern crate futures;
extern crate open;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use self::futures::future::Future;  
use self::hyper::server::{Http, Request, Response, Service};
use self::hyper::{Method, StatusCode};
use std::sync::Arc;

struct Route<D> where D: Middleware + 'static {
    method: Method,
    path: String,
    func: Arc<Fn(D::M) -> Response>,
}
pub struct HyperApp<D> where D: Middleware + 'static {
    port: u16,
    app: D,
    open_browser: bool,
    routes: Vec<Route<D>>,
    default_route: Arc<Fn(D::M) -> Response>,
}
pub fn not_found_route<D>(_a: D) -> Response  where D: 'static {
    Response::new().with_status(StatusCode::NotFound)
}
pub trait Middleware {
    type M;
    fn middleware(&self, req: Request, f: Arc<Fn(Self::M) -> Response>) -> Response;
}
impl<D> HyperApp<D> where D: Middleware + 'static, {
    pub fn new(d: D) -> HyperApp<D> {
        HyperApp {
            port: 3000,
            app: d,
            open_browser: true,
            routes: Vec::new(),
            default_route: Arc::new(not_found_route::<D::M>),
        }
    }
    pub fn add_route<F: 'static, S: Into<String>>(&mut self, method: Method, path: S, func: F) -> &mut Self where
    F: Fn(D::M) -> Response {
        let route = Route {
            method: method,
            path: path.into(),
            func: Arc::new(func),
        };
        self.routes.push(route);
        self
    }
    pub fn set_default_route<F: 'static>(&mut self, func: F) -> &mut Self where
    F: Fn(D::M) -> Response {
        self.default_route = Arc::new(func);
        self
    }
    pub fn production(&mut self) -> &mut Self {
        self.open_browser = false;
        self
    }
    pub fn open_browser(&mut self, b: bool) -> &mut Self {
        self.open_browser = b;
        self
    }
    pub fn port(&mut self, port: u16) -> &mut Self {
        self.port = port;
        self
    }
    pub fn run(self) {
        if self.open_browser {
            let mut url = "http://localhost".to_string();
            if self.port != 80 {
                url = url + ":" + &self.port.to_string();
            }
            match open::that(url) {
                Ok(exit_status) => {
                    if exit_status.success() {
                        // println!("Look at your browser !");
                    } else {
                        if let Some(code) = exit_status.code() {
                            println!("Command returned non-zero exit status {}!", code);
                        } else {
                            println!("Command returned with unknown exit status!");
                        }
                    }
                }
                Err(why) => println!("Failure to execute command: {}", why),
            }
        }
        let socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), self.port);
        let arc = Arc::new(self);
        let server = Http::new().bind(&socket_addr, move || Ok(arc.clone())).unwrap();
        println!("The server will be running at {}", socket_addr);
        let _server = server.run().unwrap();
    }
}
fn matched_index<D>(v: &Vec<Route<D>>, i: usize, method: Method, path: String) -> usize where D: Middleware + 'static {
    if v.len() == i {
        i
    } else {
        let r = &(v)[i];
        if method == r.method && path == r.path {
            i
        } else {
            matched_index(v, i+1, method, path)
        }
    }
}
impl<D> Service for HyperApp<D> where D: Middleware + 'static {
    // boilerplate hooking up hyper's server types
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    // The future representing the eventual Response your call will
    // resolve to. This can change to whatever Future you need.
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;
    fn call(&self, req: Request) -> Self::Future {
        let method = req.method().to_owned();
        let path = req.path().to_owned();
        let matched_index = matched_index(&(self.routes), 0, method, path);
        let response = if (self.routes).len() == 0 {
            self.app.middleware(req, self.default_route.clone())
        } else if matched_index == (self.routes).len() {
            self.app.middleware(req, self.default_route.clone())
        } else {
            let r = &(self.routes)[matched_index];
            self.app.middleware(req, r.func.clone())
        };

        Box::new(futures::future::ok(response))
    }
}