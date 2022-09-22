// https://www.goldsborough.me/rust/web/tutorial/2018/01/20/17-01-11-writing_a_microservice_in_rust/
// https://www.secretfader.com/blog/2019/01/parsing-validating-assembling-urls-rust/

mod db;
mod error;
mod parser;
mod response;
mod structs;
use crate::db::write_to_db;
use crate::parser::parse_form;
use crate::response::post_response;

extern crate futures;
extern crate hyper;

#[macro_use]
extern crate log;
extern crate env_logger;
extern crate serde_json;

use futures::{future::Future, Stream};
use hyper::{
    server::{Http, Request, Response, Service},
    Method::Post,
    {Error, StatusCode::NotFound},
};
use std::net::SocketAddr;

/// Main Struct
/// Contains `call` function
struct Shortener;
impl Service for Shortener {
    type Request = Request;
    type Response = Response;
    type Error = Error;
    type Future = Box<dyn Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        match (req.method(), req.path()) {
            (&Post, "/") => {
                let fut = req
                    .body()
                    .concat2()
                    .and_then(parse_form)
                    .and_then(write_to_db) // TODO
                    .then(post_response);
                // after receiving request
                // add future to Heap memory
                Box::new(fut)
            }
            _ => Box::new(futures::future::ok(Response::new().with_status(NotFound))),
        }
    }
}
fn main() {
    // using
    // 'RUST_LOG="info" cargo run' to log events
    env_logger::init();
    // address used by the server
    let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    let server = Http::new()
        .bind(
            // bind &addr to server
            &addr,
            // 'closure' function
            || Ok(Shortener {}),
        )
        .unwrap();
    info!("running at {}", addr);
    server.run().unwrap();
}
