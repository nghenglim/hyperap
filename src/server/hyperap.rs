extern crate hyper;
extern crate futures;
extern crate open;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use self::futures::future::Future;  
use self::hyper::server::{Http, Request, Response, Service};
use self::hyper::{Method};
use std::sync::Arc;

struct Route<D> where D: HyperapCore + 'static {
    method: Method,
    path: String,
    definition: Arc<Option<D::R>>,
    func: Arc<Fn(D::M) -> D::Resp>,
}
pub struct Hyperap<D> where D: HyperapCore + 'static {
    port: u16,
    app: D,
    open_browser: bool,
    routes: Vec<Route<D>>,
}
pub struct MiddlewareParam<M, R, Resp> {
    pub req: Request,
    pub route_definition: Arc<Option<R>>,
    pub func: Arc<Fn(M) -> Resp>,
}
pub trait HyperapCore {
    // the main param type that is being obtained by controller
    type M; 
    // the route definition such as swagger definition which is mainly for middleware to use
    type R; 
    type Resp; 
    fn default_route(Self::M) -> Self::Resp;
    fn middleware(&self, param: MiddlewareParam<Self::M, Self::R, Self::Resp>) -> Box<Future<Item = Response, Error = hyper::Error>>;
}
impl<D> Hyperap<D> where D: HyperapCore + 'static, {
    pub fn new(d: D) -> Hyperap<D> {
        Hyperap {
            port: 3000,
            app: d,
            open_browser: true,
            routes: Vec::new(),
        }
    }
    pub fn add_route<F: 'static, S: Into<String>>(&mut self, method: Method, path: S, func: F, definition: D::R) -> &mut Self where
    F: Fn(D::M) -> D::Resp {
        let route = Route {
            method: method,
            path: path.into(),
            definition: Arc::new(Some(definition)),
            func: Arc::new(func),
        };
        self.routes.push(route);
        self
    }
    pub fn add_pure_route<F: 'static, S: Into<String>>(&mut self, method: Method, path: S, func: F) -> &mut Self where
    F: Fn(D::M) -> D::Resp {
        let route = Route {
            method: method,
            path: path.into(),
            definition: Arc::new(None),
            func: Arc::new(func),
        };
        self.routes.push(route);
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
fn matched_index<D>(v: &Vec<Route<D>>, i: usize, method: Method, path: String) -> usize where D: HyperapCore + 'static {
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
impl<D> Service for Hyperap<D> where D: HyperapCore + 'static {
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
        if (self.routes).len() == 0 {
            self.app.middleware(MiddlewareParam {
                req: req,
                func: Arc::new(D::default_route).clone(),
                route_definition: Arc::new(None),
            })
        } else if matched_index >= (self.routes).len() {
            self.app.middleware(MiddlewareParam {
                req: req,
                func: Arc::new(D::default_route).clone(),
                route_definition: Arc::new(None),
            })
        } else {
            let r = &(self.routes)[matched_index];
            self.app.middleware(MiddlewareParam {
                req: req,
                func: r.func.clone(),
                route_definition: (r.definition).clone(),
            })
        }
    }
}