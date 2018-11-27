extern crate actix;
extern crate actix_web;
extern crate env_logger;
extern crate sha2;

use std::{env, time};

use actix::prelude::*;
use actix_web::{http, middleware, server, ws, App, Error, HttpRequest, HttpResponse};
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

fn ws_index(req: &HttpRequest) -> Result<HttpResponse, Error> {
    ws::start(req, Ws)
}

struct Ws;

impl Actor for Ws {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<ws::Message, ws::ProtocolError> for Ws {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        match msg {
            ws::Message::Ping(msg) => ctx.pong(&msg),
            ws::Message::Text(text) => ctx.text(text),
            ws::Message::Binary(bin) => ctx.binary(bin),
            ws::Message::Close(_) => {
                ctx.stop();
            }
            _ => (),
        }
    }
}

fn main() {
    env::set_var("RUST_LOG", "actix_web=debug");
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    server::new(|| {
        App::new()
            .middleware(middleware::Logger::default())
            .resource("/sha256", |r| r.f(hash_sha2))
            .resource("/ws/", |r| r.method(http::Method::GET).f(ws_index))
            .default_resource(|r| r.f(index))
    }).bind("0.0.0.0:8000")
    .expect("Cannot bind to 0.0.0.0:8000")
    .run();
}
