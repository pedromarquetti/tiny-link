use hyper::{StatusCode, Uri};
use rand::{distributions::Alphanumeric, Rng};
use serde_json::json;
use warp::{path::FullPath, Rejection, Reply};

use crate::{
    db::{DbConnection, Link, TinyLink},
    error::{convert_to_rejection, Error},
};
use diesel::prelude::*;

/// writes received long URL to db, returns short Url that will be echoed to user
pub async fn create_link(new_link: Link, conn: &mut DbConnection) -> Result<impl Reply, Rejection> {
    use crate::schema::tiny_link;

    if let Err(e) = parse_form(&new_link.long_url) {
        return Err(convert_to_rejection(e));
    }

    // generating random String to be used as short url
    let rand: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect();
    let payload: TinyLink = TinyLink {
        long_link: new_link.long_url,
        short_link: rand,
    };
    let query = diesel::insert_into(tiny_link::table)
        // inserting TinyLink with long + short url
        .values::<&TinyLink>(&payload)
        .returning(tiny_link::short_link)
        // .get_result(&mut connection)
        .execute(conn)
        .map_err(convert_to_rejection)?;
    Ok(warp::reply::with_status(
        warp::reply::json(&json!({ "data": payload })),
        StatusCode::CREATED,
    ))
}

/// Queries db on GET request with 6-character id to find related long link
///
/// Tries t
pub async fn read_from_db(path: FullPath, conn: DbConnection) -> Result<impl Reply, Rejection> {
    use crate::schema::tiny_link::{long_link, short_link, table};

    let mut db_conn = conn;

    let path = path.as_str().to_string();
    if !valid_recvd_path(&path) {
        return Err(convert_to_rejection(Error::invalid_path()));
    }

    let query = table
        .select(long_link) // get long link
        .filter(short_link.eq(path.as_str())) // where short_link == path
        // .first::<String>(conn)
        .first::<String>(&mut db_conn)
        .map_err(convert_to_rejection)?;

    let payload: TinyLink = TinyLink {
        long_link: query,
        short_link: String::from(path.as_str()),
    };
    Ok(warp::reply::json(&json!({ "data": payload })))
}

/// Used for GET Requests
///
/// Checks if specified path matches requirements
pub fn valid_recvd_path(mut path: &str) -> bool {
    path.to_string().remove(0); // removing '/' from the recvd path
    if path.len() <= 5 {
        error!("Invalid Path! {}({})", &path, &path.len());
        return false;
    }
    true
}

/// Checks if received data has valid
///
/// returns error if no "url" field is supplied or if Url::parse fails
pub fn parse_form(long_url: &str) -> Result<(), Error> {
    let parsed = long_url
        .parse::<Uri>()
        .map_err(|_| Error::invalid_forms())?;
    parsed
        .scheme()
        .map(|_| ())
        .ok_or_else(|| Error::invalid_forms())
}
