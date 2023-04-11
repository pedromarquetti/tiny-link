mod api;
mod ui;

use warp::{Filter, Rejection, Reply};

use crate::db::Pool;

/// Routing table for API
pub fn builder(pool: Pool) -> impl Filter<Extract = impl Reply, Error = Rejection> {
    let pool_filter = warp::any().map(move || pool.get());

    let api_get_short_link = warp::get()
        .and(warp::path::full())
        .and(pool_filter.clone())
        .and_then(api::read_from_db);

    let api_post_new_short_link = warp::post()
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and(pool_filter.clone())
        .and_then(api::create_link);

    let api_endpoints = api_get_short_link.or(api_post_new_short_link);

    api_endpoints
}
