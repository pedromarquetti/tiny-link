pub(crate) mod api;
use crate::{
    db::{DbConnection, Pool},
    error::{convert_to_rejection, handle_rejection},
};
use std::convert::Infallible;

use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Error as R2D2Error, PooledConnection},
};
use warp::{path::FullPath, Filter, Rejection, Reply};

/// Routing table for API
pub fn builder(
    pool: Pool,
) -> impl Filter<Extract = impl Reply + 'static, Error = Rejection> + Clone {
    let pool_filter = warp::any().map(
        // creates filter that returns db pool
        // .unwrap() is used here, if the pool crashes, the server will panic
        move || pool.get().map_err(convert_to_rejection).unwrap(),
    );

    let api_get_long_link = warp::get()
        .and(warp::path::full())
        .and(pool_filter.clone())
        .and_then(api::read_from_db);

    let api_post_long_link = warp::post()
        .and(warp::path::full())
        .and(pool_filter.clone())
        .and_then(api::read_from_db);

    let api_endpoints = api_get_long_link.or(api_post_long_link);

    api_endpoints
}
