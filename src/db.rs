use std::time::Duration;

use diesel::prelude::*;
use diesel::r2d2::PooledConnection;
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool as R2D2Pool},
};
use serde_derive::{Deserialize, Serialize};
use warp::Rejection;

pub type R2D2Err = r2d2::Error;
pub type Pool = R2D2Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = Result<PooledConnection<ConnectionManager<PgConnection>>, R2D2Err>;

use crate::error::convert_to_rejection;
use crate::schema::{tiny_link, users};

#[derive(Queryable, Serialize, Debug, Deserialize)]
/// This struct represents the long url the user wants to shorten
pub struct Link {
    pub long_url: String,
}
#[derive(Queryable, Insertable, Debug, Serialize)]
#[diesel(table_name = tiny_link)]
/// This struct represents the shortened link
pub struct TinyLink {
    pub long_link: String,
    pub short_link: String,
}

#[derive(Queryable, Insertable, Debug, Serialize, Deserialize)]
#[diesel(table_name = users)]
/// user_role has to be "admin", "user" or "guest"
pub struct User {
    pub user_name: String,
    pub user_role: String,
    pub user_pwd: String,
}

/// Generates new connection pool to db
pub fn connect_to_db(url: String) -> Result<Pool, Rejection> {
    let manager = ConnectionManager::<PgConnection>::new(url);

    // Pool::new(manager)
    Pool::builder()
        .connection_timeout(Duration::from_secs(1))
        .build(manager)
        .map_err(convert_to_rejection)
}
