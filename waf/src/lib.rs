#![warn(clippy::all)]

extern crate tiny_http;
use serde::Serialize;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, ToSocketAddrs};
use std::str::FromStr;
use std::sync::Arc;
use std::thread;

pub struct App {
    addr: SocketAddr,
    num_threads: u8,
    middlewares: Vec<MiddleWare>,
}
impl App {
    pub fn default() -> Self {
        App {
            addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 3000),
            middlewares: vec![],
            num_threads: 8,
        }
    }

    pub fn set_addr<A: ToSocketAddrs>(&mut self, addr: A) {
        self.addr = addr.to_socket_addrs().unwrap().next().unwrap();
    }

    pub fn set_num_threads(&mut self, num_threads: u8) {
        self.num_threads = num_threads;
    }

    pub fn add_middleware(&mut self, m: MiddleWare) {
        self.middlewares.push(m);
    }

    pub fn add_router(&mut self, r: Router) {
        for m in r.middlewares {
            self.middlewares.push(m);
        }
    }

    pub fn listen(self) {
        let server = Arc::new(tiny_http::Server::http(self.addr).unwrap());
        println!("Now listening on port {:?}", self.addr.port());
        let middlewares = Arc::new(self.middlewares);

        let mut handles = Vec::new();

        for _ in 0..self.num_threads {
            let server = server.clone();
            let middlewares = middlewares.clone();

            handles.push(thread::spawn(move || {
                for rq in server.incoming_requests() {
                    let mut req = Req {
                        method: rq.method().clone(),
                        path: rq.url().to_string(),
                    };
                    let mut res = Res {
                        status: 500,
                        headers: vec![],
                        body: String::new(),
                    };
                    let mut iter = middlewares.iter();
                    while let Some(f) = iter.next() {
                        if f(&mut req, &mut res) {
                            break;
                        }
                    }
                    let mut response = tiny_http::Response::from_data(res.body);
                    for h in &res.headers {
                        response.add_header(tiny_http::Header::from_str(h).unwrap())
                    }
                    let _ = rq.respond(response);
                }
            }));
        }

        for h in handles {
            h.join().unwrap();
        }
    }
}

pub struct Router {
    middlewares: Vec<MiddleWare>,
}

impl Router {
    pub fn default() -> Self {
        Router {
            middlewares: vec![],
        }
    }
    pub fn get(&mut self, path: String, mi: RouteHandler) {
        self.route(tiny_http::Method::Get, path, mi);
    }
    pub fn put(&mut self, path: String, mi: RouteHandler) {
        self.route(tiny_http::Method::Put, path, mi);
    }
    pub fn post(&mut self, path: String, mi: RouteHandler) {
        self.route(tiny_http::Method::Post, path, mi);
    }
    pub fn delete(&mut self, path: String, mi: RouteHandler) {
        self.route(tiny_http::Method::Delete, path, mi);
    }
    pub fn route(&mut self, method: tiny_http::Method, path: String, mi: RouteHandler) {
        self.middlewares
            .push(Box::new(move |req: &mut Req, res: &mut Res| {
                if req.method == method && req.path == path {
                    mi(req, res);
                    true
                } else {
                    false
                }
            }));
    }
}

type RouteHandler = Box<dyn Send + Sync + Fn(&mut Req, &mut Res) -> ()>;

type MiddleWare = Box<dyn Send + Sync + Fn(&mut Req, &mut Res) -> bool>;

pub struct Req {
    method: tiny_http::Method,
    path: String,
}

pub struct Res {
    status: u16,
    headers: Vec<String>,
    body: String,
}

impl Res {
    pub fn return_json<S: Serialize>(&mut self, obj: S) {
        self.status = 200;
        self.body = serde_json::to_string(&obj).unwrap();
        self.headers
            .push("Content-Type:application/json".to_string());
    }

    pub fn return_text(&mut self, text: String) {
        self.status = 200;
        self.body = text;
        self.headers
            .push("Content-Type:text/plain; charset=UTF-8".to_string());
    }
}
