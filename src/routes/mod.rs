pub(crate) mod api;
use crate::{
    db::Pool,
    error::{convert_to_rejection, Error},
};

use r2d2::Error as R2D2Err;
use warp::{Filter, Rejection, Reply};

/// Routing table for API
pub fn builder() -> impl Filter<Extract = impl Reply, Error = Rejection> {
    let api_get_short_link = warp::get()
        .and(warp::path::full())
        .and_then(api::read_from_db);

    let api_post_new_short_link = warp::post()
        .and(warp::body::json())
        .and_then(api::create_link);

    let api_endpoints = api_get_short_link.or(api_post_new_short_link);

    api_endpoints
}
