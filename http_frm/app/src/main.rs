#[macro_use] extern crate rocket;
use server::Server;

#[rocket::main]
async fn main() {
    let s: Server = server::new();
    s.run().await;
}