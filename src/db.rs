use crate::schema::tiny_link;
use crate::structs::{LongUrl, TinyLink};

use diesel::prelude::*;
use diesel::{pg::PgConnection, result::Error as db_err};
use futures::future::FutureResult;
use hyper::Error;
use std::io;

/// writes received long URL to db, returns short Url that will be echoed to user
pub fn write_to_db(
    recvd_long_url: String,
    db_connection: &PgConnection,
) -> FutureResult<String, Error> {
    // TODO:
    // 1. try to save recvd_long_url + short version
    // 2. return error if failed
    // 3. return short url

    let result: Result<String, db_err> = diesel::insert_into(tiny_link::table)
        // .values(&recvd_long_url)
        .values(&TinyLink {
            long_link: recvd_long_url,
            short_link: "teste".to_string(), // this has to be a short (6) random id
        })
        .returning(tiny_link::short_link)
        .get_result(db_connection);
    match result {
        Ok(shortened) => futures::future::ok(shortened), // all ok, sending back short url

        Err(error) => {
            // something happened, creating error
            error!("Error writing to database: {}", error.to_string());
            futures::future::err(hyper::Error::from(io::Error::new(
                io::ErrorKind::Other,
                format!("DB Error: {}", error),
            )))
        }
    }
}

pub fn read_from_db(path: &str) -> Option<&str> {
    Some(path)
}
