#[macro_use] extern crate rocket;
// mod server;
// pub use server::Server;

#[derive(Debug)]
// pub struct Server {
struct Server {

}

impl Server {
    pub fn init() -> Server {
        Server {}
    }

    async fn run(self) -> Result<(), rocket::Error> {
        /*
        rocket::build()
            .mount("/version", routes![version])
            .mount("/healthz", routes![healthz])
            .mount("/metrics", routes![metrics])
        */

        rocket::build().mount("/", routes![version])
        .ignite().await?
        .launch().await
    }

    /*
    #[launch] // could only be applied to function without parameters
    fn play() -> _ {
        rocket::build().mount("/version", routes![version])
    }
    */
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

fn new() -> Server {
    Server{}
}

#[rocket::main]
async fn main() {
    let s: Server = new();
    s.run().await;
}