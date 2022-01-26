use rocket_prometheus::PrometheusMetrics;

fn main() {
    let prometheus = PrometheusMetrics::new();
    rocket::ignite()
        .attach(prometheus.clone())
        .mount("/metrics", prometheus)
        .launch();
}