use diesel::prelude::*;
use diesel::r2d2::PooledConnection;
use diesel::{
    pg::PgConnection,
    prelude::*,
    r2d2::{ConnectionManager, Error as R2D2Err, Pool as R2D2Pool},
    result::Error as DBError,
};

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
pub fn connect_to_db(url: String) -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(url);

    Pool::builder()
        .build(manager)
        .expect("Error building connection pool")
    // Pool::new(manager).expect("Error building pool")
}
