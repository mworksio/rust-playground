#[macro_use] extern crate rocket;

#[derive(Debug)]
pub struct Server {
}

impl Server {
    pub fn init(&self) -> Server {
        Server{}
    }
}

impl http_server::HttpServer for Server {
    fn run(&self) {
        rocket::async_main( async {
            rocket::build().mount("/", routes![version, metrics, healthz]).launch().await;
        })
    }
}

#[get("/version")]
fn version() -> &'static str {
    "0.0.1"
}

#[get("/healthz")]
fn healthz() -> &'static str {
    "ok"
}

#[get("/metrics")]
fn metrics() -> &'static str {
    "{prom}"
}

pub fn new() -> impl http_server::HttpServer {
    Server{}
}
