// https://www.goldsborough.me/rust/web/tutorial/2018/01/20/17-01-11-writing_a_microservice_in_rust/
// https://www.secretfader.com/blog/2019/01/parsing-validating-assembling-urls-rust/

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;

mod models;
mod schema;

mod db;
mod error;
mod parser;
mod response;
mod structs;
use crate::error::make_error_response;
use crate::parser::{parse_form, validate_path};
use crate::response::post_response;
use crate::{db::write_to_db, response::get_response};

extern crate futures;
extern crate hyper;

#[macro_use]
extern crate log;
extern crate env_logger;
extern crate serde_json;

use diesel::{pg::PgConnection, Connection};

use futures::{future::Future, Stream};
use hyper::{
    server::{Http, Request, Response, Service},
    Error,
    Method::{Get, Post},
    StatusCode::{InternalServerError, NotFound},
};
use std::net::SocketAddr;

use dotenvy::dotenv;
use std::env;

const DEFAULT_DATABASE_URL: &'static str = "postgresql://postgres@localhost:5432";

fn connect_to_db() -> Option<PgConnection> {
    dotenv().ok(); // checks for .env file
    let database_url = env::var("DATABASE_URL").unwrap_or(String::from(DEFAULT_DATABASE_URL));

    match PgConnection::establish(&database_url) {
        Ok(connection) => Some(connection),
        Err(error) => {
            error!("Error connecting to database: {}", error);
            None
        }
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
        let mut db_connection: PgConnection = match connect_to_db() {
            Some(connection) => connection,
            None => {
                return Box::new(make_error_response(
                    "DB Connection Error!",
                    InternalServerError,
                ));
            }
        };
        match req.method() {
            &Post => {
                let fut = req
                    .body()
                    .concat2()
                    .and_then(parse_form) // checks if it's a valid form
                    .and_then(move |long_url| write_to_db(long_url, &mut db_connection)) // TODO: will submit result to DB
                    .then(post_response);
                // after receiving request
                // add future to Heap memory
                Box::new(fut)
            }
            &Get => {
                let path: String = req.path().to_string();
                let validator: Result<String, String> = validate_path(path);

                let response = match validator {
                    Ok(ok_value) => {
                        // valid path,
                        // query db
                        get_response(&ok_value)
                    }
                    Err(err) => make_error_response(&err, NotFound),
                };

                Box::new(response)
            }

            err => Box::new(futures::future::ok(
                Response::new()
                    .with_status(NotFound)
                    .with_body(format!("Error: {:}", err)),
            )),
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
