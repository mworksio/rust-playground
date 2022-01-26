use http_server::HttpServer;

fn main() {
    let s = rocket_server::new();
    s.run();
}