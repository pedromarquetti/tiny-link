use hyper::{StatusCode, Uri};
use rand::{distributions::Alphanumeric, Rng};
use serde_json::json;
use warp::{Rejection, Reply};

use crate::{
    db::{DbConnection, Link, TinyLink},
    error::{convert_to_rejection, Error},
    routes::ui,
};
use diesel::{prelude::*, result::Error as DieselError};

/// writes received long URL to db, returns short Url that will be echoed to user
pub async fn create_link(new_link: Link, ok_conn: DbConnection) -> Result<impl Reply, Rejection> {
    use crate::schema::tiny_link;

    let mut conn = ok_conn.map_err(convert_to_rejection)?;

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
        .execute(&mut conn)
        .map_err(convert_to_rejection)?;
    Ok(warp::reply::with_status(
        warp::reply::json(&json!({ "message": payload })),
        StatusCode::CREATED,
    ))
}

/// Queries db on GET request with 6-character id to find related long link
pub async fn redirect_to_link(
    recvd_path: String,
    ok_conn: DbConnection,
) -> Result<Box<dyn Reply>, Rejection> {
    use crate::schema::tiny_link::{long_link, short_link, table};
    let mime = mime_guess::from_path(&recvd_path);

    // received request has file
    if !mime.is_empty() {
        // serve file normally, don't treat it like a short-link id
        return Ok(Box::new(ui::serve_other(&recvd_path).await?));
    }

    // received path is not JS or CSS (or does not have known MIME)
    // user is probably trying to query db, continuing...
    let mut conn = ok_conn.map_err(convert_to_rejection)?;

    let current_path = match validate_path(recvd_path) {
        Err(_) => {
            return Ok(Box::new(ui::serve_other("404.html").await?));
        }
        Ok(full_path) => full_path,
    };

    let query = match table
        .select(long_link) // get long link
        .filter(short_link.eq(current_path.as_str())) // where short_link == path
        .first::<String>(&mut conn)
    {
        Ok(link) => link,
        Err(DieselError::NotFound) => {
            return Ok(Box::new(ui::serve_other("404.html").await?));
        }
        Err(err) => return Err(convert_to_rejection(err)),
    };

    let payload: TinyLink = TinyLink {
        long_link: query,
        short_link: current_path,
    };
    let uri = payload.long_link.parse::<Uri>().unwrap();
    Ok(Box::new(warp::redirect::redirect(uri)))
}

/// Used for GET Requests
///
/// Checks if specified path matches requirements
fn validate_path(path: String) -> Result<String, ()> {
    if path.len() <= 5 || path.len() >= 7 {
        return Err(());
    }
    Ok(path)
}

/// Checks if received data is valid, the server only accepts HTTP or HTTPS schemas
///
/// returns error if no "url" field is supplied or if Url::parse fails
fn parse_form(long_url: &str) -> Result<Uri, Rejection> {
    let parse = long_url.parse::<Uri>().map_err(convert_to_rejection)?;

    if parse.scheme_str() == Some("http") || parse.scheme_str() == Some("https") {
        return Ok(parse);
    }
    return Err(Error::invalid_forms("Form URI is not 'https' or 'http'").into());
}
