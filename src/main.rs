#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;

mod db;
mod error;
mod jwt;
mod models;
mod routes;
mod schema;

#[macro_use]
extern crate log;
extern crate env_logger;
extern crate serde_json;

use dotenvy::dotenv;

use crate::db::{connect_to_db, Pool};
use crate::error::handle_rejection;
use std::env;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use warp::{Filter, Rejection};

pub fn db_url() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL not set")
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

    let routes = routes::builder(db_pool)
        .recover(handle_rejection)
        .boxed()
        .and(warp::addr::remote())
        .map(|routes, address: Option<SocketAddr>| {
            match address {
                Some(ok_address) => info!("Incoming Request from: {:?}", ok_address),
                None => error!("No SocketAddress Found"),
            }
            return routes;
        });

    // address used by the server
    let ip = env::var("SERVER_IP").unwrap_or("0.0.0.0".into());
    let port = env::var("SERVER_PORT").unwrap_or("3000".into());

    let address = SocketAddr::new(
        IpAddr::from_str(&ip).expect("expected valid IP address"),
        port.parse::<u16>().expect("expected valid SERVER_PORT"),
    );

    info!("running server at {} ", address);

    warp::serve(routes).bind(address).await;
    Ok(())
}
