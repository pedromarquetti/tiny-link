use hyper::{StatusCode, Uri};
use rand::{distributions::Alphanumeric, Rng};
use serde_json::json;
use url::{ParseError, Url};
use warp::{path::FullPath, Rejection, Reply};

use crate::{
    db::{connect_to_db, DbConnection, Link, Pool, R2D2Err, TinyLink},
    db_url,
    error::{convert_to_rejection, Error},
};
use diesel::prelude::*;

/// writes received long URL to db, returns short Url that will be echoed to user
pub async fn create_link(new_link: Link) -> Result<impl Reply, Rejection> {
    use crate::schema::tiny_link;

    // getting pool from connect_to_db
    let pool: Result<Pool, R2D2Err> = connect_to_db(db_url());

    // reject error if connect_to_db is R2D2Err
    if let Err(e) = pool {
        return Err(convert_to_rejection(e));
    }
    let pool = pool.unwrap();
    let mut conn = pool.get().unwrap();

    if let Err(e) = parse_form(&new_link.long_url) {
        return Err(e);
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
    diesel::insert_into(tiny_link::table)
        // inserting TinyLink with long + short url
        .values::<&TinyLink>(&payload)
        .returning(tiny_link::short_link)
        // .get_result(&mut connection)
        .execute(&mut conn)
        .map_err(convert_to_rejection)?;
    Ok(warp::reply::with_status(
        warp::reply::json(&json!({ "data": payload })),
        StatusCode::CREATED,
    ))
}

/// Queries db on GET request with 6-character id to find related long link
///
/// Tries t
pub async fn read_from_db(full_path: FullPath) -> Result<impl Reply, Rejection> {
    use crate::schema::tiny_link::{long_link, short_link, table};

    // getting pool from connect_to_db
    let pool: Result<Pool, R2D2Err> = connect_to_db(db_url());

    // reject error if connect_to_db is R2D2Err
    if let Err(e) = pool {
        return Err(convert_to_rejection(e));
    }
    let pool = pool.unwrap();
    let mut conn = pool.get().unwrap();

    let full_path = match valid_recvd_path(full_path.as_str().to_string()) {
        Err(_) => return Err(convert_to_rejection(Error::invalid_path())),
        Ok(full_path) => full_path,
    };

    let query = table
        .select(long_link) // get long link
        .filter(short_link.eq(full_path.as_str())) // where short_link == path
        .first::<String>(&mut conn)
        .map_err(convert_to_rejection)?;

    let payload: TinyLink = TinyLink {
        long_link: query,
        short_link: full_path,
    };
    let uri = payload.long_link.parse::<Uri>().unwrap();
    // Ok(Box::new(warp::redirect::temporary(uri)))
    Ok(Box::new(warp::redirect::redirect(uri)))
}

/// Used for GET Requests
///
/// Checks if specified path matches requirements
pub fn valid_recvd_path(mut path: String) -> Result<String, ()> {
    // removing '/' from the recvd path
    path.remove(0);
    if path.len() <= 5 {
        error!("Invalid Path! {}({})", &path, &path.len());
        return Err(());
    }
    Ok(path)
}

/// Checks if received data has valid
///
/// returns error if no "url" field is supplied or if Url::parse fails
pub fn parse_form(long_url: &str) -> Result<Uri, Rejection> {
    match long_url.parse::<Uri>() {
        Ok(url) => Ok(url),
        Err(err) => Err(convert_to_rejection(err)),
    }
}
