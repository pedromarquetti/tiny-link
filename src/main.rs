#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;

mod models;
mod schema;

use crate::db::connect_to_db;
use crate::error::handle_rejection;
mod db;
mod error;
mod routes;

#[macro_use]
extern crate log;
extern crate env_logger;
extern crate serde_json;

use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool},
};
use dotenvy::dotenv;

use std::env;
use std::net::SocketAddr;
use warp::Filter;

const DEFAULT_DATABASE_URL: &'static str = "postgresql://postgres@localhost:5432";

pub fn db_url() -> String {
    env::var("DATABASE_URL").unwrap_or(String::from(DEFAULT_DATABASE_URL))
}

#[tokio::main]
async fn main() {
    // using
    // cargo run' to log events
    // or
    // systemfd --no-pid -s http::3030 -- cargo watch -x 'run'
    // for autoreload
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "tiny_link")
    }

    env_logger::init(); // initializes pretty logger

    dotenv().ok(); // checks for .env file

    let pool: Pool<ConnectionManager<PgConnection>> = connect_to_db(db_url());

    let routes = routes::builder(pool).recover(handle_rejection).boxed();

    // address used by the server
    let backend_addr: SocketAddr = "0.0.0.0:3000".parse::<SocketAddr>().unwrap();

    info!("running server at {} ", backend_addr);

    warp::serve(routes).bind(backend_addr).await;
}
