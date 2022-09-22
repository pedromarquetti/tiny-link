// https://www.goldsborough.me/rust/web/tutorial/2018/01/20/17-01-11-writing_a_microservice_in_rust/
// https://www.secretfader.com/blog/2019/01/parsing-validating-assembling-urls-rust/
extern crate futures;
extern crate hyper;

#[macro_use]
extern crate log;
extern crate env_logger;
extern crate serde_json;

use futures::{
    future::{Future, FutureResult},
    Stream,
};
use hyper::{
    header::{ContentLength, ContentType},
    server::{Http, Request, Response, Server, Service},
    Body,
    Method::{Get, Post},
    StatusCode,
    {
        Chunk, Error,
        StatusCode::{Found, InternalServerError, NotFound},
    },
};
use serde_json::json;
use std::collections::HashMap;
use std::io;
use std::net::SocketAddr;
use url::{form_urlencoded, ParseError, Url};

#[derive(Eq, Hash, PartialEq)]
struct LongUrl {
    url: String,
}

/// Checks if received data matches `LongUrl` using a HashMap
/// returns error if no "url" field is supplied
fn parse_form(form_chunk: Chunk) -> FutureResult<LongUrl, Error> {
    let mut form: HashMap<String, String> = form_urlencoded::parse(form_chunk.as_ref())
        .into_owned()
        .collect::<HashMap<String, String>>();
    if !form.contains_key("url") {
        futures::future::err(Error::from(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Missing URL",
        )))
    } else {
        let cur_url = form.remove("url").unwrap();
        if cur_url.contains(" ") || cur_url == String::from("") {
            futures::future::err(Error::from(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid URL",
            )))
        } else {
            futures::future::ok(LongUrl { url: cur_url })
        }
    }
}

/// writes received long URL to db, returns short Url that will be echoed to user
fn write_to_db(entry: LongUrl) -> FutureResult<LongUrl, Error> {
    // TODO: save 'entry' to db...

    futures::future::ok(entry)
}

fn make_error_response(
    error_message: &str,
    error_type: StatusCode,
) -> FutureResult<Response, Error> {
    let payload: String = json!({ "error": error_message }).to_string();
    let response: Response = Response::new()
        .with_status(error_type)
        .with_header(ContentLength(payload.len() as u64))
        .with_header(ContentType::json())
        .with_body(payload);
    info!("ERROR> {:?}", response);
    futures::future::ok(response) // sending response
}

/// Send back response to user
fn post_response(res: Result<LongUrl, Error>) -> FutureResult<Response, Error> {
    match res {
        Ok(success_result) => {
            let payload: String = json!(
                { "url": success_result.url }
            )
            .to_string();
            let response: Response = Response::new()
                .with_header(ContentLength(payload.len() as u64))
                .with_header(ContentType::json())
                .with_body(payload);

            futures::future::ok(response) // sending response
        }
        Err(error) => make_error_response(&error.to_string(), InternalServerError),
    }
}

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
