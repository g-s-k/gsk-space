extern crate actix_web;
extern crate env_logger;
extern crate sha2;

use std::{env, time};

use actix_web::{middleware, server, App, HttpRequest};
use sha2::{Digest, Sha256};

fn index(_req: &HttpRequest) -> &'static str {
    "\"Hello World!\""
}

fn hash_sha2(req: &HttpRequest) -> String {
    let mut hash_slinging_slasher = Sha256::new();
    let salt = format!("{:?}", time::SystemTime::now());
    hash_slinging_slasher.input(salt);
    for (key, value) in req.headers().iter() {
        hash_slinging_slasher.input(key);
        hash_slinging_slasher.input(value);
    }
    format!("{:x}", hash_slinging_slasher.result())
}

fn main() {
    env::set_var("RUST_LOG", "actix_web=debug");
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    server::new(|| {
        App::new()
            .middleware(middleware::Logger::default())
            .resource("/sha256", |r| r.f(hash_sha2))
            .default_resource(|r| r.f(index))
    }).bind("0.0.0.0:8000")
    .expect("Cannot bind to 0.0.0.0:8000")
    .run();
}
