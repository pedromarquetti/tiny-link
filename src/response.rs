use crate::{error::make_error_response, structs::LongUrl};
use futures::future::FutureResult;
use hyper::{
    header::{ContentLength, ContentType},
    server::Response,
    Error, StatusCode,
};

use serde_json::json;

/// Send back response to user
pub fn post_response(res: Result<LongUrl, Error>) -> FutureResult<Response, Error> {
    match res {
        Ok(success_result) => {
            let payload: String = json!(
                { "url": success_result.long_url }
            )
            .to_string();
            let response: Response = Response::new()
                .with_header(ContentLength(payload.len() as u64))
                .with_header(ContentType::json())
                .with_body(payload);
            info!("Success Response sent: {:?}", response);

            futures::future::ok(response) // sending response
        }
        Err(error) => {
            let msg: String = error.to_string();

            match msg.as_str() {
                "Missing URL"
                | "No empty strings or spaces!"
                | "Could not parse URL, relative URL without a base"
                | "Could not parse URL, empty host"
                | "Could not parse URL, invalid domain character"
                | "Could not parse URL, invalid IPv4 address" => {
                    make_error_response(&msg, StatusCode::BadRequest)
                }
                err => make_error_response(
                    format!("Internal server Error! {}", err).as_str(),
                    StatusCode::InternalServerError,
                ),
            }
        }
    }
}
