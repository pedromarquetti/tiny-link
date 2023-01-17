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
use crate::parser::{parse_form, validate_path};
use crate::response::{get_response, post_response};

use hyper::body::Bytes;
use warp::cors::Builder;
use warp::path::FullPath;

#[macro_use]
extern crate log;
extern crate env_logger;
extern crate serde_json;

use diesel::{pg::PgConnection, Connection};
use dotenvy::dotenv;
use futures::future;
use hyper::StatusCode;
use std::env;
use std::net::SocketAddr;
use warp::{http::Method, Filter};

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

#[tokio::main]
async fn main() {
    // using
    // 'RUST_LOG="info" cargo run' to log events
    // or
    // RUST_LOG="info" systemfd --no-pid -s http::3030 -- cargo watch -x 'run'
    // for autoreload

    env_logger::init();

    let cors: Builder = warp::cors().allow_methods(&[Method::GET, Method::POST]);

    // let get_req_filter = warp::path::full().map(async |path: FullPath| {});

    let backend_filter = warp::method()
        .and(warp::body::bytes())
        // https://stackoverflow.com/questions/73303927/how-to-get-path-from-url-in-warp
        .and(warp::path::full())
        .map(move |method: Method, body: Bytes, path: FullPath| {
            let db_connection: Result<PgConnection, String> = match connect_to_db() {
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
                        Err(invalid_path) => {
                            make_error_response(invalid_path.as_str(), StatusCode::NOT_ACCEPTABLE)
                        }
                    }
                }
                err => {
                    error!("'{}' is not a method", err);
                    make_error_response(
                        format!("'{}' is not a method", err).as_str(),
                        StatusCode::METHOD_NOT_ALLOWED,
                    )
                }
            }
        })
        .with(cors);

    // address used by the server
    let backend_addr: SocketAddr = "0.0.0.0:3000".parse::<SocketAddr>().unwrap();
    let backend_server = warp::serve(backend_filter).bind(backend_addr);

    let frontend_addr: SocketAddr = "0.0.0.0:3032".parse::<SocketAddr>().unwrap();
    let front_end_filters = warp::any().map(|| {
        info!("port 80 server");
        "oi"
    });
    let frontend_server = warp::serve(front_end_filters).bind(frontend_addr);

    future::join(backend_server, frontend_server).await;
}
