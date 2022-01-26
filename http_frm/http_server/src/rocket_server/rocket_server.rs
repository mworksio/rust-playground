// // #![feature(plugin)]
// #![plugin(rocket_codegen)]

// #![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

// use crate::server;

// pub mod server;
// pub use server::Server;

#[derive(Debug)]
pub struct Server {
    // s: rocket::Rocket<rocket::Build>, 
}

impl Server {
    pub fn init(&self) -> Server {
        Server{}
        // Server {
            // s: rocket::build().mount("/", routes![version, healthz, metrics]),
        // }
        // rocket::ignite().mount("/", routes![version, healthz, metrics]).launch();
    }

    // pub async fn ignite(&self) -> Result<(), rocket::Error> {
    //     self.s.ignite().await?
    // }

    // pub async fn launch(&self) -> Result<(), rocket::Error> {
        // self.s.launch().await
    // }
    // pub fn launch(&self) {
    //     self.s.launch();
    // }
   
    /*
    pub async fn ignite(&self) -> Result<(), rocket::Error> {
        rocket::build()
            .mount("/", routes![version, healthz, metrics])
            .ignite().await?
            .launch().await
    }
    */
}

impl http_server::HttpServer for Server {
    fn run(&self) {
        // self.ignite().await;
        // self.ignite();
        // rocket();
        // self.launch();
        // rocket::ignite().mount("/", routes![version, healthz, metrics]).launch();
        // rocket::build().ignite().mount("/", routes![version]).launch();
        // rocket::build().mount("/", routes![version]).ignite().launch();
        // rocket::inite().mount("/", routes![version]).launch();
        rocket().launch();
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

// pub fn new() -> impl http_server::HttpServer {
//     Server{}
// }

pub fn rocket() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .mount("/", routes![version, healthz, metrics])
}