#[macro_use] extern crate rocket;
// #[cfg(test)] mod tests;

#[get("/world")]
fn world() -> &'static str {
    "Hello, world!"
}

#[get("/rust")]
fn rust() -> &'static str {
    "Hello, Rust!"
}

#[get("/<name>")]
fn who(name: &str) -> String {
    format!("Hi, {}", name)    
}

#[get("/<name>/<age>")]
fn wave(name: &str, age: u8) -> String {
    format!("你好！, {} year old named {}!", age, name)
}

#[derive(FromFormField)]
enum Lang {
    #[field(value = "en")]
    English,
    #[field(value = "ru")]
    #[field(value = "py")]
    Russian
}

#[derive(FromForm)]
struct Options<'r> {
    emoji: bool,
    name: Option<&'r str>,
}

#[get("/?<lang>&<opt..>")]
fn hello(lang: Option<Lang>, opt: Options<'_>) -> String {
    let mut greeting = String::new();
    if opt.emoji {
        greeting.push_str("����");
    }

    match lang {
        Some(Lang::Russian) => greeting.push_str("Привет"),
        Some(Lang::English) => greeting.push_str("Hello"),
        None => greeting.push_str("Hi Null"),
    }

    if let Some(name) = opt.name {
        greeting.push_str(", ");
        greeting.push_str(name);
    }

    greeting.push('!');
    greeting
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/hello", routes![world, who, rust])
        .mount("/wave", routes![wave])
        .mount("/", routes![hello])
}

// #[get("/<name>/<age>")]
// fn hello(name: &str, age: u8) -> String {
//     format!("Hello, {} year of old named {}!", age, name)
// }

// #[launch]
// fn rocket() -> _ {
//     rocket::build().mount("/hello", routes![hello])
// }

