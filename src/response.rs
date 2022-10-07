use crate::{
    error::make_error_response,
    structs::{LongUrl, ShortUrl, TinyLink},
};
use futures::future::FutureResult;
use hyper::{
    header::{ContentLength, ContentType},
    server::Response,
    Error,
    StatusCode::{BadRequest, InternalServerError},
};

use serde_json::json;

/// Send back Short URL to user
///
/// If Url parse failed returns make_error_response()
pub fn post_response(res: Result<String, Error>) -> FutureResult<Response, Error> {
    match res {
        Ok(success_result) => {
            let payload: String = json!({ "url": success_result }).to_string();
            let response: Response = Response::new()
                .with_header(ContentLength(payload.len() as u64))
                .with_header(ContentType::json())
                .with_body(payload);
            info!("Success Response sent: {:?}", response);

            futures::future::ok(response) // sending response
        }
        Err(error) => {
            let msg: String = error.to_string();
            make_error_response(msg.as_str(), BadRequest)
        }
    }
}

/// Sends GET response with Long URL
pub fn get_response(path: Option<TinyLink>) -> FutureResult<Response, Error> {
    match path {
        Some(path) => {
            let payload: String = json!(
                { "short_link": path.short_link,"long_link":path.long_link }
            )
            .to_string();

            let response: Response = Response::new()
                .with_header(ContentLength(payload.len() as u64))
                .with_header(ContentType::json())
                .with_body(payload);
            info!("Success Response sent: {:?}", response);

            futures::future::ok(response) // sending response
        }
        None => make_error_response("Fail on get_response()", InternalServerError),
    }
}
