use hyper::StatusCode;
use serde_json::{json, Value};
use warp::reply::{Json, WithStatus};

/// Returns error Response based on suplied parameters
pub fn make_error_response(error_message: &str, status_code: StatusCode) -> WithStatus<Json> {
    let payload: Value = json!({ "error": error_message });

    error!("ERROR {:?}", payload);
    warp::reply::with_status(warp::reply::json::<Value>(&payload), status_code)
}
