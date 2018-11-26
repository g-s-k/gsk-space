extern crate actix_web;

use actix_web::{server, App, HttpRequest};

fn index(req: &HttpRequest) -> &'static str {
    println!("{:?}", req);
    "\"Hello World!\""
}

fn main() {
    server::new(|| App::new().resource("/", |r| r.f(index)))
        .bind("0.0.0.0:8000")
        .unwrap()
        .run();
}
