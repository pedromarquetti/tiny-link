#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;

mod models;
mod schema;

mod db;
mod error;
mod routes;

#[macro_use]
extern crate log;
extern crate env_logger;
extern crate serde_json;

use dotenvy::dotenv;

use crate::db::{connect_to_db, Pool};
use crate::error::handle_rejection;
use std::env;
use std::net::SocketAddr;
use warp::{Filter, Rejection};

const DEFAULT_DATABASE_URL: &'static str = "postgresql://postgres@localhost:5432";

pub fn db_url() -> String {
    env::var("DATABASE_URL").unwrap_or(String::from(DEFAULT_DATABASE_URL))
}

#[tokio::main]
async fn main() -> Result<(), Rejection> {
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
    let db_pool: Pool = connect_to_db(db_url())?;

    let routes = routes::builder(db_pool).recover(handle_rejection).boxed();

    // address used by the server
    let address: SocketAddr = "0.0.0.0:3000".parse::<SocketAddr>().unwrap();

    info!("running server at {} ", address);

    warp::serve(routes).bind(address).await;
    Ok(())
}
