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
            info!("Response sent: {:?}", response);

            futures::future::ok(response) // sending response
        }
        Err(error) => {
            let msg: String = error.to_string();

            match msg.as_str() {
                "Missing URL"
                | "No empty strings or spaces!"
                | "Could not parse URL, relative URL without a base"
                | "Could not parse URL, empty host" => {
                    make_error_response(&msg, StatusCode::BadRequest)
                }
                _ => make_error_response("Internal server Error!", StatusCode::InternalServerError),
            }
        }
    }
}
