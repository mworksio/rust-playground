// #![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[rocket::main]
async fn main() {
    rocket::build().mount("/hello", routes![hello]).ignite().await?.launch().await
}