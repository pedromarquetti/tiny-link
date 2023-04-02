use std::time::Duration;

use diesel::prelude::*;
use diesel::r2d2::PooledConnection;
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool as R2D2Pool},
};

pub type R2D2Err = r2d2::Error;
pub type Pool = R2D2Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;
// pub type DbConnection = Result<PooledConnection<ConnectionManager<PgConnection>>, R2D2Err>;

use crate::schema::tiny_link;

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

/// Generates new connection pool to db
pub fn connect_to_db(url: String) -> Result<Pool, R2D2Err> {
    let manager = ConnectionManager::<PgConnection>::new(url);

    // Pool::new(manager)
    Pool::builder()
        .connection_timeout(Duration::from_secs(1))
        .build(manager)
}
