mod api;
mod auth;
mod ui;
mod user;

use auth::auth;
use warp::{path, Filter, Rejection, Reply};

use crate::db::Pool;

/// Routing table for API
pub fn builder(pool: Pool) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let pool_filter = warp::any().map(move || pool.get());

    // user endpoints
    let create_user = warp::post()
        .and(path("api"))
        .and(path("user"))
        .and(path("create"))
        .and(path::end())
        .and(warp::body::content_length_limit(1024 * 10))
        .and(path!("api" / "user" / "create"))
        .and(warp::body::json())
        .and(pool_filter.clone())
        .and_then(user::create_user);

    // use path to redirect to corresponding url
    let api_redirect = warp::get()
        .and(warp::path::param())
        // the server will only accept non empty paths
        .and(warp::path::end())
        .and(pool_filter.clone())
        .and_then(api::redirect_to_link);

    // create new link
    let api_new_short_link = warp::post()
        .and(path("api"))
        .and(path("link"))
        .and(path("create"))
        .and(path::end())
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and(pool_filter.clone())
        .and_then(api::create_link);

    // ui endpoints
    let serve_index = warp::get().and(warp::path::end()).and_then(ui::serve_index);

    let api_endpoints = api_redirect.or(api_new_short_link);

    serve_index.or(api_endpoints)
}
