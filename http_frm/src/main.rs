use server::Server;

#[rocket::main]
async fn main() {
    let s: Server = server::new();
    s.run().await;
}