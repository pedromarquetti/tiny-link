use hyper::{http::HeaderValue, StatusCode};
use rust_embed::{EmbeddedFile, RustEmbed};
use warp::{reply::Response, Rejection, Reply};

use crate::error::{convert_to_rejection, Error};

#[derive(RustEmbed)]
#[folder = "ui"]
struct Assets;

pub async fn serve_index() -> Result<impl Reply, Rejection> {
    serve_file("index.html")
}

pub async fn serve_other(file: &str) -> Result<impl Reply, Rejection> {
    serve_file(file)
}

fn serve_file(file: &str) -> Result<impl Reply, Rejection> {
    let assets: EmbeddedFile = if let Some(a) = Assets::get(file) {
        a
    } else {
        let a = Assets::get("404.html").ok_or_else(|| {
            convert_to_rejection(Error::custom("Could not find file", StatusCode::NOT_FOUND))
        })?;
        a
    };

    let mut response = Response::new(assets.data.into());
    response
        .headers_mut()
        .insert("Content-Type", HeaderValue::from_str("text/html").unwrap());
    Ok(response)
}
