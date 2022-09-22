use futures::future::FutureResult;
use hyper::{
    header::{ContentLength, ContentType},
    server::Response,
    Error, StatusCode,
};
use serde_json::json;

/// Makes error Response based on suplied parameters
pub fn make_error_response(
    error_message: &str,
    error_code: StatusCode,
) -> FutureResult<Response, Error> {
    let payload: String = json!({ "error": error_message }).to_string();
    let response: Response = Response::new()
        .with_status(error_code)
        .with_header(ContentLength(payload.len() as u64))
        .with_header(ContentType::json())
        .with_body(payload);
    info!("ERROR> {:?}", response);
    futures::future::ok(response) // sending response
}
