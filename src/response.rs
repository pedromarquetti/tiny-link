use crate::{error::make_error_response, structs::TinyLink};
use hyper::{Error, StatusCode};

use diesel::result::Error as db_err;
use serde_json::{json, Value};
use warp::reply::{Json, WithStatus};

/// Send back Short URL to user
///
/// If Url parse failed returns make_error_response()
pub fn post_response(res: Result<String, db_err>) -> WithStatus<warp::reply::Json> {
    match res {
        Ok(success_result) => {
            let payload: Value = json!({ "url": success_result });

            info!("Success POST Response sent: {:?}", payload);

            // sending response
            warp::reply::with_status(warp::reply::json::<Value>(&payload), StatusCode::OK)
            // warp::reply::json::<Value>(&payload)
        }
        Err(error) => {
            // failed writing to db
            make_error_response(
                format!("{}", error).as_str(),
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        }
    }
}

/// Sends GET response with Long URL
pub fn get_response(path: Option<TinyLink>) -> WithStatus<warp::reply::Json> {
    match path {
        Some(path) => {
            let payload: Value = json!(
                { "short_link": path.short_link,"long_link":path.long_link }
            );
            info!("Success Response sent: {:?}", payload);

            // sending response
            warp::reply::with_status(warp::reply::json::<Value>(&payload), StatusCode::OK)
        }
        None => make_error_response(
            "Specified path not found on DB!",
            StatusCode::INTERNAL_SERVER_ERROR,
        ),
    }
}
