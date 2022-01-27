#[macro_use] extern crate rocket;
// extern crate rocket_prometheus;

pub use rocket_prometheus::PrometheusMetrics;

#[derive(Debug)]
pub struct Server {
    // r: rocket::Rocket<rocket::Build>,
}

impl Server {
    // pub fn init(&self) -> Server {
    //     Server{
    //         r: rocket::build().mount("/", routes![version, metrics, healthz]),
    //     }
    // }
}

impl http_server::HttpServer for Server {
    fn run(&self) {
        rocket::async_main( async {
            let prometheus = PrometheusMetrics::new();
            rocket::build()
                .mount("/", routes![version, healthz])
                .launch().await;
            /*
            rocket::build().ignite()
                .attach(prometheus.clone()) // error[E0599]: no method named `attach` found for opaque type `impl std::future::Future<Output = Result<Rocket<Ignite>, rocket::Error>>` in the current scope
                .mount("/metrics", prometheus)
                .launch().await;
            */
        })
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

// #[get("/metrics")]
// fn metrics() -> &'static str {
//     "{prom}"
// }

pub fn new() -> impl http_server::HttpServer {
    Server{}
}
