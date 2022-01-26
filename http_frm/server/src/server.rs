#[macro_use] extern crate rocket;

// pub mod server;
// pub use server::Server;

#[derive(Debug)]
pub struct Server {

}

impl Server {
    pub fn init() -> Server {
        Server {}
    }

    pub async fn run(self) -> Result<(), rocket::Error> {
        rocket::build()
            .mount("/", routes![version, healthz, metrics])
            .ignite().await?
            .launch().await
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

pub fn new() -> Server {
    Server{}
}
