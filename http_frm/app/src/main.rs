// #[macro_use] extern crate rocket;
// use server::{Server, HttpServer};
// use http_server::HttpServer;
// use http_server::{HttpServer};
// use rocket_server::{Server};

fn run(s: & dyn http_server::HttpServer) {
    s.run();
}

// #[rocket::main]
fn main() {
    // let s: Server = server::new();
    // let s = rocket_server::new(); 
    // run(&s).await;
    // run(&s);

    let s = rocket_server::Server{};
    // s.init();
    run(&s);
}