mod api;
mod ui;

use warp::{path, reply::Response, Filter, Rejection, Reply};

use crate::db::Pool;

/// Routing table for API
pub fn builder(pool: Pool) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let pool_filter = warp::any().map(move || pool.get());

    // use path to redirect to corresponding url
    let api_get_short_link = warp::get()
        .and(warp::path::param())
        // the server will only accept non empty paths
        .and(warp::path::end())
        .and(pool_filter.clone())
        .and_then(api::read_from_db);

    // create new link
    let api_post_new_short_link = warp::post()
        .and(warp::body::content_length_limit(1024 * 16))
        .and(path!("api" / "create"))
        .and(warp::body::json())
        .and(pool_filter.clone())
        .and_then(api::create_link);

    // ui endpoints
    let serve_index = warp::get().and(warp::path::end()).and_then(ui::serve_index);

    let api_endpoints = api_get_short_link.or(api_post_new_short_link);

    serve_index.or(api_endpoints)
}
