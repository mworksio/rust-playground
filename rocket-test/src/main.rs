use crate::Server;
#[macro_use] extern crate rocket;

fn main() {
    let s = Server::new();
    s.run();
}