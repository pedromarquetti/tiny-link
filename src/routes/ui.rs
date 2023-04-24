use hyper::{http::HeaderValue, StatusCode};
use rust_embed::RustEmbed;
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
    let file_ext = mime_guess::from_path(file);
    let (assets, file_ext) = if let Some(a) = Assets::get(file) {
        (a, file_ext.first_or_text_plain())
    } else {
        let a = Assets::get("404.html").ok_or_else(|| {
            convert_to_rejection(Error::custom("Could not find file", StatusCode::NOT_FOUND))
        })?;
        (a, file_ext.first_or_text_plain())
    };

    let mut response = Response::new(assets.data.into());
    response.headers_mut().insert(
        "Content-Type",
        HeaderValue::from_str(file_ext.as_ref()).unwrap(),
    );
    Ok(response)
}
