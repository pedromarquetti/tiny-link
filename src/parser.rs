use crate::structs::ShortUrl;
use hyper::body::Bytes;
use std::collections::HashMap;
use url::{form_urlencoded, ParseError, Url};

/// Checks if received data matches `LongUrl` using a HashMap
/// returns error if no "url" field is supplied or if Url::parse fails
pub fn parse_form(form_chunk: &Bytes) -> Result<String, String> {
    let mut form: HashMap<String, String> = form_urlencoded::parse(&form_chunk)
        .into_owned()
        .collect::<HashMap<String, String>>();
    if !form.contains_key("url") {
        Err("'url' key not found on received form".to_string())
    } else {
        let input: String = form.remove("url").unwrap();
        let url: Result<Url, ParseError> = Url::parse(&input);
        match url {
            Ok(url) => {
                info!("Parse Successful");
                Ok(url.to_string())
            }
            Err(err) => {
                // err while parsing form
                info!("Parse Error>> {:}", err.to_string());
                Err(format!("error: {}", err))
            }
        }
    }
}

/// Checks if specified path matches requirements
pub fn validate_path(mut path: String) -> Result<ShortUrl, String> {
    path.remove(0); // removing '/' from the recvd path
    if path.len() <= 5 {
        error!("Invalid Path! {}({})", &path, &path.len());
        Err("Invalid Path, the provided path must be 6 characters long".to_string())
    } else {
        Ok(ShortUrl {
            // sending back valid path
            short_url: path.to_string(),
        })
    }
}
