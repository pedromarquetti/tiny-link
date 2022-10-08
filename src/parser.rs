use crate::structs::ShortUrl;
use futures::future::FutureResult;
use hyper::{Chunk, Error};
use std::collections::HashMap;
use std::io;
use url::{form_urlencoded, ParseError, Url};

/// Checks if received data matches `LongUrl` using a HashMap
/// returns error if no "url" field is supplied or if Url::parse fails
pub fn parse_form(form_chunk: Chunk) -> FutureResult<String, Error> {
    let mut form: HashMap<String, String> = form_urlencoded::parse(form_chunk.as_ref())
        .into_owned()
        .collect::<HashMap<String, String>>();
    if !form.contains_key("url") {
        futures::future::err(Error::from(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Missing URL param",
        )))
    } else {
        let input: String = form.remove("url").unwrap();
        let url: Result<Url, ParseError> = Url::parse(&input);
        match url {
            Ok(url) => {
                info!("Parse Successful");
                futures::future::ok(url.to_string())
            }
            Err(err) => {
                info!("Parse Error>> {:}", err.to_string());
                futures::future::err(Error::from(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Could not parse URL, {}", err.to_string()),
                )))
            }
        }
    }
}

/// Checks if specified path matches requirements
pub fn validate_path(mut path: String) -> Result<ShortUrl, String> {
    path.remove(0); // removing '/' from the recvd path
    if path.len() <= 5 {
        error!("Invalid Path! {}({})", &path, &path.len());
        Err("Invalid Path".to_string())
    } else {
        Ok(ShortUrl {
            // sending back valid path
            short_url: path.to_string(),
        })
    }
}
