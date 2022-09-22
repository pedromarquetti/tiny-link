use crate::structs::LongUrl;
use futures::future::FutureResult;
use hyper::{Chunk, Error};
use std::collections::HashMap;
use std::io;

use url::form_urlencoded;

/// Checks if received data matches `LongUrl` using a HashMap
/// returns error if no "url" field is supplied
pub fn parse_form(form_chunk: Chunk) -> FutureResult<LongUrl, Error> {
    let mut form: HashMap<String, String> = form_urlencoded::parse(form_chunk.as_ref())
        .into_owned()
        .collect::<HashMap<String, String>>();
    if !form.contains_key("url") {
        futures::future::err(Error::from(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Missing URL",
        )))
    } else {
        let cur_url = form.remove("url").unwrap();
        if cur_url.contains(" ") || cur_url == String::from("") {
            futures::future::err(Error::from(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid URL",
            )))
        } else {
            futures::future::ok(LongUrl { url: cur_url })
        }
    }
}
