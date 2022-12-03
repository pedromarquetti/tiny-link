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
use crate::db::{read_from_db, write_to_db};
use crate::error::make_error_response;
use crate::response::{get_response, post_response};
use crate::structs::TinyLink;
use futures::Map;
use hyper::body::Bytes;
use hyper::http::Error;
use serde_json::{json, Value};
use structs::ShortUrl;
use warp::reply::Json;
use warp::{http::Response, path::FullPath};
// use crate::error::make_error_response;
use crate::parser::{parse_form, validate_path};
// use crate::response::post_response;
// use crate::{db::write_to_db, response::get_response};
// use db::read_from_db;

// extern crate futures;
// extern crate hyper;

#[macro_use]
extern crate log;
extern crate env_logger;
extern crate serde_json;

use diesel::{pg::PgConnection, Connection};

use futures::{future::Future, Stream};
// use hyper::{
//     server::{Http, Request, Response, Service},
//     Error,
//     Method::{Get, Post},
//     StatusCode::{InternalServerError, NotFound},
// };
use hyper::{Body, Request, StatusCode};
use std::convert::Infallible;
use std::net::SocketAddr;

// use warp::http::Method;
use warp::{http::Method, reply::WithStatus, Filter};

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
// struct Shortener;
// impl Service for Shortener {
//     type Request = Request;
//     type Response = Response;
//     type Error = Error;
//     type Future = Box<dyn Future<Item = Self::Response, Error = Self::Error>>;
//     fn call(&self, req: Request) -> Self::Future {
// let mut db_connection: PgConnection = match connect_to_db() {
//     Some(connection) => connection,
//     None => {
//         return ""
//         // return Box::new(make_error_response(
//         //     "DB Connection Error!",
//         //     InternalServerError,
//         // ));
//     }
// };
//         match req.method() {
//             &Post => {
//                 let fut = req
//                     .body()
//                     .concat2()
//                     .and_then(parse_form) // checks if it's a valid form
//                     .and_then(move |long_url| write_to_db(long_url, &mut db_connection))
//                     .then(post_response);
//                 // after receiving request
//                 // add future to Heap memory
//                 Box::new(fut)
//             }
//             &Get => {
//                 let path: String = req.path().to_string(); // path with shortened url
//                 let validator: Result<ShortUrl, String> = validate_path(path);
//                 let response = match validator {
//                     Ok(ok_value) => {
//                         // valid short url,
//                         // query db
//                         // get_response(&ok_value)
//                         get_response(read_from_db(ok_value, &mut db_connection))
//                     }
//                     Err(err) => make_error_response(&err, NotFound),
//                 };
//                 Box::new(response)
//             }
//             err => Box::new(make_error_response(
//                 format!("{} is not a valid method!", err).as_str(),
//                 NotFound,
//             )),
//         }
//     }
// }

#[tokio::main]
async fn main() {
    // using
    // 'RUST_LOG="info" cargo run' to log events
    env_logger::init();

    // address used by the server
    let addr: SocketAddr = "0.0.0.0:8080".parse::<SocketAddr>().unwrap();

    // let mut db_connection: PgConnection = match connect_to_db() {
    //     Some(connection) => connection,
    //     None => {
    //         return "";
    //         // return Box::new(make_error_response(
    //         //     "DB Connection Error!",
    //         //     InternalServerError,
    //         // ));
    //     }
    // };

    let method_mapper = warp::method()
        .and(warp::body::bytes())
        // https://stackoverflow.com/questions/73303927/how-to-get-path-from-url-in-warp
        .and(warp::path::full())
        .map(move |method: Method, body: Bytes, path: FullPath| {
            let mut db_connection: Result<PgConnection, String> = match connect_to_db() {
                Some(connection) => Ok(connection),
                None => Err("unable to connect to db".to_string()),
            };

            match method {
                Method::POST => {
                    match parse_form(&body) {
                        Ok(parsed) => {
                            // recieved post form ok
                            post_response(write_to_db(parsed, &mut db_connection.unwrap()))
                        }
                        Err(err) => {
                            make_error_response(err.as_str(), StatusCode::INTERNAL_SERVER_ERROR)
                        }
                    }
                }
                Method::GET => {
                    match validate_path(path.as_str().to_string()) {
                        Ok(path) => {
                            // path is valid...
                            match db_connection {
                                Ok(mut conn) => get_response(read_from_db(path, &mut conn)),
                                Err(error) => make_error_response(
                                    error.as_str(),
                                    StatusCode::INTERNAL_SERVER_ERROR,
                                ),
                            }
                        }
                        Err(_) => make_error_response("Invalid path", StatusCode::NOT_ACCEPTABLE),
                    }

                    // match db_connection {
                    //     Ok(mut conn) => get_response(read_from_db(
                    //         ShortUrl {
                    //             short_url: "()".to_string(),
                    //         },
                    //         &mut conn,
                    //     )),
                    //     Err(error) => {
                    //         make_error_response(error.as_str(), StatusCode::INTERNAL_SERVER_ERROR)
                    //     }
                    // }
                }
                err => {
                    error!("{} is not a method", err);

                    make_error_response("METHOD not allowed", StatusCode::NOT_ACCEPTABLE)
                }
            }
        });

    warp::serve(method_mapper).run(addr).await;
    info!("running at {}", addr);
}
